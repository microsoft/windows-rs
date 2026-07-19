#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor : super::PSECURITY_DESCRIPTOR, requestedstringsdrevision : u32, securityinformation : super::SECURITY_INFORMATION, stringsecuritydescriptor : *mut windows_sys::core::PSTR, stringsecuritydescriptorlen : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor : super::PSECURITY_DESCRIPTOR, requestedstringsdrevision : u32, securityinformation : super::SECURITY_INFORMATION, stringsecuritydescriptor : *mut windows_sys::core::PWSTR, stringsecuritydescriptorlen : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ConvertSidToStringSidA(sid : super::PSID, stringsid : *mut windows_sys::core::PSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ConvertSidToStringSidW(sid : super::PSID, stringsid : *mut windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor : windows_sys::core::PCSTR, stringsdrevision : u32, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor : windows_sys::core::PCWSTR, stringsdrevision : u32, securitydescriptor : *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ConvertStringSidToSidA(stringsid : windows_sys::core::PCSTR, sid : *mut super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ConvertStringSidToSidW(stringsid : windows_sys::core::PCWSTR, sid : *mut super::PSID) -> windows_sys::core::BOOL);
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
