#[inline]
pub unsafe fn MSChapSrvChangePassword<P0, P1, P2>(servername: P0, username: P1, lmoldpresent: P2, lmoldowfpassword: *const LM_OWF_PASSWORD, lmnewowfpassword: *const LM_OWF_PASSWORD, ntoldowfpassword: *const LM_OWF_PASSWORD, ntnewowfpassword: *const LM_OWF_PASSWORD) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("advapi32.dll" "system" fn MSChapSrvChangePassword(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, lmoldpresent : super::super::Foundation:: BOOLEAN, lmoldowfpassword : *const LM_OWF_PASSWORD, lmnewowfpassword : *const LM_OWF_PASSWORD, ntoldowfpassword : *const LM_OWF_PASSWORD, ntnewowfpassword : *const LM_OWF_PASSWORD) -> u32);
    MSChapSrvChangePassword(servername.param().abi(), username.param().abi(), lmoldpresent.param().abi(), core::mem::transmute(lmoldowfpassword), core::mem::transmute(lmnewowfpassword), core::mem::transmute(ntoldowfpassword), core::mem::transmute(ntnewowfpassword))
}
#[inline]
pub unsafe fn MSChapSrvChangePassword2<P0, P1, P4>(servername: P0, username: P1, newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent: P4, newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<super::super::Foundation::BOOLEAN>,
{
    windows_targets::link!("advapi32.dll" "system" fn MSChapSrvChangePassword2(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, newpasswordencryptedwitholdnt : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt : *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent : super::super::Foundation:: BOOLEAN, newpasswordencryptedwitholdlm : *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt : *const ENCRYPTED_LM_OWF_PASSWORD) -> u32);
    MSChapSrvChangePassword2(servername.param().abi(), username.param().abi(), core::mem::transmute(newpasswordencryptedwitholdnt), core::mem::transmute(oldntowfpasswordencryptedwithnewnt), lmpresent.param().abi(), core::mem::transmute(newpasswordencryptedwitholdlm), core::mem::transmute(oldlmowfpasswordencryptedwithnewlmornt))
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
impl windows_core::TypeKind for CYPHER_BLOCK {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for ENCRYPTED_LM_OWF_PASSWORD {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for LM_OWF_PASSWORD {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for SAMPR_ENCRYPTED_USER_PASSWORD {
    type TypeKind = windows_core::CopyType;
}
