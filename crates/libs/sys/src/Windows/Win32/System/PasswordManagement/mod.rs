windows_targets::link!("advapi32.dll" "system" fn MSChapSrvChangePassword(servername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, lmoldpresent : super::super::Foundation:: BOOLEAN, lmoldowfpassword : *const LM_OWF_PASSWORD, lmnewowfpassword : *const LM_OWF_PASSWORD, ntoldowfpassword : *const LM_OWF_PASSWORD, ntnewowfpassword : *const LM_OWF_PASSWORD) -> u32);
windows_targets::link!("advapi32.dll" "system" fn MSChapSrvChangePassword2(servername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, newpasswordencryptedwitholdnt : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt : *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent : super::super::Foundation:: BOOLEAN, newpasswordencryptedwitholdlm : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt : *const ENCRYPTED_LM_OWF_PASSWORD) -> u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CYPHER_BLOCK {
    pub data: [i8; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENCRYPTED_LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAMPR_ENCRYPTED_USER_PASSWORD {
    pub Buffer: [u8; 516],
}
