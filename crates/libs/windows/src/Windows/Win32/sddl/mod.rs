#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, requestedstringsdrevision: u32, securityinformation: super::winnt::SECURITY_INFORMATION, stringsecuritydescriptor: *mut windows_core::PSTR, stringsecuritydescriptorlen: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, requestedstringsdrevision : u32, securityinformation : super::winnt::SECURITY_INFORMATION, stringsecuritydescriptor : *mut windows_core::PSTR, stringsecuritydescriptorlen : *mut u32) -> windows_core::BOOL);
    unsafe { ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor, requestedstringsdrevision, securityinformation, stringsecuritydescriptor as _, stringsecuritydescriptorlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, requestedstringsdrevision: u32, securityinformation: super::winnt::SECURITY_INFORMATION, stringsecuritydescriptor: *mut windows_core::PWSTR, stringsecuritydescriptorlen: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, requestedstringsdrevision : u32, securityinformation : super::winnt::SECURITY_INFORMATION, stringsecuritydescriptor : *mut windows_core::PWSTR, stringsecuritydescriptorlen : *mut u32) -> windows_core::BOOL);
    unsafe { ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor, requestedstringsdrevision, securityinformation, stringsecuritydescriptor as _, stringsecuritydescriptorlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertSidToStringSidA(sid: super::winnt::PSID, stringsid: *mut windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ConvertSidToStringSidA(sid : super::winnt::PSID, stringsid : *mut windows_core::PSTR) -> windows_core::BOOL);
    unsafe { ConvertSidToStringSidA(sid, stringsid as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertSidToStringSidW(sid: super::winnt::PSID, stringsid: *mut windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ConvertSidToStringSidW(sid : super::winnt::PSID, stringsid : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { ConvertSidToStringSidW(sid, stringsid as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorA<P0>(stringsecuritydescriptor: P0, stringsdrevision: u32, securitydescriptor: *mut super::winnt::PSECURITY_DESCRIPTOR, securitydescriptorsize: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor : windows_core::PCSTR, stringsdrevision : u32, securitydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR, securitydescriptorsize : *mut u32) -> windows_core::BOOL);
    unsafe { ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor.param().abi(), stringsdrevision, securitydescriptor as _, securitydescriptorsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorW<P0>(stringsecuritydescriptor: P0, stringsdrevision: u32, securitydescriptor: *mut super::winnt::PSECURITY_DESCRIPTOR, securitydescriptorsize: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor : windows_core::PCWSTR, stringsdrevision : u32, securitydescriptor : *mut super::winnt::PSECURITY_DESCRIPTOR, securitydescriptorsize : *mut u32) -> windows_core::BOOL);
    unsafe { ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor.param().abi(), stringsdrevision, securitydescriptor as _, securitydescriptorsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertStringSidToSidA<P0>(stringsid: P0, sid: *mut super::winnt::PSID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSidToSidA(stringsid : windows_core::PCSTR, sid : *mut super::winnt::PSID) -> windows_core::BOOL);
    unsafe { ConvertStringSidToSidA(stringsid.param().abi(), sid as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ConvertStringSidToSidW<P0>(stringsid: P0, sid: *mut super::winnt::PSID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ConvertStringSidToSidW(stringsid : windows_core::PCWSTR, sid : *mut super::winnt::PSID) -> windows_core::BOOL);
    unsafe { ConvertStringSidToSidW(stringsid.param().abi(), sid as _) }
}
pub const SDDL_ACE_BEGINC: u32 = 40;
pub const SDDL_ACE_COND_BEGINC: u32 = 40;
pub const SDDL_ACE_COND_BLOB_PREFIXC: u32 = 35;
pub const SDDL_ACE_COND_COMPOSITEVALUE_BEGINC: u32 = 123;
pub const SDDL_ACE_COND_COMPOSITEVALUE_ENDC: u32 = 125;
pub const SDDL_ACE_COND_COMPOSITEVALUE_SEPERATORC: u32 = 44;
pub const SDDL_ACE_COND_ENDC: u32 = 41;
pub const SDDL_ACE_COND_SID_BEGINC: u32 = 40;
pub const SDDL_ACE_COND_SID_ENDC: u32 = 41;
pub const SDDL_ACE_COND_STRING_BEGINC: u32 = 34;
pub const SDDL_ACE_COND_STRING_ENDC: u32 = 34;
pub const SDDL_ACE_ENDC: u32 = 41;
pub const SDDL_ALIAS_SIZE: u32 = 2;
pub const SDDL_DELIMINATORC: u32 = 58;
pub const SDDL_REVISION: u32 = 1;
pub const SDDL_REVISION_1: u32 = 1;
pub const SDDL_SEPERATORC: u32 = 59;
pub const SDDL_SPACEC: u32 = 32;
