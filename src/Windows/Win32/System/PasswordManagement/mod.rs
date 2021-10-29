#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CYPHER_BLOCK {
    pub data: [super::super::Foundation::CHAR; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl CYPHER_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CYPHER_BLOCK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CYPHER_BLOCK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CYPHER_BLOCK").field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CYPHER_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CYPHER_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CYPHER_BLOCK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ENCRYPTED_LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ENCRYPTED_LM_OWF_PASSWORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ENCRYPTED_LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ENCRYPTED_LM_OWF_PASSWORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENCRYPTED_LM_OWF_PASSWORD").field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ENCRYPTED_LM_OWF_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ENCRYPTED_LM_OWF_PASSWORD {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENCRYPTED_LM_OWF_PASSWORD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct LM_OWF_PASSWORD {
    pub data: [CYPHER_BLOCK; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl LM_OWF_PASSWORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LM_OWF_PASSWORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LM_OWF_PASSWORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LM_OWF_PASSWORD").field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LM_OWF_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LM_OWF_PASSWORD {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LM_OWF_PASSWORD {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MSChapSrvChangePassword<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(
    servername: Param0,
    username: Param1,
    lmoldpresent: Param2,
    lmoldowfpassword: *const LM_OWF_PASSWORD,
    lmnewowfpassword: *const LM_OWF_PASSWORD,
    ntoldowfpassword: *const LM_OWF_PASSWORD,
    ntnewowfpassword: *const LM_OWF_PASSWORD,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MSChapSrvChangePassword(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, lmoldpresent: super::super::Foundation::BOOLEAN, lmoldowfpassword: *const LM_OWF_PASSWORD, lmnewowfpassword: *const LM_OWF_PASSWORD, ntoldowfpassword: *const LM_OWF_PASSWORD, ntnewowfpassword: *const LM_OWF_PASSWORD) -> u32;
        }
        ::std::mem::transmute(MSChapSrvChangePassword(servername.into_param().abi(), username.into_param().abi(), lmoldpresent.into_param().abi(), ::std::mem::transmute(lmoldowfpassword), ::std::mem::transmute(lmnewowfpassword), ::std::mem::transmute(ntoldowfpassword), ::std::mem::transmute(ntnewowfpassword)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MSChapSrvChangePassword2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(
    servername: Param0,
    username: Param1,
    newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD,
    oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD,
    lmpresent: Param4,
    newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD,
    oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MSChapSrvChangePassword2(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, newpasswordencryptedwitholdnt: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldntowfpasswordencryptedwithnewnt: *const ENCRYPTED_LM_OWF_PASSWORD, lmpresent: super::super::Foundation::BOOLEAN, newpasswordencryptedwitholdlm: *const SAMPR_ENCRYPTED_USER_PASSWORD, oldlmowfpasswordencryptedwithnewlmornt: *const ENCRYPTED_LM_OWF_PASSWORD) -> u32;
        }
        ::std::mem::transmute(MSChapSrvChangePassword2(
            servername.into_param().abi(),
            username.into_param().abi(),
            ::std::mem::transmute(newpasswordencryptedwitholdnt),
            ::std::mem::transmute(oldntowfpasswordencryptedwithnewnt),
            lmpresent.into_param().abi(),
            ::std::mem::transmute(newpasswordencryptedwitholdlm),
            ::std::mem::transmute(oldlmowfpasswordencryptedwithnewlmornt),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SAMPR_ENCRYPTED_USER_PASSWORD {
    pub Buffer: [u8; 516],
}
impl SAMPR_ENCRYPTED_USER_PASSWORD {}
impl ::std::default::Default for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAMPR_ENCRYPTED_USER_PASSWORD").field("Buffer", &self.Buffer).finish()
    }
}
impl ::std::cmp::PartialEq for SAMPR_ENCRYPTED_USER_PASSWORD {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer
    }
}
impl ::std::cmp::Eq for SAMPR_ENCRYPTED_USER_PASSWORD {}
unsafe impl ::windows::runtime::Abi for SAMPR_ENCRYPTED_USER_PASSWORD {
    type Abi = Self;
    type DefaultType = Self;
}
