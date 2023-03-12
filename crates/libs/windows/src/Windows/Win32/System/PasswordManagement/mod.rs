#[doc = "*Required features: `\"Win32_System_PasswordManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MSChapSrvChangePassword<P0, P1, P2>(servername: P0, username: P1, lmoldpresent: P2, lmoldowfpassword: *const LM_OWF_PASSWORD, lmnewowfpassword: *const LM_OWF_PASSWORD, ntoldowfpassword: *const LM_OWF_PASSWORD, ntnewowfpassword: *const LM_OWF_PASSWORD) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn MSChapSrvChangePassword ( servername : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR , lmoldpresent : super::super::Foundation:: BOOLEAN , lmoldowfpassword : *const LM_OWF_PASSWORD , lmnewowfpassword : *const LM_OWF_PASSWORD , ntoldowfpassword : *const LM_OWF_PASSWORD , ntnewowfpassword : *const LM_OWF_PASSWORD ) -> u32 );
    MSChapSrvChangePassword(servername.into_param().abi(), username.into_param().abi(), lmoldpresent.into_param().abi(), lmoldowfpassword, lmnewowfpassword, ntoldowfpassword, ntnewowfpassword)
}
#[doc = "*Required features: `\"Win32_System_PasswordManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MSChapSrvChangePassword2<P0, P1, P2>(servername: P0, username: P1, newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent: P2, newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows::imp::link ! ( "advapi32.dll""system" fn MSChapSrvChangePassword2 ( servername : :: windows::core::PCWSTR , username : :: windows::core::PCWSTR , newpasswordencryptedwitholdnt : *const SAMPR_ENCRYPTED_USER_PASSWORD , oldntowfpasswordencryptedwithnewnt : *const ENCRYPTED_LM_OWF_PASSWORD , lmpresent : super::super::Foundation:: BOOLEAN , newpasswordencryptedwitholdlm : *const SAMPR_ENCRYPTED_USER_PASSWORD , oldlmowfpasswordencryptedwithnewlmornt : *const ENCRYPTED_LM_OWF_PASSWORD ) -> u32 );
    MSChapSrvChangePassword2(servername.into_param().abi(), username.into_param().abi(), newpasswordencryptedwitholdnt, oldntowfpasswordencryptedwithnewnt, lmpresent.into_param().abi(), newpasswordencryptedwitholdlm, oldlmowfpasswordencryptedwithnewlmornt)
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_PasswordManagement\"`*"]
pub struct CYPHER_BLOCK {
    pub data: [u8; 8],
}
impl ::core::marker::Copy for CYPHER_BLOCK {}
impl ::core::clone::Clone for CYPHER_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CYPHER_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CYPHER_BLOCK").field("data", &self.data).finish()
    }
}
impl ::windows::core::TypeKind for CYPHER_BLOCK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CYPHER_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for CYPHER_BLOCK {}
impl ::core::default::Default for CYPHER_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_PasswordManagement\"`*"]
pub struct ENCRYPTED_LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
impl ::core::marker::Copy for ENCRYPTED_LM_OWF_PASSWORD {}
impl ::core::clone::Clone for ENCRYPTED_LM_OWF_PASSWORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENCRYPTED_LM_OWF_PASSWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTED_LM_OWF_PASSWORD").field("data", &self.data).finish()
    }
}
impl ::windows::core::TypeKind for ENCRYPTED_LM_OWF_PASSWORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ENCRYPTED_LM_OWF_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for ENCRYPTED_LM_OWF_PASSWORD {}
impl ::core::default::Default for ENCRYPTED_LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_PasswordManagement\"`*"]
pub struct LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
impl ::core::marker::Copy for LM_OWF_PASSWORD {}
impl ::core::clone::Clone for LM_OWF_PASSWORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LM_OWF_PASSWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LM_OWF_PASSWORD").field("data", &self.data).finish()
    }
}
impl ::windows::core::TypeKind for LM_OWF_PASSWORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LM_OWF_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl ::core::cmp::Eq for LM_OWF_PASSWORD {}
impl ::core::default::Default for LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAMPR_ENCRYPTED_USER_PASSWORD").field("Buffer", &self.Buffer).finish()
    }
}
impl ::windows::core::TypeKind for SAMPR_ENCRYPTED_USER_PASSWORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for SAMPR_ENCRYPTED_USER_PASSWORD {}
impl ::core::default::Default for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
