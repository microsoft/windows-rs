#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_PasswordManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MSChapSrvChangePassword(servername: ::windows_sys::core::PCWSTR, username: ::windows_sys::core::PCWSTR, lmoldpresent: super::super::Foundation::BOOLEAN, lmoldowfpassword: *const LM_OWF_PASSWORD, lmnewowfpassword: *const LM_OWF_PASSWORD, ntoldowfpassword: *const LM_OWF_PASSWORD, ntnewowfpassword: *const LM_OWF_PASSWORD) -> u32;
    #[doc = "*Required features: `\"Win32_System_PasswordManagement\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MSChapSrvChangePassword2(servername: ::windows_sys::core::PCWSTR, username: ::windows_sys::core::PCWSTR, newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent: super::super::Foundation::BOOLEAN, newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD) -> u32;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_PasswordManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CYPHER_BLOCK {
    pub data: [super::super::Foundation::CHAR; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CYPHER_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CYPHER_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_PasswordManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENCRYPTED_LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENCRYPTED_LM_OWF_PASSWORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENCRYPTED_LM_OWF_PASSWORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_PasswordManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LM_OWF_PASSWORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LM_OWF_PASSWORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_PasswordManagement\"`*"]
pub struct SAMPR_ENCRYPTED_USER_PASSWORD {
    pub Buffer: [u8; 516],
}
impl ::core::marker::Copy for SAMPR_ENCRYPTED_USER_PASSWORD {}
impl ::core::clone::Clone for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn clone(&self) -> Self {
        *self
    }
}
