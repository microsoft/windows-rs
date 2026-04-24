#[inline]
pub unsafe fn MSChapSrvChangePassword(servername: windows_core::PCWSTR, username: windows_core::PCWSTR, lmoldpresent: bool, lmoldowfpassword: *const LM_OWF_PASSWORD, lmnewowfpassword: *const LM_OWF_PASSWORD, ntoldowfpassword: *const LM_OWF_PASSWORD, ntnewowfpassword: *const LM_OWF_PASSWORD) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn MSChapSrvChangePassword(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, lmoldpresent : bool, lmoldowfpassword : *const LM_OWF_PASSWORD, lmnewowfpassword : *const LM_OWF_PASSWORD, ntoldowfpassword : *const LM_OWF_PASSWORD, ntnewowfpassword : *const LM_OWF_PASSWORD) -> u32);
    unsafe { MSChapSrvChangePassword(core::mem::transmute(servername), core::mem::transmute(username), lmoldpresent, lmoldowfpassword, lmnewowfpassword, ntoldowfpassword, ntnewowfpassword) }
}
#[inline]
pub unsafe fn MSChapSrvChangePassword2(servername: windows_core::PCWSTR, username: windows_core::PCWSTR, newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent: bool, newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn MSChapSrvChangePassword2(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, newpasswordencryptedwitholdnt : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt : *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent : bool, newpasswordencryptedwitholdlm : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt : *const ENCRYPTED_LM_OWF_PASSWORD) -> u32);
    unsafe { MSChapSrvChangePassword2(core::mem::transmute(servername), core::mem::transmute(username), newpasswordencryptedwitholdnt, oldntowfpasswordencryptedwithnewnt, lmpresent, newpasswordencryptedwitholdlm, oldlmowfpasswordencryptedwithnewlmornt) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CYPHER_BLOCK {
    pub data: [i8; 8],
}
impl Default for CYPHER_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCRYPTED_LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
impl Default for ENCRYPTED_LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
impl Default for LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAMPR_ENCRYPTED_USER_PASSWORD {
    pub Buffer: [u8; 516],
}
impl Default for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
