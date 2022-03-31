#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CYPHER_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CYPHER_BLOCK").field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CYPHER_BLOCK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CYPHER_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CYPHER_BLOCK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CYPHER_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CYPHER_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENCRYPTED_LM_OWF_PASSWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTED_LM_OWF_PASSWORD").field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENCRYPTED_LM_OWF_PASSWORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENCRYPTED_LM_OWF_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENCRYPTED_LM_OWF_PASSWORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENCRYPTED_LM_OWF_PASSWORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENCRYPTED_LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LM_OWF_PASSWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LM_OWF_PASSWORD").field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LM_OWF_PASSWORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LM_OWF_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LM_OWF_PASSWORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LM_OWF_PASSWORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_PasswordManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MSChapSrvChangePassword<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(servername: Param0, username: Param1, lmoldpresent: Param2, lmoldowfpassword: *const LM_OWF_PASSWORD, lmnewowfpassword: *const LM_OWF_PASSWORD, ntoldowfpassword: *const LM_OWF_PASSWORD, ntnewowfpassword: *const LM_OWF_PASSWORD) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MSChapSrvChangePassword(servername: ::windows::core::PCWSTR, username: ::windows::core::PCWSTR, lmoldpresent: super::super::Foundation::BOOLEAN, lmoldowfpassword: *const LM_OWF_PASSWORD, lmnewowfpassword: *const LM_OWF_PASSWORD, ntoldowfpassword: *const LM_OWF_PASSWORD, ntnewowfpassword: *const LM_OWF_PASSWORD) -> u32;
        }
        ::core::mem::transmute(MSChapSrvChangePassword(servername.into_param().abi(), username.into_param().abi(), lmoldpresent.into_param().abi(), ::core::mem::transmute(lmoldowfpassword), ::core::mem::transmute(lmnewowfpassword), ::core::mem::transmute(ntoldowfpassword), ::core::mem::transmute(ntnewowfpassword)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_PasswordManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MSChapSrvChangePassword2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(servername: Param0, username: Param1, newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent: Param4, newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MSChapSrvChangePassword2(servername: ::windows::core::PCWSTR, username: ::windows::core::PCWSTR, newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent: super::super::Foundation::BOOLEAN, newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD) -> u32;
        }
        ::core::mem::transmute(MSChapSrvChangePassword2(servername.into_param().abi(), username.into_param().abi(), ::core::mem::transmute(newpasswordencryptedwitholdnt), ::core::mem::transmute(oldntowfpasswordencryptedwithnewnt), lmpresent.into_param().abi(), ::core::mem::transmute(newpasswordencryptedwitholdlm), ::core::mem::transmute(oldlmowfpasswordencryptedwithnewlmornt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
unsafe impl ::windows::core::Abi for SAMPR_ENCRYPTED_USER_PASSWORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAMPR_ENCRYPTED_USER_PASSWORD>()) == 0 }
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
