#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn MSChapSrvChangePassword(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, lmoldpresent: super::super::Foundation::BOOLEAN, lmoldowfpassword: *const LM_OWF_PASSWORD, lmnewowfpassword: *const LM_OWF_PASSWORD, ntoldowfpassword: *const LM_OWF_PASSWORD, ntnewowfpassword: *const LM_OWF_PASSWORD) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MSChapSrvChangePassword2(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent: super::super::Foundation::BOOLEAN, newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CYPHER_BLOCK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ENCRYPTED_LM_OWF_PASSWORD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LM_OWF_PASSWORD(i32);
#[repr(C)]
pub struct SAMPR_ENCRYPTED_USER_PASSWORD(i32);
