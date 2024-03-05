::windows_targets::link!("advapi32.dll" "system" fn MSChapSrvChangePassword(servername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, lmoldpresent : super::super::Foundation:: BOOLEAN, lmoldowfpassword : *const LM_OWF_PASSWORD, lmnewowfpassword : *const LM_OWF_PASSWORD, ntoldowfpassword : *const LM_OWF_PASSWORD, ntnewowfpassword : *const LM_OWF_PASSWORD) -> u32);
::windows_targets::link!("advapi32.dll" "system" fn MSChapSrvChangePassword2(servername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, newpasswordencryptedwitholdnt : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt : *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent : super::super::Foundation:: BOOLEAN, newpasswordencryptedwitholdlm : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt : *const ENCRYPTED_LM_OWF_PASSWORD) -> u32);
#[repr(C)]
pub struct CYPHER_BLOCK {
    pub data: [i8; 8],
}
impl Copy for CYPHER_BLOCK {}
impl Clone for CYPHER_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ENCRYPTED_LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
impl Copy for ENCRYPTED_LM_OWF_PASSWORD {}
impl Clone for ENCRYPTED_LM_OWF_PASSWORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
impl Copy for LM_OWF_PASSWORD {}
impl Clone for LM_OWF_PASSWORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SAMPR_ENCRYPTED_USER_PASSWORD {
    pub Buffer: [u8; 516],
}
impl Copy for SAMPR_ENCRYPTED_USER_PASSWORD {}
impl Clone for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn clone(&self) -> Self {
        *self
    }
}
