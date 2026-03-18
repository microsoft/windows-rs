#[inline]
pub unsafe fn MSChapSrvChangePassword<P0, P1>(servername: P0, username: P1, lmoldpresent: bool, lmoldowfpassword: *mut LM_OWF_PASSWORD, lmnewowfpassword: *mut LM_OWF_PASSWORD, ntoldowfpassword: *mut LM_OWF_PASSWORD, ntnewowfpassword: *mut LM_OWF_PASSWORD) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn MSChapSrvChangePassword(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, lmoldpresent : bool, lmoldowfpassword : *mut LM_OWF_PASSWORD, lmnewowfpassword : *mut LM_OWF_PASSWORD, ntoldowfpassword : *mut LM_OWF_PASSWORD, ntnewowfpassword : *mut LM_OWF_PASSWORD) -> u32);
    unsafe { MSChapSrvChangePassword(servername.param().abi(), username.param().abi(), lmoldpresent, lmoldowfpassword as _, lmnewowfpassword as _, ntoldowfpassword as _, ntnewowfpassword as _) }
}
#[inline]
pub unsafe fn MSChapSrvChangePassword2<P0, P1>(servername: P0, username: P1, newpasswordencryptedwitholdnt: *mut SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *mut ENCRYPTED_LM_OWF_PASSWORD, lmpresent: bool, newpasswordencryptedwitholdlm: *mut SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *mut ENCRYPTED_LM_OWF_PASSWORD) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn MSChapSrvChangePassword2(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, newpasswordencryptedwitholdnt : *mut SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt : *mut ENCRYPTED_LM_OWF_PASSWORD, lmpresent : bool, newpasswordencryptedwitholdlm : *mut SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt : *mut ENCRYPTED_LM_OWF_PASSWORD) -> u32);
    unsafe { MSChapSrvChangePassword2(servername.param().abi(), username.param().abi(), newpasswordencryptedwitholdnt as _, oldntowfpasswordencryptedwithnewnt as _, lmpresent, newpasswordencryptedwitholdlm as _, oldlmowfpasswordencryptedwithnewlmornt as _) }
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
