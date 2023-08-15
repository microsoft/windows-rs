#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRegistryValueWithFallbackW<P0, P1, P2, P3, P4>(hkeyprimary: P0, pwszprimarysubkey: P1, hkeyfallback: P2, pwszfallbacksubkey: P3, pwszvalue: P4, dwflags: u32, pdwtype: ::core::option::Option<*mut u32>, pvdata: ::core::option::Option<*mut ::core::ffi::c_void>, cbdatain: u32, pcbdataout: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<HKEY>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("api-ms-win-core-state-helpers-l1-1-0.dll" "system" fn GetRegistryValueWithFallbackW(hkeyprimary : HKEY, pwszprimarysubkey : ::windows_core::PCWSTR, hkeyfallback : HKEY, pwszfallbacksubkey : ::windows_core::PCWSTR, pwszvalue : ::windows_core::PCWSTR, dwflags : u32, pdwtype : *mut u32, pvdata : *mut ::core::ffi::c_void, cbdatain : u32, pcbdataout : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    GetRegistryValueWithFallbackW(hkeyprimary.into_param().abi(), pwszprimarysubkey.into_param().abi(), hkeyfallback.into_param().abi(), pwszfallbacksubkey.into_param().abi(), pwszvalue.into_param().abi(), dwflags, ::core::mem::transmute(pdwtype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvdata.unwrap_or(::std::ptr::null_mut())), cbdatain, ::core::mem::transmute(pcbdataout.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCloseKey<P0>(hkey: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegCloseKey(hkey : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegCloseKey(hkey.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegConnectRegistryA<P0, P1>(lpmachinename: P0, hkey: P1, phkresult: *mut HKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegConnectRegistryA(lpmachinename : ::windows_core::PCSTR, hkey : HKEY, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegConnectRegistryA(lpmachinename.into_param().abi(), hkey.into_param().abi(), phkresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[inline]
pub unsafe fn RegConnectRegistryExA<P0, P1>(lpmachinename: P0, hkey: P1, flags: u32, phkresult: *mut HKEY) -> i32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegConnectRegistryExA(lpmachinename : ::windows_core::PCSTR, hkey : HKEY, flags : u32, phkresult : *mut HKEY) -> i32);
    RegConnectRegistryExA(lpmachinename.into_param().abi(), hkey.into_param().abi(), flags, phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
#[inline]
pub unsafe fn RegConnectRegistryExW<P0, P1>(lpmachinename: P0, hkey: P1, flags: u32, phkresult: *mut HKEY) -> i32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegConnectRegistryExW(lpmachinename : ::windows_core::PCWSTR, hkey : HKEY, flags : u32, phkresult : *mut HKEY) -> i32);
    RegConnectRegistryExW(lpmachinename.into_param().abi(), hkey.into_param().abi(), flags, phkresult)
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegConnectRegistryW<P0, P1>(lpmachinename: P0, hkey: P1, phkresult: *mut HKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegConnectRegistryW(lpmachinename : ::windows_core::PCWSTR, hkey : HKEY, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegConnectRegistryW(lpmachinename.into_param().abi(), hkey.into_param().abi(), phkresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCopyTreeA<P0, P1, P2>(hkeysrc: P0, lpsubkey: P1, hkeydest: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegCopyTreeA(hkeysrc : HKEY, lpsubkey : ::windows_core::PCSTR, hkeydest : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegCopyTreeA(hkeysrc.into_param().abi(), lpsubkey.into_param().abi(), hkeydest.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCopyTreeW<P0, P1, P2>(hkeysrc: P0, lpsubkey: P1, hkeydest: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegCopyTreeW(hkeysrc : HKEY, lpsubkey : ::windows_core::PCWSTR, hkeydest : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegCopyTreeW(hkeysrc.into_param().abi(), lpsubkey.into_param().abi(), hkeydest.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCreateKeyA<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi(), phkresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyExA<P0, P1, P2>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: ::core::option::Option<*mut REG_CREATE_KEY_DISPOSITION>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyExA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, reserved : u32, lpclass : ::windows_core::PCSTR, dwoptions : REG_OPEN_CREATE_OPTIONS, samdesired : REG_SAM_FLAGS, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyExA(hkey.into_param().abi(), lpsubkey.into_param().abi(), reserved, lpclass.into_param().abi(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), phkresult, ::core::mem::transmute(lpdwdisposition.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyExW<P0, P1, P2>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: ::core::option::Option<*mut REG_CREATE_KEY_DISPOSITION>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyExW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, reserved : u32, lpclass : ::windows_core::PCWSTR, dwoptions : REG_OPEN_CREATE_OPTIONS, samdesired : REG_SAM_FLAGS, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyExW(hkey.into_param().abi(), lpsubkey.into_param().abi(), reserved, lpclass.into_param().abi(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), phkresult, ::core::mem::transmute(lpdwdisposition.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyTransactedA<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: ::core::option::Option<*mut REG_CREATE_KEY_DISPOSITION>, htransaction: P3, pextendedparemeter: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyTransactedA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, reserved : u32, lpclass : ::windows_core::PCSTR, dwoptions : REG_OPEN_CREATE_OPTIONS, samdesired : REG_SAM_FLAGS, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION, htransaction : super::super::Foundation:: HANDLE, pextendedparemeter : *const ::core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyTransactedA(hkey.into_param().abi(), lpsubkey.into_param().abi(), reserved, lpclass.into_param().abi(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), phkresult, ::core::mem::transmute(lpdwdisposition.unwrap_or(::std::ptr::null_mut())), htransaction.into_param().abi(), ::core::mem::transmute(pextendedparemeter.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegCreateKeyTransactedW<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, reserved: u32, lpclass: P2, dwoptions: REG_OPEN_CREATE_OPTIONS, samdesired: REG_SAM_FLAGS, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut HKEY, lpdwdisposition: ::core::option::Option<*mut REG_CREATE_KEY_DISPOSITION>, htransaction: P3, pextendedparemeter: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyTransactedW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, reserved : u32, lpclass : ::windows_core::PCWSTR, dwoptions : REG_OPEN_CREATE_OPTIONS, samdesired : REG_SAM_FLAGS, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut REG_CREATE_KEY_DISPOSITION, htransaction : super::super::Foundation:: HANDLE, pextendedparemeter : *const ::core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyTransactedW(hkey.into_param().abi(), lpsubkey.into_param().abi(), reserved, lpclass.into_param().abi(), dwoptions, samdesired, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), phkresult, ::core::mem::transmute(lpdwdisposition.unwrap_or(::std::ptr::null_mut())), htransaction.into_param().abi(), ::core::mem::transmute(pextendedparemeter.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegCreateKeyW<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegCreateKeyW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegCreateKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi(), phkresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyA<P0, P1>(hkey: P0, lpsubkey: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyExA<P0, P1>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyExA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, samdesired : u32, reserved : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyExA(hkey.into_param().abi(), lpsubkey.into_param().abi(), samdesired, reserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyExW<P0, P1>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyExW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, samdesired : u32, reserved : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyExW(hkey.into_param().abi(), lpsubkey.into_param().abi(), samdesired, reserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyTransactedA<P0, P1, P2>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32, htransaction: P2, pextendedparameter: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyTransactedA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, samdesired : u32, reserved : u32, htransaction : super::super::Foundation:: HANDLE, pextendedparameter : *const ::core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyTransactedA(hkey.into_param().abi(), lpsubkey.into_param().abi(), samdesired, reserved, htransaction.into_param().abi(), ::core::mem::transmute(pextendedparameter.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyTransactedW<P0, P1, P2>(hkey: P0, lpsubkey: P1, samdesired: u32, reserved: u32, htransaction: P2, pextendedparameter: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyTransactedW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, samdesired : u32, reserved : u32, htransaction : super::super::Foundation:: HANDLE, pextendedparameter : *const ::core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyTransactedW(hkey.into_param().abi(), lpsubkey.into_param().abi(), samdesired, reserved, htransaction.into_param().abi(), ::core::mem::transmute(pextendedparameter.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyValueA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyValueA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, lpvaluename : ::windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyValueA(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvaluename.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyValueW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, lpvaluename : ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyValueW(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvaluename.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteKeyW<P0, P1>(hkey: P0, lpsubkey: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteKeyW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteTreeA<P0, P1>(hkey: P0, lpsubkey: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteTreeA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteTreeA(hkey.into_param().abi(), lpsubkey.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteTreeW<P0, P1>(hkey: P0, lpsubkey: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteTreeW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteTreeW(hkey.into_param().abi(), lpsubkey.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteValueA<P0, P1>(hkey: P0, lpvaluename: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteValueA(hkey : HKEY, lpvaluename : ::windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteValueA(hkey.into_param().abi(), lpvaluename.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDeleteValueW<P0, P1>(hkey: P0, lpvaluename: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDeleteValueW(hkey : HKEY, lpvaluename : ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegDeleteValueW(hkey.into_param().abi(), lpvaluename.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDisablePredefinedCache() -> ::windows_core::Result<()> {
    ::windows_targets::link!("advapi32.dll" "system" fn RegDisablePredefinedCache() -> super::super::Foundation:: WIN32_ERROR);
    RegDisablePredefinedCache().ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDisablePredefinedCacheEx() -> ::windows_core::Result<()> {
    ::windows_targets::link!("advapi32.dll" "system" fn RegDisablePredefinedCacheEx() -> super::super::Foundation:: WIN32_ERROR);
    RegDisablePredefinedCacheEx().ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegDisableReflectionKey<P0>(hbase: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegDisableReflectionKey(hbase : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegDisableReflectionKey(hbase.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnableReflectionKey<P0>(hbase: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegEnableReflectionKey(hbase : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegEnableReflectionKey(hbase.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyA<P0>(hkey: P0, dwindex: u32, lpname: ::core::option::Option<&mut [u8]>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegEnumKeyA(hkey : HKEY, dwindex : u32, lpname : ::windows_core::PSTR, cchname : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumKeyA(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len() as _)).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyExA<P0>(hkey: P0, dwindex: u32, lpname: ::windows_core::PSTR, lpcchname: *mut u32, lpreserved: ::core::option::Option<*const u32>, lpclass: ::windows_core::PSTR, lpcchclass: ::core::option::Option<*mut u32>, lpftlastwritetime: ::core::option::Option<*mut super::super::Foundation::FILETIME>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegEnumKeyExA(hkey : HKEY, dwindex : u32, lpname : ::windows_core::PSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : ::windows_core::PSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumKeyExA(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpname), lpcchname, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpclass), ::core::mem::transmute(lpcchclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpftlastwritetime.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyExW<P0>(hkey: P0, dwindex: u32, lpname: ::windows_core::PWSTR, lpcchname: *mut u32, lpreserved: ::core::option::Option<*const u32>, lpclass: ::windows_core::PWSTR, lpcchclass: ::core::option::Option<*mut u32>, lpftlastwritetime: ::core::option::Option<*mut super::super::Foundation::FILETIME>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegEnumKeyExW(hkey : HKEY, dwindex : u32, lpname : ::windows_core::PWSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : ::windows_core::PWSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumKeyExW(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpname), lpcchname, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lpclass), ::core::mem::transmute(lpcchclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpftlastwritetime.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumKeyW<P0>(hkey: P0, dwindex: u32, lpname: ::core::option::Option<&mut [u16]>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegEnumKeyW(hkey : HKEY, dwindex : u32, lpname : ::windows_core::PWSTR, cchname : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumKeyW(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len() as _)).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumValueA<P0>(hkey: P0, dwindex: u32, lpvaluename: ::windows_core::PSTR, lpcchvaluename: *mut u32, lpreserved: ::core::option::Option<*const u32>, lptype: ::core::option::Option<*mut u32>, lpdata: ::core::option::Option<*mut u8>, lpcbdata: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegEnumValueA(hkey : HKEY, dwindex : u32, lpvaluename : ::windows_core::PSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumValueA(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpvaluename), lpcchvaluename, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegEnumValueW<P0>(hkey: P0, dwindex: u32, lpvaluename: ::windows_core::PWSTR, lpcchvaluename: *mut u32, lpreserved: ::core::option::Option<*const u32>, lptype: ::core::option::Option<*mut u32>, lpdata: ::core::option::Option<*mut u8>, lpcbdata: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegEnumValueW(hkey : HKEY, dwindex : u32, lpvaluename : ::windows_core::PWSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegEnumValueW(hkey.into_param().abi(), dwindex, ::core::mem::transmute(lpvaluename), lpcchvaluename, ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegFlushKey<P0>(hkey: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegFlushKey(hkey : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegFlushKey(hkey.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegGetKeySecurity<P0>(hkey: P0, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegGetKeySecurity(hkey : HKEY, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegGetKeySecurity(hkey.into_param().abi(), securityinformation, psecuritydescriptor, lpcbsecuritydescriptor).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegGetValueA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvalue: P2, dwflags: REG_ROUTINE_FLAGS, pdwtype: ::core::option::Option<*mut REG_VALUE_TYPE>, pvdata: ::core::option::Option<*mut ::core::ffi::c_void>, pcbdata: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegGetValueA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, lpvalue : ::windows_core::PCSTR, dwflags : REG_ROUTINE_FLAGS, pdwtype : *mut REG_VALUE_TYPE, pvdata : *mut ::core::ffi::c_void, pcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegGetValueA(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvalue.into_param().abi(), dwflags, ::core::mem::transmute(pdwtype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbdata.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegGetValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvalue: P2, dwflags: REG_ROUTINE_FLAGS, pdwtype: ::core::option::Option<*mut REG_VALUE_TYPE>, pvdata: ::core::option::Option<*mut ::core::ffi::c_void>, pcbdata: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegGetValueW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, lpvalue : ::windows_core::PCWSTR, dwflags : REG_ROUTINE_FLAGS, pdwtype : *mut REG_VALUE_TYPE, pvdata : *mut ::core::ffi::c_void, pcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegGetValueW(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvalue.into_param().abi(), dwflags, ::core::mem::transmute(pdwtype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbdata.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadAppKeyA<P0>(lpfile: P0, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegLoadAppKeyA(lpfile : ::windows_core::PCSTR, phkresult : *mut HKEY, samdesired : u32, dwoptions : u32, reserved : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadAppKeyA(lpfile.into_param().abi(), phkresult, samdesired, dwoptions, reserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadAppKeyW<P0>(lpfile: P0, phkresult: *mut HKEY, samdesired: u32, dwoptions: u32, reserved: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegLoadAppKeyW(lpfile : ::windows_core::PCWSTR, phkresult : *mut HKEY, samdesired : u32, dwoptions : u32, reserved : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadAppKeyW(lpfile.into_param().abi(), phkresult, samdesired, dwoptions, reserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadKeyA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpfile: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegLoadKeyA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, lpfile : ::windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpfile.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadKeyW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpfile: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegLoadKeyW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, lpfile : ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpfile.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadMUIStringA<P0, P1, P2>(hkey: P0, pszvalue: P1, pszoutbuf: ::core::option::Option<&mut [u8]>, pcbdata: ::core::option::Option<*mut u32>, flags: u32, pszdirectory: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegLoadMUIStringA(hkey : HKEY, pszvalue : ::windows_core::PCSTR, pszoutbuf : ::windows_core::PSTR, cboutbuf : u32, pcbdata : *mut u32, flags : u32, pszdirectory : ::windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadMUIStringA(hkey.into_param().abi(), pszvalue.into_param().abi(), ::core::mem::transmute(pszoutbuf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszoutbuf.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pcbdata.unwrap_or(::std::ptr::null_mut())), flags, pszdirectory.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegLoadMUIStringW<P0, P1, P2>(hkey: P0, pszvalue: P1, pszoutbuf: ::windows_core::PWSTR, cboutbuf: u32, pcbdata: ::core::option::Option<*mut u32>, flags: u32, pszdirectory: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegLoadMUIStringW(hkey : HKEY, pszvalue : ::windows_core::PCWSTR, pszoutbuf : ::windows_core::PWSTR, cboutbuf : u32, pcbdata : *mut u32, flags : u32, pszdirectory : ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegLoadMUIStringW(hkey.into_param().abi(), pszvalue.into_param().abi(), ::core::mem::transmute(pszoutbuf), cboutbuf, ::core::mem::transmute(pcbdata.unwrap_or(::std::ptr::null_mut())), flags, pszdirectory.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegNotifyChangeKeyValue<P0, P1, P2, P3>(hkey: P0, bwatchsubtree: P1, dwnotifyfilter: REG_NOTIFY_FILTER, hevent: P2, fasynchronous: P3) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegNotifyChangeKeyValue(hkey : HKEY, bwatchsubtree : super::super::Foundation:: BOOL, dwnotifyfilter : REG_NOTIFY_FILTER, hevent : super::super::Foundation:: HANDLE, fasynchronous : super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
    RegNotifyChangeKeyValue(hkey.into_param().abi(), bwatchsubtree.into_param().abi(), dwnotifyfilter, hevent.into_param().abi(), fasynchronous.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenCurrentUser(samdesired: u32, phkresult: *mut HKEY) -> ::windows_core::Result<()> {
    ::windows_targets::link!("advapi32.dll" "system" fn RegOpenCurrentUser(samdesired : u32, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenCurrentUser(samdesired, phkresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyA<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi(), phkresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyExA<P0, P1>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyExA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, uloptions : u32, samdesired : REG_SAM_FLAGS, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyExA(hkey.into_param().abi(), lpsubkey.into_param().abi(), uloptions, samdesired, phkresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyExW<P0, P1>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyExW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, uloptions : u32, samdesired : REG_SAM_FLAGS, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyExW(hkey.into_param().abi(), lpsubkey.into_param().abi(), uloptions, samdesired, phkresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyTransactedA<P0, P1, P2>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: P2, pextendedparemeter: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyTransactedA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, uloptions : u32, samdesired : REG_SAM_FLAGS, phkresult : *mut HKEY, htransaction : super::super::Foundation:: HANDLE, pextendedparemeter : *const ::core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyTransactedA(hkey.into_param().abi(), lpsubkey.into_param().abi(), uloptions, samdesired, phkresult, htransaction.into_param().abi(), ::core::mem::transmute(pextendedparemeter.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyTransactedW<P0, P1, P2>(hkey: P0, lpsubkey: P1, uloptions: u32, samdesired: REG_SAM_FLAGS, phkresult: *mut HKEY, htransaction: P2, pextendedparemeter: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyTransactedW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, uloptions : u32, samdesired : REG_SAM_FLAGS, phkresult : *mut HKEY, htransaction : super::super::Foundation:: HANDLE, pextendedparemeter : *const ::core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyTransactedW(hkey.into_param().abi(), lpsubkey.into_param().abi(), uloptions, samdesired, phkresult, htransaction.into_param().abi(), ::core::mem::transmute(pextendedparemeter.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenKeyW<P0, P1>(hkey: P0, lpsubkey: P1, phkresult: *mut HKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegOpenKeyW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi(), phkresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOpenUserClassesRoot<P0>(htoken: P0, dwoptions: u32, samdesired: u32, phkresult: *mut HKEY) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegOpenUserClassesRoot(htoken : super::super::Foundation:: HANDLE, dwoptions : u32, samdesired : u32, phkresult : *mut HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOpenUserClassesRoot(htoken.into_param().abi(), dwoptions, samdesired, phkresult).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegOverridePredefKey<P0, P1>(hkey: P0, hnewhkey: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegOverridePredefKey(hkey : HKEY, hnewhkey : HKEY) -> super::super::Foundation:: WIN32_ERROR);
    RegOverridePredefKey(hkey.into_param().abi(), hnewhkey.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryInfoKeyA<P0>(hkey: P0, lpclass: ::windows_core::PSTR, lpcchclass: ::core::option::Option<*mut u32>, lpreserved: ::core::option::Option<*const u32>, lpcsubkeys: ::core::option::Option<*mut u32>, lpcbmaxsubkeylen: ::core::option::Option<*mut u32>, lpcbmaxclasslen: ::core::option::Option<*mut u32>, lpcvalues: ::core::option::Option<*mut u32>, lpcbmaxvaluenamelen: ::core::option::Option<*mut u32>, lpcbmaxvaluelen: ::core::option::Option<*mut u32>, lpcbsecuritydescriptor: ::core::option::Option<*mut u32>, lpftlastwritetime: ::core::option::Option<*mut super::super::Foundation::FILETIME>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegQueryInfoKeyA(hkey : HKEY, lpclass : ::windows_core::PSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: WIN32_ERROR);
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
    .ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryInfoKeyW<P0>(hkey: P0, lpclass: ::windows_core::PWSTR, lpcchclass: ::core::option::Option<*mut u32>, lpreserved: ::core::option::Option<*const u32>, lpcsubkeys: ::core::option::Option<*mut u32>, lpcbmaxsubkeylen: ::core::option::Option<*mut u32>, lpcbmaxclasslen: ::core::option::Option<*mut u32>, lpcvalues: ::core::option::Option<*mut u32>, lpcbmaxvaluenamelen: ::core::option::Option<*mut u32>, lpcbmaxvaluelen: ::core::option::Option<*mut u32>, lpcbsecuritydescriptor: ::core::option::Option<*mut u32>, lpftlastwritetime: ::core::option::Option<*mut super::super::Foundation::FILETIME>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegQueryInfoKeyW(hkey : HKEY, lpclass : ::windows_core::PWSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: WIN32_ERROR);
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
    .ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryMultipleValuesA<P0>(hkey: P0, val_list: &mut [VALENTA], lpvaluebuf: ::windows_core::PSTR, ldwtotsize: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegQueryMultipleValuesA(hkey : HKEY, val_list : *mut VALENTA, num_vals : u32, lpvaluebuf : ::windows_core::PSTR, ldwtotsize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryMultipleValuesA(hkey.into_param().abi(), ::core::mem::transmute(val_list.as_ptr()), val_list.len() as _, ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(ldwtotsize.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryMultipleValuesW<P0>(hkey: P0, val_list: &mut [VALENTW], lpvaluebuf: ::windows_core::PWSTR, ldwtotsize: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegQueryMultipleValuesW(hkey : HKEY, val_list : *mut VALENTW, num_vals : u32, lpvaluebuf : ::windows_core::PWSTR, ldwtotsize : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryMultipleValuesW(hkey.into_param().abi(), ::core::mem::transmute(val_list.as_ptr()), val_list.len() as _, ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(ldwtotsize.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryReflectionKey<P0>(hbase: P0, bisreflectiondisabled: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegQueryReflectionKey(hbase : HKEY, bisreflectiondisabled : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryReflectionKey(hbase.into_param().abi(), bisreflectiondisabled).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueA<P0, P1>(hkey: P0, lpsubkey: P1, lpdata: ::windows_core::PSTR, lpcbdata: ::core::option::Option<*mut i32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegQueryValueA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, lpdata : ::windows_core::PSTR, lpcbdata : *mut i32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryValueA(hkey.into_param().abi(), lpsubkey.into_param().abi(), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueExA<P0, P1>(hkey: P0, lpvaluename: P1, lpreserved: ::core::option::Option<*const u32>, lptype: ::core::option::Option<*mut REG_VALUE_TYPE>, lpdata: ::core::option::Option<*mut u8>, lpcbdata: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegQueryValueExA(hkey : HKEY, lpvaluename : ::windows_core::PCSTR, lpreserved : *const u32, lptype : *mut REG_VALUE_TYPE, lpdata : *mut u8, lpcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryValueExA(hkey.into_param().abi(), lpvaluename.into_param().abi(), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueExW<P0, P1>(hkey: P0, lpvaluename: P1, lpreserved: ::core::option::Option<*const u32>, lptype: ::core::option::Option<*mut REG_VALUE_TYPE>, lpdata: ::core::option::Option<*mut u8>, lpcbdata: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegQueryValueExW(hkey : HKEY, lpvaluename : ::windows_core::PCWSTR, lpreserved : *const u32, lptype : *mut REG_VALUE_TYPE, lpdata : *mut u8, lpcbdata : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryValueExW(hkey.into_param().abi(), lpvaluename.into_param().abi(), ::core::mem::transmute(lpreserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegQueryValueW<P0, P1>(hkey: P0, lpsubkey: P1, lpdata: ::windows_core::PWSTR, lpcbdata: ::core::option::Option<*mut i32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegQueryValueW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, lpdata : ::windows_core::PWSTR, lpcbdata : *mut i32) -> super::super::Foundation:: WIN32_ERROR);
    RegQueryValueW(hkey.into_param().abi(), lpsubkey.into_param().abi(), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRenameKey<P0, P1, P2>(hkey: P0, lpsubkeyname: P1, lpnewkeyname: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegRenameKey(hkey : HKEY, lpsubkeyname : ::windows_core::PCWSTR, lpnewkeyname : ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegRenameKey(hkey.into_param().abi(), lpsubkeyname.into_param().abi(), lpnewkeyname.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegReplaceKeyA<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, lpnewfile: P2, lpoldfile: P3) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegReplaceKeyA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, lpnewfile : ::windows_core::PCSTR, lpoldfile : ::windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegReplaceKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpnewfile.into_param().abi(), lpoldfile.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegReplaceKeyW<P0, P1, P2, P3>(hkey: P0, lpsubkey: P1, lpnewfile: P2, lpoldfile: P3) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegReplaceKeyW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, lpnewfile : ::windows_core::PCWSTR, lpoldfile : ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegReplaceKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpnewfile.into_param().abi(), lpoldfile.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRestoreKeyA<P0, P1>(hkey: P0, lpfile: P1, dwflags: REG_RESTORE_KEY_FLAGS) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegRestoreKeyA(hkey : HKEY, lpfile : ::windows_core::PCSTR, dwflags : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegRestoreKeyA(hkey.into_param().abi(), lpfile.into_param().abi(), dwflags.0 as _).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegRestoreKeyW<P0, P1>(hkey: P0, lpfile: P1, dwflags: REG_RESTORE_KEY_FLAGS) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegRestoreKeyW(hkey : HKEY, lpfile : ::windows_core::PCWSTR, dwflags : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegRestoreKeyW(hkey.into_param().abi(), lpfile.into_param().abi(), dwflags.0 as _).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyA<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegSaveKeyA(hkey : HKEY, lpfile : ::windows_core::PCSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: WIN32_ERROR);
    RegSaveKeyA(hkey.into_param().abi(), lpfile.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyExA<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flags: REG_SAVE_FORMAT) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegSaveKeyExA(hkey : HKEY, lpfile : ::windows_core::PCSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flags : REG_SAVE_FORMAT) -> super::super::Foundation:: WIN32_ERROR);
    RegSaveKeyExA(hkey.into_param().abi(), lpfile.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), flags).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyExW<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flags: REG_SAVE_FORMAT) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegSaveKeyExW(hkey : HKEY, lpfile : ::windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flags : REG_SAVE_FORMAT) -> super::super::Foundation:: WIN32_ERROR);
    RegSaveKeyExW(hkey.into_param().abi(), lpfile.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())), flags).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSaveKeyW<P0, P1>(hkey: P0, lpfile: P1, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegSaveKeyW(hkey : HKEY, lpfile : ::windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: WIN32_ERROR);
    RegSaveKeyW(hkey.into_param().abi(), lpfile.into_param().abi(), ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RegSetKeySecurity<P0, P1>(hkey: P0, securityinformation: u32, psecuritydescriptor: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegSetKeySecurity(hkey : HKEY, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: WIN32_ERROR);
    RegSetKeySecurity(hkey.into_param().abi(), securityinformation, psecuritydescriptor.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetKeyValueA<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2, dwtype: u32, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, cbdata: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegSetKeyValueA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, lpvaluename : ::windows_core::PCSTR, dwtype : u32, lpdata : *const ::core::ffi::c_void, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetKeyValueA(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvaluename.into_param().abi(), dwtype, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), cbdata).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetKeyValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, lpvaluename: P2, dwtype: u32, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, cbdata: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegSetKeyValueW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, lpvaluename : ::windows_core::PCWSTR, dwtype : u32, lpdata : *const ::core::ffi::c_void, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetKeyValueW(hkey.into_param().abi(), lpsubkey.into_param().abi(), lpvaluename.into_param().abi(), dwtype, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), cbdata).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueA<P0, P1>(hkey: P0, lpsubkey: P1, dwtype: REG_VALUE_TYPE, lpdata: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegSetValueA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR, dwtype : REG_VALUE_TYPE, lpdata : ::windows_core::PCSTR, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetValueA(hkey.into_param().abi(), lpsubkey.into_param().abi(), dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _)).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueExA<P0, P1>(hkey: P0, lpvaluename: P1, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegSetValueExA(hkey : HKEY, lpvaluename : ::windows_core::PCSTR, reserved : u32, dwtype : REG_VALUE_TYPE, lpdata : *const u8, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetValueExA(hkey.into_param().abi(), lpvaluename.into_param().abi(), reserved, dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _)).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueExW<P0, P1>(hkey: P0, lpvaluename: P1, reserved: u32, dwtype: REG_VALUE_TYPE, lpdata: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegSetValueExW(hkey : HKEY, lpvaluename : ::windows_core::PCWSTR, reserved : u32, dwtype : REG_VALUE_TYPE, lpdata : *const u8, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetValueExW(hkey.into_param().abi(), lpvaluename.into_param().abi(), reserved, dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _)).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegSetValueW<P0, P1, P2>(hkey: P0, lpsubkey: P1, dwtype: REG_VALUE_TYPE, lpdata: P2, cbdata: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegSetValueW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR, dwtype : REG_VALUE_TYPE, lpdata : ::windows_core::PCWSTR, cbdata : u32) -> super::super::Foundation:: WIN32_ERROR);
    RegSetValueW(hkey.into_param().abi(), lpsubkey.into_param().abi(), dwtype, lpdata.into_param().abi(), cbdata).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegUnLoadKeyA<P0, P1>(hkey: P0, lpsubkey: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegUnLoadKeyA(hkey : HKEY, lpsubkey : ::windows_core::PCSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegUnLoadKeyA(hkey.into_param().abi(), lpsubkey.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegUnLoadKeyW<P0, P1>(hkey: P0, lpsubkey: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn RegUnLoadKeyW(hkey : HKEY, lpsubkey : ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RegUnLoadKeyW(hkey.into_param().abi(), lpsubkey.into_param().abi()).ok()
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
pub const KEY_ALL_ACCESS: REG_SAM_FLAGS = REG_SAM_FLAGS(983103u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_CREATE_LINK: REG_SAM_FLAGS = REG_SAM_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_CREATE_SUB_KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_ENUMERATE_SUB_KEYS: REG_SAM_FLAGS = REG_SAM_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_EXECUTE: REG_SAM_FLAGS = REG_SAM_FLAGS(131097u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_NOTIFY: REG_SAM_FLAGS = REG_SAM_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_QUERY_VALUE: REG_SAM_FLAGS = REG_SAM_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_READ: REG_SAM_FLAGS = REG_SAM_FLAGS(131097u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_SET_VALUE: REG_SAM_FLAGS = REG_SAM_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_WOW64_32KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_WOW64_64KEY: REG_SAM_FLAGS = REG_SAM_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_WOW64_RES: REG_SAM_FLAGS = REG_SAM_FLAGS(768u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const KEY_WRITE: REG_SAM_FLAGS = REG_SAM_FLAGS(131078u32);
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
pub const REGSTR_DATA_NETOS_IPX: ::windows_core::PCWSTR = ::windows_core::w!("IPX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_DATA_NETOS_NDIS: ::windows_core::PCWSTR = ::windows_core::w!("NDIS");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_DATA_NETOS_ODI: ::windows_core::PCWSTR = ::windows_core::w!("ODI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_DEFAULT_INSTANCE: ::windows_core::PCWSTR = ::windows_core::w!("0000");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_ACPIENUM: ::windows_core::PCWSTR = ::windows_core::w!("ACPI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_APM: ::windows_core::PCWSTR = ::windows_core::w!("*PNP0C05");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_BIOSENUM: ::windows_core::PCWSTR = ::windows_core::w!("BIOS");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("Class");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CONFIG: ::windows_core::PCWSTR = ::windows_core::w!("Config");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CONTROL: ::windows_core::PCWSTR = ::windows_core::w!("Control");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CRASHES: ::windows_core::PCWSTR = ::windows_core::w!("Crashes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CURRENT: ::windows_core::PCWSTR = ::windows_core::w!("Current");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_CURRENT_ENV: ::windows_core::PCWSTR = ::windows_core::w!("\\Windows 4.0");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DANGERS: ::windows_core::PCWSTR = ::windows_core::w!("Dangers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DEFAULT: ::windows_core::PCWSTR = ::windows_core::w!("Default");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DETMODVARS: ::windows_core::PCWSTR = ::windows_core::w!("DetModVars");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DEVICEPARAMETERS: ::windows_core::PCWSTR = ::windows_core::w!("Device Parameters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DEVICE_PROPERTIES: ::windows_core::PCWSTR = ::windows_core::w!("Properties");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DISPLAY_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("Display");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DOSOPTCDROM: ::windows_core::PCWSTR = ::windows_core::w!("CD-ROM");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DOSOPTMOUSE: ::windows_core::PCWSTR = ::windows_core::w!("MOUSE");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DRIVERPARAMETERS: ::windows_core::PCWSTR = ::windows_core::w!("Driver Parameters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_DRIVERS: ::windows_core::PCWSTR = ::windows_core::w!("\\Drivers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDAUTOEXECBATKEYBOARD: ::windows_core::PCWSTR = ::windows_core::w!("EBDAutoexecBatKeyboard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDAUTOEXECBATLOCAL: ::windows_core::PCWSTR = ::windows_core::w!("EBDAutoexecBatLocale");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDCONFIGSYSKEYBOARD: ::windows_core::PCWSTR = ::windows_core::w!("EBDConfigSysKeyboard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDCONFIGSYSLOCAL: ::windows_core::PCWSTR = ::windows_core::w!("EBDConfigSysLocale");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDFILESKEYBOARD: ::windows_core::PCWSTR = ::windows_core::w!("EBDFilesKeyboard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EBDFILESLOCAL: ::windows_core::PCWSTR = ::windows_core::w!("EBDFilesLocale");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EISAENUM: ::windows_core::PCWSTR = ::windows_core::w!("EISA");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_ENUM: ::windows_core::PCWSTR = ::windows_core::w!("Enum");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_EXPLORER: ::windows_core::PCWSTR = ::windows_core::w!("Explorer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_FILTERS: ::windows_core::PCWSTR = ::windows_core::w!("Filters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_INIUPDATE: ::windows_core::PCWSTR = ::windows_core::w!("IniUpdate");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_ISAENUM: ::windows_core::PCWSTR = ::windows_core::w!("ISAPnP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_JOYCURR: ::windows_core::PCWSTR = ::windows_core::w!("CurrentJoystickSettings");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_JOYSETTINGS: ::windows_core::PCWSTR = ::windows_core::w!("JoystickSettings");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_KEYBOARD_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("Keyboard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_KNOWNDOCKINGSTATES: ::windows_core::PCWSTR = ::windows_core::w!("Hardware Profiles");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_LOGCONFIG: ::windows_core::PCWSTR = ::windows_core::w!("LogConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_LOGON: ::windows_core::PCWSTR = ::windows_core::w!("\\Logon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_LOWER_FILTER_LEVEL_DEFAULT: ::windows_core::PCWSTR = ::windows_core::w!("*Lower");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_MEDIA_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("MEDIA");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_MODEM_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("Modem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_MODES: ::windows_core::PCWSTR = ::windows_core::w!("Modes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_MONITOR_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("Monitor");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_MOUSE_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("Mouse");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_NDISINFO: ::windows_core::PCWSTR = ::windows_core::w!("NDISInfo");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_NETWORK: ::windows_core::PCWSTR = ::windows_core::w!("Network");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_NETWORKPROVIDER: ::windows_core::PCWSTR = ::windows_core::w!("\\NetworkProvider");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_NETWORK_PERSISTENT: ::windows_core::PCWSTR = ::windows_core::w!("\\Persistent");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_NETWORK_RECENT: ::windows_core::PCWSTR = ::windows_core::w!("\\Recent");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_OVERRIDE: ::windows_core::PCWSTR = ::windows_core::w!("Override");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCIENUM: ::windows_core::PCWSTR = ::windows_core::w!("PCI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCMCIA: ::windows_core::PCWSTR = ::windows_core::w!("PCMCIA\\");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCMCIAENUM: ::windows_core::PCWSTR = ::windows_core::w!("PCMCIA");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCMCIA_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("PCMCIA");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCMTD: ::windows_core::PCWSTR = ::windows_core::w!("MTD-");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PCUNKNOWN: ::windows_core::PCWSTR = ::windows_core::w!("UNKNOWN_MANUFACTURER");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_POL_COMPUTERS: ::windows_core::PCWSTR = ::windows_core::w!("Computers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_POL_DEFAULT: ::windows_core::PCWSTR = ::windows_core::w!(".default");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_POL_USERGROUPDATA: ::windows_core::PCWSTR = ::windows_core::w!("GroupData\\UserGroups\\Priority");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_POL_USERGROUPS: ::windows_core::PCWSTR = ::windows_core::w!("UserGroups");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_POL_USERS: ::windows_core::PCWSTR = ::windows_core::w!("Users");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PORTS_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("ports");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PRINTERS: ::windows_core::PCWSTR = ::windows_core::w!("Printers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_PRINT_PROC: ::windows_core::PCWSTR = ::windows_core::w!("\\Print Processors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_ROOTENUM: ::windows_core::PCWSTR = ::windows_core::w!("Root");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_RUNHISTORY: ::windows_core::PCWSTR = ::windows_core::w!("RunHistory");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_SCSI_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("SCSIAdapter");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_SETUP: ::windows_core::PCWSTR = ::windows_core::w!("\\Setup");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_SHARES: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network\\LanMan");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_SYSTEM: ::windows_core::PCWSTR = ::windows_core::w!("System");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_SYSTEMBOARD: ::windows_core::PCWSTR = ::windows_core::w!("*PNP0C01");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_UPPER_FILTER_LEVEL_DEFAULT: ::windows_core::PCWSTR = ::windows_core::w!("*Upper");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_USER: ::windows_core::PCWSTR = ::windows_core::w!("User");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_VPOWERDENUM: ::windows_core::PCWSTR = ::windows_core::w!("VPOWERD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_KEY_WINOLDAPP: ::windows_core::PCWSTR = ::windows_core::w!("WinOldApp");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_ATT_PC: ::windows_core::PCWSTR = ::windows_core::w!("AT&T PC");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_HP_VECTRA: ::windows_core::PCWSTR = ::windows_core::w!("HP Vectra");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPC: ::windows_core::PCWSTR = ::windows_core::w!("IBM PC");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPCAT: ::windows_core::PCWSTR = ::windows_core::w!("IBM PC/AT");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPCCONV: ::windows_core::PCWSTR = ::windows_core::w!("IBM PC Convertible");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPCJR: ::windows_core::PCWSTR = ::windows_core::w!("IBM PCjr");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPCXT: ::windows_core::PCWSTR = ::windows_core::w!("IBM PC/XT");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPCXT_286: ::windows_core::PCWSTR = ::windows_core::w!("IBM PC/XT 286");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS1: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/1");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_25: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-25");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_30: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-30");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_30_286: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-30 286");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_50: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-50");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_50Z: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-50Z");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_55SX: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-55SX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_60: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-60");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_65SX: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-65SX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_70: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-70");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_70_80: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-70/80");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_80: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-80");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_90: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-90");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_IBMPS2_P70: ::windows_core::PCWSTR = ::windows_core::w!("IBM PS/2-P70");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_PHOENIX_PCAT: ::windows_core::PCWSTR = ::windows_core::w!("Phoenix PC/AT Compatible");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_UNKNOWN: ::windows_core::PCWSTR = ::windows_core::w!("Unknown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MACHTYPE_ZENITH_PC: ::windows_core::PCWSTR = ::windows_core::w!("Zenith PC");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_MAX_VALUE_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_ADDRARB: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\Arbitrators\\AddrArb");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_AEDEBUG: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows NT\\CurrentVersion\\AeDebug");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_APPEARANCE: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel\\Appearance");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_APPPATCH: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\AppPatches");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_APPPATHS: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\App Paths");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_BIOSINFO: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\BiosInfo");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_BUSINFORMATION: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\PnP\\BusInformation");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CDFS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\FileSystem\\CDFS");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKBADAPPS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\CheckBadApps");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKBADAPPS400: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\CheckBadApps400");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKDISK: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKDISKSET: ::windows_core::PCWSTR = ::windows_core::w!("Settings");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKDISKUDRVS: ::windows_core::PCWSTR = ::windows_core::w!("NoUnknownDDErrDrvs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHECKVERDLLS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\CheckVerDLLs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHILD_PREFIX: ::windows_core::PCWSTR = ::windows_core::w!("Child");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHKLASTCHECK: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive\\LastCheck");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CHKLASTSURFAN: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Check Drive\\LastSurfaceAnalysis");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\Class");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CLASS_NT: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\Class");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CODEPAGE: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\Nls\\Codepage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CODEVICEINSTALLERS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\CoDeviceInstallers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_COLORS: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel\\Colors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_COMPUTRNAME: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\ComputerName\\ComputerName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CONTROLPANEL: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CONTROLSFOLDER: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Controls Folder");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CRITICALDEVICEDATABASE: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\CriticalDeviceDatabase");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CURRENTCONTROLSET: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CURRENT_CONTROL_SET: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CURSORS: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel\\Cursors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_CVNETWORK: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DESKTOP: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel\\Desktop");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DETECT: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Detect");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DEVICEINSTALLER: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Device Installer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DEVICE_CLASSES: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\DeviceClasses");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DIFX: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\DIFX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DISPLAYSETTINGS: ::windows_core::PCWSTR = ::windows_core::w!("Display\\Settings");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DMAARB: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\Arbitrators\\DMAArb");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DRIVERSIGN: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Driver Signing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_DRIVERSIGN_POLICY: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Policies\\Microsoft\\Windows NT\\Driver Signing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_ENUM: ::windows_core::PCWSTR = ::windows_core::w!("Enum");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_ENVIRONMENTS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\Print\\Environments");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_EVENTLABELS: ::windows_core::PCWSTR = ::windows_core::w!("AppEvents\\EventLabels");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_EXPLORER: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FAULT: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Fault");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FILESYSTEM: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\FileSystem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FILESYSTEM_NOVOLTRACK: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\FileSystem\\NoVolTrack");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FLOATINGPOINTPROCESSOR: ::windows_core::PCWSTR = ::windows_core::w!("HARDWARE\\DESCRIPTION\\System\\FloatingPointProcessor");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FLOATINGPOINTPROCESSOR0: ::windows_core::PCWSTR = ::windows_core::w!("HARDWARE\\DESCRIPTION\\System\\FloatingPointProcessor\\0");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_FONTS: ::windows_core::PCWSTR = ::windows_core::w!("Display\\Fonts");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_GRPCONV: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\GrpConv");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_HACKINIFILE: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\HackIniFiles");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_HWPROFILES: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Hardware Profiles");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_HWPROFILESCURRENT: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Hardware Profiles\\Current");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_ICONS: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel\\Icons");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_IDCONFIGDB: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\IDConfigDB");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_INSTALLEDFILES: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\InstalledFiles");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_IOARB: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\Arbitrators\\IOArb");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_IOS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\IOS");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_IRQARB: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\Arbitrators\\IRQArb");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_KEYBOARD: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel\\Keyboard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_KNOWN16DLLS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\Known16DLLs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_KNOWNDLLS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\KnownDLLs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_KNOWNVXDS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\KnownVxDs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LASTBACKUP: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastBackup");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LASTCHECK: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastCheck");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LASTGOOD: ::windows_core::PCWSTR = ::windows_core::w!("System\\LastKnownGoodRecovery\\LastGood");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LASTGOODTMP: ::windows_core::PCWSTR = ::windows_core::w!("System\\LastKnownGoodRecovery\\LastGood.Tmp");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LASTOPTIMIZE: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\LastOptimize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_LOOKSCHEMES: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel\\Appearance\\Schemes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_METRICS: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel\\Desktop\\WindowMetrics");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_MONITORS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\Print\\Monitors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_MOUSE: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel\\Mouse");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_MSDOSOPTS: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\MS-DOSOptions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_MULTIMEDIA_AUDIO: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Multimedia\\Audio");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_MULTI_FUNCTION: ::windows_core::PCWSTR = ::windows_core::w!("MF");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NCPSERVER: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\NcpServer\\Parameters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NETEQUIV: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network\\Equivalent");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NETWORK_USERSETTINGS: ::windows_core::PCWSTR = ::windows_core::w!("Network");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NEWDOSBOX: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\MS-DOSSpecialConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NONDRIVERSIGN: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Non-Driver Signing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NONDRIVERSIGN_POLICY: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Policies\\Microsoft\\Windows NT\\Non-Driver Signing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NOSUGGMSDOS: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\NoMSDOSWarn");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NT_CURRENTVERSION: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows NT\\CurrentVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_NWREDIR: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\NWREDIR");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PCIIR: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\Pnp\\PciIrqRouting");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PER_HW_ID_STORAGE: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows NT\\CurrentVersion\\PerHwIdStorage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PIFCONVERT: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\PIFConvert");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_POLICIES: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Policies");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PRINT: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\Print");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PRINTERS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\Print\\Printers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PROPERTYSYSTEM: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\PropertySystem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PROVIDERS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\Print\\Providers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_PWDPROVIDER: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\PwdProvider");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_REALMODENET: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Network\\Real Mode Net");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_REINSTALL: ::windows_core::PCWSTR = ::windows_core::w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Reinstall");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RELIABILITY: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Reliability");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RELIABILITY_POLICY: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Policies\\Microsoft\\Windows NT\\Reliability");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RELIABILITY_POLICY_REPORTSNAPSHOT: ::windows_core::PCWSTR = ::windows_core::w!("ReportSnapshot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RELIABILITY_POLICY_SHUTDOWNREASONUI: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownReasonUI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RELIABILITY_POLICY_SNAPSHOT: ::windows_core::PCWSTR = ::windows_core::w!("Snapshot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_ROOT: ::windows_core::PCWSTR = ::windows_core::w!("Enum\\Root");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RUN: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RUNONCE: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RUNONCEEX: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunOnceEx");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RUNSERVICES: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunServices");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_RUNSERVICESONCE: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\RunServicesOnce");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SCHEMES: ::windows_core::PCWSTR = ::windows_core::w!("AppEvents\\Schemes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SCREENSAVE: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel\\Desktop");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SERVICES: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SETUP: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SHUTDOWN: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\Shutdown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SOUND: ::windows_core::PCWSTR = ::windows_core::w!("Control Panel\\Sound");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SYSTEMENUM: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Enum");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_SYSTRAY: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\SysTray");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_TIMEZONE: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\TimeZoneInformation");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_UNINSTALL: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_UPDATE: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\Update");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VCOMM: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\VCOMM");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VMM: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\VMM");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VMM32FILES: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\VMM32Files");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VNETSUP: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\VNETSUP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VOLUMECACHE: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VolumeCaches");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VPOWERD: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\VxD\\VPOWERD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_VXD: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Services\\VxD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_WARNVERDLLS: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\SessionManager\\WarnVerDLLs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_WINBOOT: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\WinBoot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_WINDOWSAPPLETS: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Applets");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_WINLOGON: ::windows_core::PCWSTR = ::windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Winlogon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PATH_WMI_SECURITY: ::windows_core::PCWSTR = ::windows_core::w!("System\\CurrentControlSet\\Control\\Wmi\\Security");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PCI_DUAL_IDE: ::windows_core::PCWSTR = ::windows_core::w!("PCIDualIDE");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_PCI_OPTIONS: ::windows_core::PCWSTR = ::windows_core::w!("Options");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_DEFAULTLOC: ::windows_core::PCWSTR = ::windows_core::w!("UseDefaultNetLocation");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_ENABLE: ::windows_core::PCWSTR = ::windows_core::w!("Enable");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_LOWPOWERACTIVE: ::windows_core::PCWSTR = ::windows_core::w!("ScreenSaveLowPowerActive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_LOWPOWERTIMEOUT: ::windows_core::PCWSTR = ::windows_core::w!("ScreenSaveLowPowerTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_NETPATH: ::windows_core::PCWSTR = ::windows_core::w!("NetworkPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_POWEROFFACTIVE: ::windows_core::PCWSTR = ::windows_core::w!("ScreenSavePowerOffActive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_POWEROFFTIMEOUT: ::windows_core::PCWSTR = ::windows_core::w!("ScreenSavePowerOffTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_SCRPASSWORD: ::windows_core::PCWSTR = ::windows_core::w!("ScreenSave_Data");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_USESCRPASSWORD: ::windows_core::PCWSTR = ::windows_core::w!("ScreenSaveUsePassword");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VALUE_VERBOSE: ::windows_core::PCWSTR = ::windows_core::w!("Verbose");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ACDRIVESPINDOWN: ::windows_core::PCWSTR = ::windows_core::w!("ACDriveSpinDown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ACSPINDOWNPREVIOUS: ::windows_core::PCWSTR = ::windows_core::w!("ACSpinDownPrevious");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ACTIVESERVICE: ::windows_core::PCWSTR = ::windows_core::w!("ActiveService");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ADDRESS: ::windows_core::PCWSTR = ::windows_core::w!("Address");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AEDEBUG_AUTO: ::windows_core::PCWSTR = ::windows_core::w!("Auto");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AEDEBUG_DEBUGGER: ::windows_core::PCWSTR = ::windows_core::w!("Debugger");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ALPHANUMPWDS: ::windows_core::PCWSTR = ::windows_core::w!("AlphanumPwds");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APISUPPORT: ::windows_core::PCWSTR = ::windows_core::w!("APISupport");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMACTIMEOUT: ::windows_core::PCWSTR = ::windows_core::w!("APMACTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMBATTIMEOUT: ::windows_core::PCWSTR = ::windows_core::w!("APMBatTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMBIOSVER: ::windows_core::PCWSTR = ::windows_core::w!("APMBiosVer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("APMFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMMENUSUSPEND: ::windows_core::PCWSTR = ::windows_core::w!("APMMenuSuspend");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APMSHUTDOWNPOWER: ::windows_core::PCWSTR = ::windows_core::w!("APMShutDownPower");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_APPINSTPATH: ::windows_core::PCWSTR = ::windows_core::w!("AppInstallPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ASKFORCONFIG: ::windows_core::PCWSTR = ::windows_core::w!("AskForConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ASKFORCONFIGFUNC: ::windows_core::PCWSTR = ::windows_core::w!("AskForConfigFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ASYNCFILECOMMIT: ::windows_core::PCWSTR = ::windows_core::w!("AsyncFileCommit");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUDIO_BITMAP: ::windows_core::PCWSTR = ::windows_core::w!("bitmap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUDIO_ICON: ::windows_core::PCWSTR = ::windows_core::w!("icon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTHENT_AGENT: ::windows_core::PCWSTR = ::windows_core::w!("AuthenticatingAgent");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTOEXEC: ::windows_core::PCWSTR = ::windows_core::w!("Autoexec.Bat");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTOINSNOTE: ::windows_core::PCWSTR = ::windows_core::w!("AutoInsertNotification");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTOLOGON: ::windows_core::PCWSTR = ::windows_core::w!("AutoLogon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTOMOUNT: ::windows_core::PCWSTR = ::windows_core::w!("AutoMountDrives");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_AUTOSTART: ::windows_core::PCWSTR = ::windows_core::w!("AutoStart");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BASICPROPERTIES: ::windows_core::PCWSTR = ::windows_core::w!("BasicProperties");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BASICPROPERTIES_32: ::windows_core::PCWSTR = ::windows_core::w!("BasicProperties32");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BATDRIVESPINDOWN: ::windows_core::PCWSTR = ::windows_core::w!("BatDriveSpinDown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BATSPINDOWNPREVIOUS: ::windows_core::PCWSTR = ::windows_core::w!("BatSpinDownPrevious");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BEHAVIOR_ON_FAILED_VERIFY: ::windows_core::PCWSTR = ::windows_core::w!("BehaviorOnFailedVerify");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BIOSDATE: ::windows_core::PCWSTR = ::windows_core::w!("BIOSDate");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BIOSNAME: ::windows_core::PCWSTR = ::windows_core::w!("BIOSName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BIOSVERSION: ::windows_core::PCWSTR = ::windows_core::w!("BIOSVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BITSPERPIXEL: ::windows_core::PCWSTR = ::windows_core::w!("BitsPerPixel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BOOTCONFIG: ::windows_core::PCWSTR = ::windows_core::w!("BootConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BOOTCOUNT: ::windows_core::PCWSTR = ::windows_core::w!("BootCount");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BOOTDIR: ::windows_core::PCWSTR = ::windows_core::w!("BootDir");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BPP: ::windows_core::PCWSTR = ::windows_core::w!("BPP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BT: ::windows_core::PCWSTR = ::windows_core::w!("6005BT");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BUFFAGETIMEOUT: ::windows_core::PCWSTR = ::windows_core::w!("BufferAgeTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BUFFIDLETIMEOUT: ::windows_core::PCWSTR = ::windows_core::w!("BufferIdleTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_BUSTYPE: ::windows_core::PCWSTR = ::windows_core::w!("BusType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CAPABILITIES: ::windows_core::PCWSTR = ::windows_core::w!("Capabilities");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CARDSPECIFIC: ::windows_core::PCWSTR = ::windows_core::w!("CardSpecific");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDCACHESIZE: ::windows_core::PCWSTR = ::windows_core::w!("CacheSize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDCOMPATNAMES: ::windows_core::PCWSTR = ::windows_core::w!("MSCDEXCompatNames");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDEXTERRORS: ::windows_core::PCWSTR = ::windows_core::w!("ExtendedErrors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDNOREADAHEAD: ::windows_core::PCWSTR = ::windows_core::w!("NoReadAhead");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDPREFETCH: ::windows_core::PCWSTR = ::windows_core::w!("Prefetch");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDPREFETCHTAIL: ::windows_core::PCWSTR = ::windows_core::w!("PrefetchTail");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDRAWCACHE: ::windows_core::PCWSTR = ::windows_core::w!("RawCache");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDROM: ::windows_core::PCWSTR = ::windows_core::w!("GenCD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDROMCLASSNAME: ::windows_core::PCWSTR = ::windows_core::w!("CDROM");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDSHOWVERSIONS: ::windows_core::PCWSTR = ::windows_core::w!("ShowVersions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CDSVDSENSE: ::windows_core::PCWSTR = ::windows_core::w!("SVDSense");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CHECKSUM: ::windows_core::PCWSTR = ::windows_core::w!("CurrentChecksum");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("Class");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CLASSDESC: ::windows_core::PCWSTR = ::windows_core::w!("ClassDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CLASSGUID: ::windows_core::PCWSTR = ::windows_core::w!("ClassGUID");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CMDRIVFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("CMDrivFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CMENUMFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("CMEnumFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COINSTALLERS_32: ::windows_core::PCWSTR = ::windows_core::w!("CoInstallers32");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMINFO: ::windows_core::PCWSTR = ::windows_core::w!("ComInfo");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMMENT: ::windows_core::PCWSTR = ::windows_core::w!("Comment");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMPATIBLEIDS: ::windows_core::PCWSTR = ::windows_core::w!("CompatibleIDs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMPRESSIONMETHOD: ::windows_core::PCWSTR = ::windows_core::w!("CompressionAlgorithm");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMPRESSIONTHRESHOLD: ::windows_core::PCWSTR = ::windows_core::w!("CompressionThreshold");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMPUTERNAME: ::windows_core::PCWSTR = ::windows_core::w!("ComputerName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMPUTRNAME: ::windows_core::PCWSTR = ::windows_core::w!("ComputerName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_COMVERIFYBASE: ::windows_core::PCWSTR = ::windows_core::w!("COMVerifyBase");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONFIG: ::windows_core::PCWSTR = ::windows_core::w!("ConfigPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONFIGFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("ConfigFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONFIGMG: ::windows_core::PCWSTR = ::windows_core::w!("CONFIGMG");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONFIGSYS: ::windows_core::PCWSTR = ::windows_core::w!("Config.Sys");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONNECTION_TYPE: ::windows_core::PCWSTR = ::windows_core::w!("ConnectionType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONTAINERID: ::windows_core::PCWSTR = ::windows_core::w!("ContainerID");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONTIGFILEALLOC: ::windows_core::PCWSTR = ::windows_core::w!("ContigFileAllocSize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CONVMEM: ::windows_core::PCWSTR = ::windows_core::w!("ConvMem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CPU: ::windows_core::PCWSTR = ::windows_core::w!("CPU");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CRASHFUNCS: ::windows_core::PCWSTR = ::windows_core::w!("CrashFuncs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CSCONFIGFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("CSConfigFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURCONFIG: ::windows_core::PCWSTR = ::windows_core::w!("CurrentConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURDRVLET: ::windows_core::PCWSTR = ::windows_core::w!("CurrentDriveLetterAssignment");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENTCONFIG: ::windows_core::PCWSTR = ::windows_core::w!("CurrentConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENT_BUILD: ::windows_core::PCWSTR = ::windows_core::w!("CurrentBuildNumber");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENT_CSDVERSION: ::windows_core::PCWSTR = ::windows_core::w!("CSDVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENT_TYPE: ::windows_core::PCWSTR = ::windows_core::w!("CurrentType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENT_USER: ::windows_core::PCWSTR = ::windows_core::w!("Current User");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CURRENT_VERSION: ::windows_core::PCWSTR = ::windows_core::w!("CurrentVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CUSTOMCOLORS: ::windows_core::PCWSTR = ::windows_core::w!("CustomColors");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CUSTOM_PROPERTY_CACHE_DATE: ::windows_core::PCWSTR = ::windows_core::w!("CustomPropertyCacheDate");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_CUSTOM_PROPERTY_HW_ID_KEY: ::windows_core::PCWSTR = ::windows_core::w!("CustomPropertyHwIdKey");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEFAULT: ::windows_core::PCWSTR = ::windows_core::w!("Default");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DETCONFIG: ::windows_core::PCWSTR = ::windows_core::w!("DetConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DETECT: ::windows_core::PCWSTR = ::windows_core::w!("Detect");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DETECTFUNC: ::windows_core::PCWSTR = ::windows_core::w!("DetectFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DETFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("DetFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DETFUNC: ::windows_core::PCWSTR = ::windows_core::w!("DetFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVDESC: ::windows_core::PCWSTR = ::windows_core::w!("DeviceDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICEDRIVER: ::windows_core::PCWSTR = ::windows_core::w!("DeviceDriver");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICEPATH: ::windows_core::PCWSTR = ::windows_core::w!("DevicePath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICE_CHARACTERISTICS: ::windows_core::PCWSTR = ::windows_core::w!("DeviceCharacteristics");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICE_EXCLUSIVE: ::windows_core::PCWSTR = ::windows_core::w!("Exclusive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICE_INSTANCE: ::windows_core::PCWSTR = ::windows_core::w!("DeviceInstance");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICE_SECURITY_DESCRIPTOR: ::windows_core::PCWSTR = ::windows_core::w!("Security");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVICE_TYPE: ::windows_core::PCWSTR = ::windows_core::w!("DeviceType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVLOADER: ::windows_core::PCWSTR = ::windows_core::w!("DevLoader");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DEVTYPE: ::windows_core::PCWSTR = ::windows_core::w!("DeviceType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DIRECTHOST: ::windows_core::PCWSTR = ::windows_core::w!("DirectHost");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DIRTYSHUTDOWN: ::windows_core::PCWSTR = ::windows_core::w!("DirtyShutdown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DIRTYSHUTDOWNTIME: ::windows_core::PCWSTR = ::windows_core::w!("DirtyShutdownTime");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISABLECOUNT: ::windows_core::PCWSTR = ::windows_core::w!("DisableCount");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISABLEPWDCACHING: ::windows_core::PCWSTR = ::windows_core::w!("DisablePwdCaching");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISABLEREGTOOLS: ::windows_core::PCWSTR = ::windows_core::w!("DisableRegistryTools");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISCONNECT: ::windows_core::PCWSTR = ::windows_core::w!("Disconnect");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISK: ::windows_core::PCWSTR = ::windows_core::w!("GenDisk");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISKCLASSNAME: ::windows_core::PCWSTR = ::windows_core::w!("DiskDrive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPCPL_NOAPPEARANCEPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoDispAppearancePage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPCPL_NOBACKGROUNDPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoDispBackgroundPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPCPL_NODISPCPL: ::windows_core::PCWSTR = ::windows_core::w!("NoDispCPL");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPCPL_NOSCRSAVPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoDispScrSavPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPCPL_NOSETTINGSPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoDispSettingsPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPLAY: ::windows_core::PCWSTR = ::windows_core::w!("display");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DISPLAYFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("DisplayFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOCKED: ::windows_core::PCWSTR = ::windows_core::w!("CurrentDockedState");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOCKSTATE: ::windows_core::PCWSTR = ::windows_core::w!("DockState");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOES_POLLING: ::windows_core::PCWSTR = ::windows_core::w!("PollingSupportNeeded");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DONTLOADIFCONFLICT: ::windows_core::PCWSTR = ::windows_core::w!("DontLoadIfConflict");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DONTUSEMEM: ::windows_core::PCWSTR = ::windows_core::w!("DontAllocLastMem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOSCP: ::windows_core::PCWSTR = ::windows_core::w!("OEMCP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOSOPTFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("Flags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOSOPTGLOBALFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("GlobalFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOSOPTTIP: ::windows_core::PCWSTR = ::windows_core::w!("TipText");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOSPAGER: ::windows_core::PCWSTR = ::windows_core::w!("DOSPager");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOS_SPOOL_MASK: ::windows_core::PCWSTR = ::windows_core::w!("DOSSpoolMask");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DOUBLEBUFFER: ::windows_core::PCWSTR = ::windows_core::w!("DoubleBuffer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPI: ::windows_core::PCWSTR = ::windows_core::w!("dpi");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPILOGICALX: ::windows_core::PCWSTR = ::windows_core::w!("DPILogicalX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPILOGICALY: ::windows_core::PCWSTR = ::windows_core::w!("DPILogicalY");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPIPHYSICALX: ::windows_core::PCWSTR = ::windows_core::w!("DPIPhysicalX");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPIPHYSICALY: ::windows_core::PCWSTR = ::windows_core::w!("DPIPhysicalY");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DPMS: ::windows_core::PCWSTR = ::windows_core::w!("DPMS");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVER: ::windows_core::PCWSTR = ::windows_core::w!("Driver");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVERCACHEPATH: ::windows_core::PCWSTR = ::windows_core::w!("DriverCachePath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVERDATE: ::windows_core::PCWSTR = ::windows_core::w!("DriverDate");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVERDATEDATA: ::windows_core::PCWSTR = ::windows_core::w!("DriverDateData");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVERVERSION: ::windows_core::PCWSTR = ::windows_core::w!("DriverVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVESPINDOWN: ::windows_core::PCWSTR = ::windows_core::w!("DriveSpinDown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVEWRITEBEHIND: ::windows_core::PCWSTR = ::windows_core::w!("DriveWriteBehind");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRIVE_SPINDOWN: ::windows_core::PCWSTR = ::windows_core::w!("NoDispSpinDown");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRV: ::windows_core::PCWSTR = ::windows_core::w!("drv");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DRVDESC: ::windows_core::PCWSTR = ::windows_core::w!("DriverDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_DYNAMIC: ::windows_core::PCWSTR = ::windows_core::w!("Dynamic");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EISA_FLAGS: ::windows_core::PCWSTR = ::windows_core::w!("EISAFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EISA_FUNCTIONS: ::windows_core::PCWSTR = ::windows_core::w!("EISAFunctions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EISA_FUNCTIONS_MASK: ::windows_core::PCWSTR = ::windows_core::w!("EISAFunctionsMask");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EISA_RANGES: ::windows_core::PCWSTR = ::windows_core::w!("EISARanges");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EISA_SIMULATE_INT15: ::windows_core::PCWSTR = ::windows_core::w!("EISASimulateInt15");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EJECT_PRIORITY: ::windows_core::PCWSTR = ::windows_core::w!("EjectPriority");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ENABLEINTS: ::windows_core::PCWSTR = ::windows_core::w!("EnableInts");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ENUMERATOR: ::windows_core::PCWSTR = ::windows_core::w!("Enumerator");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ENUMPROPPAGES: ::windows_core::PCWSTR = ::windows_core::w!("EnumPropPages");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ENUMPROPPAGES_32: ::windows_core::PCWSTR = ::windows_core::w!("EnumPropPages32");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ESDI: ::windows_core::PCWSTR = ::windows_core::w!("ESDI\\");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EXISTS: ::windows_core::PCWSTR = ::windows_core::w!("Exists");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_EXTMEM: ::windows_core::PCWSTR = ::windows_core::w!("ExtMem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FAULT_LOGFILE: ::windows_core::PCWSTR = ::windows_core::w!("LogFile");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FIFODEPTH: ::windows_core::PCWSTR = ::windows_core::w!("FIFODepth");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FILESHARING: ::windows_core::PCWSTR = ::windows_core::w!("FileSharing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FIRSTINSTALLDATETIME: ::windows_core::PCWSTR = ::windows_core::w!("FirstInstallDateTime");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FIRSTNETDRIVE: ::windows_core::PCWSTR = ::windows_core::w!("FirstNetworkDrive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FLOP: ::windows_core::PCWSTR = ::windows_core::w!("FLOP\\");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FLOPPY: ::windows_core::PCWSTR = ::windows_core::w!("FLOPPY");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FONTSIZE: ::windows_core::PCWSTR = ::windows_core::w!("FontSize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCECL: ::windows_core::PCWSTR = ::windows_core::w!("ForceChangeLine");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCEDCONFIG: ::windows_core::PCWSTR = ::windows_core::w!("ForcedConfig");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCEFIFO: ::windows_core::PCWSTR = ::windows_core::w!("ForceFIFO");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCELOAD: ::windows_core::PCWSTR = ::windows_core::w!("ForceLoadPD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCEPMIO: ::windows_core::PCWSTR = ::windows_core::w!("ForcePMIO");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCEREBOOT: ::windows_core::PCWSTR = ::windows_core::w!("ForceReboot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FORCERMIO: ::windows_core::PCWSTR = ::windows_core::w!("ForceRMIO");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FREESPACERATIO: ::windows_core::PCWSTR = ::windows_core::w!("FreeSpaceRatio");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FRIENDLYNAME: ::windows_core::PCWSTR = ::windows_core::w!("FriendlyName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FSFILTERCLASS: ::windows_core::PCWSTR = ::windows_core::w!("FSFilterClass");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FULLTRACE: ::windows_core::PCWSTR = ::windows_core::w!("FullTrace");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_FUNCDESC: ::windows_core::PCWSTR = ::windows_core::w!("FunctionDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_GAPTIME: ::windows_core::PCWSTR = ::windows_core::w!("GapTime");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_GRB: ::windows_core::PCWSTR = ::windows_core::w!("grb");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HARDWAREID: ::windows_core::PCWSTR = ::windows_core::w!("HardwareID");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HIDESHAREPWDS: ::windows_core::PCWSTR = ::windows_core::w!("HideSharePwds");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HRES: ::windows_core::PCWSTR = ::windows_core::w!("HRes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HWDETECT: ::windows_core::PCWSTR = ::windows_core::w!("HardwareDetect");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HWMECHANISM: ::windows_core::PCWSTR = ::windows_core::w!("HWMechanism");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_HWREV: ::windows_core::PCWSTR = ::windows_core::w!("HWRevision");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ID: ::windows_core::PCWSTR = ::windows_core::w!("CurrentID");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_IDE_FORCE_SERIALIZE: ::windows_core::PCWSTR = ::windows_core::w!("ForceSerialization");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_IDE_NO_SERIALIZE: ::windows_core::PCWSTR = ::windows_core::w!("IDENoSerialize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INFNAME: ::windows_core::PCWSTR = ::windows_core::w!("InfName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INFPATH: ::windows_core::PCWSTR = ::windows_core::w!("InfPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INFSECTION: ::windows_core::PCWSTR = ::windows_core::w!("InfSection");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INFSECTIONEXT: ::windows_core::PCWSTR = ::windows_core::w!("InfSectionExt");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INHIBITRESULTS: ::windows_core::PCWSTR = ::windows_core::w!("InhibitResults");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INSICON: ::windows_core::PCWSTR = ::windows_core::w!("Icon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INSTALLER: ::windows_core::PCWSTR = ::windows_core::w!("Installer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INSTALLER_32: ::windows_core::PCWSTR = ::windows_core::w!("Installer32");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INSTALLTYPE: ::windows_core::PCWSTR = ::windows_core::w!("InstallType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_INT13: ::windows_core::PCWSTR = ::windows_core::w!("Int13");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ISAPNP: ::windows_core::PCWSTR = ::windows_core::w!("ISAPNP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ISAPNP_RDP_OVERRIDE: ::windows_core::PCWSTR = ::windows_core::w!("RDPOverRide");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYCALLOUT: ::windows_core::PCWSTR = ::windows_core::w!("JoystickCallout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYNCONFIG: ::windows_core::PCWSTR = ::windows_core::w!("Joystick%dConfiguration");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYNOEMCALLOUT: ::windows_core::PCWSTR = ::windows_core::w!("Joystick%dOEMCallout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYNOEMNAME: ::windows_core::PCWSTR = ::windows_core::w!("Joystick%dOEMName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL1: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal1");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL10: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal10");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL11: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal11");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL12: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal12");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL2: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal2");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL3: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal3");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL4: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal4");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL5: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal5");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL6: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal6");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL7: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal7");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL8: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal8");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCAL9: ::windows_core::PCWSTR = ::windows_core::w!("OEMCal9");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCALCAP: ::windows_core::PCWSTR = ::windows_core::w!("OEMCalCap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCALLOUT: ::windows_core::PCWSTR = ::windows_core::w!("OEMCallout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMCALWINCAP: ::windows_core::PCWSTR = ::windows_core::w!("OEMCalWinCap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMDATA: ::windows_core::PCWSTR = ::windows_core::w!("OEMData");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMNAME: ::windows_core::PCWSTR = ::windows_core::w!("OEMName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMPOVLABEL: ::windows_core::PCWSTR = ::windows_core::w!("OEMPOVLabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMRLABEL: ::windows_core::PCWSTR = ::windows_core::w!("OEMRLabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMTESTBUTTONCAP: ::windows_core::PCWSTR = ::windows_core::w!("OEMTestButtonCap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMTESTBUTTONDESC: ::windows_core::PCWSTR = ::windows_core::w!("OEMTestButtonDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMTESTMOVECAP: ::windows_core::PCWSTR = ::windows_core::w!("OEMTestMoveCap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMTESTMOVEDESC: ::windows_core::PCWSTR = ::windows_core::w!("OEMTestMoveDesc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMTESTWINCAP: ::windows_core::PCWSTR = ::windows_core::w!("OEMTestWinCap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMULABEL: ::windows_core::PCWSTR = ::windows_core::w!("OEMULabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMVLABEL: ::windows_core::PCWSTR = ::windows_core::w!("OEMVLabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMXYLABEL: ::windows_core::PCWSTR = ::windows_core::w!("OEMXYLabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYOEMZLABEL: ::windows_core::PCWSTR = ::windows_core::w!("OEMZLabel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_JOYUSERVALUES: ::windows_core::PCWSTR = ::windows_core::w!("JoystickUserValues");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVEBT: ::windows_core::PCWSTR = ::windows_core::w!("LastAliveBT");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVEINTERVAL: ::windows_core::PCWSTR = ::windows_core::w!("TimeStampInterval");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVEPMPOLICY: ::windows_core::PCWSTR = ::windows_core::w!("LastAlivePMPolicy");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVESTAMP: ::windows_core::PCWSTR = ::windows_core::w!("LastAliveStamp");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVESTAMPFORCED: ::windows_core::PCWSTR = ::windows_core::w!("LastAliveStampForced");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVESTAMPINTERVAL: ::windows_core::PCWSTR = ::windows_core::w!("LastAliveStampInterval");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVESTAMPPOLICYINTERVAL: ::windows_core::PCWSTR = ::windows_core::w!("LastAliveStampPolicyInterval");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTALIVEUPTIME: ::windows_core::PCWSTR = ::windows_core::w!("LastAliveUptime");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTBOOTPMDRVS: ::windows_core::PCWSTR = ::windows_core::w!("LastBootPMDrvs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTCOMPUTERNAME: ::windows_core::PCWSTR = ::windows_core::w!("LastComputerName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LASTPCIBUSNUM: ::windows_core::PCWSTR = ::windows_core::w!("LastPCIBusNum");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LAST_UPDATE_TIME: ::windows_core::PCWSTR = ::windows_core::w!("LastUpdateTime");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LEGALNOTICECAPTION: ::windows_core::PCWSTR = ::windows_core::w!("LegalNoticeCaption");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LEGALNOTICETEXT: ::windows_core::PCWSTR = ::windows_core::w!("LegalNoticeText");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LICENSINGINFO: ::windows_core::PCWSTR = ::windows_core::w!("LicensingInfo");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LINKED: ::windows_core::PCWSTR = ::windows_core::w!("Linked");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOADHI: ::windows_core::PCWSTR = ::windows_core::w!("LoadHi");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOADRMDRIVERS: ::windows_core::PCWSTR = ::windows_core::w!("LoadRMDrivers");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOCATION_INFORMATION: ::windows_core::PCWSTR = ::windows_core::w!("LocationInformation");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOCATION_INFORMATION_OVERRIDE: ::windows_core::PCWSTR = ::windows_core::w!("LocationInformationOverride");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOWERFILTERS: ::windows_core::PCWSTR = ::windows_core::w!("LowerFilters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOWER_FILTER_DEFAULT_LEVEL: ::windows_core::PCWSTR = ::windows_core::w!("LowerFilterDefaultLevel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_LOWER_FILTER_LEVELS: ::windows_core::PCWSTR = ::windows_core::w!("LowerFilterLevels");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MACHINETYPE: ::windows_core::PCWSTR = ::windows_core::w!("MachineType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MANUFACTURER: ::windows_core::PCWSTR = ::windows_core::w!("Manufacturer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAP: ::windows_core::PCWSTR = ::windows_core::w!("Map");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MATCHINGDEVID: ::windows_core::PCWSTR = ::windows_core::w!("MatchingDeviceId");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAXCONNECTIONS: ::windows_core::PCWSTR = ::windows_core::w!("MaxConnections");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAXLIP: ::windows_core::PCWSTR = ::windows_core::w!("MaxLIP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAXRES: ::windows_core::PCWSTR = ::windows_core::w!("MaxResolution");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAXRETRY: ::windows_core::PCWSTR = ::windows_core::w!("MaxRetry");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MAX_HCID_LEN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MEDIA: ::windows_core::PCWSTR = ::windows_core::w!("MediaPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MFG: ::windows_core::PCWSTR = ::windows_core::w!("Mfg");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MF_FLAGS: ::windows_core::PCWSTR = ::windows_core::w!("MFFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MINIPORT_STAT: ::windows_core::PCWSTR = ::windows_core::w!("MiniportStatus");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MINPWDLEN: ::windows_core::PCWSTR = ::windows_core::w!("MinPwdLen");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MINRETRY: ::windows_core::PCWSTR = ::windows_core::w!("MinRetry");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MODE: ::windows_core::PCWSTR = ::windows_core::w!("Mode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MODEL: ::windows_core::PCWSTR = ::windows_core::w!("Model");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MSDOSMODE: ::windows_core::PCWSTR = ::windows_core::w!("MSDOSMode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MSDOSMODEDISCARD: ::windows_core::PCWSTR = ::windows_core::w!("Discard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_MUSTBEVALIDATED: ::windows_core::PCWSTR = ::windows_core::w!("MustBeValidated");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NAMECACHECOUNT: ::windows_core::PCWSTR = ::windows_core::w!("NameCache");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NAMENUMERICTAIL: ::windows_core::PCWSTR = ::windows_core::w!("NameNumericTail");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NCP_BROWSEMASTER: ::windows_core::PCWSTR = ::windows_core::w!("BrowseMaster");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NCP_USEPEERBROWSING: ::windows_core::PCWSTR = ::windows_core::w!("Use_PeerBrowsing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NCP_USESAP: ::windows_core::PCWSTR = ::windows_core::w!("Use_Sap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NDP: ::windows_core::PCWSTR = ::windows_core::w!("NDP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETCARD: ::windows_core::PCWSTR = ::windows_core::w!("Netcard");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETCLEAN: ::windows_core::PCWSTR = ::windows_core::w!("NetClean");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETOSTYPE: ::windows_core::PCWSTR = ::windows_core::w!("NetOSType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETSETUP_DISABLE: ::windows_core::PCWSTR = ::windows_core::w!("NoNetSetup");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETSETUP_NOCONFIGPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoNetSetupConfigPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETSETUP_NOIDPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoNetSetupIDPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NETSETUP_NOSECURITYPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoNetSetupSecurityPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOCMOSORFDPT: ::windows_core::PCWSTR = ::windows_core::w!("NoCMOSorFDPT");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NODISPLAYCLASS: ::windows_core::PCWSTR = ::windows_core::w!("NoDisplayClass");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOENTIRENETWORK: ::windows_core::PCWSTR = ::windows_core::w!("NoEntireNetwork");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOFILESHARING: ::windows_core::PCWSTR = ::windows_core::w!("NoFileSharing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOFILESHARINGCTRL: ::windows_core::PCWSTR = ::windows_core::w!("NoFileSharingControl");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOIDE: ::windows_core::PCWSTR = ::windows_core::w!("NoIDE");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOINSTALLCLASS: ::windows_core::PCWSTR = ::windows_core::w!("NoInstallClass");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NONSTANDARD_ATAPI: ::windows_core::PCWSTR = ::windows_core::w!("NonStandardATAPI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOPRINTSHARING: ::windows_core::PCWSTR = ::windows_core::w!("NoPrintSharing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOPRINTSHARINGCTRL: ::windows_core::PCWSTR = ::windows_core::w!("NoPrintSharingControl");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOUSECLASS: ::windows_core::PCWSTR = ::windows_core::w!("NoUseClass");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_NOWORKGROUPCONTENTS: ::windows_core::PCWSTR = ::windows_core::w!("NoWorkgroupContents");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_OLDMSDOSVER: ::windows_core::PCWSTR = ::windows_core::w!("OldMSDOSVer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_OLDWINDIR: ::windows_core::PCWSTR = ::windows_core::w!("OldWinDir");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_OPTIMIZESFN: ::windows_core::PCWSTR = ::windows_core::w!("OptimizeSFN");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_OPTIONS: ::windows_core::PCWSTR = ::windows_core::w!("Options");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_OPTORDER: ::windows_core::PCWSTR = ::windows_core::w!("Order");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_P1284MDL: ::windows_core::PCWSTR = ::windows_core::w!("Model");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_P1284MFG: ::windows_core::PCWSTR = ::windows_core::w!("Manufacturer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PATHCACHECOUNT: ::windows_core::PCWSTR = ::windows_core::w!("PathCache");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCCARD_POWER: ::windows_core::PCWSTR = ::windows_core::w!("EnablePowerManagement");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCI: ::windows_core::PCWSTR = ::windows_core::w!("PCI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCIBIOSVER: ::windows_core::PCWSTR = ::windows_core::w!("PCIBIOSVer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCICIRQMAP: ::windows_core::PCWSTR = ::windows_core::w!("PCICIRQMap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCICOPTIONS: ::windows_core::PCWSTR = ::windows_core::w!("PCICOptions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMCIA_ALLOC: ::windows_core::PCWSTR = ::windows_core::w!("AllocMemWin");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMCIA_ATAD: ::windows_core::PCWSTR = ::windows_core::w!("ATADelay");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMCIA_MEM: ::windows_core::PCWSTR = ::windows_core::w!("Memory");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMCIA_OPT: ::windows_core::PCWSTR = ::windows_core::w!("Options");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMCIA_SIZ: ::windows_core::PCWSTR = ::windows_core::w!("MinRegionSize");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCMTDRIVER: ::windows_core::PCWSTR = ::windows_core::w!("MTD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PCSSDRIVER: ::windows_core::PCWSTR = ::windows_core::w!("Driver");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PHYSICALDEVICEOBJECT: ::windows_core::PCWSTR = ::windows_core::w!("PhysicalDeviceObject");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PMODE_INT13: ::windows_core::PCWSTR = ::windows_core::w!("PModeInt13");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PNPBIOSVER: ::windows_core::PCWSTR = ::windows_core::w!("PnPBIOSVer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PNPSTRUCOFFSET: ::windows_core::PCWSTR = ::windows_core::w!("PnPStrucOffset");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_POLICY: ::windows_core::PCWSTR = ::windows_core::w!("Policy");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_POLLING: ::windows_core::PCWSTR = ::windows_core::w!("Polling");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PORTNAME: ::windows_core::PCWSTR = ::windows_core::w!("PortName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PORTSUBCLASS: ::windows_core::PCWSTR = ::windows_core::w!("PortSubClass");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PREFREDIR: ::windows_core::PCWSTR = ::windows_core::w!("PreferredRedir");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRESERVECASE: ::windows_core::PCWSTR = ::windows_core::w!("PreserveCase");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRESERVELONGNAMES: ::windows_core::PCWSTR = ::windows_core::w!("PreserveLongNames");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRINTERS_HIDETABS: ::windows_core::PCWSTR = ::windows_core::w!("NoPrinterTabs");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRINTERS_MASK: ::windows_core::PCWSTR = ::windows_core::w!("PrintersMask");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRINTERS_NOADD: ::windows_core::PCWSTR = ::windows_core::w!("NoAddPrinter");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRINTERS_NODELETE: ::windows_core::PCWSTR = ::windows_core::w!("NoDeletePrinter");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRINTSHARING: ::windows_core::PCWSTR = ::windows_core::w!("PrintSharing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRIORITY: ::windows_core::PCWSTR = ::windows_core::w!("Priority");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRIVATE: ::windows_core::PCWSTR = ::windows_core::w!("Private");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRIVATEFUNC: ::windows_core::PCWSTR = ::windows_core::w!("PrivateFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRIVATEPROBLEM: ::windows_core::PCWSTR = ::windows_core::w!("PrivateProblem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRODUCTID: ::windows_core::PCWSTR = ::windows_core::w!("ProductId");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PRODUCTTYPE: ::windows_core::PCWSTR = ::windows_core::w!("ProductType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PROFILEFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("ProfileFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PROPERTIES: ::windows_core::PCWSTR = ::windows_core::w!("Properties");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PROTINIPATH: ::windows_core::PCWSTR = ::windows_core::w!("ProtIniPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PROVIDER_NAME: ::windows_core::PCWSTR = ::windows_core::w!("ProviderName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDEXPIRATION: ::windows_core::PCWSTR = ::windows_core::w!("PwdExpiration");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_CHANGEORDER: ::windows_core::PCWSTR = ::windows_core::w!("ChangeOrder");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_CHANGEPWD: ::windows_core::PCWSTR = ::windows_core::w!("ChangePassword");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_CHANGEPWDHWND: ::windows_core::PCWSTR = ::windows_core::w!("ChangePasswordHwnd");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_DESC: ::windows_core::PCWSTR = ::windows_core::w!("Description");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_GETPWDSTATUS: ::windows_core::PCWSTR = ::windows_core::w!("GetPasswordStatus");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_ISNP: ::windows_core::PCWSTR = ::windows_core::w!("NetworkProvider");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_PWDPROVIDER_PATH: ::windows_core::PCWSTR = ::windows_core::w!("ProviderPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RDINTTHRESHOLD: ::windows_core::PCWSTR = ::windows_core::w!("RDIntThreshold");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_READAHEADTHRESHOLD: ::windows_core::PCWSTR = ::windows_core::w!("ReadAheadThreshold");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_READCACHING: ::windows_core::PCWSTR = ::windows_core::w!("ReadCaching");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REALNETSTART: ::windows_core::PCWSTR = ::windows_core::w!("RealNetStart");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REASONCODE: ::windows_core::PCWSTR = ::windows_core::w!("ReasonCode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REFRESHRATE: ::windows_core::PCWSTR = ::windows_core::w!("RefreshRate");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REGITEMDELETEMESSAGE: ::windows_core::PCWSTR = ::windows_core::w!("Removal Message");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REGORGANIZATION: ::windows_core::PCWSTR = ::windows_core::w!("RegisteredOrganization");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REGOWNER: ::windows_core::PCWSTR = ::windows_core::w!("RegisteredOwner");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REINSTALL_DEVICEINSTANCEIDS: ::windows_core::PCWSTR = ::windows_core::w!("DeviceInstanceIds");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REINSTALL_DISPLAYNAME: ::windows_core::PCWSTR = ::windows_core::w!("DisplayName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REINSTALL_STRING: ::windows_core::PCWSTR = ::windows_core::w!("ReinstallString");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REMOTE_PATH: ::windows_core::PCWSTR = ::windows_core::w!("RemotePath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REMOVABLE: ::windows_core::PCWSTR = ::windows_core::w!("Removable");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REMOVAL_POLICY: ::windows_core::PCWSTR = ::windows_core::w!("RemovalPolicy");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REMOVEROMOKAY: ::windows_core::PCWSTR = ::windows_core::w!("RemoveRomOkay");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REMOVEROMOKAYFUNC: ::windows_core::PCWSTR = ::windows_core::w!("RemoveRomOkayFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESERVED_DEVNODE: ::windows_core::PCWSTR = ::windows_core::w!("HTREE\\RESERVED\\0");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESOLUTION: ::windows_core::PCWSTR = ::windows_core::w!("Resolution");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESOURCES: ::windows_core::PCWSTR = ::windows_core::w!("Resources");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESOURCE_MAP: ::windows_core::PCWSTR = ::windows_core::w!("ResourceMap");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESOURCE_PICKER_EXCEPTIONS: ::windows_core::PCWSTR = ::windows_core::w!("ResourcePickerExceptions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESOURCE_PICKER_TAGS: ::windows_core::PCWSTR = ::windows_core::w!("ResourcePickerTags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESTRICTRUN: ::windows_core::PCWSTR = ::windows_core::w!("RestrictRun");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RESUMERESET: ::windows_core::PCWSTR = ::windows_core::w!("ResumeReset");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REVISION: ::windows_core::PCWSTR = ::windows_core::w!("Revision");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_REVLEVEL: ::windows_core::PCWSTR = ::windows_core::w!("RevisionLevel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_ROOT_DEVNODE: ::windows_core::PCWSTR = ::windows_core::w!("HTREE\\ROOT\\0");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_RUNLOGINSCRIPT: ::windows_core::PCWSTR = ::windows_core::w!("ProcessLoginScript");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SCANNER: ::windows_core::PCWSTR = ::windows_core::w!("SCANNER");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SCAN_ONLY_FIRST: ::windows_core::PCWSTR = ::windows_core::w!("ScanOnlyFirstDrive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SCSI: ::windows_core::PCWSTR = ::windows_core::w!("SCSI\\");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SCSILUN: ::windows_core::PCWSTR = ::windows_core::w!("SCSILUN");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SCSITID: ::windows_core::PCWSTR = ::windows_core::w!("SCSITargetID");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SEARCHMODE: ::windows_core::PCWSTR = ::windows_core::w!("SearchMode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SEARCHOPTIONS: ::windows_core::PCWSTR = ::windows_core::w!("SearchOptions");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SECCPL_NOADMINPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoAdminPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SECCPL_NOPROFILEPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoProfilePage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SECCPL_NOPWDPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoPwdPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SECCPL_NOSECCPL: ::windows_core::PCWSTR = ::windows_core::w!("NoSecCPL");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SERVICE: ::windows_core::PCWSTR = ::windows_core::w!("Service");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SETUPFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("SetupFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SETUPMACHINETYPE: ::windows_core::PCWSTR = ::windows_core::w!("SetupMachineType");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SETUPN: ::windows_core::PCWSTR = ::windows_core::w!("SetupN");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SETUPNPATH: ::windows_core::PCWSTR = ::windows_core::w!("SetupNPath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SETUPPROGRAMRAN: ::windows_core::PCWSTR = ::windows_core::w!("SetupProgramRan");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_FLAGS: ::windows_core::PCWSTR = ::windows_core::w!("Flags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_PATH: ::windows_core::PCWSTR = ::windows_core::w!("Path");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_REMARK: ::windows_core::PCWSTR = ::windows_core::w!("Remark");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_RO_PASS: ::windows_core::PCWSTR = ::windows_core::w!("Parm2");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_RW_PASS: ::windows_core::PCWSTR = ::windows_core::w!("Parm1");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARES_TYPE: ::windows_core::PCWSTR = ::windows_core::w!("Type");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHARE_IRQ: ::windows_core::PCWSTR = ::windows_core::w!("ForceIRQSharing");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHELLVERSION: ::windows_core::PCWSTR = ::windows_core::w!("ShellVersion");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHOWDOTS: ::windows_core::PCWSTR = ::windows_core::w!("ShowDots");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHOWREASONUI: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownReasonUI");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWNREASON: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownReason");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWNREASON_CODE: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownReasonCode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWNREASON_COMMENT: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownReasonComment");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWNREASON_PROCESS: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownReasonProcess");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWNREASON_USERNAME: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownReasonUserName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWN_FLAGS: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWN_IGNORE_PREDEFINED: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownIgnorePredefinedReasons");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SHUTDOWN_STATE_SNAPSHOT: ::windows_core::PCWSTR = ::windows_core::w!("ShutdownStateSnapshot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SILENTINSTALL: ::windows_core::PCWSTR = ::windows_core::w!("SilentInstall");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SLSUPPORT: ::windows_core::PCWSTR = ::windows_core::w!("SLSupport");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SOFTCOMPATMODE: ::windows_core::PCWSTR = ::windows_core::w!("SoftCompatMode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SRCPATH: ::windows_core::PCWSTR = ::windows_core::w!("SourcePath");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SRVNAMECACHE: ::windows_core::PCWSTR = ::windows_core::w!("ServerNameCache");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SRVNAMECACHECOUNT: ::windows_core::PCWSTR = ::windows_core::w!("ServerNameCacheMax");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SRVNAMECACHENETPROV: ::windows_core::PCWSTR = ::windows_core::w!("ServerNameCacheNumNets");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_START_ON_BOOT: ::windows_core::PCWSTR = ::windows_core::w!("StartOnBoot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_STAT: ::windows_core::PCWSTR = ::windows_core::w!("Status");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_STATICDRIVE: ::windows_core::PCWSTR = ::windows_core::w!("StaticDrive");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_STATICVXD: ::windows_core::PCWSTR = ::windows_core::w!("StaticVxD");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_STDDOSOPTION: ::windows_core::PCWSTR = ::windows_core::w!("StdOption");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SUBMODEL: ::windows_core::PCWSTR = ::windows_core::w!("Submodel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SUPPORTBURST: ::windows_core::PCWSTR = ::windows_core::w!("SupportBurst");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SUPPORTLFN: ::windows_core::PCWSTR = ::windows_core::w!("SupportLFN");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SUPPORTTUNNELLING: ::windows_core::PCWSTR = ::windows_core::w!("SupportTunnelling");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYMBOLIC_LINK: ::windows_core::PCWSTR = ::windows_core::w!("SymbolicLink");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYNCDATAXFER: ::windows_core::PCWSTR = ::windows_core::w!("SyncDataXfer");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSDM: ::windows_core::PCWSTR = ::windows_core::w!("SysDM");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSDMFUNC: ::windows_core::PCWSTR = ::windows_core::w!("SysDMFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTEMCPL_NOCONFIGPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoConfigPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTEMCPL_NODEVMGRPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoDevMgrPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTEMCPL_NOFILESYSPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoFileSysPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTEMCPL_NOVIRTMEMPAGE: ::windows_core::PCWSTR = ::windows_core::w!("NoVirtMemPage");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTEMROOT: ::windows_core::PCWSTR = ::windows_core::w!("SystemRoot");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTRAYBATFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("PowerFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTRAYPCCARDFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("PCMCIAFlags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_SYSTRAYSVCS: ::windows_core::PCWSTR = ::windows_core::w!("Services");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TABLE_STAT: ::windows_core::PCWSTR = ::windows_core::w!("TableStatus");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TAPE: ::windows_core::PCWSTR = ::windows_core::w!("TAPE");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TRANSITION: ::windows_core::PCWSTR = ::windows_core::w!("Transition");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TRANSPORT: ::windows_core::PCWSTR = ::windows_core::w!("Transport");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZACTBIAS: ::windows_core::PCWSTR = ::windows_core::w!("ActiveTimeBias");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZBIAS: ::windows_core::PCWSTR = ::windows_core::w!("Bias");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZDLTBIAS: ::windows_core::PCWSTR = ::windows_core::w!("DaylightBias");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZDLTFLAG: ::windows_core::PCWSTR = ::windows_core::w!("DaylightFlag");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZDLTNAME: ::windows_core::PCWSTR = ::windows_core::w!("DaylightName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZDLTSTART: ::windows_core::PCWSTR = ::windows_core::w!("DaylightStart");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZNOAUTOTIME: ::windows_core::PCWSTR = ::windows_core::w!("DisableAutoDaylightTimeSet");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZNOCHANGEEND: ::windows_core::PCWSTR = ::windows_core::w!("NoChangeEnd");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZNOCHANGESTART: ::windows_core::PCWSTR = ::windows_core::w!("NoChangeStart");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZSTDBIAS: ::windows_core::PCWSTR = ::windows_core::w!("StandardBias");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZSTDNAME: ::windows_core::PCWSTR = ::windows_core::w!("StandardName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_TZSTDSTART: ::windows_core::PCWSTR = ::windows_core::w!("StandardStart");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UI_NUMBER: ::windows_core::PCWSTR = ::windows_core::w!("UINumber");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UI_NUMBER_DESC_FORMAT: ::windows_core::PCWSTR = ::windows_core::w!("UINumberDescFormat");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UNDOCK_WITHOUT_LOGON: ::windows_core::PCWSTR = ::windows_core::w!("UndockWithoutLogon");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UNINSTALLER_COMMANDLINE: ::windows_core::PCWSTR = ::windows_core::w!("UninstallString");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UNINSTALLER_DISPLAYNAME: ::windows_core::PCWSTR = ::windows_core::w!("DisplayName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UPGRADE: ::windows_core::PCWSTR = ::windows_core::w!("Upgrade");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UPPERFILTERS: ::windows_core::PCWSTR = ::windows_core::w!("UpperFilters");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UPPER_FILTER_DEFAULT_LEVEL: ::windows_core::PCWSTR = ::windows_core::w!("UpperFilterDefaultLevel");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_UPPER_FILTER_LEVELS: ::windows_core::PCWSTR = ::windows_core::w!("UpperFilterLevels");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_USERSETTINGS: ::windows_core::PCWSTR = ::windows_core::w!("AdapterSettings");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_USER_NAME: ::windows_core::PCWSTR = ::windows_core::w!("UserName");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_USRDRVLET: ::windows_core::PCWSTR = ::windows_core::w!("UserDriveLetterAssignment");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VDD: ::windows_core::PCWSTR = ::windows_core::w!("vdd");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VER: ::windows_core::PCWSTR = ::windows_core::w!("Ver");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VERIFYKEY: ::windows_core::PCWSTR = ::windows_core::w!("VerifyKey");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VIRTUALHDIRQ: ::windows_core::PCWSTR = ::windows_core::w!("VirtualHDIRQ");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VOLIDLETIMEOUT: ::windows_core::PCWSTR = ::windows_core::w!("VolumeIdleTimeout");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VPOWERDFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("Flags");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VRES: ::windows_core::PCWSTR = ::windows_core::w!("VRes");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_VXDGROUPS: ::windows_core::PCWSTR = ::windows_core::w!("VXDGroups");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WAITFORUNDOCK: ::windows_core::PCWSTR = ::windows_core::w!("WaitForUndock");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WAITFORUNDOCKFUNC: ::windows_core::PCWSTR = ::windows_core::w!("WaitForUndockFunc");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WIN31FILESYSTEM: ::windows_core::PCWSTR = ::windows_core::w!("Win31FileSystem");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WIN31PROVIDER: ::windows_core::PCWSTR = ::windows_core::w!("Win31Provider");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WINBOOTDIR: ::windows_core::PCWSTR = ::windows_core::w!("WinbootDir");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WINCP: ::windows_core::PCWSTR = ::windows_core::w!("ACP");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WINDIR: ::windows_core::PCWSTR = ::windows_core::w!("WinDir");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WINOLDAPP_DISABLED: ::windows_core::PCWSTR = ::windows_core::w!("Disabled");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WINOLDAPP_NOREALMODE: ::windows_core::PCWSTR = ::windows_core::w!("NoRealMode");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WORKGROUP: ::windows_core::PCWSTR = ::windows_core::w!("Workgroup");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WRAPPER: ::windows_core::PCWSTR = ::windows_core::w!("Wrapper");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WRINTTHRESHOLD: ::windows_core::PCWSTR = ::windows_core::w!("WRIntThreshold");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WRKGRP_FORCEMAPPING: ::windows_core::PCWSTR = ::windows_core::w!("WrkgrpForceMapping");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REGSTR_VAL_WRKGRP_REQUIRED: ::windows_core::PCWSTR = ::windows_core::w!("WrkgrpRequired");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_BINARY: REG_VALUE_TYPE = REG_VALUE_TYPE(3u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_CREATED_NEW_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_DWORD: REG_VALUE_TYPE = REG_VALUE_TYPE(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_DWORD_BIG_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(5u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_DWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_EXPAND_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_FORCE_RESTORE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_FULL_RESOURCE_DESCRIPTOR: REG_VALUE_TYPE = REG_VALUE_TYPE(9u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_KEY_INSTDEV: ::windows_core::PCWSTR = ::windows_core::w!("Installed");
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_LATEST_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_LINK: REG_VALUE_TYPE = REG_VALUE_TYPE(6u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_MUI_STRING_TRUNCATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_MULTI_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(7u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NONE: REG_VALUE_TYPE = REG_VALUE_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NOTIFY_CHANGE_ATTRIBUTES: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NOTIFY_CHANGE_LAST_SET: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NOTIFY_CHANGE_NAME: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NOTIFY_CHANGE_SECURITY: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(8u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NOTIFY_THREAD_AGNOSTIC: REG_NOTIFY_FILTER = REG_NOTIFY_FILTER(268435456u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_NO_COMPRESSION: REG_SAVE_FORMAT = REG_SAVE_FORMAT(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPENED_EXISTING_KEY: REG_CREATE_KEY_DISPOSITION = REG_CREATE_KEY_DISPOSITION(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_BACKUP_RESTORE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_CREATE_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_DONT_VIRTUALIZE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(16u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_NON_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_OPEN_LINK: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(8u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_RESERVED: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_OPTION_VOLATILE: REG_OPEN_CREATE_OPTIONS = REG_OPEN_CREATE_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_PROCESS_APPKEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_QWORD: REG_VALUE_TYPE = REG_VALUE_TYPE(11u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_QWORD_LITTLE_ENDIAN: REG_VALUE_TYPE = REG_VALUE_TYPE(11u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_RESOURCE_LIST: REG_VALUE_TYPE = REG_VALUE_TYPE(8u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_RESOURCE_REQUIREMENTS_LIST: REG_VALUE_TYPE = REG_VALUE_TYPE(10u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_SECURE_CONNECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_STANDARD_FORMAT: REG_SAVE_FORMAT = REG_SAVE_FORMAT(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_SZ: REG_VALUE_TYPE = REG_VALUE_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_USE_CURRENT_SECURITY_CONTEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const REG_WHOLE_HIVE_VOLATILE: REG_RESTORE_KEY_FLAGS = REG_RESTORE_KEY_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_NOEXPAND: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_ANY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(65535u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_DWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(24u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_QWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(72u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_BINARY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_DWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_EXPAND_SZ: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_MULTI_SZ: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_NONE: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_QWORD: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_RT_REG_SZ: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_SUBKEY_WOW6432KEY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_SUBKEY_WOW6464KEY: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_WOW64_MASK: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(196608u32);
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub const RRF_ZEROONFAILURE: REG_ROUTINE_FLAGS = REG_ROUTINE_FLAGS(536870912u32);
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
impl ::windows_core::TypeKind for REG_CREATE_KEY_DISPOSITION {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for REG_NOTIFY_FILTER {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for REG_OPEN_CREATE_OPTIONS {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for REG_RESTORE_KEY_FLAGS {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for REG_ROUTINE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for REG_SAM_FLAGS {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for REG_SAVE_FORMAT {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for REG_VALUE_TYPE {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for DSKTLSYSTEMTIME {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for HKEY {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub struct PVALUEA {
    pub pv_valuename: ::windows_core::PSTR,
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
impl ::windows_core::TypeKind for PVALUEA {
    type TypeKind = ::windows_core::CopyType;
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
    pub pv_valuename: ::windows_core::PWSTR,
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
impl ::windows_core::TypeKind for PVALUEW {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for REG_PROVIDER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for REG_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Registry\"`*"]
pub struct VALENTA {
    pub ve_valuename: ::windows_core::PSTR,
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
impl ::windows_core::TypeKind for VALENTA {
    type TypeKind = ::windows_core::CopyType;
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
    pub ve_valuename: ::windows_core::PWSTR,
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
impl ::windows_core::TypeKind for VALENTW {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for val_context {
    type TypeKind = ::windows_core::CopyType;
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
