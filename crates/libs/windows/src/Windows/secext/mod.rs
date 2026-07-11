#[inline]
pub unsafe fn GetComputerObjectNameA(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: Option<windows_core::PSTR>, nsize: *mut u32) -> bool {
    windows_core::link!("secur32.dll" "system" fn GetComputerObjectNameA(nameformat : EXTENDED_NAME_FORMAT, lpnamebuffer : windows_core::PSTR, nsize : *mut u32) -> bool);
    unsafe { GetComputerObjectNameA(nameformat, lpnamebuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn GetComputerObjectNameW(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: Option<windows_core::PWSTR>, nsize: *mut u32) -> bool {
    windows_core::link!("secur32.dll" "system" fn GetComputerObjectNameW(nameformat : EXTENDED_NAME_FORMAT, lpnamebuffer : windows_core::PWSTR, nsize : *mut u32) -> bool);
    unsafe { GetComputerObjectNameW(nameformat, lpnamebuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn GetUserNameExA(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: Option<windows_core::PSTR>, nsize: *mut u32) -> bool {
    windows_core::link!("secur32.dll" "system" fn GetUserNameExA(nameformat : EXTENDED_NAME_FORMAT, lpnamebuffer : windows_core::PSTR, nsize : *mut u32) -> bool);
    unsafe { GetUserNameExA(nameformat, lpnamebuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn GetUserNameExW(nameformat: EXTENDED_NAME_FORMAT, lpnamebuffer: Option<windows_core::PWSTR>, nsize: *mut u32) -> bool {
    windows_core::link!("secur32.dll" "system" fn GetUserNameExW(nameformat : EXTENDED_NAME_FORMAT, lpnamebuffer : windows_core::PWSTR, nsize : *mut u32) -> bool);
    unsafe { GetUserNameExW(nameformat, lpnamebuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn TranslateNameA<P0>(lpaccountname: P0, accountnameformat: EXTENDED_NAME_FORMAT, desirednameformat: EXTENDED_NAME_FORMAT, lptranslatedname: Option<windows_core::PSTR>, nsize: *mut u32) -> bool
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("secur32.dll" "system" fn TranslateNameA(lpaccountname : windows_core::PCSTR, accountnameformat : EXTENDED_NAME_FORMAT, desirednameformat : EXTENDED_NAME_FORMAT, lptranslatedname : windows_core::PSTR, nsize : *mut u32) -> bool);
    unsafe { TranslateNameA(lpaccountname.param().abi(), accountnameformat, desirednameformat, lptranslatedname.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn TranslateNameW<P0>(lpaccountname: P0, accountnameformat: EXTENDED_NAME_FORMAT, desirednameformat: EXTENDED_NAME_FORMAT, lptranslatedname: Option<windows_core::PWSTR>, nsize: *mut u32) -> bool
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("secur32.dll" "system" fn TranslateNameW(lpaccountname : windows_core::PCWSTR, accountnameformat : EXTENDED_NAME_FORMAT, desirednameformat : EXTENDED_NAME_FORMAT, lptranslatedname : windows_core::PWSTR, nsize : *mut u32) -> bool);
    unsafe { TranslateNameW(lpaccountname.param().abi(), accountnameformat, desirednameformat, lptranslatedname.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
pub type EXTENDED_NAME_FORMAT = i32;
pub const NameCanonical: EXTENDED_NAME_FORMAT = 7;
pub const NameCanonicalEx: EXTENDED_NAME_FORMAT = 9;
pub const NameDisplay: EXTENDED_NAME_FORMAT = 3;
pub const NameDnsDomain: EXTENDED_NAME_FORMAT = 12;
pub const NameFullyQualifiedDN: EXTENDED_NAME_FORMAT = 1;
pub const NameGivenName: EXTENDED_NAME_FORMAT = 13;
pub const NameSamCompatible: EXTENDED_NAME_FORMAT = 2;
pub const NameServicePrincipal: EXTENDED_NAME_FORMAT = 10;
pub const NameSurname: EXTENDED_NAME_FORMAT = 14;
pub const NameUniqueId: EXTENDED_NAME_FORMAT = 6;
pub const NameUnknown: EXTENDED_NAME_FORMAT = 0;
pub const NameUserPrincipal: EXTENDED_NAME_FORMAT = 8;
pub type PEXTENDED_NAME_FORMAT = *mut EXTENDED_NAME_FORMAT;
