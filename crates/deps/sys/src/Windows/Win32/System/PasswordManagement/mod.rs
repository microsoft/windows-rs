#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_PasswordManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MSChapSrvChangePassword(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, lmoldpresent: super::super::Foundation::BOOLEAN, lmoldowfpassword: *const LM_OWF_PASSWORD, lmnewowfpassword: *const LM_OWF_PASSWORD, ntoldowfpassword: *const LM_OWF_PASSWORD, ntnewowfpassword: *const LM_OWF_PASSWORD) -> u32;
    #[doc = "*Required features: `Win32_System_PasswordManagement`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MSChapSrvChangePassword2(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent: super::super::Foundation::BOOLEAN, newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
pub struct CYPHER_BLOCK(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ENCRYPTED_LM_OWF_PASSWORD(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct LM_OWF_PASSWORD(i32);
pub struct SAMPR_ENCRYPTED_USER_PASSWORD(i32);
