#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_AUDIT_ALL: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_A_ACL: u32 = 32768u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_A_CREATE: u32 = 8192u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_A_DELETE: u32 = 16384u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_A_OPEN: u32 = 4096u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_A_OWNER: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_A_WRITE: u32 = 8192u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_CLOSE: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_F_ACL: u32 = 2048u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_F_CREATE: u32 = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_F_DELETE: u32 = 1024u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_F_OPEN: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_F_WRITE: u32 = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_S_ACL: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_S_CREATE: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_S_DELETE: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_S_OPEN: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AA_S_WRITE: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_ACCESS_LIST_PARMNUM: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_ATTR_PARMNUM: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_AUDIT: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_COUNT_PARMNUM: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_FAIL_ACL: u32 = 2048u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_FAIL_DELETE: u32 = 1024u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_FAIL_MASK: u32 = 3840u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_FAIL_OPEN: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_FAIL_SHIFT: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_FAIL_WRITE: u32 = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_GROUP: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACCESS_INFO_0 {
    pub acc0_resource_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACCESS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACCESS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACCESS_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACCESS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACCESS_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACCESS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACCESS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACCESS_INFO_1 {
    pub acc1_resource_name: super::super::Foundation::PWSTR,
    pub acc1_attr: u32,
    pub acc1_count: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACCESS_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACCESS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACCESS_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACCESS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACCESS_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACCESS_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACCESS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct ACCESS_INFO_1002 {
    pub acc1002_attr: u32,
}
impl ::core::marker::Copy for ACCESS_INFO_1002 {}
impl ::core::clone::Clone for ACCESS_INFO_1002 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACCESS_INFO_1002 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACCESS_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACCESS_INFO_1002>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACCESS_INFO_1002 {}
impl ::core::default::Default for ACCESS_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACCESS_LIST {
    pub acl_ugname: super::super::Foundation::PWSTR,
    pub acl_access: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACCESS_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACCESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACCESS_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACCESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACCESS_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACCESS_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACCESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_RESOURCE_NAME_PARMNUM: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_SUCCESS_ACL: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_SUCCESS_DELETE: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_SUCCESS_MASK: u32 = 240u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_SUCCESS_OPEN: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACCESS_SUCCESS_WRITE: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACTION_ADMINUNLOCK: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ACTION_LOCKOUT: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct ADMIN_OTHER_INFO {
    pub alrtad_errcode: u32,
    pub alrtad_numstrings: u32,
}
impl ::core::marker::Copy for ADMIN_OTHER_INFO {}
impl ::core::clone::Clone for ADMIN_OTHER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADMIN_OTHER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ADMIN_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ADMIN_OTHER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for ADMIN_OTHER_INFO {}
impl ::core::default::Default for ADMIN_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_ACCLIM {
    pub ae_al_compname: u32,
    pub ae_al_username: u32,
    pub ae_al_resname: u32,
    pub ae_al_limit: u32,
}
impl ::core::marker::Copy for AE_ACCLIM {}
impl ::core::clone::Clone for AE_ACCLIM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_ACCLIM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_ACCLIM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_ACCLIM>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_ACCLIM {}
impl ::core::default::Default for AE_ACCLIM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_ACCLIMITEXCD: u32 = 17u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_ACCRESTRICT: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_ACLMOD: u32 = 12u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_ACLMOD {
    pub ae_am_compname: u32,
    pub ae_am_username: u32,
    pub ae_am_resname: u32,
    pub ae_am_action: u32,
    pub ae_am_datalen: u32,
}
impl ::core::marker::Copy for AE_ACLMOD {}
impl ::core::clone::Clone for AE_ACLMOD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_ACLMOD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_ACLMOD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_ACLMOD>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_ACLMOD {}
impl ::core::default::Default for AE_ACLMOD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_ACLMODFAIL: u32 = 19u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_ADD: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_ADMIN: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_ADMINDIS: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_ADMINPRIVREQD: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_ADMIN_CLOSE: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_AUTODIS: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_BADPW: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_CLOSEFILE: u32 = 9u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_CLOSEFILE {
    pub ae_cf_compname: u32,
    pub ae_cf_username: u32,
    pub ae_cf_resname: u32,
    pub ae_cf_fileid: u32,
    pub ae_cf_duration: u32,
    pub ae_cf_reason: u32,
}
impl ::core::marker::Copy for AE_CLOSEFILE {}
impl ::core::clone::Clone for AE_CLOSEFILE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_CLOSEFILE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_CLOSEFILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_CLOSEFILE>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_CLOSEFILE {}
impl ::core::default::Default for AE_CLOSEFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_CONNREJ: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_CONNREJ {
    pub ae_cr_compname: u32,
    pub ae_cr_username: u32,
    pub ae_cr_netname: u32,
    pub ae_cr_reason: u32,
}
impl ::core::marker::Copy for AE_CONNREJ {}
impl ::core::clone::Clone for AE_CONNREJ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_CONNREJ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_CONNREJ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_CONNREJ>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_CONNREJ {}
impl ::core::default::Default for AE_CONNREJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_CONNSTART: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_CONNSTART {
    pub ae_ct_compname: u32,
    pub ae_ct_username: u32,
    pub ae_ct_netname: u32,
    pub ae_ct_connid: u32,
}
impl ::core::marker::Copy for AE_CONNSTART {}
impl ::core::clone::Clone for AE_CONNSTART {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_CONNSTART {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_CONNSTART {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_CONNSTART>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_CONNSTART {}
impl ::core::default::Default for AE_CONNSTART {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_CONNSTOP: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_CONNSTOP {
    pub ae_cp_compname: u32,
    pub ae_cp_username: u32,
    pub ae_cp_netname: u32,
    pub ae_cp_connid: u32,
    pub ae_cp_reason: u32,
}
impl ::core::marker::Copy for AE_CONNSTOP {}
impl ::core::clone::Clone for AE_CONNSTOP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_CONNSTOP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_CONNSTOP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_CONNSTOP>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_CONNSTOP {}
impl ::core::default::Default for AE_CONNSTOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_DELETE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_ERROR: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_GENERAL: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_GENERIC {
    pub ae_ge_msgfile: u32,
    pub ae_ge_msgnum: u32,
    pub ae_ge_params: u32,
    pub ae_ge_param1: u32,
    pub ae_ge_param2: u32,
    pub ae_ge_param3: u32,
    pub ae_ge_param4: u32,
    pub ae_ge_param5: u32,
    pub ae_ge_param6: u32,
    pub ae_ge_param7: u32,
    pub ae_ge_param8: u32,
    pub ae_ge_param9: u32,
}
impl ::core::marker::Copy for AE_GENERIC {}
impl ::core::clone::Clone for AE_GENERIC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_GENERIC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_GENERIC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_GENERIC>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_GENERIC {}
impl ::core::default::Default for AE_GENERIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_GENERIC_TYPE: u32 = 21u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_GUEST: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_LIM_DELETED: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_LIM_DISABLED: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_LIM_EXPIRED: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_LIM_INVAL_WKSTA: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_LIM_LOGONHOURS: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_LIM_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_LOCKOUT: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_LOCKOUT {
    pub ae_lk_compname: u32,
    pub ae_lk_username: u32,
    pub ae_lk_action: u32,
    pub ae_lk_bad_pw_count: u32,
}
impl ::core::marker::Copy for AE_LOCKOUT {}
impl ::core::clone::Clone for AE_LOCKOUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_LOCKOUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_LOCKOUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_LOCKOUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_LOCKOUT {}
impl ::core::default::Default for AE_LOCKOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_MOD: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_NETLOGDENIED: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_NETLOGOFF: u32 = 15u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_NETLOGOFF {
    pub ae_nf_compname: u32,
    pub ae_nf_username: u32,
    pub ae_nf_reserved1: u32,
    pub ae_nf_reserved2: u32,
}
impl ::core::marker::Copy for AE_NETLOGOFF {}
impl ::core::clone::Clone for AE_NETLOGOFF {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_NETLOGOFF {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_NETLOGOFF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_NETLOGOFF>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_NETLOGOFF {}
impl ::core::default::Default for AE_NETLOGOFF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_NETLOGON: u32 = 14u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_NETLOGON {
    pub ae_no_compname: u32,
    pub ae_no_username: u32,
    pub ae_no_privilege: u32,
    pub ae_no_authflags: u32,
}
impl ::core::marker::Copy for AE_NETLOGON {}
impl ::core::clone::Clone for AE_NETLOGON {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_NETLOGON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_NETLOGON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_NETLOGON>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_NETLOGON {}
impl ::core::default::Default for AE_NETLOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_NOACCESSPERM: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_NORMAL: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_NORMAL_CLOSE: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_RESACCESS: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_RESACCESS {
    pub ae_ra_compname: u32,
    pub ae_ra_username: u32,
    pub ae_ra_resname: u32,
    pub ae_ra_operation: u32,
    pub ae_ra_returncode: u32,
    pub ae_ra_restype: u32,
    pub ae_ra_fileid: u32,
}
impl ::core::marker::Copy for AE_RESACCESS {}
impl ::core::clone::Clone for AE_RESACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_RESACCESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_RESACCESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_RESACCESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_RESACCESS {}
impl ::core::default::Default for AE_RESACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_RESACCESS2: u32 = 18u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_RESACCESSREJ: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_RESACCESSREJ {
    pub ae_rr_compname: u32,
    pub ae_rr_username: u32,
    pub ae_rr_resname: u32,
    pub ae_rr_operation: u32,
}
impl ::core::marker::Copy for AE_RESACCESSREJ {}
impl ::core::clone::Clone for AE_RESACCESSREJ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_RESACCESSREJ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_RESACCESSREJ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_RESACCESSREJ>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_RESACCESSREJ {}
impl ::core::default::Default for AE_RESACCESSREJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_SERVICESTAT: u32 = 11u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_SERVICESTAT {
    pub ae_ss_compname: u32,
    pub ae_ss_username: u32,
    pub ae_ss_svcname: u32,
    pub ae_ss_status: u32,
    pub ae_ss_code: u32,
    pub ae_ss_text: u32,
    pub ae_ss_returnval: u32,
}
impl ::core::marker::Copy for AE_SERVICESTAT {}
impl ::core::clone::Clone for AE_SERVICESTAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_SERVICESTAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_SERVICESTAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_SERVICESTAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_SERVICESTAT {}
impl ::core::default::Default for AE_SERVICESTAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_SESSDIS: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_SESSLOGOFF: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_SESSLOGOFF {
    pub ae_sf_compname: u32,
    pub ae_sf_username: u32,
    pub ae_sf_reason: u32,
}
impl ::core::marker::Copy for AE_SESSLOGOFF {}
impl ::core::clone::Clone for AE_SESSLOGOFF {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_SESSLOGOFF {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_SESSLOGOFF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_SESSLOGOFF>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_SESSLOGOFF {}
impl ::core::default::Default for AE_SESSLOGOFF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_SESSLOGON: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_SESSLOGON {
    pub ae_so_compname: u32,
    pub ae_so_username: u32,
    pub ae_so_privilege: u32,
}
impl ::core::marker::Copy for AE_SESSLOGON {}
impl ::core::clone::Clone for AE_SESSLOGON {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_SESSLOGON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_SESSLOGON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_SESSLOGON>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_SESSLOGON {}
impl ::core::default::Default for AE_SESSLOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_SESSPWERR: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_SESSPWERR {
    pub ae_sp_compname: u32,
    pub ae_sp_username: u32,
}
impl ::core::marker::Copy for AE_SESSPWERR {}
impl ::core::clone::Clone for AE_SESSPWERR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_SESSPWERR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_SESSPWERR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_SESSPWERR>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_SESSPWERR {}
impl ::core::default::Default for AE_SESSPWERR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_SES_CLOSE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_SRVCONT: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_SRVPAUSED: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_SRVSTART: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_SRVSTATUS: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_SRVSTATUS {
    pub ae_sv_status: u32,
}
impl ::core::marker::Copy for AE_SRVSTATUS {}
impl ::core::clone::Clone for AE_SRVSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_SRVSTATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_SRVSTATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_SRVSTATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_SRVSTATUS {}
impl ::core::default::Default for AE_SRVSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_SRVSTOP: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_UASMOD: u32 = 13u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AE_UASMOD {
    pub ae_um_compname: u32,
    pub ae_um_username: u32,
    pub ae_um_resname: u32,
    pub ae_um_rectype: u32,
    pub ae_um_action: u32,
    pub ae_um_datalen: u32,
}
impl ::core::marker::Copy for AE_UASMOD {}
impl ::core::clone::Clone for AE_UASMOD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AE_UASMOD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AE_UASMOD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AE_UASMOD>()) == 0 }
    }
}
impl ::core::cmp::Eq for AE_UASMOD {}
impl ::core::default::Default for AE_UASMOD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_UAS_GROUP: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_UAS_MODALS: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_UAS_USER: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_UNSHARE: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_USER: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AE_USERLIMIT: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type AF_OP = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AF_OP_PRINT: AF_OP = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AF_OP_COMM: AF_OP = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AF_OP_SERVER: AF_OP = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const AF_OP_ACCOUNTS: AF_OP = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ALERTSZ: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ALIGN_SIZE: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ALLOCATE_RESPONSE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AT_ENUM {
    pub JobId: u32,
    pub JobTime: usize,
    pub DaysOfMonth: u32,
    pub DaysOfWeek: u8,
    pub Flags: u8,
    pub Command: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AT_ENUM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AT_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AT_ENUM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AT_ENUM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AT_ENUM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AT_ENUM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AT_ENUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AT_INFO {
    pub JobTime: usize,
    pub DaysOfMonth: u32,
    pub DaysOfWeek: u8,
    pub Flags: u8,
    pub Command: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AT_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AT_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct AUDIT_ENTRY {
    pub ae_len: u32,
    pub ae_reserved: u32,
    pub ae_time: u32,
    pub ae_type: u32,
    pub ae_data_offset: u32,
    pub ae_data_size: u32,
}
impl ::core::marker::Copy for AUDIT_ENTRY {}
impl ::core::clone::Clone for AUDIT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AUDIT_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIT_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIT_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIT_ENTRY {}
impl ::core::default::Default for AUDIT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type BIND_FLAGS1 = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCN_ADD: BIND_FLAGS1 = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCN_REMOVE: BIND_FLAGS1 = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCN_UPDATE: BIND_FLAGS1 = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCN_ENABLE: BIND_FLAGS1 = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCN_DISABLE: BIND_FLAGS1 = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCN_BINDING_PATH: BIND_FLAGS1 = 256i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCN_PROPERTYCHANGE: BIND_FLAGS1 = 512i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCN_NET: BIND_FLAGS1 = 65536i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCN_NETTRANS: BIND_FLAGS1 = 131072i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCN_NETCLIENT: BIND_FLAGS1 = 262144i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCN_NETSERVICE: BIND_FLAGS1 = 524288i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CLTYPE_LEN: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CNLEN: u32 = 15u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type COMPONENT_CHARACTERISTICS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_VIRTUAL: COMPONENT_CHARACTERISTICS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_SOFTWARE_ENUMERATED: COMPONENT_CHARACTERISTICS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_PHYSICAL: COMPONENT_CHARACTERISTICS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_HIDDEN: COMPONENT_CHARACTERISTICS = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_NO_SERVICE: COMPONENT_CHARACTERISTICS = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_NOT_USER_REMOVABLE: COMPONENT_CHARACTERISTICS = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_MULTIPORT_INSTANCED_ADAPTER: COMPONENT_CHARACTERISTICS = 64i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_HAS_UI: COMPONENT_CHARACTERISTICS = 128i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_SINGLE_INSTANCE: COMPONENT_CHARACTERISTICS = 256i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_FILTER: COMPONENT_CHARACTERISTICS = 1024i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_DONTEXPOSELOWER: COMPONENT_CHARACTERISTICS = 4096i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_HIDE_BINDING: COMPONENT_CHARACTERISTICS = 8192i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_NDIS_PROTOCOL: COMPONENT_CHARACTERISTICS = 16384i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_FIXED_BINDING: COMPONENT_CHARACTERISTICS = 131072i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_LW_FILTER: COMPONENT_CHARACTERISTICS = 262144i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONFIG_INFO_0 {
    pub cfgi0_key: super::super::Foundation::PWSTR,
    pub cfgi0_data: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONFIG_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONFIG_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONFIG_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONFIG_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONFIG_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONFIG_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONFIG_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const COULD_NOT_VERIFY_VOLUMES: i32 = -1073727512i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CREATE_BYPASS_CSC: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CREATE_CRED_RESET: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CREATE_GLOBAL_MAPPING: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CREATE_NO_CONNECT: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CREATE_PERSIST_MAPPING: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CREATE_REQUIRE_CONNECTION_INTEGRITY: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CREATE_REQUIRE_CONNECTION_PRIVACY: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CREATE_WRITE_THROUGH_SEMANTICS: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CRYPT_KEY_LEN: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const CRYPT_TXT_LEN: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type DEFAULT_PAGES = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DPP_ADVANCED: DEFAULT_PAGES = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DEF_MAX_BADPW: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DEF_MAX_PWHIST: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DEF_MIN_PWLEN: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DEF_PWUNIQUENESS: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DEVLEN: u32 = 80u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_CONNECTION_FAILURE: i32 = 1073756226i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_ACTIVEDIRECTORY_OFFLINE: i32 = -1073727301i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_CLUSTERINFO_FAILED: i32 = -1073727307i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_COMPUTERINFO_FAILED: i32 = -1073727308i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_CREATEEVENT_FAILED: i32 = -1073727309i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_CREATE_REPARSEPOINT_FAILURE: i32 = -1073727321i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_CREATE_REPARSEPOINT_SUCCESS: i32 = 1073756370i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_CROSS_FOREST_TRUST_INFO_FAILED: i32 = -1073727274i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_DCINFO_FAILED: i32 = -1073727306i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_DSCONNECT_FAILED: i32 = -2147469122i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_DUPLICATE_LINK: i32 = -1073727277i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_HANDLENAMESPACE_FAILED: i32 = -1073727304i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_LINKS_OVERLAP: i32 = -1073727280i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_LINK_OVERLAP: i32 = -1073727279i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_MUTLIPLE_ROOTS_NOT_SUPPORTED: i32 = -1073727289i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_NO_DFS_DATA: i32 = -1073727294i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_ON_ROOT: i32 = -2147469114i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_OVERLAPPING_DIRECTORIES: i32 = -1073727319i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_PREFIXTABLE_FAILED: i32 = -1073727305i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_REFLECTIONENGINE_FAILED: i32 = -1073727302i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_REGISTERSTORE_FAILED: i32 = -1073727303i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_REMOVE_LINK_FAILED: i32 = -1073727284i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_RESYNCHRONIZE_FAILED: i32 = -1073727285i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_ROOTSYNCINIT_FAILED: i32 = -1073727310i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_SECURITYINIT_FAILED: i32 = -1073727313i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_SITECACHEINIT_FAILED: i32 = -1073727311i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_SITESUPPOR_FAILED: i32 = -1073727300i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_TARGET_LIST_INCORRECT: i32 = -1073727281i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_THREADINIT_FAILED: i32 = -1073727312i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_TOO_MANY_ERRORS: i32 = -1073727315i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_TRUSTED_DOMAIN_INFO_FAILED: i32 = -1073727276i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_UNSUPPORTED_FILESYSTEM: i32 = -1073727320i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ERROR_WINSOCKINIT_FAILED: i32 = -1073727314i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_INFO_ACTIVEDIRECTORY_ONLINE: i32 = 1073756332i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_INFO_CROSS_FOREST_TRUST_INFO_SUCCESS: i32 = 1073756375i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_INFO_DOMAIN_REFERRAL_MIN_OVERFLOW: i32 = 1073756361i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_INFO_DS_RECONNECTED: i32 = 1073756353i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_INFO_FINISH_BUILDING_NAMESPACE: i32 = 1073756357i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_INFO_FINISH_INIT: i32 = 1073756355i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_INFO_RECONNECT_DATA: i32 = 1073756356i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_INFO_TRUSTED_DOMAIN_INFO_SUCCESS: i32 = 1073756373i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_INIT_SUCCESS: i32 = 1073756376i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_MAX_DNR_ATTEMPTS: i32 = 1073756229i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_OPEN_FAILURE: i32 = 1073756231i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_REFERRAL_FAILURE: i32 = 1073756227i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_REFERRAL_REQUEST: i32 = 1073756142i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_REFERRAL_SUCCESS: i32 = 1073756228i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ROOT_SHARE_ACQUIRE_FAILED: i32 = -2147469095i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_ROOT_SHARE_ACQUIRE_SUCCESS: i32 = 1073756378i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_SPECIAL_REFERRAL_FAILURE: i32 = 1073756230i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_WARN_DOMAIN_REFERRAL_OVERFLOW: i32 = -2147469112i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_WARN_INCOMPLETE_MOVE: i32 = -2147469110i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_WARN_METADATA_LINK_INFO_INVALID: i32 = -2147469106i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DFS_WARN_METADATA_LINK_TYPE_INCORRECT: i32 = -2147469107i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DNLEN: u32 = 15u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct DSREG_JOIN_INFO {
    pub joinType: DSREG_JOIN_TYPE,
    pub pJoinCertificate: *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub pszDeviceId: super::super::Foundation::PWSTR,
    pub pszIdpDomain: super::super::Foundation::PWSTR,
    pub pszTenantId: super::super::Foundation::PWSTR,
    pub pszJoinUserEmail: super::super::Foundation::PWSTR,
    pub pszTenantDisplayName: super::super::Foundation::PWSTR,
    pub pszMdmEnrollmentUrl: super::super::Foundation::PWSTR,
    pub pszMdmTermsOfUseUrl: super::super::Foundation::PWSTR,
    pub pszMdmComplianceUrl: super::super::Foundation::PWSTR,
    pub pszUserSettingSyncUrl: super::super::Foundation::PWSTR,
    pub pUserInfo: *mut DSREG_USER_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for DSREG_JOIN_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for DSREG_JOIN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for DSREG_JOIN_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for DSREG_JOIN_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSREG_JOIN_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for DSREG_JOIN_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for DSREG_JOIN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type DSREG_JOIN_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DSREG_UNKNOWN_JOIN: DSREG_JOIN_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DSREG_DEVICE_JOIN: DSREG_JOIN_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const DSREG_WORKPLACE_JOIN: DSREG_JOIN_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSREG_USER_INFO {
    pub pszUserEmail: super::super::Foundation::PWSTR,
    pub pszUserKeyId: super::super::Foundation::PWSTR,
    pub pszUserKeyName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSREG_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSREG_USER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSREG_USER_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSREG_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSREG_USER_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSREG_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSREG_USER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ENCRYPTED_PWLEN: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type ENUM_BINDING_PATHS_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EBP_ABOVE: ENUM_BINDING_PATHS_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EBP_BELOW: ENUM_BINDING_PATHS_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ERRLOG2_BASE: u32 = 5700u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ERRLOG_BASE: u32 = 3100u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct ERRLOG_OTHER_INFO {
    pub alrter_errcode: u32,
    pub alrter_offset: u32,
}
impl ::core::marker::Copy for ERRLOG_OTHER_INFO {}
impl ::core::clone::Clone for ERRLOG_OTHER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ERRLOG_OTHER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ERRLOG_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ERRLOG_OTHER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for ERRLOG_OTHER_INFO {}
impl ::core::default::Default for ERRLOG_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ERROR_LOG {
    pub el_len: u32,
    pub el_reserved: u32,
    pub el_time: u32,
    pub el_error: u32,
    pub el_name: super::super::Foundation::PWSTR,
    pub el_text: super::super::Foundation::PWSTR,
    pub el_data: *mut u8,
    pub el_data_size: u32,
    pub el_nstrings: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ERROR_LOG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ERROR_LOG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ERROR_LOG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ERROR_LOG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ERROR_LOG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ERROR_LOG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ERROR_LOG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BAD_ACCOUNT_NAME: i32 = -1073734816i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BAD_SERVICE_STATE: i32 = -1073734808i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOOT_SYSTEM_DRIVERS_FAILED: i32 = -1073734798i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_CANT_READ_REGISTRY: i32 = 1073749853i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_ELECTION_RECEIVED: i32 = 8012i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_ELECTION_SENT_FIND_MASTER_FAILED: i32 = 1073749838i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_ELECTION_SENT_GETBLIST_FAILED: i32 = 1073749837i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_GETBROWSERLIST_THRESHOLD_EXCEEDED: i32 = 1073749855i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_ILLEGAL_DATAGRAM: i32 = -2147475642i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_ILLEGAL_DATAGRAM_THRESHOLD: i32 = -1073733808i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_MAILSLOT_DATAGRAM_THRESHOLD_EXCEEDED: i32 = 1073749854i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_NAME_CONVERSION_FAILED: i32 = -1073733814i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_NON_MASTER_MASTER_ANNOUNCE: i32 = -2147475643i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_NON_PDC_WON_ELECTION: i32 = 1073749852i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_OLD_BACKUP_FOUND: i32 = 1073749848i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_OTHER_MASTER_ON_NET: i32 = -1073733821i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_PDC_LOST_ELECTION: i32 = 1073749851i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BOWSER_PROMOTED_WHILE_ALREADY_MASTER: i32 = -2147475644i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_ADAPTER_BIND_FAILED: i32 = -1073727120i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_ADAPTER_FILTER_FAILED: i32 = -1073727122i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_ADAPTER_LINK_SPEED_QUERY_FAILED: i32 = -1073727124i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_ADAPTER_MAC_ADDR_QUERY_FAILED: i32 = -1073727123i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_ADAPTER_NAME_QUERY_FAILED: i32 = -1073727121i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_BUFFER_POOL_CREATION_FAILED: i32 = -1073727214i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_DEVICE_CREATION_FAILED: i32 = -1073727221i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_ETHERNET_NOT_OFFERED: i32 = -1073727218i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_INIT_MALLOC_FAILED: i32 = -1073727213i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_MINIPORT_INIT_FAILED: i32 = -1073727219i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_MINIPORT_REGISTER_FAILED: i32 = -1073727222i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_MINIPROT_DEVNAME_MISSING: i32 = -1073727223i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_NO_BRIDGE_MAC_ADDR: i32 = -1073727220i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_PACKET_POOL_CREATION_FAILED: i32 = -1073727215i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_PROTOCOL_REGISTER_FAILED: i32 = -1073727224i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_THREAD_CREATION_FAILED: i32 = -1073727217i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BRIDGE_THREAD_REF_FAILED: i32 = -1073727216i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_BACKUP_STOPPED: i32 = -1073733792i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_DEPENDANT_SERVICE_FAILED: i32 = -1073733807i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_DOMAIN_LIST_FAILED: i32 = -2147475626i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_DOMAIN_LIST_RETRIEVED: i32 = 8026i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_ELECTION_SENT_LANMAN_NT_STARTED: i32 = 1073749839i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_ELECTION_SENT_LANMAN_NT_STOPPED: i32 = 1073749857i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_ELECTION_SENT_ROLE_CHANGED: i32 = 1073749859i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_GETBLIST_RECEIVED_NOT_MASTER: i32 = -1073733790i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_ILLEGAL_CONFIG: i32 = -2147475625i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED: i32 = -1073733815i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED_NO_MASTER: i32 = -1073733804i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED_STOPPING: i32 = -1073733805i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_NOT_STARTED_IPX_CONFIG_MISMATCH: i32 = -1073733788i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_OTHERDOMAIN_ADD_FAILED: i32 = -1073733813i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_ROLE_CHANGE_FAILED: i32 = -1073733816i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_SERVER_LIST_FAILED: i32 = -2147475627i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_SERVER_LIST_RETRIEVED: i32 = 8025i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_BROWSER_STATUS_BITS_UPDATE_FAILED: i32 = -1073733817i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_CALL_TO_FUNCTION_FAILED: i32 = -1073734819i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_CALL_TO_FUNCTION_FAILED_II: i32 = -1073734818i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_CIRCULAR_DEPENDENCY_AUTO: i32 = -1073734806i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_CIRCULAR_DEPENDENCY_DEMAND: i32 = -1073734807i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_COMMAND_NOT_INTERACTIVE: i32 = -1073733924i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_COMMAND_START_FAILED: i32 = -1073733923i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_CONNECTION_TIMEOUT: i32 = -1073734815i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_ComputerNameChange: i32 = -2147477637i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DAV_REDIR_DELAYED_WRITE_FAILED: i32 = -2147468848i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DCOM_ASSERTION_FAILURE: i32 = -1073731812i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DCOM_COMPLUS_DISABLED: i32 = -1073731810i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DCOM_INVALID_ENDPOINT_DATA: i32 = -1073731811i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DEPEND_ON_LATER_GROUP: i32 = -1073734804i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DEPEND_ON_LATER_SERVICE: i32 = -1073734805i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_NOTSUPP: i32 = -2147472466i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_NOTSUPP_PRIMARY_DN: i32 = -2147472454i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_OTHER: i32 = -2147472463i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_OTHER_PRIMARY_DN: i32 = -2147472451i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_REFUSED: i32 = -2147472465i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_REFUSED_PRIMARY_DN: i32 = -2147472453i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SECURITY: i32 = -2147472464i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SECURITY_PRIMARY_DN: i32 = -2147472452i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SERVERFAIL: i32 = -2147472467i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SERVERFAIL_PRIMARY_DN: i32 = -2147472455i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_TIMEOUT: i32 = -2147472468i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_TIMEOUT_PRIMARY_DN: i32 = -2147472456i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_NOTSUPP: i32 = -2147472460i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_OTHER: i32 = -2147472457i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_REFUSED: i32 = -2147472459i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_SECURITY: i32 = -2147472458i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_SERVERFAIL: i32 = -2147472461i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_TIMEOUT: i32 = -2147472462i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_NOTSUPP: i32 = -2147472490i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_OTHER: i32 = -2147472487i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_REFUSED: i32 = -2147472489i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_SECURITY: i32 = -2147472488i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_SERVERFAIL: i32 = -2147472491i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_TIMEOUT: i32 = -2147472492i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTERED_ADAPTER: i32 = 1073753024i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTERED_ADAPTER_PRIMARY_DN: i32 = 1073753026i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTERED_PTR: i32 = 1073753025i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_NOTSUPP: i32 = -2147472496i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_NOTSUPP_PRIMARY_DN: i32 = -2147472484i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_OTHER: i32 = -2147472493i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_OTHER_PRIMARY_DN: i32 = -2147472481i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_REFUSED: i32 = -2147472495i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_REFUSED_PRIMARY_DN: i32 = -2147472483i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SECURITY: i32 = -2147472494i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SECURITY_PRIMARY_DN: i32 = -2147472482i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SERVERFAIL: i32 = -2147472497i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SERVERFAIL_PRIMARY_DN: i32 = -2147472485i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_TIMEOUT: i32 = -2147472498i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSAPI_REGISTRATION_FAILED_TIMEOUT_PRIMARY_DN: i32 = -2147472486i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNSDomainNameChange: i32 = -2147477636i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNS_CACHE_NETWORK_PERF_WARNING: i32 = -2147472598i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNS_CACHE_START_FAILURE_LOW_MEMORY: i32 = -1073730817i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_CONTROL: i32 = -1073730822i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_DLL: i32 = -1073730824i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_DONE_EVENT: i32 = -1073730821i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_ENTRY: i32 = -1073730823i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_RPC: i32 = -1073730820i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_SHUTDOWN_NOTIFY: i32 = -1073730819i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNS_CACHE_START_FAILURE_NO_UPDATE: i32 = -1073730818i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_DNS_CACHE_UNABLE_TO_REACH_SERVER_WARNING: i32 = -2147472597i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_KEYNAME_SIZE_ZERO: i32 = -1073725118i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_KEYNAME_TOO_LONG: i32 = -1073725120i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_REFERESH: i32 = -1073725124i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_OPENING_MACHINE_POLICY_ROOT_KEY: i32 = -1073725122i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_OPENING_MACHINE_POLICY_SUBKEY: i32 = -1073725116i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_OPENING_USER_POLICY_ROOT_KEY: i32 = -1073725121i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_OPENING_USER_POLICY_SUBKEY: i32 = -1073725115i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_PROCESSING_MACHINE_POLICY_FIELD: i32 = -1073725114i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_PROCESSING_USER_POLICY_FIELD: i32 = -1073725113i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_SETTING_APP_MARKING: i32 = -1073725111i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_SETTING_TCP_AUTOTUNING: i32 = -1073725112i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_USER_POLICY_KEYNAME_SIZE_ZERO: i32 = -1073725117i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_USER_POLICY_KEYNAME_TOO_LONG: i32 = -1073725119i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_ERROR_USER_POLICY_REFERESH: i32 = -1073725123i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_APP_MARKING_ALLOWED: i32 = 1073758335i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_APP_MARKING_IGNORED: i32 = 1073758334i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_APP_MARKING_NOT_CONFIGURED: i32 = 1073758333i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_LOCAL_SETTING_DONT_USE_NLA: i32 = 1073758336i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_MACHINE_POLICY_REFRESH_NO_CHANGE: i32 = 1073758324i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_MACHINE_POLICY_REFRESH_WITH_CHANGE: i32 = 1073758325i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_HIGHLY_RESTRICTED: i32 = 1073758330i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_NORMAL: i32 = 1073758332i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_NOT_CONFIGURED: i32 = 1073758328i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_OFF: i32 = 1073758329i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_RESTRICTED: i32 = 1073758331i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_USER_POLICY_REFRESH_NO_CHANGE: i32 = 1073758326i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_INFO_USER_POLICY_REFRESH_WITH_CHANGE: i32 = 1073758327i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_URL_QOS_APPLICATION_CONFLICT: i32 = 1073758337i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_CONFLICT: i32 = -2147467040i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_NO_FULLPATH_APPNAME: i32 = -2147467038i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_PROFILE_NOT_SPECIFIED: i32 = -2147467044i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_QUOTA_EXCEEDED: i32 = -2147467042i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_VERSION: i32 = -2147467046i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_TEST_1: i32 = -2147467048i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_TEST_2: i32 = -2147467047i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_USER_POLICY_CONFLICT: i32 = -2147467039i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_USER_POLICY_NO_FULLPATH_APPNAME: i32 = -2147467037i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_USER_POLICY_PROFILE_NOT_SPECIFIED: i32 = -2147467043i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_USER_POLICY_QUOTA_EXCEEDED: i32 = -2147467041i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EQOS_WARNING_USER_POLICY_VERSION: i32 = -2147467045i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EventLogProductInfo: i32 = -2147477639i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EventlogAbnormalShutdown: i32 = -2147477640i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EventlogStarted: i32 = -2147477643i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EventlogStopped: i32 = -2147477642i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_EventlogUptime: i32 = -2147477635i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FIRST_LOGON_FAILED: i32 = -1073734811i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FIRST_LOGON_FAILED_II: i32 = -1073734786i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_ACCESS_CHECKS_DISABLED: i32 = -2147470131i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_ACCESS_CHECKS_FAILED_UNKNOWN: i32 = -1073728305i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_ACCESS_CHECKS_FAILED_USER: i32 = -2147470130i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_ASSERT: i32 = -1073728318i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_BAD_REG_DATA: i32 = -2147470101i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_CANNOT_COMMUNICATE: i32 = -1073728314i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_CANNOT_CREATE_UUID: i32 = -1073728300i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_CANNOT_START_BACKUP_RESTORE_IN_PROGRESS: i32 = -1073728303i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_CANT_OPEN_PREINSTALL: i32 = -1073728273i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_CANT_OPEN_STAGE: i32 = -1073728274i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_DATABASE_SPACE: i32 = -1073728313i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_DISK_WRITE_CACHE_ENABLED: i32 = -2147470136i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_DS_POLL_ERROR_SUMMARY: i32 = -2147470086i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_DUPLICATE_IN_CXTION: i32 = -1073728266i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_DUPLICATE_IN_CXTION_SYSVOL: i32 = -1073728267i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_ERROR: i32 = -1073728324i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_ERROR_REPLICA_SET_DELETED: i32 = -2147470088i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_HUGE_FILE: i32 = -2147470125i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_IN_ERROR_STATE: i32 = -1073728269i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_JET_1414: i32 = -1073728311i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_JOIN_FAIL_TIME_SKEW: i32 = -1073728276i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_LONG_JOIN: i32 = -2147470140i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_LONG_JOIN_DONE: i32 = -2147470139i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_MOVED_PREEXISTING: i32 = -2147470128i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_NO_DNS_ATTRIBUTE: i32 = -2147470123i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_NO_SID: i32 = -1073728298i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_OVERLAPS_LOGGING: i32 = -1073728283i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_OVERLAPS_OTHER_STAGE: i32 = -1073728279i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_OVERLAPS_ROOT: i32 = -1073728280i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_OVERLAPS_STAGE: i32 = -1073728281i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_OVERLAPS_WORKING: i32 = -1073728282i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_PREPARE_ROOT_FAILED: i32 = -1073728278i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_REPLICA_IN_JRNL_WRAP_ERROR: i32 = -1073728263i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_REPLICA_NO_ROOT_CHANGE: i32 = -1073728268i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_REPLICA_SET_CREATE_FAIL: i32 = -1073728272i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_REPLICA_SET_CREATE_OK: i32 = 1073755377i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_REPLICA_SET_CXTIONS: i32 = 1073755378i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_RMTCO_TIME_SKEW: i32 = -1073728275i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_ROOT_HAS_MOVED: i32 = -1073728265i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_ROOT_NOT_VALID: i32 = -1073728285i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_STAGE_NOT_VALID: i32 = -1073728284i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_STAGING_AREA_FULL: i32 = -2147470126i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_STARTING: i32 = 1073755325i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_STOPPED: i32 = 1073755327i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_STOPPED_ASSERT: i32 = -1073728319i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_STOPPED_FORCE: i32 = -1073728320i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_STOPPING: i32 = 1073755326i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_SYSVOL_NOT_READY: i32 = -2147470134i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_SYSVOL_NOT_READY_PRIMARY: i32 = -2147470133i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_SYSVOL_READY: i32 = 1073755340i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_FRS_VOLUME_NOT_SUPPORTED: i32 = -1073728317i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_INVALID_DRIVER_DEPENDENCY: i32 = -1073734809i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_IPX_CREATE_DEVICE: i32 = -1073732318i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_IPX_ILLEGAL_CONFIG: i32 = -2147474145i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_IPX_INTERNAL_NET_INVALID: i32 = -1073732320i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_IPX_NEW_DEFAULT_TYPE: i32 = 1073751325i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_IPX_NO_ADAPTERS: i32 = -1073732317i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_IPX_NO_FRAME_TYPES: i32 = -1073732319i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_IPX_SAP_ANNOUNCE: i32 = -2147474146i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_BAD_BACKUP_WINS_ADDR: i32 = -2147479344i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_BAD_PRIMARY_WINS_ADDR: i32 = -2147479343i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_CREATE_ADDRESS: i32 = -1073737517i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_CREATE_CONNECTION: i32 = -1073737516i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_CREATE_DEVICE: i32 = -1073737513i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_CREATE_DRIVER: i32 = -1073737524i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_DUPLICATE_NAME: i32 = -1073737505i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_DUPLICATE_NAME_ERROR: i32 = -1073737503i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_NAME_RELEASE: i32 = -1073737504i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_NAME_SERVER_ADDRS: i32 = -1073737518i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_NON_OS_INIT: i32 = -1073737515i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_NO_BACKUP_WINS: i32 = -2147479346i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_NO_DEVICES: i32 = -2147479336i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_NO_RESOURCES: i32 = -1073737502i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_NO_WINS: i32 = -2147479345i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_OPEN_REG_LINKAGE: i32 = -1073737511i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_OPEN_REG_NAMESERVER: i32 = -2147479332i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_OPEN_REG_PARAMS: i32 = -1073737523i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_READ_BIND: i32 = -1073737510i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_READ_EXPORT: i32 = -1073737509i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NBT_TIMERS: i32 = -1073737514i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_ADAPTER_CHECK_ERROR: i32 = -1073736793i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_ADAPTER_DISABLED: i32 = -2147478634i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_ADAPTER_NOT_FOUND: i32 = -1073736821i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_BAD_IO_BASE_ADDRESS: i32 = -1073736812i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_BAD_VERSION: i32 = -1073736818i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_CABLE_DISCONNECTED_ERROR: i32 = -2147478615i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_DMA_CONFLICT: i32 = -2147478629i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_DRIVER_FAILURE: i32 = -1073736819i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_HARDWARE_FAILURE: i32 = -1073736822i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_INTERRUPT_CONFLICT: i32 = -2147478630i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_INTERRUPT_CONNECT: i32 = -1073736820i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_INVALID_DOWNLOAD_FILE_ERROR: i32 = -1073736804i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_INVALID_VALUE_FROM_ADAPTER: i32 = -1073736814i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_IO_PORT_CONFLICT: i32 = -2147478633i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_LOBE_FAILUE_ERROR: i32 = -2147478621i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_MAXFRAMESIZE_ERROR: i32 = -2147478625i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_MAXINTERNALBUFS_ERROR: i32 = -2147478624i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_MAXMULTICAST_ERROR: i32 = -2147478623i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_MAXRECEIVES_ERROR: i32 = -2147478627i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_MAXTRANSMITS_ERROR: i32 = -2147478626i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_MEMORY_CONFLICT: i32 = -2147478631i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_MISSING_CONFIGURATION_PARAMETER: i32 = -1073736813i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_NETWORK_ADDRESS: i32 = -1073736816i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_OUT_OF_RESOURCE: i32 = -1073736823i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_PORT_OR_DMA_CONFLICT: i32 = -2147478632i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_PRODUCTID_ERROR: i32 = -2147478622i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_RECEIVE_SPACE_SMALL: i32 = 1073746837i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_REMOVE_RECEIVED_ERROR: i32 = -2147478619i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_RESET_FAILURE_CORRECTION: i32 = -2147478614i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_RESET_FAILURE_ERROR: i32 = -2147478616i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_RESOURCE_CONFLICT: i32 = -1073736824i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_SIGNAL_LOSS_ERROR: i32 = -2147478620i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_TIMEOUT: i32 = -2147478641i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_TOKEN_RING_CORRECTION: i32 = 1073746854i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_NDIS_UNSUPPORTED_CONFIGURATION: i32 = -1073736815i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_ADMISSIONCONTROL_OVERFLOW: i32 = -2147469537i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_BAD_BESTEFFORT_LIMIT: i32 = -2147469548i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_BINDING_FAILED: i32 = -1073727720i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_GPC_REGISTER_FAILED: i32 = -1073727824i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_INIT_DEVICE_FAILED: i32 = -1073727717i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_MISSING_ADAPTER_REGISTRY_DATA: i32 = -1073727719i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_NETWORK_ADDRESS_FAIL: i32 = -1073727712i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_NO_RESOURCES_FOR_INIT: i32 = -1073727823i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_QUERY_OID_GEN_LINK_SPEED: i32 = -1073727721i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_QUERY_OID_GEN_MAXIMUM_FRAME_SIZE: i32 = -1073727723i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_QUERY_OID_GEN_MAXIMUM_TOTAL_SIZE: i32 = -1073727722i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_REGISTER_ADDRESS_FAMILY_FAILED: i32 = -1073727718i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_REGISTER_MINIPORT_FAILED: i32 = -1073727821i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_REGISTER_PROTOCOL_FAILED: i32 = -1073727822i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_RESOURCE_POOL: i32 = -1073727714i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_WAN_LIMITED_BESTEFFORT: i32 = -2147469539i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_PS_WMI_INSTANCE_NAME_FAILED: i32 = -1073727716i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_AT_THREAD_MAX: i32 = -2147480622i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CANT_BIND_TRANSPORT: i32 = -2147480616i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CANT_BUILD_SMB_HEADER: i32 = -2147480613i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CANT_CREATE_DEVICE: i32 = -2147480646i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CANT_CREATE_THREAD: i32 = -2147480645i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CANT_GET_SECURITY_CONTEXT: i32 = -2147480614i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CANT_READ_REGISTRY: i32 = -2147480621i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CANT_REGISTER_ADDRESS: i32 = -2147480615i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CANT_SET_THREAD: i32 = -2147480644i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CLOSE_BEHIND: i32 = -2147480637i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CONNECTION: i32 = -2147480629i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CONNECTION_REFERENCE: i32 = -2147480633i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_CONTEXTS: i32 = -2147480624i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_DELAYED_SET_ATTRIBUTES_FAILED: i32 = -2147480618i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_DELETEONCLOSE_FAILED: i32 = -2147480617i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_DISPOSITION: i32 = -2147480625i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_ENCRYPT: i32 = -2147480630i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_FAILED_UNLOCK: i32 = -2147480639i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_INVALID_LOCK_REPLY: i32 = -2147480641i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_INVALID_OPLOCK: i32 = -2147480634i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_INVALID_REPLY: i32 = -2147480643i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_INVALID_SMB: i32 = -2147480642i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_MAXCMDS: i32 = -2147480627i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_OPLOCK_SMB: i32 = -2147480626i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_PRIMARY_TRANSPORT_CONNECT_FAILED: i32 = -2147480619i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_RESOURCE_SHORTAGE: i32 = -2147480647i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_SECURITY_SIGNATURE_MISMATCH: i32 = -2147480612i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_SERVER_REFERENCE: i32 = -2147480632i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_SMB_REFERENCE: i32 = -2147480631i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_TIMEOUT: i32 = -2147480635i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_TIMEZONE_BIAS_TOO_LARGE: i32 = -2147480620i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_UNEXPECTED_ERROR: i32 = -2147480636i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RDR_WRITE_BEHIND_FLUSH_FAILED: i32 = -2147480623i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_READFILE_TIMEOUT: i32 = -1073734814i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_REVERTED_TO_LASTKNOWNGOOD: i32 = -1073734817i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_ACTIVATION_ERROR: i32 = -1073731817i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_CREATEDEBUGGERPROCESS_FAILURE: i32 = -1073731794i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_CREATEPROCESS_FAILURE: i32 = -1073731824i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_DEFAULT_LAUNCH_ACCESS_DENIED: i32 = -1073731821i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_LAUNCH_ACCESS_DENIED: i32 = -1073731822i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_REMOTE_SIDE_ERROR: i32 = -1073731818i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_REMOTE_SIDE_ERROR_WITH_FILE: i32 = -1073731816i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_REMOTE_SIDE_UNAVAILABLE: i32 = -1073731815i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_RUNAS_CANT_LOGIN: i32 = -1073731820i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_RUNAS_CREATEPROCESS_FAILURE: i32 = -1073731823i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_SERVER_NOT_RESPONDING: i32 = -1073731813i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_SERVER_START_TIMEOUT: i32 = -1073731814i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_START_SERVICE_FAILURE: i32 = -1073731819i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RPCSS_STOP_SERVICE_FAILURE: i32 = -1073731795i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_RUNNING_LASTKNOWNGOOD: i32 = -1073734797i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SCOPE_LABEL_TOO_LONG: i32 = -2147479331i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SCOPE_TOO_LONG: i32 = -2147479330i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SECOND_LOGON_FAILED: i32 = -1073734810i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_CONFIG_BACKOUT_FAILED: i32 = -1073734787i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_CONTROL_SUCCESS: i32 = 1073748859i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_CRASH: i32 = -1073734793i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_CRASH_NO_ACTION: i32 = -1073734790i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_DIFFERENT_PID_CONNECTED: i32 = -2147476609i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_EXIT_FAILED: i32 = -1073734801i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_EXIT_FAILED_SPECIFIC: i32 = -1073734800i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_LOGON_TYPE_NOT_GRANTED: i32 = -1073734783i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_NOT_INTERACTIVE: i32 = -1073734794i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_RECOVERY_FAILED: i32 = -1073734792i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_SCESRV_FAILED: i32 = -1073734791i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_SHUTDOWN_FAILED: i32 = -1073734781i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_START_AT_BOOT_FAILED: i32 = -1073734799i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_START_FAILED: i32 = -1073734824i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_START_FAILED_GROUP: i32 = -1073734822i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_START_FAILED_II: i32 = -1073734823i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_START_FAILED_NONE: i32 = -1073734821i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_START_HUNG: i32 = -1073734802i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_START_TYPE_CHANGED: i32 = 1073748864i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_STATUS_SUCCESS: i32 = 1073748860i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SERVICE_STOP_SUCCESS_WITH_REASON: i32 = 1073748866i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SEVERE_SERVICE_FAILED: i32 = -1073734803i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_BIND_DUP_NAME: i32 = -1073739319i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_BIND_TO_TRANSPORT: i32 = -2147481144i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_CHANGE_DOMAIN_NAME: i32 = -2147481136i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_CREATE_DEVICE: i32 = -1073739822i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_CREATE_PROCESS: i32 = -1073739821i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_CREATE_THREAD: i32 = -1073739820i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_GROW_TABLE: i32 = -2147481639i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_LOAD_DRIVER: i32 = -2147481140i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_MAP_ERROR: i32 = -2147481138i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_OPEN_NPFS: i32 = -1073739817i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_RECREATE_SHARE: i32 = -2147481137i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_START_SCAVENGER: i32 = -1073739814i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_CANT_UNLOAD_DRIVER: i32 = -2147481139i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_DISK_FULL: i32 = -2147481635i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_DOS_ATTACK_DETECTED: i32 = -2147481623i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_INVALID_REGISTRY_VALUE: i32 = -2147481142i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_INVALID_REQUEST: i32 = -1073739818i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_INVALID_SD: i32 = -2147481141i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_IRP_STACK_SIZE: i32 = -1073739813i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_KEY_NOT_CREATED: i32 = -1073739322i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_KEY_NOT_FOUND: i32 = -1073739323i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_NETWORK_ERROR: i32 = -2147481636i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_NONPAGED_POOL_LIMIT: i32 = -1073739807i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_NO_BLOCKING_IO: i32 = -2147481624i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_NO_FREE_CONNECTIONS: i32 = -2147481626i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_NO_FREE_RAW_WORK_ITEM: i32 = -2147481625i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_NO_NONPAGED_POOL: i32 = -1073739805i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_NO_PAGED_POOL: i32 = -1073739804i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_NO_TRANSPORTS_BOUND: i32 = -1073739321i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_NO_VIRTUAL_MEMORY: i32 = -1073739808i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_NO_WORK_ITEM: i32 = -2147481627i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_OUT_OF_WORK_ITEM_DOS: i32 = -2147481621i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_PAGED_POOL_LIMIT: i32 = -1073739806i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_RESOURCE_SHORTAGE: i32 = -1073739823i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_SERVICE_FAILED: i32 = -1073739824i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_TOO_MANY_DOS: i32 = -2147481622i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_TXF_INIT_FAILED: i32 = -2147481135i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_SRV_UNEXPECTED_DISC: i32 = -1073739819i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_STREAMS_ALLOCB_FAILURE: i32 = -2147479647i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_STREAMS_ALLOCB_FAILURE_CNT: i32 = -2147479646i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_STREAMS_ESBALLOC_FAILURE: i32 = -2147479645i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_STREAMS_ESBALLOC_FAILURE_CNT: i32 = -2147479644i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_STREAMS_STRLOG: i32 = -1073737824i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TAKE_OWNERSHIP: i32 = -1073734796i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP6_STARTED: i32 = 1073744924i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_ADAPTER_REG_FAILURE: i32 = -1073737633i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_ADDRESS_CONFLICT1: i32 = -1073737626i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_ADDRESS_CONFLICT2: i32 = -1073737625i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_AUTOCONFIGURED_ADDRESS_LIMIT_REACHED: i32 = -2147479444i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_AUTOCONFIGURED_ROUTE_LIMIT_REACHED: i32 = -2147479443i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_CREATE_DEVICE_FAILED: i32 = -1073737724i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_DHCP_INIT_FAILED: i32 = -2147479458i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_INTERFACE_BIND_FAILURE: i32 = -1073737617i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_INVALID_ADDRESS: i32 = -1073737637i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_INVALID_DEFAULT_GATEWAY: i32 = -2147479456i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_INVALID_MASK: i32 = -1073737636i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_IPV4_UNINSTALLED: i32 = 1073746027i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_IP_INIT_FAILED: i32 = -1073737628i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_MEDIA_CONNECT: i32 = 1073746025i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_MEDIA_DISCONNECT: i32 = 1073746026i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_NO_ADAPTER_RESOURCES: i32 = -1073737635i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_NO_ADDRESS_LIST: i32 = -1073737631i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_NO_BINDINGS: i32 = -1073737629i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_NO_MASK: i32 = -1073737638i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_NO_MASK_LIST: i32 = -1073737630i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_NO_RESOURCES_FOR_INIT: i32 = -1073737723i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_NTE_CONTEXT_LIST_FAILURE: i32 = -1073737624i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_OUT_OF_ORDER_FRAGMENTS_EXCEEDED: i32 = -2147479442i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_PCF_CLEAR_FILTER_FAILURE: i32 = -1073737530i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_PCF_MISSING_CAPABILITY: i32 = -2147479357i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_PCF_MULTICAST_OID_ISSUE: i32 = -2147479358i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_PCF_NO_ARP_FILTER: i32 = -2147479355i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_PCF_SET_FILTER_FAILURE: i32 = -2147479356i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_TCP_CONNECTIONS_PERF_IMPACTED: i32 = -2147479418i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_TCP_CONNECT_LIMIT_REACHED: i32 = -2147479422i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_TCP_GLOBAL_EPHEMERAL_PORT_SPACE_EXHAUSTED: i32 = -2147479417i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_TCP_INIT_FAILED: i32 = -1073737599i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_TCP_MPP_ATTACKS_DETECTED: i32 = -2147479419i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_TCP_TIME_WAIT_COLLISION: i32 = -2147479421i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_TCP_WSD_WS_RESTRICTED: i32 = -2147479420i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_TOO_MANY_GATEWAYS: i32 = -2147479451i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_TOO_MANY_NETS: i32 = -1073737639i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_UDP_GLOBAL_EPHEMERAL_PORT_SPACE_EXHAUSTED: i32 = -2147479382i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TCPIP_UDP_LIMIT_REACHED: i32 = -2147479383i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSACT_INVALID: i32 = -1073734812i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSACT_TIMEOUT: i32 = -1073734813i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSPORT_ADAPTER_NOT_FOUND: i32 = -1073732818i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSPORT_BAD_PROTOCOL: i32 = 1073750835i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSPORT_BINDING_FAILED: i32 = -1073732819i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSPORT_QUERY_OID_FAILED: i32 = -1073732816i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSPORT_REGISTER_FAILED: i32 = -1073732820i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSPORT_RESOURCE_LIMIT: i32 = -2147474646i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSPORT_RESOURCE_POOL: i32 = -2147474647i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSPORT_RESOURCE_SPECIFIC: i32 = -2147474645i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSPORT_SET_OID_FAILED: i32 = -1073732817i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSPORT_TOO_MANY_LINKS: i32 = 1073750834i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRANSPORT_TRANSFER_DATA: i32 = 1073750833i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRK_INTERNAL_ERROR: i32 = -1073729324i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRK_SERVICE_CORRUPT_LOG: i32 = -1073729321i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRK_SERVICE_DUPLICATE_VOLIDS: i32 = 1073754331i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRK_SERVICE_MOVE_QUOTA_EXCEEDED: i32 = -2147471140i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRK_SERVICE_START_FAILURE: i32 = -1073729322i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRK_SERVICE_START_SUCCESS: i32 = 1073754325i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRK_SERVICE_VOLUME_CLAIM: i32 = 1073754330i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRK_SERVICE_VOLUME_CREATE: i32 = 1073754329i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_TRK_SERVICE_VOL_QUOTA_EXCEEDED: i32 = -2147471144i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_UP_DRIVER_ON_MP: i32 = -1073735724i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_WEBCLIENT_CLOSE_DELETE_FAILED: i32 = -2147468746i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_WEBCLIENT_CLOSE_PROPPATCH_FAILED: i32 = -2147468745i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_WEBCLIENT_CLOSE_PUT_FAILED: i32 = -2147468747i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_WEBCLIENT_SETINFO_PROPPATCH_FAILED: i32 = -2147468744i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_WINNAT_SESSION_LIMIT_REACHED: i32 = -2147466648i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_WINSOCK_CLOSESOCKET_STUCK: i32 = -2147467646i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_WINSOCK_TDI_FILTER_DETECTED: i32 = -2147467647i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVENT_WSK_OWNINGTHREAD_PARAMETER_IGNORED: i32 = -1073725824i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EVLEN: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EXTRA_EXIT_POINT: i32 = -1073727524i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EXTRA_EXIT_POINT_DELETED: i32 = -1073727520i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EXTRA_EXIT_POINT_NOT_DELETED: i32 = -1073727519i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EXTRA_VOLUME: i32 = -1073727521i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EXTRA_VOLUME_DELETED: i32 = -1073727514i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const EXTRA_VOLUME_NOT_DELETED: i32 = -1073727513i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FLAT_STRING {
    pub MaximumLength: i16,
    pub Length: i16,
    pub Buffer: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FLAT_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FLAT_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FLAT_STRING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FLAT_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLAT_STRING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FLAT_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FLAT_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type FORCE_LEVEL_FLAGS = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_NOFORCE: FORCE_LEVEL_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_FORCE: FORCE_LEVEL_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_LOTS_OF_FORCE: FORCE_LEVEL_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const GNLEN: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const GROUPIDMASK: u32 = 32768u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const GROUP_ALL_PARMNUM: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const GROUP_ATTRIBUTES_PARMNUM: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const GROUP_COMMENT_PARMNUM: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_INFO_0 {
    pub grpi0_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GROUP_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_INFO_1 {
    pub grpi1_name: super::super::Foundation::PWSTR,
    pub grpi1_comment: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GROUP_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_INFO_1002 {
    pub grpi1002_comment: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_INFO_1002 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_INFO_1002 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GROUP_INFO_1002 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_INFO_1002>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_INFO_1002 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct GROUP_INFO_1005 {
    pub grpi1005_attributes: u32,
}
impl ::core::marker::Copy for GROUP_INFO_1005 {}
impl ::core::clone::Clone for GROUP_INFO_1005 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GROUP_INFO_1005 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GROUP_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_INFO_1005>()) == 0 }
    }
}
impl ::core::cmp::Eq for GROUP_INFO_1005 {}
impl ::core::default::Default for GROUP_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_INFO_2 {
    pub grpi2_name: super::super::Foundation::PWSTR,
    pub grpi2_comment: super::super::Foundation::PWSTR,
    pub grpi2_group_id: u32,
    pub grpi2_attributes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GROUP_INFO_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_INFO_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_INFO_3 {
    pub grpi3_name: super::super::Foundation::PWSTR,
    pub grpi3_comment: super::super::Foundation::PWSTR,
    pub grpi3_group_sid: super::super::Foundation::PSID,
    pub grpi3_attributes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GROUP_INFO_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_INFO_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const GROUP_NAME_PARMNUM: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_USERS_INFO_0 {
    pub grui0_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_USERS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_USERS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GROUP_USERS_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_USERS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_USERS_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_USERS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_USERS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_USERS_INFO_1 {
    pub grui1_name: super::super::Foundation::PWSTR,
    pub grui1_attributes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_USERS_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_USERS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GROUP_USERS_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_USERS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_USERS_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_USERS_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_USERS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNetScheduleAccountInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszservername: Param0, ccaccount: u32, wszaccount: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNetScheduleAccountInformation(pwszservername: super::super::Foundation::PWSTR, ccaccount: u32, wszaccount: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        GetNetScheduleAccountInformation(pwszservername.into_param().abi(), ::core::mem::transmute(ccaccount), ::core::mem::transmute(wszaccount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct HARDWARE_ADDRESS {
    pub Address: [u8; 6],
}
impl ::core::marker::Copy for HARDWARE_ADDRESS {}
impl ::core::clone::Clone for HARDWARE_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HARDWARE_ADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HARDWARE_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HARDWARE_ADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HARDWARE_ADDRESS {}
impl ::core::default::Default for HARDWARE_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const HARDWARE_ADDRESS_LENGTH: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct HLOG {
    pub time: u32,
    pub last_flags: u32,
    pub offset: u32,
    pub rec_offset: u32,
}
impl ::core::marker::Copy for HLOG {}
impl ::core::clone::Clone for HLOG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HLOG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HLOG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HLOG>()) == 0 }
    }
}
impl ::core::cmp::Eq for HLOG {}
impl ::core::default::Default for HLOG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct IEnumNetCfgBindingInterface(::windows::core::IUnknown);
impl IEnumNetCfgBindingInterface {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingInterface>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetCfgBindingInterface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetCfgBindingInterface>(result__)
    }
}
impl ::core::convert::From<IEnumNetCfgBindingInterface> for ::windows::core::IUnknown {
    fn from(value: IEnumNetCfgBindingInterface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetCfgBindingInterface> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetCfgBindingInterface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetCfgBindingInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumNetCfgBindingInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetCfgBindingInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetCfgBindingInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetCfgBindingInterface {}
unsafe impl ::windows::core::Interface for IEnumNetCfgBindingInterface {
    type Vtable = IEnumNetCfgBindingInterfaceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae90_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetCfgBindingInterfaceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct IEnumNetCfgBindingPath(::windows::core::IUnknown);
impl IEnumNetCfgBindingPath {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingPath>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetCfgBindingPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetCfgBindingPath>(result__)
    }
}
impl ::core::convert::From<IEnumNetCfgBindingPath> for ::windows::core::IUnknown {
    fn from(value: IEnumNetCfgBindingPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetCfgBindingPath> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetCfgBindingPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetCfgBindingPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumNetCfgBindingPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetCfgBindingPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetCfgBindingPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetCfgBindingPath {}
unsafe impl ::windows::core::Interface for IEnumNetCfgBindingPath {
    type Vtable = IEnumNetCfgBindingPathVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae91_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetCfgBindingPathVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct IEnumNetCfgComponent(::windows::core::IUnknown);
impl IEnumNetCfgComponent {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgComponent>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumNetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetCfgComponent>(result__)
    }
}
impl ::core::convert::From<IEnumNetCfgComponent> for ::windows::core::IUnknown {
    fn from(value: IEnumNetCfgComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNetCfgComponent> for ::windows::core::IUnknown {
    fn from(value: &IEnumNetCfgComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumNetCfgComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumNetCfgComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNetCfgComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNetCfgComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetCfgComponent {}
unsafe impl ::windows::core::Interface for IEnumNetCfgComponent {
    type Vtable = IEnumNetCfgComponentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae92_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetCfgComponentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const INTERFACE_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const INVALID_TRACEID: u32 = 4294967295u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfg(::windows::core::IUnknown);
impl INetCfg {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Initialize(&self, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvreserved)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Uninitialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Apply(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn EnumComponents(&self, pguidclass: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumNetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidclass), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetCfgComponent>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindComponent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwinfid: Param0) -> ::windows::core::Result<INetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pszwinfid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<INetCfgComponent>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn QueryNetCfgClass(&self, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidclass), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
}
impl ::core::convert::From<INetCfg> for ::windows::core::IUnknown {
    fn from(value: INetCfg) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfg> for ::windows::core::IUnknown {
    fn from(value: &INetCfg) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfg {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfg {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfg {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfg {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfg {}
unsafe impl ::windows::core::Interface for INetCfg {
    type Vtable = INetCfgVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae93_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, ppenumcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, pcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgBindingInterface(::windows::core::IUnknown);
impl INetCfgBindingInterface {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn GetUpperComponent(&self) -> ::windows::core::Result<INetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetCfgComponent>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn GetLowerComponent(&self) -> ::windows::core::Result<INetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetCfgComponent>(result__)
    }
}
impl ::core::convert::From<INetCfgBindingInterface> for ::windows::core::IUnknown {
    fn from(value: INetCfgBindingInterface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgBindingInterface> for ::windows::core::IUnknown {
    fn from(value: &INetCfgBindingInterface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgBindingInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgBindingInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgBindingInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgBindingInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgBindingInterface {}
unsafe impl ::windows::core::Interface for INetCfgBindingInterface {
    type Vtable = INetCfgBindingInterfaceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae94_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgBindingInterfaceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwinterfacename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgBindingPath(::windows::core::IUnknown);
impl INetCfgBindingPath {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn IsSamePathAs<'a, Param0: ::windows::core::IntoParam<'a, INetCfgBindingPath>>(&self, ppath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ppath.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn IsSubPathOf<'a, Param0: ::windows::core::IntoParam<'a, INetCfgBindingPath>>(&self, ppath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ppath.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn IsEnabled(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPathToken(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<INetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INetCfgComponent>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn GetDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn EnumBindingInterfaces(&self) -> ::windows::core::Result<IEnumNetCfgBindingInterface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetCfgBindingInterface>(result__)
    }
}
impl ::core::convert::From<INetCfgBindingPath> for ::windows::core::IUnknown {
    fn from(value: INetCfgBindingPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgBindingPath> for ::windows::core::IUnknown {
    fn from(value: &INetCfgBindingPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgBindingPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgBindingPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgBindingPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgBindingPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgBindingPath {}
unsafe impl ::windows::core::Interface for INetCfgBindingPath {
    type Vtable = INetCfgBindingPathVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae96_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgBindingPathVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwpathtoken: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinterfaces: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenuminterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgClass(::windows::core::IUnknown);
impl INetCfgClass {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindComponent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwinfid: Param0) -> ::windows::core::Result<INetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszwinfid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<INetCfgComponent>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn EnumComponents(&self) -> ::windows::core::Result<IEnumNetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetCfgComponent>(result__)
    }
}
impl ::core::convert::From<INetCfgClass> for ::windows::core::IUnknown {
    fn from(value: INetCfgClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgClass> for ::windows::core::IUnknown {
    fn from(value: &INetCfgClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgClass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgClass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgClass {}
unsafe impl ::windows::core::Interface for INetCfgClass {
    type Vtable = INetCfgClassVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae97_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgClassVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgClassSetup(::windows::core::IUnknown);
impl INetCfgClassSetup {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectAndInstall<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, pobotoken: *const OBO_TOKEN) -> ::windows::core::Result<INetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(pobotoken), ::core::mem::transmute(&mut result__)).from_abi::<INetCfgComponent>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Install<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwinfid: Param0, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: Param4, pszwanswersections: Param5) -> ::windows::core::Result<INetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszwinfid.into_param().abi(), ::core::mem::transmute(pobotoken), ::core::mem::transmute(dwsetupflags), ::core::mem::transmute(dwupgradefrombuildno), pszwanswerfile.into_param().abi(), pszwanswersections.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<INetCfgComponent>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeInstall<'a, Param0: ::windows::core::IntoParam<'a, INetCfgComponent>>(&self, pcomponent: Param0, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pcomponent.into_param().abi(), ::core::mem::transmute(pobotoken), ::core::mem::transmute(pmszwrefs)).ok()
    }
}
impl ::core::convert::From<INetCfgClassSetup> for ::windows::core::IUnknown {
    fn from(value: INetCfgClassSetup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgClassSetup> for ::windows::core::IUnknown {
    fn from(value: &INetCfgClassSetup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgClassSetup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgClassSetup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgClassSetup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgClassSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgClassSetup {}
unsafe impl ::windows::core::Interface for INetCfgClassSetup {
    type Vtable = INetCfgClassSetupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae9d_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgClassSetupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomponent: ::windows::core::RawPtr, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgClassSetup2(::windows::core::IUnknown);
impl INetCfgClassSetup2 {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectAndInstall<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, pobotoken: *const OBO_TOKEN) -> ::windows::core::Result<INetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(pobotoken), ::core::mem::transmute(&mut result__)).from_abi::<INetCfgComponent>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Install<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwinfid: Param0, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: Param4, pszwanswersections: Param5) -> ::windows::core::Result<INetCfgComponent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszwinfid.into_param().abi(), ::core::mem::transmute(pobotoken), ::core::mem::transmute(dwsetupflags), ::core::mem::transmute(dwupgradefrombuildno), pszwanswerfile.into_param().abi(), pszwanswersections.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<INetCfgComponent>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeInstall<'a, Param0: ::windows::core::IntoParam<'a, INetCfgComponent>>(&self, pcomponent: Param0, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pcomponent.into_param().abi(), ::core::mem::transmute(pobotoken), ::core::mem::transmute(pmszwrefs)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn UpdateNonEnumeratedComponent<'a, Param0: ::windows::core::IntoParam<'a, INetCfgComponent>>(&self, picomp: Param0, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), picomp.into_param().abi(), ::core::mem::transmute(dwsetupflags), ::core::mem::transmute(dwupgradefrombuildno)).ok()
    }
}
impl ::core::convert::From<INetCfgClassSetup2> for INetCfgClassSetup {
    fn from(value: INetCfgClassSetup2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgClassSetup2> for INetCfgClassSetup {
    fn from(value: &INetCfgClassSetup2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INetCfgClassSetup> for INetCfgClassSetup2 {
    fn into_param(self) -> ::windows::core::Param<'a, INetCfgClassSetup> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INetCfgClassSetup> for &INetCfgClassSetup2 {
    fn into_param(self) -> ::windows::core::Param<'a, INetCfgClassSetup> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INetCfgClassSetup2> for ::windows::core::IUnknown {
    fn from(value: INetCfgClassSetup2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgClassSetup2> for ::windows::core::IUnknown {
    fn from(value: &INetCfgClassSetup2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgClassSetup2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgClassSetup2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgClassSetup2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgClassSetup2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgClassSetup2 {}
unsafe impl ::windows::core::Interface for INetCfgClassSetup2 {
    type Vtable = INetCfgClassSetup2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8aea0_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgClassSetup2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomponent: ::windows::core::RawPtr, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, picomp: ::windows::core::RawPtr, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgComponent(::windows::core::IUnknown);
impl INetCfgComponent {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwdisplayname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszwdisplayname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpText(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetId(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn GetCharacteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn GetInstanceGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPnpDevNodeId(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn GetClassGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBindName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn GetDeviceStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_System_Registry'*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn OpenParamKey(&self) -> ::windows::core::Result<super::super::System::Registry::HKEY> {
        let mut result__: super::super::System::Registry::HKEY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Registry::HKEY>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RaisePropertyUi<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, hwndparent: Param0, dwflags: u32, punkcontext: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(dwflags), punkcontext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<INetCfgComponent> for ::windows::core::IUnknown {
    fn from(value: INetCfgComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgComponent> for ::windows::core::IUnknown {
    fn from(value: &INetCfgComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgComponent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponent {}
unsafe impl ::windows::core::Interface for INetCfgComponent {
    type Vtable = INetCfgComponentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae99_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwdisplayname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwhelptext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwdevnodeid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwbindname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phkey: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgComponentBindings(::windows::core::IUnknown);
impl INetCfgComponentBindings {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn BindTo<'a, Param0: ::windows::core::IntoParam<'a, INetCfgComponent>>(&self, pnccitem: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pnccitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn UnbindFrom<'a, Param0: ::windows::core::IntoParam<'a, INetCfgComponent>>(&self, pnccitem: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pnccitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsBindingInterface<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwflags: u32, pszwinterfacename: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pszwinterfacename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn IsBoundTo<'a, Param0: ::windows::core::IntoParam<'a, INetCfgComponent>>(&self, pnccitem: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pnccitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn IsBindableTo<'a, Param0: ::windows::core::IntoParam<'a, INetCfgComponent>>(&self, pnccitem: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pnccitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn EnumBindingPaths(&self, dwflags: u32) -> ::windows::core::Result<IEnumNetCfgBindingPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(&mut result__)).from_abi::<IEnumNetCfgBindingPath>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn MoveBefore<'a, Param0: ::windows::core::IntoParam<'a, INetCfgBindingPath>, Param1: ::windows::core::IntoParam<'a, INetCfgBindingPath>>(&self, pncbitemsrc: Param0, pncbitemdest: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pncbitemsrc.into_param().abi(), pncbitemdest.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn MoveAfter<'a, Param0: ::windows::core::IntoParam<'a, INetCfgBindingPath>, Param1: ::windows::core::IntoParam<'a, INetCfgBindingPath>>(&self, pncbitemsrc: Param0, pncbitemdest: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pncbitemsrc.into_param().abi(), pncbitemdest.into_param().abi()).ok()
    }
}
impl ::core::convert::From<INetCfgComponentBindings> for ::windows::core::IUnknown {
    fn from(value: INetCfgComponentBindings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgComponentBindings> for ::windows::core::IUnknown {
    fn from(value: &INetCfgComponentBindings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgComponentBindings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgComponentBindings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgComponentBindings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentBindings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentBindings {}
unsafe impl ::windows::core::Interface for INetCfgComponentBindings {
    type Vtable = INetCfgComponentBindingsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae9e_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentBindingsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszwinterfacename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncbitemsrc: ::windows::core::RawPtr, pncbitemdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncbitemsrc: ::windows::core::RawPtr, pncbitemdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgComponentControl(::windows::core::IUnknown);
impl INetCfgComponentControl {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, INetCfgComponent>, Param1: ::windows::core::IntoParam<'a, INetCfg>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, picomp: Param0, pinetcfg: Param1, finstalling: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), picomp.into_param().abi(), pinetcfg.into_param().abi(), finstalling.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn ApplyRegistryChanges(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn ApplyPnpChanges<'a, Param0: ::windows::core::IntoParam<'a, INetCfgPnpReconfigCallback>>(&self, picallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), picallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn CancelChanges(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<INetCfgComponentControl> for ::windows::core::IUnknown {
    fn from(value: INetCfgComponentControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgComponentControl> for ::windows::core::IUnknown {
    fn from(value: &INetCfgComponentControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgComponentControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgComponentControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgComponentControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentControl {}
unsafe impl ::windows::core::Interface for INetCfgComponentControl {
    type Vtable = INetCfgComponentControlVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238df_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentControlVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, picomp: ::windows::core::RawPtr, pinetcfg: ::windows::core::RawPtr, finstalling: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgComponentNotifyBinding(::windows::core::IUnknown);
impl INetCfgComponentNotifyBinding {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn QueryBindingPath<'a, Param1: ::windows::core::IntoParam<'a, INetCfgBindingPath>>(&self, dwchangeflag: u32, pipath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchangeflag), pipath.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn NotifyBindingPath<'a, Param1: ::windows::core::IntoParam<'a, INetCfgBindingPath>>(&self, dwchangeflag: u32, pipath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchangeflag), pipath.into_param().abi()).ok()
    }
}
impl ::core::convert::From<INetCfgComponentNotifyBinding> for ::windows::core::IUnknown {
    fn from(value: INetCfgComponentNotifyBinding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgComponentNotifyBinding> for ::windows::core::IUnknown {
    fn from(value: &INetCfgComponentNotifyBinding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgComponentNotifyBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgComponentNotifyBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgComponentNotifyBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentNotifyBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentNotifyBinding {}
unsafe impl ::windows::core::Interface for INetCfgComponentNotifyBinding {
    type Vtable = INetCfgComponentNotifyBindingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238e1_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentNotifyBindingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgComponentNotifyGlobal(::windows::core::IUnknown);
impl INetCfgComponentNotifyGlobal {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn GetSupportedNotifications(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn SysQueryBindingPath<'a, Param1: ::windows::core::IntoParam<'a, INetCfgBindingPath>>(&self, dwchangeflag: u32, pipath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchangeflag), pipath.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn SysNotifyBindingPath<'a, Param1: ::windows::core::IntoParam<'a, INetCfgBindingPath>>(&self, dwchangeflag: u32, pipath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchangeflag), pipath.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn SysNotifyComponent<'a, Param1: ::windows::core::IntoParam<'a, INetCfgComponent>>(&self, dwchangeflag: u32, picomp: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchangeflag), picomp.into_param().abi()).ok()
    }
}
impl ::core::convert::From<INetCfgComponentNotifyGlobal> for ::windows::core::IUnknown {
    fn from(value: INetCfgComponentNotifyGlobal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgComponentNotifyGlobal> for ::windows::core::IUnknown {
    fn from(value: &INetCfgComponentNotifyGlobal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgComponentNotifyGlobal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgComponentNotifyGlobal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgComponentNotifyGlobal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentNotifyGlobal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentNotifyGlobal {}
unsafe impl ::windows::core::Interface for INetCfgComponentNotifyGlobal {
    type Vtable = INetCfgComponentNotifyGlobalVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238e2_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentNotifyGlobalVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwnotifications: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeflag: u32, picomp: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgComponentPropertyUi(::windows::core::IUnknown);
impl INetCfgComponentPropertyUi {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn QueryPropertyUi<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punkreserved: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkreserved.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn SetContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punkreserved: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punkreserved.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MergePropPages<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: Param3, pszstartpage: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdefpages), ::core::mem::transmute(pahpspprivate), ::core::mem::transmute(pcpages), hwndparent.into_param().abi(), ::core::mem::transmute(pszstartpage)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ValidateProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndsheet: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwndsheet.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn ApplyProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn CancelProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<INetCfgComponentPropertyUi> for ::windows::core::IUnknown {
    fn from(value: INetCfgComponentPropertyUi) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgComponentPropertyUi> for ::windows::core::IUnknown {
    fn from(value: &INetCfgComponentPropertyUi) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgComponentPropertyUi {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgComponentPropertyUi {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgComponentPropertyUi {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentPropertyUi {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentPropertyUi {}
unsafe impl ::windows::core::Interface for INetCfgComponentPropertyUi {
    type Vtable = INetCfgComponentPropertyUiVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238e0_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentPropertyUiVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndsheet: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgComponentSetup(::windows::core::IUnknown);
impl INetCfgComponentSetup {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Install(&self, dwsetupflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsetupflags)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Upgrade(&self, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsetupflags), ::core::mem::transmute(dwupgradefombuildno)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadAnswerFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwanswerfile: Param0, pszwanswersections: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszwanswerfile.into_param().abi(), pszwanswersections.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn Removing(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<INetCfgComponentSetup> for ::windows::core::IUnknown {
    fn from(value: INetCfgComponentSetup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgComponentSetup> for ::windows::core::IUnknown {
    fn from(value: &INetCfgComponentSetup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgComponentSetup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgComponentSetup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgComponentSetup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentSetup {}
unsafe impl ::windows::core::Interface for INetCfgComponentSetup {
    type Vtable = INetCfgComponentSetupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238e3_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentSetupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsetupflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgComponentSysPrep(::windows::core::IUnknown);
impl INetCfgComponentSysPrep {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAdapterParameters<'a, Param0: ::windows::core::IntoParam<'a, INetCfgSysPrep>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pncsp: Param0, pszwanswersections: Param1, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pncsp.into_param().abi(), pszwanswersections.into_param().abi(), ::core::mem::transmute(padapterinstanceguid)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreAdapterParameters<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwanswerfile: Param0, pszwanswersection: Param1, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszwanswerfile.into_param().abi(), pszwanswersection.into_param().abi(), ::core::mem::transmute(padapterinstanceguid)).ok()
    }
}
impl ::core::convert::From<INetCfgComponentSysPrep> for ::windows::core::IUnknown {
    fn from(value: INetCfgComponentSysPrep) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgComponentSysPrep> for ::windows::core::IUnknown {
    fn from(value: &INetCfgComponentSysPrep) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgComponentSysPrep {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgComponentSysPrep {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgComponentSysPrep {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentSysPrep {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentSysPrep {}
unsafe impl ::windows::core::Interface for INetCfgComponentSysPrep {
    type Vtable = INetCfgComponentSysPrepVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae9a_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentSysPrepVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncsp: ::windows::core::RawPtr, pszwanswersections: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersection: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgComponentUpperEdge(::windows::core::IUnknown);
impl INetCfgComponentUpperEdge {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn GetInterfaceIdsForAdapter<'a, Param0: ::windows::core::IntoParam<'a, INetCfgComponent>>(&self, padapter: Param0, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), padapter.into_param().abi(), ::core::mem::transmute(pdwnuminterfaces), ::core::mem::transmute(ppguidinterfaceids)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn AddInterfacesToAdapter<'a, Param0: ::windows::core::IntoParam<'a, INetCfgComponent>>(&self, padapter: Param0, dwnuminterfaces: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), padapter.into_param().abi(), ::core::mem::transmute(dwnuminterfaces)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn RemoveInterfacesFromAdapter<'a, Param0: ::windows::core::IntoParam<'a, INetCfgComponent>>(&self, padapter: Param0, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), padapter.into_param().abi(), ::core::mem::transmute(dwnuminterfaces), ::core::mem::transmute(pguidinterfaceids)).ok()
    }
}
impl ::core::convert::From<INetCfgComponentUpperEdge> for ::windows::core::IUnknown {
    fn from(value: INetCfgComponentUpperEdge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgComponentUpperEdge> for ::windows::core::IUnknown {
    fn from(value: &INetCfgComponentUpperEdge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgComponentUpperEdge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgComponentUpperEdge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgComponentUpperEdge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentUpperEdge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentUpperEdge {}
unsafe impl ::windows::core::Interface for INetCfgComponentUpperEdge {
    type Vtable = INetCfgComponentUpperEdgeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932238e4_bea1_11d0_9298_00c04fc99dcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgComponentUpperEdgeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, dwnuminterfaces: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgLock(::windows::core::IUnknown);
impl INetCfgLock {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AcquireWriteLock<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, cmstimeout: u32, pszwclientdescription: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cmstimeout), pszwclientdescription.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn ReleaseWriteLock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWriteLocked(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
impl ::core::convert::From<INetCfgLock> for ::windows::core::IUnknown {
    fn from(value: INetCfgLock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgLock> for ::windows::core::IUnknown {
    fn from(value: &INetCfgLock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgLock {}
unsafe impl ::windows::core::Interface for INetCfgLock {
    type Vtable = INetCfgLockVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae9f_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgLockVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmstimeout: u32, pszwclientdescription: super::super::Foundation::PWSTR, ppszwclientdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwclientdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgPnpReconfigCallback(::windows::core::IUnknown);
impl INetCfgPnpReconfigCallback {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendPnpReconfig<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, layer: NCPNP_RECONFIG_LAYER, pszwupper: Param1, pszwlower: Param2, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(layer), pszwupper.into_param().abi(), pszwlower.into_param().abi(), ::core::mem::transmute(pvdata), ::core::mem::transmute(dwsizeofdata)).ok()
    }
}
impl ::core::convert::From<INetCfgPnpReconfigCallback> for ::windows::core::IUnknown {
    fn from(value: INetCfgPnpReconfigCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgPnpReconfigCallback> for ::windows::core::IUnknown {
    fn from(value: &INetCfgPnpReconfigCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgPnpReconfigCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgPnpReconfigCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgPnpReconfigCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgPnpReconfigCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgPnpReconfigCallback {}
unsafe impl ::windows::core::Interface for INetCfgPnpReconfigCallback {
    type Vtable = INetCfgPnpReconfigCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d84bd35_e227_11d2_b700_00a0c98a6a85);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgPnpReconfigCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, layer: NCPNP_RECONFIG_LAYER, pszwupper: super::super::Foundation::PWSTR, pszwlower: super::super::Foundation::PWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetCfgSysPrep(::windows::core::IUnknown);
impl INetCfgSysPrep {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HrSetupSetFirstDword<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszsection: Param0, pwszkey: Param1, dwvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszsection.into_param().abi(), pwszkey.into_param().abi(), ::core::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HrSetupSetFirstString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszsection: Param0, pwszkey: Param1, pwszvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwszsection.into_param().abi(), pwszkey.into_param().abi(), pwszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HrSetupSetFirstStringAsBool<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszsection: Param0, pwszkey: Param1, fvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwszsection.into_param().abi(), pwszkey.into_param().abi(), fvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HrSetupSetFirstMultiSzField<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszsection: Param0, pwszkey: Param1, pmszvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pwszsection.into_param().abi(), pwszkey.into_param().abi(), pmszvalue.into_param().abi()).ok()
    }
}
impl ::core::convert::From<INetCfgSysPrep> for ::windows::core::IUnknown {
    fn from(value: INetCfgSysPrep) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetCfgSysPrep> for ::windows::core::IUnknown {
    fn from(value: &INetCfgSysPrep) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetCfgSysPrep {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetCfgSysPrep {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetCfgSysPrep {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetCfgSysPrep {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgSysPrep {}
unsafe impl ::windows::core::Interface for INetCfgSysPrep {
    type Vtable = INetCfgSysPrepVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0e8ae98_306e_11d1_aacf_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetCfgSysPrepVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, dwvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pwszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pmszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetLanConnectionUiInfo(::windows::core::IUnknown);
impl INetLanConnectionUiInfo {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
    pub unsafe fn GetDeviceGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<INetLanConnectionUiInfo> for ::windows::core::IUnknown {
    fn from(value: INetLanConnectionUiInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetLanConnectionUiInfo> for ::windows::core::IUnknown {
    fn from(value: &INetLanConnectionUiInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetLanConnectionUiInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetLanConnectionUiInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetLanConnectionUiInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetLanConnectionUiInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetLanConnectionUiInfo {}
unsafe impl ::windows::core::Interface for INetLanConnectionUiInfo {
    type Vtable = INetLanConnectionUiInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08956a6_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetLanConnectionUiInfoVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct INetRasConnectionIpUiInfo(::windows::core::IUnknown);
impl INetRasConnectionIpUiInfo {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUiInfo(&self) -> ::windows::core::Result<RASCON_IPUI> {
        let mut result__: RASCON_IPUI = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<RASCON_IPUI>(result__)
    }
}
impl ::core::convert::From<INetRasConnectionIpUiInfo> for ::windows::core::IUnknown {
    fn from(value: INetRasConnectionIpUiInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INetRasConnectionIpUiInfo> for ::windows::core::IUnknown {
    fn from(value: &INetRasConnectionIpUiInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetRasConnectionIpUiInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INetRasConnectionIpUiInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INetRasConnectionIpUiInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INetRasConnectionIpUiInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetRasConnectionIpUiInfo {}
unsafe impl ::windows::core::Interface for INetRasConnectionIpUiInfo {
    type Vtable = INetRasConnectionIpUiInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaedcf58_31fe_11d1_aad2_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetRasConnectionIpUiInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut RASCON_IPUI) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const IPX_PROTOCOL_BASE: u32 = 131071u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const IPX_PROTOCOL_RIP: u32 = 131072u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct IProvisioningDomain(::windows::core::IUnknown);
impl IProvisioningDomain {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwpathtofolder: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszwpathtofolder.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Data_Xml_MsXml', 'Win32_Foundation'*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszwdomain: Param0, pszwlanguage: Param1, pszwxpathquery: Param2) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszwdomain.into_param().abi(), pszwlanguage.into_param().abi(), pszwxpathquery.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Data::Xml::MsXml::IXMLDOMNodeList>(result__)
    }
}
impl ::core::convert::From<IProvisioningDomain> for ::windows::core::IUnknown {
    fn from(value: IProvisioningDomain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProvisioningDomain> for ::windows::core::IUnknown {
    fn from(value: &IProvisioningDomain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProvisioningDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IProvisioningDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProvisioningDomain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProvisioningDomain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvisioningDomain {}
unsafe impl ::windows::core::Interface for IProvisioningDomain {
    type Vtable = IProvisioningDomainVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc96fbd50_24dd_11d8_89fb_00904b2ea9c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisioningDomainVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwpathtofolder: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwdomain: super::super::Foundation::PWSTR, pszwlanguage: super::super::Foundation::PWSTR, pszwxpathquery: super::super::Foundation::PWSTR, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation")))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[repr(transparent)]
pub struct IProvisioningProfileWireless(::windows::core::IUnknown);
impl IProvisioningProfileWireless {
    #[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrxmlwirelessconfigprofile: Param0, bstrxmlconnectionconfigprofile: Param1, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrxmlwirelessconfigprofile.into_param().abi(), bstrxmlconnectionconfigprofile.into_param().abi(), ::core::mem::transmute(padapterinstanceguid), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IProvisioningProfileWireless> for ::windows::core::IUnknown {
    fn from(value: IProvisioningProfileWireless) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProvisioningProfileWireless> for ::windows::core::IUnknown {
    fn from(value: &IProvisioningProfileWireless) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProvisioningProfileWireless {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IProvisioningProfileWireless {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProvisioningProfileWireless {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProvisioningProfileWireless {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvisioningProfileWireless {}
unsafe impl ::windows::core::Interface for IProvisioningProfileWireless {
    type Vtable = IProvisioningProfileWirelessVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc96fbd51_24dd_11d8_89fb_00904b2ea9c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisioningProfileWirelessVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxmlwirelessconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrxmlconnectionconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, padapterinstanceguid: *const ::windows::core::GUID, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const IR_PROMISCUOUS: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const IR_PROMISCUOUS_MULTICAST: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn I_NetLogonControl2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, functioncode: u32, querylevel: u32, data: *const u8, buffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_NetLogonControl2(servername: super::super::Foundation::PWSTR, functioncode: u32, querylevel: u32, data: *const u8, buffer: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(I_NetLogonControl2(servername.into_param().abi(), ::core::mem::transmute(functioncode), ::core::mem::transmute(querylevel), ::core::mem::transmute(data), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const JOB_ADD_CURRENT_DATE: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const JOB_EXEC_ERROR: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const JOB_NONINTERACTIVE: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const JOB_RUNS_TODAY: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const JOB_RUN_PERIODICALLY: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const KNOWLEDGE_INCONSISTENCY_DETECTED: i32 = -1073727511i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LG_INCLUDE_INDIRECT: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_CNLEN: u32 = 15u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_DEVLEN: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_DNLEN: u32 = 15u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_GNLEN: u32 = 20u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_MAXCOMMENTSZ: u32 = 48u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_NNLEN: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_PATHLEN: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_PWLEN: u32 = 14u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_QNLEN: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_SERVICE_ACTIVE: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_SERVICE_CONTINUE_PENDING: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_SERVICE_PAUSED: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_SERVICE_PAUSE_PENDING: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_SNLEN: u32 = 15u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_STXTLEN: u32 = 63u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_UNCLEN: u32 = 17u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM20_UNLEN: u32 = 20u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LM_REDIR_FAILURE: i32 = 1073756225i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LOCALGROUP_COMMENT_PARMNUM: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOCALGROUP_INFO_0 {
    pub lgrpi0_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOCALGROUP_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOCALGROUP_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LOCALGROUP_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOCALGROUP_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOCALGROUP_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOCALGROUP_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOCALGROUP_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOCALGROUP_INFO_1 {
    pub lgrpi1_name: super::super::Foundation::PWSTR,
    pub lgrpi1_comment: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOCALGROUP_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOCALGROUP_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LOCALGROUP_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOCALGROUP_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOCALGROUP_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOCALGROUP_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOCALGROUP_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOCALGROUP_INFO_1002 {
    pub lgrpi1002_comment: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOCALGROUP_INFO_1002 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOCALGROUP_INFO_1002 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LOCALGROUP_INFO_1002 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOCALGROUP_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOCALGROUP_INFO_1002>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOCALGROUP_INFO_1002 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOCALGROUP_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOCALGROUP_MEMBERS_INFO_0 {
    pub lgrmi0_sid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOCALGROUP_MEMBERS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOCALGROUP_MEMBERS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LOCALGROUP_MEMBERS_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOCALGROUP_MEMBERS_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct LOCALGROUP_MEMBERS_INFO_1 {
    pub lgrmi1_sid: super::super::Foundation::PSID,
    pub lgrmi1_sidusage: super::super::Security::SID_NAME_USE,
    pub lgrmi1_name: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for LOCALGROUP_MEMBERS_INFO_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for LOCALGROUP_MEMBERS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for LOCALGROUP_MEMBERS_INFO_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOCALGROUP_MEMBERS_INFO_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct LOCALGROUP_MEMBERS_INFO_2 {
    pub lgrmi2_sid: super::super::Foundation::PSID,
    pub lgrmi2_sidusage: super::super::Security::SID_NAME_USE,
    pub lgrmi2_domainandname: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for LOCALGROUP_MEMBERS_INFO_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for LOCALGROUP_MEMBERS_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for LOCALGROUP_MEMBERS_INFO_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOCALGROUP_MEMBERS_INFO_2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOCALGROUP_MEMBERS_INFO_3 {
    pub lgrmi3_domainandname: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOCALGROUP_MEMBERS_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOCALGROUP_MEMBERS_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LOCALGROUP_MEMBERS_INFO_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOCALGROUP_MEMBERS_INFO_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LOCALGROUP_NAME_PARMNUM: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOCALGROUP_USERS_INFO_0 {
    pub lgrui0_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOCALGROUP_USERS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOCALGROUP_USERS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LOCALGROUP_USERS_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOCALGROUP_USERS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOCALGROUP_USERS_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOCALGROUP_USERS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOCALGROUP_USERS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LOGFLAGS_BACKWARD: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LOGFLAGS_FORWARD: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LOGFLAGS_SEEK: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LOWER_GET_HINT_MASK: u32 = 65280u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const LOWER_HINT_MASK: u32 = 255u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogErrorA(dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PSTR, dwerrorcode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogErrorA(dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PSTR, dwerrorcode: u32);
        }
        LogErrorA(::core::mem::transmute(dwmessageid), ::core::mem::transmute(cnumberofsubstrings), ::core::mem::transmute(plpwssubstrings), ::core::mem::transmute(dwerrorcode))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogErrorW(dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PWSTR, dwerrorcode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogErrorW(dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PWSTR, dwerrorcode: u32);
        }
        LogErrorW(::core::mem::transmute(dwmessageid), ::core::mem::transmute(cnumberofsubstrings), ::core::mem::transmute(plpwssubstrings), ::core::mem::transmute(dwerrorcode))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogEventA(weventtype: u32, dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogEventA(weventtype: u32, dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PSTR);
        }
        LogEventA(::core::mem::transmute(weventtype), ::core::mem::transmute(dwmessageid), ::core::mem::transmute(cnumberofsubstrings), ::core::mem::transmute(plpwssubstrings))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogEventW(weventtype: u32, dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PWSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogEventW(weventtype: u32, dwmessageid: u32, cnumberofsubstrings: u32, plpwssubstrings: *const super::super::Foundation::PWSTR);
        }
        LogEventW(::core::mem::transmute(weventtype), ::core::mem::transmute(dwmessageid), ::core::mem::transmute(cnumberofsubstrings), ::core::mem::transmute(plpwssubstrings))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MACHINE_UNJOINED: i32 = -1073727507i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MAJOR_VERSION_MASK: u32 = 15u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MAXCOMMENTSZ: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MAXPERMENTRIES: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MAX_LANMAN_MESSAGE_ID: u32 = 5899u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MAX_NERR: u32 = 2999u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MAX_PASSWD_LEN: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MAX_PREFERRED_LENGTH: u32 = 4294967295u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MAX_PROTOCOL_DLL_LEN: u32 = 48u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MAX_PROTOCOL_NAME_LEN: u32 = 40u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_BOUNDARY_REACHED: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_IIF: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_NOT_FORWARDING: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_NOT_LAST_HOP: u32 = 10u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_NO_ERROR: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_NO_MULTICAST: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_NO_ROUTE: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_NO_SPACE: u32 = 13u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_OIF_PRUNED: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_OLD_ROUTER: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_PROHIBITED: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_PRUNED_UPSTREAM: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_REACHED_CORE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MFE_WRONG_IF: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MIN_LANMAN_MESSAGE_ID: u32 = 2100u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MISSING_EXIT_POINT: i32 = -1073727523i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MISSING_EXIT_POINT_CREATED: i32 = -1073727518i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MISSING_EXIT_POINT_NOT_CREATED: i32 = -1073727517i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MISSING_VOLUME: i32 = -1073727522i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MISSING_VOLUME_CREATED: i32 = -1073727516i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MISSING_VOLUME_NOT_CREATED: i32 = -1073727515i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_DOMAIN_ID_PARMNUM: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_DOMAIN_NAME_PARMNUM: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_FORCE_LOGOFF_PARMNUM: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_LOCKOUT_DURATION_PARMNUM: u32 = 10u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_LOCKOUT_OBSERVATION_WINDOW_PARMNUM: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_LOCKOUT_THRESHOLD_PARMNUM: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_MAX_PASSWD_AGE_PARMNUM: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_MIN_PASSWD_AGE_PARMNUM: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_MIN_PASSWD_LEN_PARMNUM: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_PASSWD_HIST_LEN_PARMNUM: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_PRIMARY_PARMNUM: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MODALS_ROLE_PARMNUM: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct MPR_PROTOCOL_0 {
    pub dwProtocolId: u32,
    pub wszProtocol: [u16; 41],
    pub wszDLLName: [u16; 49],
}
impl ::core::marker::Copy for MPR_PROTOCOL_0 {}
impl ::core::clone::Clone for MPR_PROTOCOL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MPR_PROTOCOL_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MPR_PROTOCOL_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_PROTOCOL_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MPR_PROTOCOL_0 {}
impl ::core::default::Default for MPR_PROTOCOL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MRINFO_DISABLED_FLAG: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MRINFO_DOWN_FLAG: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MRINFO_LEAF_FLAG: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MRINFO_PIM_FLAG: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MRINFO_QUERIER_FLAG: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MRINFO_TUNNEL_FLAG: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct MSA_INFO_0 {
    pub State: MSA_INFO_STATE,
}
impl ::core::marker::Copy for MSA_INFO_0 {}
impl ::core::clone::Clone for MSA_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MSA_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MSA_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSA_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MSA_INFO_0 {}
impl ::core::default::Default for MSA_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type MSA_INFO_LEVEL = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MsaInfoLevel0: MSA_INFO_LEVEL = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MsaInfoLevelMax: MSA_INFO_LEVEL = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type MSA_INFO_STATE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MsaInfoNotExist: MSA_INFO_STATE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MsaInfoNotService: MSA_INFO_STATE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MsaInfoCannotInstall: MSA_INFO_STATE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MsaInfoCanInstall: MSA_INFO_STATE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MsaInfoInstalled: MSA_INFO_STATE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MSGNAME_FORWARDED_FROM: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MSGNAME_FORWARDED_TO: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MSGNAME_NOT_FORWARDED: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSG_INFO_0 {
    pub msgi0_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSG_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSG_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSG_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSG_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSG_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSG_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSG_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSG_INFO_1 {
    pub msgi1_name: super::super::Foundation::PWSTR,
    pub msgi1_forward_flag: u32,
    pub msgi1_forward: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSG_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSG_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MSG_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSG_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSG_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSG_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSG_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const MS_ROUTER_VERSION: u32 = 1536u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn MprSetupProtocolEnum(dwtransportid: u32, lplpbuffer: *mut *mut u8, lpdwentriesread: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprSetupProtocolEnum(dwtransportid: u32, lplpbuffer: *mut *mut u8, lpdwentriesread: *mut u32) -> u32;
        }
        ::core::mem::transmute(MprSetupProtocolEnum(::core::mem::transmute(dwtransportid), ::core::mem::transmute(lplpbuffer), ::core::mem::transmute(lpdwentriesread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn MprSetupProtocolFree(lpbuffer: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprSetupProtocolFree(lpbuffer: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(MprSetupProtocolFree(::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NCPNP_RECONFIG_LAYER = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCRL_NDIS: NCPNP_RECONFIG_LAYER = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCRL_TDI: NCPNP_RECONFIG_LAYER = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NCRP_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCRP_QUERY_PROPERTY_UI: NCRP_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCRP_SHOW_PROPERTY_UI: NCRP_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_AT_Exec_Err: u32 = 3178u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_AT_cannot_read: u32 = 3174u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_AT_cannot_write: u32 = 3129u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_AT_sched_err: u32 = 3175u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_AT_schedule_file_created: u32 = 3176u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Access_File_Bad: u32 = 3122u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Build_Name: u32 = 3170u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Cant_Make_Msg_File: u32 = 3130u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_DiskFT: u32 = 3221u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_DriverNotLoaded: u32 = 5727u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Entries_Lost: u32 = 3114u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Error_in_DLL: u32 = 3256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Exec_Netservr_NoMem: u32 = 3131u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_FT_ErrLog_Too_Large: u32 = 3258u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_FT_Update_In_Progress: u32 = 3259u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_FailedToGetComputerName: u32 = 5726u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_FailedToRegisterSC: u32 = 5724u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_FailedToSetServiceStatus: u32 = 5725u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_File_Changed: u32 = 3253u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Files_Dont_Fit: u32 = 3254u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_HardErr_From_Server: u32 = 3182u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_HotFix: u32 = 3181u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Init_Chardev_Err: u32 = 3124u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Init_Exec_Fail: u32 = 3105u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Init_OpenCreate_Err: u32 = 3110u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Init_Seg_Overflow: u32 = 3120u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Internal_Error: u32 = 3100u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Invalid_Config_File: u32 = 3252u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Invalid_Config_Line: u32 = 3251u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Ioctl_Error: u32 = 3108u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Joined_Domain: u32 = 3260u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Joined_Workgroup: u32 = 3261u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Lazy_Write_Err: u32 = 3180u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_LocalSecFail1: u32 = 3183u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_LocalSecFail2: u32 = 3184u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_LocalSecFail3: u32 = 3185u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_LocalSecGeneralFail: u32 = 3186u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Mail_Slt_Err: u32 = 3173u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Mailslot_err: u32 = 3127u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Message_Send: u32 = 3172u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Missing_Parameter: u32 = 3250u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Msg_Log_Err: u32 = 3150u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Msg_Sem_Shutdown: u32 = 3141u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Msg_Shutdown: u32 = 3140u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Msg_Unexpected_SMB_Type: u32 = 3152u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Name_Expansion: u32 = 3171u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Ncb_Error: u32 = 3106u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Ncb_TooManyErr: u32 = 3126u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetBios: u32 = 3111u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetLogonFailedToInitializeAuthzRm: u32 = 5821u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetLogonFailedToInitializeRPCSD: u32 = 5822u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetWkSta_Internal_Error: u32 = 3190u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetWkSta_NCB_Err: u32 = 3195u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetWkSta_No_Resource: u32 = 3191u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetWkSta_Reset_Err: u32 = 3197u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetWkSta_SMB_Err: u32 = 3192u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetWkSta_Stuck_VC_Err: u32 = 3194u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetWkSta_Too_Many: u32 = 3198u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetWkSta_VC_Err: u32 = 3193u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetWkSta_Write_Behind_Err: u32 = 3196u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Net_Not_Started: u32 = 3107u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonAddNameFailure: u32 = 5741u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonAuthDCFail: u32 = 3210u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonAuthDomainDowngraded: u32 = 5791u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonAuthNoDomainController: u32 = 5719u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonAuthNoTrustLsaSecret: u32 = 5720u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonAuthNoTrustSamAccount: u32 = 5721u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonAuthNoUplevelDomainController: u32 = 5790u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonBadSiteName: u32 = 5779u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonBadSubnetName: u32 = 5780u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonBrowserDriver: u32 = 5740u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonChangeLogCorrupt: u32 = 5705u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDcOldSiteCovered: u32 = 5794u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDcSiteCovered: u32 = 5784u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDcSiteNotCovered: u32 = 5785u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDcSiteNotCoveredAuto: u32 = 5795u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDnsDeregAborted: u32 = 5808u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDnsHostNameLowerCasingFailed: u32 = 5825u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDownLevelLogoffFailed: u32 = 5708u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDownLevelLogonFailed: u32 = 5707u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDuplicateMachineAccounts: u32 = 5738u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDynamicDnsDeregisterFailure: u32 = 5775u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDynamicDnsFailure: u32 = 5782u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDynamicDnsRegisterFailure: u32 = 5774u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonDynamicDnsServerFailure: u32 = 5781u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedAccountDelta: u32 = 5735u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedDnsHostNameUpdate: u32 = 5789u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedDomainDelta: u32 = 5729u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedFileCreate: u32 = 5776u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedGlobalGroupDelta: u32 = 5730u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedLocalGroupDelta: u32 = 5731u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedPolicyDelta: u32 = 5733u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedPrimary: u32 = 3223u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedSecretDelta: u32 = 5736u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedSpnUpdate: u32 = 5788u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedToAddAuthzRpcInterface: u32 = 5820u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedToAddRpcInterface: u32 = 5702u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedToCreateShare: u32 = 5706u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedToReadMailslot: u32 = 5703u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedToRegisterSC: u32 = 5704u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedToUpdateTrustList: u32 = 5701u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedTrustedDomainDelta: u32 = 5734u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFailedUserDelta: u32 = 5732u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFullSyncCallFailed: u32 = 5714u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFullSyncCallSuccess: u32 = 5713u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFullSyncFailed: u32 = 5718u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonFullSyncSuccess: u32 = 5717u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonGcOldSiteCovered: u32 = 5796u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonGcSiteCovered: u32 = 5786u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonGcSiteNotCovered: u32 = 5787u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonGcSiteNotCoveredAuto: u32 = 5797u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonGetSubnetToSite: u32 = 5777u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonInvalidDwordParameterValue: u32 = 5804u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonInvalidGenericParameterValue: u32 = 5803u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonLanmanBdcsNotAllowed: u32 = 5772u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonMachinePasswdSetSucceeded: u32 = 5823u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonMsaPasswdSetSucceeded: u32 = 5824u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonNTLogoffFailed: u32 = 5710u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonNTLogonFailed: u32 = 5709u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonNdncOldSiteCovered: u32 = 5798u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonNdncSiteCovered: u32 = 5792u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonNdncSiteNotCovered: u32 = 5793u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonNdncSiteNotCoveredAuto: u32 = 5799u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonNoAddressToSiteMapping: u32 = 5802u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonNoDynamicDns: u32 = 5773u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonNoDynamicDnsManual: u32 = 5806u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonNoSiteForClient: u32 = 5778u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonNoSiteForClients: u32 = 5807u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonPartialSiteMappingForClients: u32 = 5810u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonPartialSyncCallFailed: u32 = 5712u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonPartialSyncCallSuccess: u32 = 5711u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonPartialSyncFailed: u32 = 5716u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonPartialSyncSuccess: u32 = 5715u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonPasswdSetFailed: u32 = 3224u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonRejectedRemoteDynamicDnsDeregister: u32 = 5814u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonRejectedRemoteDynamicDnsRegister: u32 = 5813u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonRemoteDynamicDnsDeregisterFailure: u32 = 5812u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonRemoteDynamicDnsRegisterFailure: u32 = 5811u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonRemoteDynamicDnsUpdateRequestFailure: u32 = 5815u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonRequireSignOrSealError: u32 = 3227u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonRpcCallCancelled: u32 = 5783u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonRpcPortRequestFailure: u32 = 5809u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonSSIInitError: u32 = 5700u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonServerAuthFailed: u32 = 5722u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonServerAuthFailedNoAccount: u32 = 5805u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonServerAuthNoTrustSamAccount: u32 = 5723u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonSessionTypeWrong: u32 = 5770u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonSpnCrackNamesFailure: u32 = 5801u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonSpnMultipleSamAccountNames: u32 = 5800u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonSyncError: u32 = 3226u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonSystemError: u32 = 5737u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonTooManyGlobalGroups: u32 = 5739u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonTrackingError: u32 = 3225u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonUserValidationReqInitialTimeOut: u32 = 5816u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonUserValidationReqRecurringTimeOut: u32 = 5817u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonUserValidationReqWaitInitialWarning: u32 = 5818u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NetlogonUserValidationReqWaitRecurringWarning: u32 = 5819u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_NoTranportLoaded: u32 = 5728u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_OEM_Code: u32 = 3299u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReleaseMem_Alert: u32 = 3128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Remote_API: u32 = 3125u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplAccessDenied: u32 = 3222u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplBadExport: u32 = 3219u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplBadImport: u32 = 3218u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplBadMsg: u32 = 3215u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplCannotMasterDir: u32 = 3207u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplLogonFailed: u32 = 3211u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplLostMaster: u32 = 3209u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplMaxFiles: u32 = 3213u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplMaxTreeDepth: u32 = 3214u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplNetErr: u32 = 3212u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplSignalFileErr: u32 = 3220u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplSysErr: u32 = 3216u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplUpdateError: u32 = 3208u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplUserCurDir: u32 = 3206u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_ReplUserLoged: u32 = 3217u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Resource_Shortage: u32 = 3101u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplAdapterResource: u32 = 5756u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplBackupDatabase: u32 = 5765u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplCheckConfigs: u32 = 5760u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplCheckSecurity: u32 = 5764u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplCreateProfiles: u32 = 5761u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplFileCopy: u32 = 5757u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplFileDelete: u32 = 5758u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplFilePerms: u32 = 5759u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplInitDatabase: u32 = 5766u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplInitRestoredDatabase: u32 = 5769u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplMessages: u32 = 5742u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplRegistry: u32 = 5762u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplReplaceRPLDISK: u32 = 5763u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplRestoreDatabaseFailure: u32 = 5767u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplRestoreDatabaseSuccess: u32 = 5768u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplSystem: u32 = 5744u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplUpgradeDBTo40: u32 = 5771u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplWkstaBbcFile: u32 = 5751u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplWkstaFileChecksum: u32 = 5749u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplWkstaFileLineCount: u32 = 5750u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplWkstaFileOpen: u32 = 5746u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplWkstaFileRead: u32 = 5747u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplWkstaFileSize: u32 = 5752u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplWkstaInternal: u32 = 5753u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplWkstaMemory: u32 = 5748u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplWkstaNetwork: u32 = 5755u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplWkstaTimeout: u32 = 5745u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplWkstaWrongVersion: u32 = 5754u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_RplXnsBoot: u32 = 5743u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_SMB_Illegal: u32 = 3112u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Server_Lock_Failure: u32 = 3132u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Service_Fail: u32 = 3113u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Srv_Close_Failure: u32 = 3205u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Srv_No_Mem_Grow: u32 = 3121u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Srv_Thread_Failure: u32 = 3204u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Srvnet_NB_Open: u32 = 3177u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Srvnet_Not_Started: u32 = 3123u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_System_Error: u32 = 3257u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_System_Semaphore: u32 = 3109u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_UPS_CannotOpenDriver: u32 = 3233u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_UPS_CmdFileConfig: u32 = 3235u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_UPS_CmdFileError: u32 = 3232u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_UPS_CmdFileExec: u32 = 3236u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_UPS_PowerBack: u32 = 3234u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_UPS_PowerOut: u32 = 3230u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_UPS_Shutdown: u32 = 3231u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Unable_To_Lock_Segment: u32 = 3102u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Unable_To_Unlock_Segment: u32 = 3103u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Uninstall_Service: u32 = 3104u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_VIO_POPUP_ERR: u32 = 3151u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Wksta_Bad_Mailslot_SMB: u32 = 3165u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Wksta_BiosThreadFailure: u32 = 3162u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Wksta_Compname: u32 = 3161u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Wksta_HostTab_Full: u32 = 3164u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Wksta_Infoseg: u32 = 3160u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Wksta_IniSeg: u32 = 3163u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Wksta_SSIRelogon: u32 = 3167u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Wksta_UASInit: u32 = 3166u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NELOG_Wrong_DLL_Version: u32 = 3255u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ACFFileIOFail: u32 = 2229u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ACFNoParent: u32 = 2232u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ACFNoRoom: u32 = 2228u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ACFNotFound: u32 = 2219u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ACFNotLoaded: u32 = 2227u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ACFTooManyLists: u32 = 2230u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_AccountExpired: u32 = 2239u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_AccountLockedOut: u32 = 2702u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_AccountUndefined: u32 = 2238u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_AcctLimitExceeded: u32 = 2434u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ActiveConns: u32 = 2402u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_AddForwarded: u32 = 2275u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_AlertExists: u32 = 2430u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_AlreadyCloudDomainJoined: u32 = 2700u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_AlreadyExists: u32 = 2276u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_AlreadyForwarded: u32 = 2274u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_AlreadyLoggedOn: u32 = 2200u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BASE: u32 = 2100u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadAsgType: u32 = 2251u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadComponent: u32 = 2356u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadControlRecv: u32 = 2193u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadDest: u32 = 2382u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadDev: u32 = 2341u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadDevString: u32 = 2340u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadDomainJoinInfo: u32 = 2712u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadDosFunction: u32 = 2502u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadDosRetCode: u32 = 2500u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadEventName: u32 = 2143u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadFileCheckSum: u32 = 2504u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadOfflineJoinInfo: u32 = 2710u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadPassword: u32 = 2203u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadPasswordCore: u32 = 2403u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadQueueDevString: u32 = 2334u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadQueuePriority: u32 = 2335u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadReceive: u32 = 2282u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadRecipient: u32 = 2433u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadServiceName: u32 = 2185u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadServiceProgName: u32 = 2188u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadSource: u32 = 2381u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadTransactConfig: u32 = 2141u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadUasConfig: u32 = 2450u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BadUsername: u32 = 2202u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BrowserConfiguredToNotRun: u32 = 2550u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BrowserNotStarted: u32 = 2139u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BrowserTableIncomplete: u32 = 2319u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_BufTooSmall: u32 = 2123u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CallingRplSrvr: u32 = 2515u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CanNotGrowSegment: u32 = 2233u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CanNotGrowUASFile: u32 = 2456u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CannotUnjoinAadDomain: u32 = 2727u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CantConnectRplSrvr: u32 = 2513u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CantCreateJoinInfo: u32 = 2711u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CantLoadOfflineHive: u32 = 2717u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CantOpenImageFile: u32 = 2514u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CantType: u32 = 2357u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CantVerifyHostname: u32 = 2716u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CfgCompNotFound: u32 = 2146u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CfgParamNotFound: u32 = 2147u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ClientNameNotFound: u32 = 2312u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_CommDevInUse: u32 = 2343u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ComputerAccountNotFound: u32 = 2697u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ConnectionInsecure: u32 = 2718u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DCNotFound: u32 = 2453u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DS8DCNotFound: u32 = 2722u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DS8DCRequired: u32 = 2720u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DS9DCNotFound: u32 = 2725u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DataTypeInvalid: u32 = 2167u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DatabaseUpToDate: u32 = 2248u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DefaultJoinRequired: u32 = 2694u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DelComputerName: u32 = 2278u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DeleteLater: u32 = 2298u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DestExists: u32 = 2153u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DestIdle: u32 = 2158u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DestInvalidOp: u32 = 2159u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DestInvalidState: u32 = 2162u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DestNoRoom: u32 = 2157u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DestNotFound: u32 = 2152u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DevInUse: u32 = 2404u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DevInvalidOpCode: u32 = 2331u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DevNotFound: u32 = 2332u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DevNotOpen: u32 = 2333u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DevNotRedirected: u32 = 2107u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DeviceIsShared: u32 = 2252u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DeviceNotShared: u32 = 2311u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DeviceShareConflict: u32 = 2318u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsAlreadyShared: u32 = 2664u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsBadRenamePath: u32 = 2671u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsCantCreateJunctionPoint: u32 = 2669u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsCantRemoveDfsRoot: u32 = 2682u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsCantRemoveLastServerShare: u32 = 2677u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsChildOrParentInDfs: u32 = 2683u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsCyclicalName: u32 = 2674u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsDataIsIdentical: u32 = 2681u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsDuplicateService: u32 = 2676u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsInconsistent: u32 = 2679u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsInternalCorruption: u32 = 2660u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsInternalError: u32 = 2690u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsLeafVolume: u32 = 2667u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsNoSuchServer: u32 = 2673u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsNoSuchShare: u32 = 2665u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsNoSuchVolume: u32 = 2662u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsNotALeafVolume: u32 = 2666u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsNotSupportedInServerDfs: u32 = 2675u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsServerNotDfsAware: u32 = 2670u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsServerUpgraded: u32 = 2680u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsVolumeAlreadyExists: u32 = 2663u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsVolumeDataCorrupt: u32 = 2661u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsVolumeHasMultipleServers: u32 = 2668u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsVolumeIsInterDfs: u32 = 2678u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DfsVolumeIsOffline: u32 = 2672u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DifferentServers: u32 = 2383u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DriverNotFound: u32 = 2166u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DupNameReboot: u32 = 2144u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DuplicateName: u32 = 2297u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_DuplicateShare: u32 = 2118u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ErrCommRunSrv: u32 = 2389u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ErrorExecingGhost: u32 = 2391u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ExecFailure: u32 = 2315u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_FileIdNotFound: u32 = 2314u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_GroupExists: u32 = 2223u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_GroupNotFound: u32 = 2220u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_GrpMsgProcessor: u32 = 2280u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ImageParamErr: u32 = 2508u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InUseBySpooler: u32 = 2342u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_IncompleteDel: u32 = 2299u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InternalError: u32 = 2140u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidAPI: u32 = 2142u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidComputer: u32 = 2351u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidDatabase: u32 = 2247u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidDevice: u32 = 2294u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidLana: u32 = 2400u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidLogSeek: u32 = 2440u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidLogonHours: u32 = 2241u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidMachineNameForJoin: u32 = 2724u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidMaxUsers: u32 = 2122u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidUASOp: u32 = 2451u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidWorkgroupName: u32 = 2695u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_InvalidWorkstation: u32 = 2240u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_IsDfsShare: u32 = 2321u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ItemNotFound: u32 = 2115u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_JobInvalidState: u32 = 2164u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_JobNoRoom: u32 = 2156u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_JobNotFound: u32 = 2151u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_JoinPerformedMustRestart: u32 = 2713u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LDAPCapableDCRequired: u32 = 2721u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LanmanIniError: u32 = 2131u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LastAdmin: u32 = 2452u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LineTooLong: u32 = 2149u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LocalDrive: u32 = 2405u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LocalForward: u32 = 2279u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LogFileChanged: u32 = 2378u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LogFileCorrupt: u32 = 2379u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LogOverflow: u32 = 2377u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LogonDomainExists: u32 = 2216u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LogonNoUserPath: u32 = 2211u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LogonScriptError: u32 = 2212u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LogonServerConflict: u32 = 2210u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LogonServerNotFound: u32 = 2215u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LogonTrackingError: u32 = 2454u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_LogonsPaused: u32 = 2209u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_MaxLenExceeded: u32 = 2354u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_MsgAlreadyStarted: u32 = 2271u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_MsgInitFailed: u32 = 2272u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_MsgNotStarted: u32 = 2284u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_MultipleNets: u32 = 2300u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NameInUse: u32 = 2283u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NameNotForwarded: u32 = 2288u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NameNotFound: u32 = 2273u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NameUsesIncompatibleCodePage: u32 = 2696u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NetNameNotFound: u32 = 2310u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NetNotStarted: u32 = 2102u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NetlogonNotStarted: u32 = 2455u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NetworkError: u32 = 2136u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoAlternateServers: u32 = 2467u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoCommDevs: u32 = 2337u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoComputerName: u32 = 2270u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoForwardName: u32 = 2286u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoJoinPending: u32 = 2714u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoNetworkResource: u32 = 2105u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoOfflineJoinInfo: u32 = 2709u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoRoom: u32 = 2119u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoRplBootSystem: u32 = 2505u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoSuchAlert: u32 = 2432u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoSuchConnection: u32 = 2462u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoSuchServer: u32 = 2460u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NoSuchSession: u32 = 2461u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NonDosFloppyUsed: u32 = 2510u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NonValidatedLogon: u32 = 2217u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NotInCache: u32 = 2235u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NotInDispatchTbl: u32 = 2192u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NotLocalDomain: u32 = 2320u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NotLocalName: u32 = 2285u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NotLoggedOn: u32 = 2201u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_NotPrimary: u32 = 2226u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_OpenFiles: u32 = 2401u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PasswordCantChange: u32 = 2243u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PasswordExpired: u32 = 2242u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PasswordFilterError: u32 = 2705u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PasswordHistConflict: u32 = 2244u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PasswordMismatch: u32 = 2458u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PasswordMustChange: u32 = 2701u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PasswordNotComplexEnough: u32 = 2704u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PasswordTooLong: u32 = 2703u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PasswordTooRecent: u32 = 2246u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PasswordTooShort: u32 = 2245u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PausedRemote: u32 = 2281u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PersonalSku: u32 = 2698u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_PlainTextSecretsRequired: u32 = 2726u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ProcNoRespond: u32 = 2160u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ProcNotFound: u32 = 2168u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ProfileCleanup: u32 = 2372u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ProfileFileTooBig: u32 = 2370u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ProfileLoadErr: u32 = 2374u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ProfileOffset: u32 = 2371u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ProfileSaveErr: u32 = 2375u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ProfileUnknownCmd: u32 = 2373u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ProgNeedsExtraMem: u32 = 2501u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ProvisioningBlobUnsupported: u32 = 2719u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_QExists: u32 = 2154u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_QInvalidState: u32 = 2163u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_QNoRoom: u32 = 2155u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_QNotFound: u32 = 2150u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_QueueNotFound: u32 = 2338u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RPL_CONNECTED: u32 = 2519u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RedirectedPath: u32 = 2117u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RemoteBootFailed: u32 = 2503u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RemoteErr: u32 = 2127u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RemoteFull: u32 = 2287u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RemoteOnly: u32 = 2106u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ResourceExists: u32 = 2225u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ResourceNotFound: u32 = 2222u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplAdapterInfoCorrupted: u32 = 2625u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplAdapterNameUnavailable: u32 = 2633u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplAdapterNotFound: u32 = 2637u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplBackupDatabase: u32 = 2636u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplBadDatabase: u32 = 2612u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplBadRegistry: u32 = 2611u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplBootInUse: u32 = 2635u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplBootInfoCorrupted: u32 = 2628u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplBootNameUnavailable: u32 = 2640u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplBootNotFound: u32 = 2631u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplBootRestart: u32 = 2511u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplBootServiceTerm: u32 = 2517u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplBootStartFailed: u32 = 2518u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplCannotEnum: u32 = 2615u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplConfigInfoCorrupted: u32 = 2623u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplConfigNameUnavailable: u32 = 2641u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplConfigNotEmpty: u32 = 2634u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplConfigNotFound: u32 = 2624u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplIncompatibleProfile: u32 = 2632u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplInternal: u32 = 2626u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplLoadrDiskErr: u32 = 2507u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplLoadrNetBiosErr: u32 = 2506u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplNeedsRPLUSERAcct: u32 = 2630u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplNoAdaptersStarted: u32 = 2610u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplNotRplServer: u32 = 2614u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplProfileInfoCorrupted: u32 = 2619u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplProfileNameUnavailable: u32 = 2621u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplProfileNotEmpty: u32 = 2622u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplProfileNotFound: u32 = 2620u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplRplfilesShare: u32 = 2613u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplSrvrCallFailed: u32 = 2512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplVendorInfoCorrupted: u32 = 2627u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplVendorNameUnavailable: u32 = 2639u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplVendorNotFound: u32 = 2638u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplWkstaInfoCorrupted: u32 = 2616u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplWkstaNameUnavailable: u32 = 2618u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplWkstaNeedsUserAcct: u32 = 2629u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RplWkstaNotFound: u32 = 2617u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_RunSrvPaused: u32 = 2385u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_SameAsComputerName: u32 = 2253u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServerNotStarted: u32 = 2114u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServiceCtlBusy: u32 = 2187u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServiceCtlNotValid: u32 = 2191u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServiceCtlTimeout: u32 = 2186u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServiceEntryLocked: u32 = 2183u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServiceInstalled: u32 = 2182u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServiceKillProc: u32 = 2190u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServiceNotCtrl: u32 = 2189u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServiceNotInstalled: u32 = 2184u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServiceNotStarting: u32 = 2194u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServiceTableFull: u32 = 2181u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ServiceTableLocked: u32 = 2180u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_SetupAlreadyJoined: u32 = 2691u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_SetupCheckDNSConfig: u32 = 2699u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_SetupDomainController: u32 = 2693u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_SetupNotJoined: u32 = 2692u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ShareMem: u32 = 2104u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ShareNotFound: u32 = 2392u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_SourceIsDir: u32 = 2380u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_SpeGroupOp: u32 = 2234u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_SpoolNoMemory: u32 = 2165u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_SpoolerNotLoaded: u32 = 2161u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_StandaloneLogon: u32 = 2214u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_StartingRplBoot: u32 = 2516u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_Success: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_SyncRequired: u32 = 2249u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TargetVersionUnsupported: u32 = 2723u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TimeDiffAtDC: u32 = 2457u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TmpFile: u32 = 2316u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TooManyAlerts: u32 = 2431u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TooManyConnections: u32 = 2465u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TooManyEntries: u32 = 2362u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TooManyFiles: u32 = 2466u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TooManyImageParams: u32 = 2509u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TooManyItems: u32 = 2121u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TooManyNames: u32 = 2277u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TooManyServers: u32 = 2463u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TooManySessions: u32 = 2464u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TooMuchData: u32 = 2317u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TruncatedBroadcast: u32 = 2289u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_TryDownLevel: u32 = 2470u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UPSDriverNotStarted: u32 = 2480u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UPSInvalidCommPort: u32 = 2482u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UPSInvalidConfig: u32 = 2481u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UPSShutdownFailed: u32 = 2484u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UPSSignalAsserted: u32 = 2483u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UnableToAddName_F: u32 = 2205u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UnableToAddName_W: u32 = 2204u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UnableToDelName_F: u32 = 2207u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UnableToDelName_W: u32 = 2206u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UnknownDevDir: u32 = 2116u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UnknownServer: u32 = 2103u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UseNotFound: u32 = 2250u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UserExists: u32 = 2224u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UserInGroup: u32 = 2236u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UserLogon: u32 = 2231u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UserNotFound: u32 = 2221u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_UserNotInGroup: u32 = 2237u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_ValuesNotSet: u32 = 2715u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_WkstaInconsistentState: u32 = 2137u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_WkstaNotStarted: u32 = 2138u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NERR_WriteFault: u32 = 2295u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETBIOS_NAME_LEN: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_E_ACTIVE_RAS_CONNECTIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180506i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_E_ADAPTER_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180505i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_E_ALREADY_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180512i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_E_COMPONENT_REMOVED_PENDING_REBOOT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180504i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_E_DUPLICATE_INSTANCEID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180501i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_E_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180510i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_E_MAX_FILTER_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180503i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_E_NEED_REBOOT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180507i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_E_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180511i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_E_NO_WRITE_LOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180508i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_E_VMSWITCH_ACTIVE_OVER_ADAPTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180502i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_S_CAUSED_SETUP_CHANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(303140i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_S_COMMIT_NOW: ::windows::core::HRESULT = ::windows::core::HRESULT(303141i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_S_DISABLE_QUERY: ::windows::core::HRESULT = ::windows::core::HRESULT(303138i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_S_REBOOT: ::windows::core::HRESULT = ::windows::core::HRESULT(303136i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETCFG_S_STILL_REFERENCED: ::windows::core::HRESULT = ::windows::core::HRESULT(303139i32);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_BACKUP_CHANGE_LOG: u32 = 65532u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_BREAKPOINT: u32 = 65535u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_CHANGE_PASSWORD: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_FIND_USER: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_FORCE_DNS_REG: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_PDC_REPLICATE: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_QUERY: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_QUERY_DNS_REG: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_QUERY_ENC_TYPES: u32 = 13u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_REDISCOVER: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_REPLICATE: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_SET_DBFLAG: u32 = 65534u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_SYNCHRONIZE: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_TC_QUERY: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_TC_VERIFY: u32 = 10u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_TRANSPORT_NOTIFY: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_TRUNCATE_LOG: u32 = 65533u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_CONTROL_UNLOAD_NETLOGON_DLL: u32 = 65531u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_DNS_UPDATE_FAILURE: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_FULL_SYNC_REPLICATION: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_HAS_IP: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_HAS_TIMESERV: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct NETLOGON_INFO_1 {
    pub netlog1_flags: u32,
    pub netlog1_pdc_connection_status: u32,
}
impl ::core::marker::Copy for NETLOGON_INFO_1 {}
impl ::core::clone::Clone for NETLOGON_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NETLOGON_INFO_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NETLOGON_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETLOGON_INFO_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for NETLOGON_INFO_1 {}
impl ::core::default::Default for NETLOGON_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NETLOGON_INFO_2 {
    pub netlog2_flags: u32,
    pub netlog2_pdc_connection_status: u32,
    pub netlog2_trusted_dc_name: super::super::Foundation::PWSTR,
    pub netlog2_tc_connection_status: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETLOGON_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETLOGON_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NETLOGON_INFO_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NETLOGON_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETLOGON_INFO_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NETLOGON_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NETLOGON_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct NETLOGON_INFO_3 {
    pub netlog3_flags: u32,
    pub netlog3_logon_attempts: u32,
    pub netlog3_reserved1: u32,
    pub netlog3_reserved2: u32,
    pub netlog3_reserved3: u32,
    pub netlog3_reserved4: u32,
    pub netlog3_reserved5: u32,
}
impl ::core::marker::Copy for NETLOGON_INFO_3 {}
impl ::core::clone::Clone for NETLOGON_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NETLOGON_INFO_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NETLOGON_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETLOGON_INFO_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for NETLOGON_INFO_3 {}
impl ::core::default::Default for NETLOGON_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NETLOGON_INFO_4 {
    pub netlog4_trusted_dc_name: super::super::Foundation::PWSTR,
    pub netlog4_trusted_domain_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETLOGON_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETLOGON_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NETLOGON_INFO_4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NETLOGON_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETLOGON_INFO_4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NETLOGON_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NETLOGON_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_REDO_NEEDED: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_REPLICATION_IN_PROGRESS: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_REPLICATION_NEEDED: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOGON_VERIFY_STATUS_RETURNED: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOG_NetlogonNonWindowsSupportsSecureRpc: u32 = 5826u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOG_NetlogonUnsecureRpcClient: u32 = 5827u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOG_NetlogonUnsecureRpcMachineAllowedBySsdl: u32 = 5830u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOG_NetlogonUnsecureRpcTrust: u32 = 5828u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOG_NetlogonUnsecureRpcTrustAllowedBySsdl: u32 = 5831u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETLOG_NetlogonUnsecuredRpcMachineTemporarilyAllowed: u32 = 5829u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETMAN_VARTYPE_HARDWARE_ADDRESS: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETMAN_VARTYPE_STRING: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETMAN_VARTYPE_ULONG: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_ACCT_DELETE: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_ALT_SAMACCOUNTNAME: u32 = 131072u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_DNS_NAME_CHANGES_ONLY: u32 = 4096u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_INSTALL_INVOCATION: u32 = 262144u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NETSETUP_JOIN_STATUS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetSetupUnknownStatus: NETSETUP_JOIN_STATUS = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetSetupUnjoined: NETSETUP_JOIN_STATUS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetSetupWorkgroupName: NETSETUP_JOIN_STATUS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetSetupDomainName: NETSETUP_JOIN_STATUS = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NETSETUP_NAME_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetSetupUnknown: NETSETUP_NAME_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetSetupMachine: NETSETUP_NAME_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetSetupWorkgroup: NETSETUP_NAME_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetSetupDomain: NETSETUP_NAME_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetSetupNonExistentDomain: NETSETUP_NAME_TYPE = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetSetupDnsMachine: NETSETUP_NAME_TYPE = 5i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NETSETUP_PROVISION = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_PROVISION_DOWNLEVEL_PRIV_SUPPORT: NETSETUP_PROVISION = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_PROVISION_REUSE_ACCOUNT: NETSETUP_PROVISION = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_PROVISION_USE_DEFAULT_PASSWORD: NETSETUP_PROVISION = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_PROVISION_SKIP_ACCOUNT_SEARCH: NETSETUP_PROVISION = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_PROVISION_ROOT_CA_CERTS: NETSETUP_PROVISION = 16u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NETSETUP_PROVISIONING_PARAMS {
    pub dwVersion: u32,
    pub lpDomain: super::super::Foundation::PWSTR,
    pub lpHostName: super::super::Foundation::PWSTR,
    pub lpMachineAccountOU: super::super::Foundation::PWSTR,
    pub lpDcName: super::super::Foundation::PWSTR,
    pub dwProvisionOptions: NETSETUP_PROVISION,
    pub aCertTemplateNames: *mut super::super::Foundation::PWSTR,
    pub cCertTemplateNames: u32,
    pub aMachinePolicyNames: *mut super::super::Foundation::PWSTR,
    pub cMachinePolicyNames: u32,
    pub aMachinePolicyPaths: *mut super::super::Foundation::PWSTR,
    pub cMachinePolicyPaths: u32,
    pub lpNetbiosName: super::super::Foundation::PWSTR,
    pub lpSiteName: super::super::Foundation::PWSTR,
    pub lpPrimaryDNSDomain: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETSETUP_PROVISIONING_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETSETUP_PROVISIONING_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NETSETUP_PROVISIONING_PARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NETSETUP_PROVISIONING_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETSETUP_PROVISIONING_PARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NETSETUP_PROVISIONING_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NETSETUP_PROVISIONING_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_PROVISIONING_PARAMS_CURRENT_VERSION: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_PROVISIONING_PARAMS_WIN8_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_PROVISION_CHECK_PWD_ONLY: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_PROVISION_PERSISTENTSITE: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NETWORK_INSTALL_TIME = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NSF_PRIMARYINSTALL: NETWORK_INSTALL_TIME = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NSF_POSTSYSINSTALL: NETWORK_INSTALL_TIME = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NETWORK_NAME {
    pub Name: FLAT_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETWORK_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETWORK_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NETWORK_NAME {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NETWORK_NAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETWORK_NAME>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NETWORK_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NETWORK_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NETWORK_UPGRADE_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NSF_WIN16_UPGRADE: NETWORK_UPGRADE_TYPE = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NSF_WIN95_UPGRADE: NETWORK_UPGRADE_TYPE = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NSF_WINNT_WKS_UPGRADE: NETWORK_UPGRADE_TYPE = 64i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NSF_WINNT_SVR_UPGRADE: NETWORK_UPGRADE_TYPE = 128i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NSF_WINNT_SBS_UPGRADE: NETWORK_UPGRADE_TYPE = 256i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NSF_COMPONENT_UPDATE: NETWORK_UPGRADE_TYPE = 512i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NET_COMPUTER_NAME_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetPrimaryComputerName: NET_COMPUTER_NAME_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetAlternateComputerNames: NET_COMPUTER_NAME_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetAllComputerNames: NET_COMPUTER_NAME_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetComputerNameTypeMax: NET_COMPUTER_NAME_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NET_DFS_ENUM: i32 = 1073756324i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NET_DFS_ENUMEX: i32 = 1073756325i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_DISPLAY_GROUP {
    pub grpi3_name: super::super::Foundation::PWSTR,
    pub grpi3_comment: super::super::Foundation::PWSTR,
    pub grpi3_group_id: u32,
    pub grpi3_attributes: u32,
    pub grpi3_next_index: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_DISPLAY_GROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_DISPLAY_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NET_DISPLAY_GROUP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_DISPLAY_GROUP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_DISPLAY_GROUP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_DISPLAY_GROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_DISPLAY_GROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_DISPLAY_MACHINE {
    pub usri2_name: super::super::Foundation::PWSTR,
    pub usri2_comment: super::super::Foundation::PWSTR,
    pub usri2_flags: USER_ACCOUNT_FLAGS,
    pub usri2_user_id: u32,
    pub usri2_next_index: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_DISPLAY_MACHINE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_DISPLAY_MACHINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NET_DISPLAY_MACHINE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_DISPLAY_MACHINE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_DISPLAY_MACHINE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_DISPLAY_MACHINE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_DISPLAY_MACHINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_DISPLAY_USER {
    pub usri1_name: super::super::Foundation::PWSTR,
    pub usri1_comment: super::super::Foundation::PWSTR,
    pub usri1_flags: USER_ACCOUNT_FLAGS,
    pub usri1_full_name: super::super::Foundation::PWSTR,
    pub usri1_user_id: u32,
    pub usri1_next_index: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_DISPLAY_USER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_DISPLAY_USER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NET_DISPLAY_USER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_DISPLAY_USER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_DISPLAY_USER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_DISPLAY_USER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_DISPLAY_USER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NET_IGNORE_UNSUPPORTED_FLAGS: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NET_JOIN_DOMAIN_JOIN_OPTIONS = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_JOIN_DOMAIN: NET_JOIN_DOMAIN_JOIN_OPTIONS = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_ACCT_CREATE: NET_JOIN_DOMAIN_JOIN_OPTIONS = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_WIN9X_UPGRADE: NET_JOIN_DOMAIN_JOIN_OPTIONS = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_DOMAIN_JOIN_IF_JOINED: NET_JOIN_DOMAIN_JOIN_OPTIONS = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_JOIN_UNSECURE: NET_JOIN_DOMAIN_JOIN_OPTIONS = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_MACHINE_PWD_PASSED: NET_JOIN_DOMAIN_JOIN_OPTIONS = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_DEFER_SPN_SET: NET_JOIN_DOMAIN_JOIN_OPTIONS = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_JOIN_DC_ACCOUNT: NET_JOIN_DOMAIN_JOIN_OPTIONS = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_JOIN_WITH_NEW_NAME: NET_JOIN_DOMAIN_JOIN_OPTIONS = 1024u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_JOIN_READONLY: NET_JOIN_DOMAIN_JOIN_OPTIONS = 2048u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_AMBIGUOUS_DC: NET_JOIN_DOMAIN_JOIN_OPTIONS = 4096u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_NO_NETLOGON_CACHE: NET_JOIN_DOMAIN_JOIN_OPTIONS = 8192u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_DONT_CONTROL_SERVICES: NET_JOIN_DOMAIN_JOIN_OPTIONS = 16384u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_SET_MACHINE_NAME: NET_JOIN_DOMAIN_JOIN_OPTIONS = 32768u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_FORCE_SPN_SET: NET_JOIN_DOMAIN_JOIN_OPTIONS = 65536u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_NO_ACCT_REUSE: NET_JOIN_DOMAIN_JOIN_OPTIONS = 131072u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_IGNORE_UNSUPPORTED_FLAGS: NET_JOIN_DOMAIN_JOIN_OPTIONS = 268435456u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SUPPORTS_REMOTE_ADMIN_PROTOCOL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SUPPORTS_RPC: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SUPPORTS_SAM_PROTOCOL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SUPPORTS_UNICODE: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SUPPORTS_LOCAL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NET_REQUEST_PROVISION_OPTIONS = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NETSETUP_PROVISION_ONLINE_CALLER: NET_REQUEST_PROVISION_OPTIONS = 1073741824u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NET_SERVER_TYPE = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_WORKSTATION: NET_SERVER_TYPE = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_SERVER: NET_SERVER_TYPE = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_SQLSERVER: NET_SERVER_TYPE = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_DOMAIN_CTRL: NET_SERVER_TYPE = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_DOMAIN_BAKCTRL: NET_SERVER_TYPE = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_TIME_SOURCE: NET_SERVER_TYPE = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_AFP: NET_SERVER_TYPE = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_NOVELL: NET_SERVER_TYPE = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_DOMAIN_MEMBER: NET_SERVER_TYPE = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_PRINTQ_SERVER: NET_SERVER_TYPE = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_DIALIN_SERVER: NET_SERVER_TYPE = 1024u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_XENIX_SERVER: NET_SERVER_TYPE = 2048u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_SERVER_UNIX: NET_SERVER_TYPE = 2048u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_NT: NET_SERVER_TYPE = 4096u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_WFW: NET_SERVER_TYPE = 8192u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_SERVER_MFPN: NET_SERVER_TYPE = 16384u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_SERVER_NT: NET_SERVER_TYPE = 32768u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_POTENTIAL_BROWSER: NET_SERVER_TYPE = 65536u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_BACKUP_BROWSER: NET_SERVER_TYPE = 131072u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_MASTER_BROWSER: NET_SERVER_TYPE = 262144u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_DOMAIN_MASTER: NET_SERVER_TYPE = 524288u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_SERVER_OSF: NET_SERVER_TYPE = 1048576u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_SERVER_VMS: NET_SERVER_TYPE = 2097152u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_WINDOWS: NET_SERVER_TYPE = 4194304u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_DFS: NET_SERVER_TYPE = 8388608u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_CLUSTER_NT: NET_SERVER_TYPE = 16777216u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_TERMINALSERVER: NET_SERVER_TYPE = 33554432u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_CLUSTER_VS_NT: NET_SERVER_TYPE = 67108864u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_DCE: NET_SERVER_TYPE = 268435456u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_ALTERNATE_XPORT: NET_SERVER_TYPE = 536870912u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_LOCAL_LIST_ONLY: NET_SERVER_TYPE = 1073741824u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_DOMAIN_ENUM: NET_SERVER_TYPE = 2147483648u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_ALL: NET_SERVER_TYPE = 4294967295u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NET_USER_ENUM_FILTER_FLAGS = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const FILTER_TEMP_DUPLICATE_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const FILTER_NORMAL_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const FILTER_INTERDOMAIN_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const FILTER_WORKSTATION_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const FILTER_SERVER_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = 32u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub PasswordMatched: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_VALIDATE_AUTHENTICATION_INPUT_ARG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NET_VALIDATE_BAD_PASSWORD_COUNT: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NET_VALIDATE_BAD_PASSWORD_TIME: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NET_VALIDATE_LOCKOUT_TIME: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_VALIDATE_OUTPUT_ARG {
    pub ChangedPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ValidationStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_VALIDATE_OUTPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_VALIDATE_OUTPUT_ARG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NET_VALIDATE_OUTPUT_ARG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_OUTPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_VALIDATE_OUTPUT_ARG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_OUTPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_OUTPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: super::super::Foundation::PWSTR,
    pub UserAccountName: super::super::Foundation::PWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMatch: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct NET_VALIDATE_PASSWORD_HASH {
    pub Length: u32,
    pub Hash: *mut u8,
}
impl ::core::marker::Copy for NET_VALIDATE_PASSWORD_HASH {}
impl ::core::clone::Clone for NET_VALIDATE_PASSWORD_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NET_VALIDATE_PASSWORD_HASH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NET_VALIDATE_PASSWORD_HASH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_VALIDATE_PASSWORD_HASH>()) == 0 }
    }
}
impl ::core::cmp::Eq for NET_VALIDATE_PASSWORD_HASH {}
impl ::core::default::Default for NET_VALIDATE_PASSWORD_HASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NET_VALIDATE_PASSWORD_HISTORY: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NET_VALIDATE_PASSWORD_HISTORY_LENGTH: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NET_VALIDATE_PASSWORD_LAST_SET: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: super::super::Foundation::PWSTR,
    pub UserAccountName: super::super::Foundation::PWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMustChangeAtNextLogon: super::super::Foundation::BOOLEAN,
    pub ClearLockout: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_VALIDATE_PASSWORD_RESET_INPUT_ARG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type NET_VALIDATE_PASSWORD_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetValidateAuthentication: NET_VALIDATE_PASSWORD_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetValidatePasswordChange: NET_VALIDATE_PASSWORD_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NetValidatePasswordReset: NET_VALIDATE_PASSWORD_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_VALIDATE_PERSISTED_FIELDS {
    pub PresentFields: u32,
    pub PasswordLastSet: super::super::Foundation::FILETIME,
    pub BadPasswordTime: super::super::Foundation::FILETIME,
    pub LockoutTime: super::super::Foundation::FILETIME,
    pub BadPasswordCount: u32,
    pub PasswordHistoryLength: u32,
    pub PasswordHistory: *mut NET_VALIDATE_PASSWORD_HASH,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_VALIDATE_PERSISTED_FIELDS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_VALIDATE_PERSISTED_FIELDS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NET_VALIDATE_PERSISTED_FIELDS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_PERSISTED_FIELDS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_VALIDATE_PERSISTED_FIELDS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_PERSISTED_FIELDS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_PERSISTED_FIELDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NON_VALIDATED_LOGON: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NOT_A_DFS_PATH: i32 = 1073756224i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NO_PERMISSION_REQUIRED: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_COLLECT_RPC_BINDING_ERROR_CONN: i32 = -1073728292i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_COLLECT_RPC_BINDING_ERROR_SET: i32 = -1073728293i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_COLLECT_RPC_CALL_ERROR_CONN: i32 = -1073728290i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_COLLECT_RPC_CALL_ERROR_SET: i32 = -1073728291i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_OPEN_RPC_BINDING_ERROR_CONN: i32 = -1073728296i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_OPEN_RPC_BINDING_ERROR_SET: i32 = -1073728297i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_OPEN_RPC_CALL_ERROR_CONN: i32 = -1073728294i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_OPEN_RPC_CALL_ERROR_SET: i32 = -1073728295i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_REGISTRY_ERROR_CONN: i32 = -1073728286i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_REGISTRY_ERROR_SET: i32 = -1073728287i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_VIRTUALALLOC_ERROR_CONN: i32 = -1073728288i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NTFRSPRF_VIRTUALALLOC_ERROR_SET: i32 = -1073728289i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_BADWANFILTER_VALUE: i32 = -1073733302i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_BIND_FAILED: i32 = -1073733320i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_CARDLISTEVENT_FAIL: i32 = -1073733301i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_CARDMALLOC_FAILED: i32 = -1073733316i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_CREATELPCEVENT_ERROR: i32 = -1073733305i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_CREATELPCPORT_ERROR: i32 = -1073733306i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_GETSOCKNAME_FAILED: i32 = -1073733319i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_HASHTABLE_MALLOC_FAILED: i32 = -1073733308i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_INVALID_FILTERNAME: i32 = -2147475123i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_KEY_NOT_FOUND: i32 = -1073733324i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_LPCHANDLEMEMORY_ERROR: i32 = -1073733303i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_LPCLISTENMEMORY_ERROR: i32 = -1073733304i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_NOCARDS: i32 = -1073733315i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_OPTBCASTINADDR_FAILED: i32 = -1073733317i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_OPTEXTENDEDADDR_FAILED: i32 = -1073733318i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_OPTMAXADAPTERNUM_ERROR: i32 = -1073733293i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_RECVSEM_FAIL: i32 = -1073733313i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_SDMDEVENT_FAIL: i32 = -1073733300i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_SENDEVENT_FAIL: i32 = -1073733312i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_SETOPTBCAST_FAILED: i32 = -1073733321i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_SOCKET_FAILED: i32 = -1073733322i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_STARTLPCWORKER_ERROR: i32 = -1073733307i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_STARTRECEIVE_ERROR: i32 = -1073733311i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_STARTWANCHECK_ERROR: i32 = -1073733294i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_STARTWANWORKER_ERROR: i32 = -1073733295i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_STARTWORKER_ERROR: i32 = -1073733310i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_TABLE_MALLOC_FAILED: i32 = -1073733309i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_THREADEVENT_FAIL: i32 = -1073733314i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_WANBIND_FAILED: i32 = -1073733296i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_WANEVENT_ERROR: i32 = -1073733291i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_WANHANDLEMEMORY_ERROR: i32 = -1073733292i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_WANSEM_FAIL: i32 = -1073733298i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_WANSOCKET_FAILED: i32 = -1073733297i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NWSAP_EVENT_WSASTARTUP_FAILED: i32 = -1073733323i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAccessAdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAccessAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetAccessAdd(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAccessDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, resource: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAccessDel(servername: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetAccessDel(servername.into_param().abi(), resource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAccessEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, basepath: Param1, recursive: u32, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAccessEnum(servername: super::super::Foundation::PWSTR, basepath: super::super::Foundation::PWSTR, recursive: u32, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetAccessEnum(servername.into_param().abi(), basepath.into_param().abi(), ::core::mem::transmute(recursive), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resume_handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAccessGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, resource: Param1, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAccessGetInfo(servername: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetAccessGetInfo(servername.into_param().abi(), resource.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAccessGetUserPerms<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, ugname: Param1, resource: Param2, perms: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAccessGetUserPerms(servername: super::super::Foundation::PWSTR, ugname: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR, perms: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetAccessGetUserPerms(servername.into_param().abi(), ugname.into_param().abi(), resource.into_param().abi(), ::core::mem::transmute(perms)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAccessSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, resource: Param1, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAccessSetInfo(servername: super::super::Foundation::PWSTR, resource: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetAccessSetInfo(servername.into_param().abi(), resource.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAddAlternateComputerName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(server: Param0, alternatename: Param1, domainaccount: Param2, domainaccountpassword: Param3, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAddAlternateComputerName(server: super::super::Foundation::PWSTR, alternatename: super::super::Foundation::PWSTR, domainaccount: super::super::Foundation::PWSTR, domainaccountpassword: super::super::Foundation::PWSTR, reserved: u32) -> u32;
        }
        ::core::mem::transmute(NetAddAlternateComputerName(server.into_param().abi(), alternatename.into_param().abi(), domainaccount.into_param().abi(), domainaccountpassword.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAddServiceAccount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, accountname: Param1, password: Param2, flags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAddServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::NTSTATUS;
        }
        NetAddServiceAccount(servername.into_param().abi(), accountname.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAlertRaise<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(alerttype: Param0, buffer: *const ::core::ffi::c_void, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAlertRaise(alerttype: super::super::Foundation::PWSTR, buffer: *const ::core::ffi::c_void, buffersize: u32) -> u32;
        }
        ::core::mem::transmute(NetAlertRaise(alerttype.into_param().abi(), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAlertRaiseEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(alerttype: Param0, variableinfo: *const ::core::ffi::c_void, variableinfosize: u32, servicename: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAlertRaiseEx(alerttype: super::super::Foundation::PWSTR, variableinfo: *const ::core::ffi::c_void, variableinfosize: u32, servicename: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetAlertRaiseEx(alerttype.into_param().abi(), ::core::mem::transmute(variableinfo), ::core::mem::transmute(variableinfosize), servicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn NetApiBufferAllocate(bytecount: u32, buffer: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetApiBufferAllocate(bytecount: u32, buffer: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(NetApiBufferAllocate(::core::mem::transmute(bytecount), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn NetApiBufferFree(buffer: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetApiBufferFree(buffer: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(NetApiBufferFree(::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn NetApiBufferReallocate(oldbuffer: *const ::core::ffi::c_void, newbytecount: u32, newbuffer: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetApiBufferReallocate(oldbuffer: *const ::core::ffi::c_void, newbytecount: u32, newbuffer: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(NetApiBufferReallocate(::core::mem::transmute(oldbuffer), ::core::mem::transmute(newbytecount), ::core::mem::transmute(newbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn NetApiBufferSize(buffer: *const ::core::ffi::c_void, bytecount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetApiBufferSize(buffer: *const ::core::ffi::c_void, bytecount: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetApiBufferSize(::core::mem::transmute(buffer), ::core::mem::transmute(bytecount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAuditClear<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(server: Param0, backupfile: Param1, service: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAuditClear(server: super::super::Foundation::PWSTR, backupfile: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetAuditClear(server.into_param().abi(), backupfile.into_param().abi(), service.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAuditRead<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(server: Param0, service: Param1, auditloghandle: *mut HLOG, offset: u32, reserved1: *mut u32, reserved2: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxlen: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAuditRead(server: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, auditloghandle: *mut HLOG, offset: u32, reserved1: *mut u32, reserved2: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxlen: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetAuditRead(server.into_param().abi(), service.into_param().abi(), ::core::mem::transmute(auditloghandle), ::core::mem::transmute(offset), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2), ::core::mem::transmute(offsetflag), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(bytesread), ::core::mem::transmute(totalavailable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetAuditWrite<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(r#type: u32, buf: *mut u8, numbytes: u32, service: Param3, reserved: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetAuditWrite(r#type: u32, buf: *mut u8, numbytes: u32, service: super::super::Foundation::PWSTR, reserved: *mut u8) -> u32;
        }
        ::core::mem::transmute(NetAuditWrite(::core::mem::transmute(r#type), ::core::mem::transmute(buf), ::core::mem::transmute(numbytes), service.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetConfigGet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(server: Param0, component: Param1, parameter: Param2, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetConfigGet(server: super::super::Foundation::PWSTR, component: super::super::Foundation::PWSTR, parameter: super::super::Foundation::PWSTR, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetConfigGet(server.into_param().abi(), component.into_param().abi(), parameter.into_param().abi(), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetConfigGetAll<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(server: Param0, component: Param1, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetConfigGetAll(server: super::super::Foundation::PWSTR, component: super::super::Foundation::PWSTR, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetConfigGetAll(server.into_param().abi(), component.into_param().abi(), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetConfigSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(server: Param0, reserved1: Param1, component: Param2, level: u32, reserved2: u32, buf: *mut u8, reserved3: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetConfigSet(server: super::super::Foundation::PWSTR, reserved1: super::super::Foundation::PWSTR, component: super::super::Foundation::PWSTR, level: u32, reserved2: u32, buf: *mut u8, reserved3: u32) -> u32;
        }
        ::core::mem::transmute(NetConfigSet(server.into_param().abi(), reserved1.into_param().abi(), component.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(reserved2), ::core::mem::transmute(buf), ::core::mem::transmute(reserved3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetCreateProvisioningPackage(pprovisioningparams: *const NETSETUP_PROVISIONING_PARAMS, pppackagebindata: *mut *mut u8, pdwpackagebindatasize: *mut u32, pppackagetextdata: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetCreateProvisioningPackage(pprovisioningparams: *const NETSETUP_PROVISIONING_PARAMS, pppackagebindata: *mut *mut u8, pdwpackagebindatasize: *mut u32, pppackagetextdata: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetCreateProvisioningPackage(::core::mem::transmute(pprovisioningparams), ::core::mem::transmute(pppackagebindata), ::core::mem::transmute(pdwpackagebindatasize), ::core::mem::transmute(pppackagetextdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetEnumerateComputerNames<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(server: Param0, nametype: NET_COMPUTER_NAME_TYPE, reserved: u32, entrycount: *mut u32, computernames: *mut *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetEnumerateComputerNames(server: super::super::Foundation::PWSTR, nametype: NET_COMPUTER_NAME_TYPE, reserved: u32, entrycount: *mut u32, computernames: *mut *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetEnumerateComputerNames(server.into_param().abi(), ::core::mem::transmute(nametype), ::core::mem::transmute(reserved), ::core::mem::transmute(entrycount), ::core::mem::transmute(computernames)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetEnumerateServiceAccounts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, flags: u32, accountscount: *mut u32, accounts: *mut *mut *mut u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetEnumerateServiceAccounts(servername: super::super::Foundation::PWSTR, flags: u32, accountscount: *mut u32, accounts: *mut *mut *mut u16) -> super::super::Foundation::NTSTATUS;
        }
        NetEnumerateServiceAccounts(servername.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(accountscount), ::core::mem::transmute(accounts)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetErrorLogClear<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(uncservername: Param0, backupfile: Param1, reserved: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetErrorLogClear(uncservername: super::super::Foundation::PWSTR, backupfile: super::super::Foundation::PWSTR, reserved: *const u8) -> u32;
        }
        ::core::mem::transmute(NetErrorLogClear(uncservername.into_param().abi(), backupfile.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetErrorLogRead<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(uncservername: Param0, reserved1: Param1, errorloghandle: *const HLOG, offset: u32, reserved2: *const u32, reserved3: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxsize: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetErrorLogRead(uncservername: super::super::Foundation::PWSTR, reserved1: super::super::Foundation::PWSTR, errorloghandle: *const HLOG, offset: u32, reserved2: *const u32, reserved3: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxsize: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetErrorLogRead(uncservername.into_param().abi(), reserved1.into_param().abi(), ::core::mem::transmute(errorloghandle), ::core::mem::transmute(offset), ::core::mem::transmute(reserved2), ::core::mem::transmute(reserved3), ::core::mem::transmute(offsetflag), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxsize), ::core::mem::transmute(bytesread), ::core::mem::transmute(totalavailable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetErrorLogWrite<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(reserved1: *const u8, code: u32, component: Param2, buffer: *const u8, numbytes: u32, msgbuf: *const u8, strcount: u32, reserved2: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetErrorLogWrite(reserved1: *const u8, code: u32, component: super::super::Foundation::PWSTR, buffer: *const u8, numbytes: u32, msgbuf: *const u8, strcount: u32, reserved2: *const u8) -> u32;
        }
        ::core::mem::transmute(NetErrorLogWrite(::core::mem::transmute(reserved1), ::core::mem::transmute(code), component.into_param().abi(), ::core::mem::transmute(buffer), ::core::mem::transmute(numbytes), ::core::mem::transmute(msgbuf), ::core::mem::transmute(strcount), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn NetFreeAadJoinInformation(pjoininfo: *const DSREG_JOIN_INFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetFreeAadJoinInformation(pjoininfo: *const DSREG_JOIN_INFO);
        }
        NetFreeAadJoinInformation(::core::mem::transmute(pjoininfo))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn NetGetAadJoinInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pcsztenantid: Param0) -> ::windows::core::Result<*mut DSREG_JOIN_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGetAadJoinInformation(pcsztenantid: super::super::Foundation::PWSTR, ppjoininfo: *mut *mut DSREG_JOIN_INFO) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut DSREG_JOIN_INFO = ::core::mem::zeroed();
        NetGetAadJoinInformation(pcsztenantid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut DSREG_JOIN_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGetAnyDCName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, domainname: Param1, buffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGetAnyDCName(servername: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, buffer: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetGetAnyDCName(servername.into_param().abi(), domainname.into_param().abi(), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGetDCName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, domainname: Param1, buffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGetDCName(servername: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, buffer: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetGetDCName(servername.into_param().abi(), domainname.into_param().abi(), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGetDisplayInformationIndex<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, prefix: Param2, index: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGetDisplayInformationIndex(servername: super::super::Foundation::PWSTR, level: u32, prefix: super::super::Foundation::PWSTR, index: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetGetDisplayInformationIndex(servername.into_param().abi(), ::core::mem::transmute(level), prefix.into_param().abi(), ::core::mem::transmute(index)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGetJoinInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpserver: Param0, lpnamebuffer: *mut super::super::Foundation::PWSTR, buffertype: *mut NETSETUP_JOIN_STATUS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGetJoinInformation(lpserver: super::super::Foundation::PWSTR, lpnamebuffer: *mut super::super::Foundation::PWSTR, buffertype: *mut NETSETUP_JOIN_STATUS) -> u32;
        }
        ::core::mem::transmute(NetGetJoinInformation(lpserver.into_param().abi(), ::core::mem::transmute(lpnamebuffer), ::core::mem::transmute(buffertype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGetJoinableOUs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpserver: Param0, lpdomain: Param1, lpaccount: Param2, lppassword: Param3, oucount: *mut u32, ous: *mut *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGetJoinableOUs(lpserver: super::super::Foundation::PWSTR, lpdomain: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, oucount: *mut u32, ous: *mut *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetGetJoinableOUs(lpserver.into_param().abi(), lpdomain.into_param().abi(), lpaccount.into_param().abi(), lppassword.into_param().abi(), ::core::mem::transmute(oucount), ::core::mem::transmute(ous)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGroupAdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGroupAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetGroupAdd(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGroupAddUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1, username: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGroupAddUser(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetGroupAddUser(servername.into_param().abi(), groupname.into_param().abi(), username.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGroupDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGroupDel(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetGroupDel(servername.into_param().abi(), groupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGroupDelUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1, username: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGroupDelUser(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetGroupDelUser(servername.into_param().abi(), groupname.into_param().abi(), username.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGroupEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGroupEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut usize) -> u32;
        }
        ::core::mem::transmute(NetGroupEnum(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resume_handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGroupGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGroupGetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetGroupGetInfo(servername.into_param().abi(), groupname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGroupGetUsers<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGroupGetUsers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32;
        }
        ::core::mem::transmute(NetGroupGetUsers(servername.into_param().abi(), groupname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGroupSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGroupSetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetGroupSetInfo(servername.into_param().abi(), groupname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetGroupSetUsers<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1, level: u32, buf: *const u8, totalentries: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetGroupSetUsers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
        }
        ::core::mem::transmute(NetGroupSetUsers(servername.into_param().abi(), groupname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(totalentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetIsServiceAccount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, accountname: Param1, isservice: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetIsServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, isservice: *mut super::super::Foundation::BOOL) -> super::super::Foundation::NTSTATUS;
        }
        NetIsServiceAccount(servername.into_param().abi(), accountname.into_param().abi(), ::core::mem::transmute(isservice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetJoinDomain<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpserver: Param0, lpdomain: Param1, lpmachineaccountou: Param2, lpaccount: Param3, lppassword: Param4, fjoinoptions: NET_JOIN_DOMAIN_JOIN_OPTIONS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetJoinDomain(lpserver: super::super::Foundation::PWSTR, lpdomain: super::super::Foundation::PWSTR, lpmachineaccountou: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, fjoinoptions: NET_JOIN_DOMAIN_JOIN_OPTIONS) -> u32;
        }
        ::core::mem::transmute(NetJoinDomain(lpserver.into_param().abi(), lpdomain.into_param().abi(), lpmachineaccountou.into_param().abi(), lpaccount.into_param().abi(), lppassword.into_param().abi(), ::core::mem::transmute(fjoinoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupAdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetLocalGroupAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetLocalGroupAdd(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupAddMember<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(servername: Param0, groupname: Param1, membersid: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetLocalGroupAddMember(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, membersid: super::super::Foundation::PSID) -> u32;
        }
        ::core::mem::transmute(NetLocalGroupAddMember(servername.into_param().abi(), groupname.into_param().abi(), membersid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupAddMembers<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1, level: u32, buf: *const u8, totalentries: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetLocalGroupAddMembers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
        }
        ::core::mem::transmute(NetLocalGroupAddMembers(servername.into_param().abi(), groupname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(totalentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetLocalGroupDel(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetLocalGroupDel(servername.into_param().abi(), groupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupDelMember<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(servername: Param0, groupname: Param1, membersid: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetLocalGroupDelMember(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, membersid: super::super::Foundation::PSID) -> u32;
        }
        ::core::mem::transmute(NetLocalGroupDelMember(servername.into_param().abi(), groupname.into_param().abi(), membersid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupDelMembers<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1, level: u32, buf: *const u8, totalentries: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetLocalGroupDelMembers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
        }
        ::core::mem::transmute(NetLocalGroupDelMembers(servername.into_param().abi(), groupname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(totalentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetLocalGroupEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32;
        }
        ::core::mem::transmute(NetLocalGroupEnum(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetLocalGroupGetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetLocalGroupGetInfo(servername.into_param().abi(), groupname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupGetMembers<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, localgroupname: Param1, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetLocalGroupGetMembers(servername: super::super::Foundation::PWSTR, localgroupname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut usize) -> u32;
        }
        ::core::mem::transmute(NetLocalGroupGetMembers(servername.into_param().abi(), localgroupname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetLocalGroupSetInfo(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetLocalGroupSetInfo(servername.into_param().abi(), groupname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetLocalGroupSetMembers<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, groupname: Param1, level: u32, buf: *const u8, totalentries: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetLocalGroupSetMembers(servername: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, totalentries: u32) -> u32;
        }
        ::core::mem::transmute(NetLocalGroupSetMembers(servername.into_param().abi(), groupname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(totalentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetMessageBufferSend<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, msgname: Param1, fromname: Param2, buf: *const u8, buflen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetMessageBufferSend(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR, fromname: super::super::Foundation::PWSTR, buf: *const u8, buflen: u32) -> u32;
        }
        ::core::mem::transmute(NetMessageBufferSend(servername.into_param().abi(), msgname.into_param().abi(), fromname.into_param().abi(), ::core::mem::transmute(buf), ::core::mem::transmute(buflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetMessageNameAdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, msgname: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetMessageNameAdd(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetMessageNameAdd(servername.into_param().abi(), msgname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetMessageNameDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, msgname: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetMessageNameDel(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetMessageNameDel(servername.into_param().abi(), msgname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetMessageNameEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *const *const u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetMessageNameEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const *const u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetMessageNameEnum(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resume_handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetMessageNameGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, msgname: Param1, level: u32, bufptr: *const *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetMessageNameGetInfo(servername: super::super::Foundation::PWSTR, msgname: super::super::Foundation::PWSTR, level: u32, bufptr: *const *const u8) -> u32;
        }
        ::core::mem::transmute(NetMessageNameGetInfo(servername.into_param().abi(), msgname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetProvisionComputerAccount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpdomain: Param0, lpmachinename: Param1, lpmachineaccountou: Param2, lpdcname: Param3, dwoptions: NETSETUP_PROVISION, pprovisionbindata: *mut *mut u8, pdwprovisionbindatasize: *mut u32, pprovisiontextdata: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetProvisionComputerAccount(lpdomain: super::super::Foundation::PWSTR, lpmachinename: super::super::Foundation::PWSTR, lpmachineaccountou: super::super::Foundation::PWSTR, lpdcname: super::super::Foundation::PWSTR, dwoptions: NETSETUP_PROVISION, pprovisionbindata: *mut *mut u8, pdwprovisionbindatasize: *mut u32, pprovisiontextdata: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetProvisionComputerAccount(lpdomain.into_param().abi(), lpmachinename.into_param().abi(), lpmachineaccountou.into_param().abi(), lpdcname.into_param().abi(), ::core::mem::transmute(dwoptions), ::core::mem::transmute(pprovisionbindata), ::core::mem::transmute(pdwprovisionbindatasize), ::core::mem::transmute(pprovisiontextdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const NetProvisioning: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2aa2b5fe_b846_4d07_810c_b21ee45320e3);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetQueryDisplayInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, index: u32, entriesrequested: u32, preferredmaximumlength: u32, returnedentrycount: *mut u32, sortedbuffer: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetQueryDisplayInformation(servername: super::super::Foundation::PWSTR, level: u32, index: u32, entriesrequested: u32, preferredmaximumlength: u32, returnedentrycount: *mut u32, sortedbuffer: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(NetQueryDisplayInformation(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(index), ::core::mem::transmute(entriesrequested), ::core::mem::transmute(preferredmaximumlength), ::core::mem::transmute(returnedentrycount), ::core::mem::transmute(sortedbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetQueryServiceAccount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, accountname: Param1, infolevel: u32, buffer: *mut *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetQueryServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, infolevel: u32, buffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS;
        }
        NetQueryServiceAccount(servername.into_param().abi(), accountname.into_param().abi(), ::core::mem::transmute(infolevel), ::core::mem::transmute(buffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetRemoteComputerSupports<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(uncservername: Param0, optionswanted: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS, optionssupported: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetRemoteComputerSupports(uncservername: super::super::Foundation::PWSTR, optionswanted: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS, optionssupported: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetRemoteComputerSupports(uncservername.into_param().abi(), ::core::mem::transmute(optionswanted), ::core::mem::transmute(optionssupported)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetRemoteTOD<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(uncservername: Param0, bufferptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetRemoteTOD(uncservername: super::super::Foundation::PWSTR, bufferptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetRemoteTOD(uncservername.into_param().abi(), ::core::mem::transmute(bufferptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetRemoveAlternateComputerName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(server: Param0, alternatename: Param1, domainaccount: Param2, domainaccountpassword: Param3, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetRemoveAlternateComputerName(server: super::super::Foundation::PWSTR, alternatename: super::super::Foundation::PWSTR, domainaccount: super::super::Foundation::PWSTR, domainaccountpassword: super::super::Foundation::PWSTR, reserved: u32) -> u32;
        }
        ::core::mem::transmute(NetRemoveAlternateComputerName(server.into_param().abi(), alternatename.into_param().abi(), domainaccount.into_param().abi(), domainaccountpassword.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetRemoveServiceAccount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, accountname: Param1, flags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetRemoveServiceAccount(servername: super::super::Foundation::PWSTR, accountname: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::NTSTATUS;
        }
        NetRemoveServiceAccount(servername.into_param().abi(), accountname.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetRenameMachineInDomain<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpserver: Param0, lpnewmachinename: Param1, lpaccount: Param2, lppassword: Param3, frenameoptions: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetRenameMachineInDomain(lpserver: super::super::Foundation::PWSTR, lpnewmachinename: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, frenameoptions: u32) -> u32;
        }
        ::core::mem::transmute(NetRenameMachineInDomain(lpserver.into_param().abi(), lpnewmachinename.into_param().abi(), lpaccount.into_param().abi(), lppassword.into_param().abi(), ::core::mem::transmute(frenameoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplExportDirAdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplExportDirAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetReplExportDirAdd(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplExportDirDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, dirname: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplExportDirDel(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetReplExportDirDel(servername.into_param().abi(), dirname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplExportDirEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplExportDirEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetReplExportDirEnum(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplExportDirGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, dirname: Param1, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplExportDirGetInfo(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetReplExportDirGetInfo(servername.into_param().abi(), dirname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplExportDirLock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, dirname: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplExportDirLock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetReplExportDirLock(servername.into_param().abi(), dirname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplExportDirSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, dirname: Param1, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplExportDirSetInfo(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetReplExportDirSetInfo(servername.into_param().abi(), dirname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplExportDirUnlock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, dirname: Param1, unlockforce: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplExportDirUnlock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, unlockforce: u32) -> u32;
        }
        ::core::mem::transmute(NetReplExportDirUnlock(servername.into_param().abi(), dirname.into_param().abi(), ::core::mem::transmute(unlockforce)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplGetInfo(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetReplGetInfo(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplImportDirAdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplImportDirAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetReplImportDirAdd(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplImportDirDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, dirname: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplImportDirDel(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetReplImportDirDel(servername.into_param().abi(), dirname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplImportDirEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplImportDirEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetReplImportDirEnum(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplImportDirGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, dirname: Param1, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplImportDirGetInfo(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetReplImportDirGetInfo(servername.into_param().abi(), dirname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplImportDirLock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, dirname: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplImportDirLock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetReplImportDirLock(servername.into_param().abi(), dirname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplImportDirUnlock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, dirname: Param1, unlockforce: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplImportDirUnlock(servername: super::super::Foundation::PWSTR, dirname: super::super::Foundation::PWSTR, unlockforce: u32) -> u32;
        }
        ::core::mem::transmute(NetReplImportDirUnlock(servername.into_param().abi(), dirname.into_param().abi(), ::core::mem::transmute(unlockforce)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetReplSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetReplSetInfo(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetReplSetInfo(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetRequestOfflineDomainJoin<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pprovisionbindata: *const u8, cbprovisionbindatasize: u32, dwoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetRequestOfflineDomainJoin(pprovisionbindata: *const u8, cbprovisionbindatasize: u32, dwoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetRequestOfflineDomainJoin(::core::mem::transmute(pprovisionbindata), ::core::mem::transmute(cbprovisionbindatasize), ::core::mem::transmute(dwoptions), lpwindowspath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetRequestProvisioningPackageInstall<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(ppackagebindata: *const u8, dwpackagebindatasize: u32, dwprovisionoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: Param3, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetRequestProvisioningPackageInstall(ppackagebindata: *const u8, dwpackagebindatasize: u32, dwprovisionoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(NetRequestProvisioningPackageInstall(::core::mem::transmute(ppackagebindata), ::core::mem::transmute(dwpackagebindatasize), ::core::mem::transmute(dwprovisionoptions), lpwindowspath.into_param().abi(), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetScheduleJobAdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, buffer: *mut u8, jobid: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetScheduleJobAdd(servername: super::super::Foundation::PWSTR, buffer: *mut u8, jobid: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetScheduleJobAdd(servername.into_param().abi(), ::core::mem::transmute(buffer), ::core::mem::transmute(jobid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetScheduleJobDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, minjobid: u32, maxjobid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetScheduleJobDel(servername: super::super::Foundation::PWSTR, minjobid: u32, maxjobid: u32) -> u32;
        }
        ::core::mem::transmute(NetScheduleJobDel(servername.into_param().abi(), ::core::mem::transmute(minjobid), ::core::mem::transmute(maxjobid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetScheduleJobEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, pointertobuffer: *mut *mut u8, prefferedmaximumlength: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetScheduleJobEnum(servername: super::super::Foundation::PWSTR, pointertobuffer: *mut *mut u8, prefferedmaximumlength: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetScheduleJobEnum(servername.into_param().abi(), ::core::mem::transmute(pointertobuffer), ::core::mem::transmute(prefferedmaximumlength), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetScheduleJobGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, jobid: u32, pointertobuffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetScheduleJobGetInfo(servername: super::super::Foundation::PWSTR, jobid: u32, pointertobuffer: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetScheduleJobGetInfo(servername.into_param().abi(), ::core::mem::transmute(jobid), ::core::mem::transmute(pointertobuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerComputerNameAdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, emulateddomainname: Param1, emulatedservername: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerComputerNameAdd(servername: super::super::Foundation::PWSTR, emulateddomainname: super::super::Foundation::PWSTR, emulatedservername: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetServerComputerNameAdd(servername.into_param().abi(), emulateddomainname.into_param().abi(), emulatedservername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerComputerNameDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, emulatedservername: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerComputerNameDel(servername: super::super::Foundation::PWSTR, emulatedservername: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetServerComputerNameDel(servername.into_param().abi(), emulatedservername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerDiskEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerDiskEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetServerDiskEnum(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resume_handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, servertype: NET_SERVER_TYPE, domain: Param7, resume_handle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, servertype: NET_SERVER_TYPE, domain: super::super::Foundation::PWSTR, resume_handle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetServerEnum(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(servertype), domain.into_param().abi(), ::core::mem::transmute(resume_handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerGetInfo(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetServerGetInfo(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, buf: *const u8, parmerror: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerSetInfo(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parmerror: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetServerSetInfo(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parmerror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerTransportAdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerTransportAdd(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const u8) -> u32;
        }
        ::core::mem::transmute(NetServerTransportAdd(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerTransportAddEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerTransportAddEx(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const u8) -> u32;
        }
        ::core::mem::transmute(NetServerTransportAddEx(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerTransportDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerTransportDel(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *const u8) -> u32;
        }
        ::core::mem::transmute(NetServerTransportDel(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServerTransportEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServerTransportEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetServerTransportEnum(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resume_handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServiceControl<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, service: Param1, opcode: u32, arg: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServiceControl(servername: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, opcode: u32, arg: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetServiceControl(servername.into_param().abi(), service.into_param().abi(), ::core::mem::transmute(opcode), ::core::mem::transmute(arg), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServiceEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServiceEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetServiceEnum(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resume_handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServiceGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, service: Param1, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServiceGetInfo(servername: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetServiceGetInfo(servername.into_param().abi(), service.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetServiceInstall<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, service: Param1, argc: u32, argv: *const super::super::Foundation::PWSTR, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetServiceInstall(servername: super::super::Foundation::PWSTR, service: super::super::Foundation::PWSTR, argc: u32, argv: *const super::super::Foundation::PWSTR, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetServiceInstall(servername.into_param().abi(), service.into_param().abi(), ::core::mem::transmute(argc), ::core::mem::transmute(argv), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetSetPrimaryComputerName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(server: Param0, primaryname: Param1, domainaccount: Param2, domainaccountpassword: Param3, reserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetSetPrimaryComputerName(server: super::super::Foundation::PWSTR, primaryname: super::super::Foundation::PWSTR, domainaccount: super::super::Foundation::PWSTR, domainaccountpassword: super::super::Foundation::PWSTR, reserved: u32) -> u32;
        }
        ::core::mem::transmute(NetSetPrimaryComputerName(server.into_param().abi(), primaryname.into_param().abi(), domainaccount.into_param().abi(), domainaccountpassword.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUnjoinDomain<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpserver: Param0, lpaccount: Param1, lppassword: Param2, funjoinoptions: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUnjoinDomain(lpserver: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, funjoinoptions: u32) -> u32;
        }
        ::core::mem::transmute(NetUnjoinDomain(lpserver.into_param().abi(), lpaccount.into_param().abi(), lppassword.into_param().abi(), ::core::mem::transmute(funjoinoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn NetUseAdd(servername: *const i8, levelflags: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUseAdd(servername: *const i8, levelflags: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetUseAdd(::core::mem::transmute(servername), ::core::mem::transmute(levelflags), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUseDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(uncservername: Param0, usename: Param1, forcelevelflags: FORCE_LEVEL_FLAGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUseDel(uncservername: super::super::Foundation::PWSTR, usename: super::super::Foundation::PWSTR, forcelevelflags: FORCE_LEVEL_FLAGS) -> u32;
        }
        ::core::mem::transmute(NetUseDel(uncservername.into_param().abi(), usename.into_param().abi(), ::core::mem::transmute(forcelevelflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUseEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(uncservername: Param0, levelflags: u32, bufptr: *mut *mut u8, preferedmaximumsize: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUseEnum(uncservername: super::super::Foundation::PWSTR, levelflags: u32, bufptr: *mut *mut u8, preferedmaximumsize: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetUseEnum(uncservername.into_param().abi(), ::core::mem::transmute(levelflags), ::core::mem::transmute(bufptr), ::core::mem::transmute(preferedmaximumsize), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUseGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(uncservername: Param0, usename: Param1, levelflags: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUseGetInfo(uncservername: super::super::Foundation::PWSTR, usename: super::super::Foundation::PWSTR, levelflags: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetUseGetInfo(uncservername.into_param().abi(), usename.into_param().abi(), ::core::mem::transmute(levelflags), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUserAdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUserAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetUserAdd(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUserChangePassword<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(domainname: Param0, username: Param1, oldpassword: Param2, newpassword: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUserChangePassword(domainname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, oldpassword: super::super::Foundation::PWSTR, newpassword: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetUserChangePassword(domainname.into_param().abi(), username.into_param().abi(), oldpassword.into_param().abi(), newpassword.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUserDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, username: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUserDel(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetUserDel(servername.into_param().abi(), username.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUserEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, filter: NET_USER_ENUM_FILTER_FLAGS, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUserEnum(servername: super::super::Foundation::PWSTR, level: u32, filter: NET_USER_ENUM_FILTER_FLAGS, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetUserEnum(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(filter), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resume_handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUserGetGroups<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, username: Param1, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUserGetGroups(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetUserGetGroups(servername.into_param().abi(), username.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUserGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, username: Param1, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUserGetInfo(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetUserGetInfo(servername.into_param().abi(), username.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUserGetLocalGroups<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, username: Param1, level: u32, flags: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUserGetLocalGroups(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, flags: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetUserGetLocalGroups(servername.into_param().abi(), username.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(flags), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUserModalsGet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUserModalsGet(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetUserModalsGet(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUserModalsSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUserModalsSet(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetUserModalsSet(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUserSetGroups<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, username: Param1, level: u32, buf: *const u8, num_entries: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUserSetGroups(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, buf: *const u8, num_entries: u32) -> u32;
        }
        ::core::mem::transmute(NetUserSetGroups(servername.into_param().abi(), username.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(num_entries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetUserSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, username: Param1, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetUserSetInfo(servername: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetUserSetInfo(servername.into_param().abi(), username.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetValidateName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpserver: Param0, lpname: Param1, lpaccount: Param2, lppassword: Param3, nametype: NETSETUP_NAME_TYPE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetValidateName(lpserver: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, lpaccount: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, nametype: NETSETUP_NAME_TYPE) -> u32;
        }
        ::core::mem::transmute(NetValidateName(lpserver.into_param().abi(), lpname.into_param().abi(), lpaccount.into_param().abi(), lppassword.into_param().abi(), ::core::mem::transmute(nametype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetValidatePasswordPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, qualifier: *mut ::core::ffi::c_void, validationtype: NET_VALIDATE_PASSWORD_TYPE, inputarg: *mut ::core::ffi::c_void, outputarg: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetValidatePasswordPolicy(servername: super::super::Foundation::PWSTR, qualifier: *mut ::core::ffi::c_void, validationtype: NET_VALIDATE_PASSWORD_TYPE, inputarg: *mut ::core::ffi::c_void, outputarg: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(NetValidatePasswordPolicy(servername.into_param().abi(), ::core::mem::transmute(qualifier), ::core::mem::transmute(validationtype), ::core::mem::transmute(inputarg), ::core::mem::transmute(outputarg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn NetValidatePasswordPolicyFree(outputarg: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetValidatePasswordPolicyFree(outputarg: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(NetValidatePasswordPolicyFree(::core::mem::transmute(outputarg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetWkstaGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetWkstaGetInfo(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetWkstaGetInfo(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetWkstaSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, buffer: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetWkstaSetInfo(servername: super::super::Foundation::PWSTR, level: u32, buffer: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetWkstaSetInfo(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buffer), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn NetWkstaTransportAdd(servername: *const i8, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetWkstaTransportAdd(servername: *const i8, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetWkstaTransportAdd(::core::mem::transmute(servername), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetWkstaTransportDel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, transportname: Param1, ucond: FORCE_LEVEL_FLAGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetWkstaTransportDel(servername: super::super::Foundation::PWSTR, transportname: super::super::Foundation::PWSTR, ucond: FORCE_LEVEL_FLAGS) -> u32;
        }
        ::core::mem::transmute(NetWkstaTransportDel(servername.into_param().abi(), transportname.into_param().abi(), ::core::mem::transmute(ucond)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn NetWkstaTransportEnum(servername: *const i8, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetWkstaTransportEnum(servername: *const i8, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetWkstaTransportEnum(::core::mem::transmute(servername), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resume_handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetWkstaUserEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetWkstaUserEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetWkstaUserEnum(servername.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(entriesread), ::core::mem::transmute(totalentries), ::core::mem::transmute(resumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetWkstaUserGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(reserved: Param0, level: u32, bufptr: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetWkstaUserGetInfo(reserved: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetWkstaUserGetInfo(reserved.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(bufptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetWkstaUserSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(reserved: Param0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetWkstaUserSetInfo(reserved: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetWkstaUserSetInfo(reserved.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buf), ::core::mem::transmute(parm_err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OBO_TOKEN {
    pub Type: OBO_TOKEN_TYPE,
    pub pncc: ::core::option::Option<INetCfgComponent>,
    pub pszwManufacturer: super::super::Foundation::PWSTR,
    pub pszwProduct: super::super::Foundation::PWSTR,
    pub pszwDisplayName: super::super::Foundation::PWSTR,
    pub fRegistered: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OBO_TOKEN {
    fn clone(&self) -> Self {
        Self {
            Type: self.Type,
            pncc: self.pncc.clone(),
            pszwManufacturer: self.pszwManufacturer,
            pszwProduct: self.pszwProduct,
            pszwDisplayName: self.pszwDisplayName,
            fRegistered: self.fRegistered,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OBO_TOKEN {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OBO_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pncc == other.pncc && self.pszwManufacturer == other.pszwManufacturer && self.pszwProduct == other.pszwProduct && self.pszwDisplayName == other.pszwDisplayName && self.fRegistered == other.fRegistered
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OBO_TOKEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBO_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type OBO_TOKEN_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const OBO_USER: OBO_TOKEN_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const OBO_COMPONENT: OBO_TOKEN_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const OBO_SOFTWARE: OBO_TOKEN_TYPE = 3i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PARMNUM_ALL: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PARMNUM_BASE_INFOLEVEL: u32 = 1000u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PARM_ERROR_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PARM_ERROR_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PASSWORD_EXPIRED: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PATHLEN: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PLATFORM_ID_DOS: u32 = 300u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PLATFORM_ID_NT: u32 = 500u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PLATFORM_ID_OS2: u32 = 400u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PLATFORM_ID_OSF: u32 = 600u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PLATFORM_ID_VMS: u32 = 700u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PREFIX_MISMATCH: i32 = -1073727510i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PREFIX_MISMATCH_FIXED: i32 = -1073727509i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PREFIX_MISMATCH_NOT_FIXED: i32 = -1073727508i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct PRINT_OTHER_INFO {
    pub alrtpr_jobid: u32,
    pub alrtpr_status: u32,
    pub alrtpr_submitted: u32,
    pub alrtpr_size: u32,
}
impl ::core::marker::Copy for PRINT_OTHER_INFO {}
impl ::core::clone::Clone for PRINT_OTHER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINT_OTHER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PRINT_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRINT_OTHER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PRINT_OTHER_INFO {}
impl ::core::default::Default for PRINT_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_COMPLETE: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_DELETED: u32 = 32768u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_DESTNOPAPER: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_DESTOFFLINE: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_DESTPAUSED: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_DEVSTATUS: u32 = 508u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_ERROR: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_INTERV: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_NOTIFY: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_QSTATUS: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_QS_PAUSED: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_QS_PRINTING: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_QS_QUEUED: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PRJOB_QS_SPOOLING: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IPV6_DHCP: u32 = 999u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_ALG: u32 = 10010u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_BGMP: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_BOOTP: u32 = 9999u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_DHCP_ALLOCATOR: u32 = 10004u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_DIFFSERV: u32 = 10008u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_DNS_PROXY: u32 = 10003u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_DTP: u32 = 10013u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_FTP: u32 = 10012u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_H323: u32 = 10011u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_IGMP: u32 = 10u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_MGM: u32 = 10009u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_MSDP: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_NAT: u32 = 10005u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_IP_VRRP: u32 = 112u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_TYPE_MCAST: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_TYPE_MS0: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_TYPE_MS1: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_TYPE_UCAST: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_VENDOR_MS0: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_VENDOR_MS1: u32 = 311u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PROTO_VENDOR_MS2: u32 = 16383u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const PWLEN: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const QNLEN: u32 = 80u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASCON_IPUI {
    pub guidConnection: ::windows::core::GUID,
    pub fIPv6Cfg: super::super::Foundation::BOOL,
    pub dwFlags: u32,
    pub pszwIpAddr: [u16; 16],
    pub pszwDnsAddr: [u16; 16],
    pub pszwDns2Addr: [u16; 16],
    pub pszwWinsAddr: [u16; 16],
    pub pszwWins2Addr: [u16; 16],
    pub pszwDnsSuffix: [u16; 256],
    pub pszwIpv6Addr: [u16; 65],
    pub dwIpv6PrefixLength: u32,
    pub pszwIpv6DnsAddr: [u16; 65],
    pub pszwIpv6Dns2Addr: [u16; 65],
    pub dwIPv4InfMetric: u32,
    pub dwIPv6InfMetric: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASCON_IPUI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASCON_IPUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASCON_IPUI {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASCON_IPUI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASCON_IPUI>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASCON_IPUI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASCON_IPUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REPL_EDIR_INFO_0 {
    pub rped0_dirname: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REPL_EDIR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REPL_EDIR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REPL_EDIR_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_EDIR_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REPL_EDIR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REPL_EDIR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REPL_EDIR_INFO_1 {
    pub rped1_dirname: super::super::Foundation::PWSTR,
    pub rped1_integrity: u32,
    pub rped1_extent: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REPL_EDIR_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REPL_EDIR_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REPL_EDIR_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_EDIR_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REPL_EDIR_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REPL_EDIR_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct REPL_EDIR_INFO_1000 {
    pub rped1000_integrity: u32,
}
impl ::core::marker::Copy for REPL_EDIR_INFO_1000 {}
impl ::core::clone::Clone for REPL_EDIR_INFO_1000 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REPL_EDIR_INFO_1000 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_1000 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_EDIR_INFO_1000>()) == 0 }
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_1000 {}
impl ::core::default::Default for REPL_EDIR_INFO_1000 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct REPL_EDIR_INFO_1001 {
    pub rped1001_extent: u32,
}
impl ::core::marker::Copy for REPL_EDIR_INFO_1001 {}
impl ::core::clone::Clone for REPL_EDIR_INFO_1001 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REPL_EDIR_INFO_1001 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_1001 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_EDIR_INFO_1001>()) == 0 }
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_1001 {}
impl ::core::default::Default for REPL_EDIR_INFO_1001 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REPL_EDIR_INFO_2 {
    pub rped2_dirname: super::super::Foundation::PWSTR,
    pub rped2_integrity: u32,
    pub rped2_extent: u32,
    pub rped2_lockcount: u32,
    pub rped2_locktime: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REPL_EDIR_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REPL_EDIR_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REPL_EDIR_INFO_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_EDIR_INFO_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REPL_EDIR_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REPL_EDIR_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_EXPORT_EXTENT_INFOLEVEL: u32 = 1001u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_EXPORT_INTEGRITY_INFOLEVEL: u32 = 1000u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_EXTENT_FILE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_EXTENT_TREE: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_GUARDTIME_INFOLEVEL: u32 = 1002u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REPL_IDIR_INFO_0 {
    pub rpid0_dirname: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REPL_IDIR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REPL_IDIR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REPL_IDIR_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REPL_IDIR_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_IDIR_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REPL_IDIR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REPL_IDIR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REPL_IDIR_INFO_1 {
    pub rpid1_dirname: super::super::Foundation::PWSTR,
    pub rpid1_state: u32,
    pub rpid1_mastername: super::super::Foundation::PWSTR,
    pub rpid1_last_update_time: u32,
    pub rpid1_lockcount: u32,
    pub rpid1_locktime: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REPL_IDIR_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REPL_IDIR_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REPL_IDIR_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REPL_IDIR_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_IDIR_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REPL_IDIR_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REPL_IDIR_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REPL_INFO_0 {
    pub rp0_role: u32,
    pub rp0_exportpath: super::super::Foundation::PWSTR,
    pub rp0_exportlist: super::super::Foundation::PWSTR,
    pub rp0_importpath: super::super::Foundation::PWSTR,
    pub rp0_importlist: super::super::Foundation::PWSTR,
    pub rp0_logonusername: super::super::Foundation::PWSTR,
    pub rp0_interval: u32,
    pub rp0_pulse: u32,
    pub rp0_guardtime: u32,
    pub rp0_random: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REPL_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REPL_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REPL_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REPL_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REPL_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REPL_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct REPL_INFO_1000 {
    pub rp1000_interval: u32,
}
impl ::core::marker::Copy for REPL_INFO_1000 {}
impl ::core::clone::Clone for REPL_INFO_1000 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REPL_INFO_1000 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REPL_INFO_1000 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_INFO_1000>()) == 0 }
    }
}
impl ::core::cmp::Eq for REPL_INFO_1000 {}
impl ::core::default::Default for REPL_INFO_1000 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct REPL_INFO_1001 {
    pub rp1001_pulse: u32,
}
impl ::core::marker::Copy for REPL_INFO_1001 {}
impl ::core::clone::Clone for REPL_INFO_1001 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REPL_INFO_1001 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REPL_INFO_1001 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_INFO_1001>()) == 0 }
    }
}
impl ::core::cmp::Eq for REPL_INFO_1001 {}
impl ::core::default::Default for REPL_INFO_1001 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct REPL_INFO_1002 {
    pub rp1002_guardtime: u32,
}
impl ::core::marker::Copy for REPL_INFO_1002 {}
impl ::core::clone::Clone for REPL_INFO_1002 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REPL_INFO_1002 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REPL_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_INFO_1002>()) == 0 }
    }
}
impl ::core::cmp::Eq for REPL_INFO_1002 {}
impl ::core::default::Default for REPL_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct REPL_INFO_1003 {
    pub rp1003_random: u32,
}
impl ::core::marker::Copy for REPL_INFO_1003 {}
impl ::core::clone::Clone for REPL_INFO_1003 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for REPL_INFO_1003 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REPL_INFO_1003 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPL_INFO_1003>()) == 0 }
    }
}
impl ::core::cmp::Eq for REPL_INFO_1003 {}
impl ::core::default::Default for REPL_INFO_1003 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_INTEGRITY_FILE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_INTEGRITY_TREE: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_INTERVAL_INFOLEVEL: u32 = 1000u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_PULSE_INFOLEVEL: u32 = 1001u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_RANDOM_INFOLEVEL: u32 = 1003u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_ROLE_BOTH: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_ROLE_EXPORT: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_ROLE_IMPORT: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_STATE_NEVER_REPLICATED: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_STATE_NO_MASTER: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_STATE_NO_SYNC: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_STATE_OK: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_UNLOCK_FORCE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const REPL_UNLOCK_NOFORCE: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RF_ADD_ALL_INTERFACES: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RF_DEMAND_UPDATE_ROUTES: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RF_MULTICAST: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RF_POWER: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RF_ROUTING: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RF_ROUTINGV6: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RIS_INTERFACE_ADDRESS_CHANGE: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RIS_INTERFACE_DISABLED: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RIS_INTERFACE_ENABLED: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RIS_INTERFACE_MEDIA_ABSENT: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RIS_INTERFACE_MEDIA_PRESENT: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const ROUTING_DOMAIN_INFO_REVISION_1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct RTR_INFO_BLOCK_HEADER {
    pub Version: u32,
    pub Size: u32,
    pub TocEntriesCount: u32,
    pub TocEntry: [RTR_TOC_ENTRY; 1],
}
impl ::core::marker::Copy for RTR_INFO_BLOCK_HEADER {}
impl ::core::clone::Clone for RTR_INFO_BLOCK_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RTR_INFO_BLOCK_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTR_INFO_BLOCK_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTR_INFO_BLOCK_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTR_INFO_BLOCK_HEADER {}
impl ::core::default::Default for RTR_INFO_BLOCK_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RTR_INFO_BLOCK_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct RTR_TOC_ENTRY {
    pub InfoType: u32,
    pub InfoSize: u32,
    pub Count: u32,
    pub Offset: u32,
}
impl ::core::marker::Copy for RTR_TOC_ENTRY {}
impl ::core::clone::Clone for RTR_TOC_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RTR_TOC_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTR_TOC_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTR_TOC_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTR_TOC_ENTRY {}
impl ::core::default::Default for RTR_TOC_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RTUTILS_MAX_PROTOCOL_DLL_LEN: u32 = 48u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RTUTILS_MAX_PROTOCOL_NAME_LEN: u32 = 40u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterAssert<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pszfailedassertion: Param0, pszfilename: Param1, dwlinenumber: u32, pszmessage: Param3) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterAssert(pszfailedassertion: super::super::Foundation::PSTR, pszfilename: super::super::Foundation::PSTR, dwlinenumber: u32, pszmessage: super::super::Foundation::PSTR);
        }
        RouterAssert(pszfailedassertion.into_param().abi(), pszfilename.into_param().abi(), ::core::mem::transmute(dwlinenumber), pszmessage.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterGetErrorStringA(dwerrorcode: u32, lplpszerrorstring: *mut super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterGetErrorStringA(dwerrorcode: u32, lplpszerrorstring: *mut super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(RouterGetErrorStringA(::core::mem::transmute(dwerrorcode), ::core::mem::transmute(lplpszerrorstring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterGetErrorStringW(dwerrorcode: u32, lplpwszerrorstring: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterGetErrorStringW(dwerrorcode: u32, lplpwszerrorstring: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(RouterGetErrorStringW(::core::mem::transmute(dwerrorcode), ::core::mem::transmute(lplpwszerrorstring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogDeregisterA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hloghandle: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogDeregisterA(hloghandle: super::super::Foundation::HANDLE);
        }
        RouterLogDeregisterA(hloghandle.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogDeregisterW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hloghandle: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogDeregisterW(hloghandle: super::super::Foundation::HANDLE);
        }
        RouterLogDeregisterW(hloghandle.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hloghandle: Param0, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwerrorcode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogEventA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwerrorcode: u32);
        }
        RouterLogEventA(hloghandle.into_param().abi(), ::core::mem::transmute(dweventtype), ::core::mem::transmute(dwmessageid), ::core::mem::transmute(dwsubstringcount), ::core::mem::transmute(plpszsubstringarray), ::core::mem::transmute(dwerrorcode))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventDataA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hloghandle: Param0, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwdatabytes: u32, lpdatabytes: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogEventDataA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwdatabytes: u32, lpdatabytes: *mut u8);
        }
        RouterLogEventDataA(hloghandle.into_param().abi(), ::core::mem::transmute(dweventtype), ::core::mem::transmute(dwmessageid), ::core::mem::transmute(dwsubstringcount), ::core::mem::transmute(plpszsubstringarray), ::core::mem::transmute(dwdatabytes), ::core::mem::transmute(lpdatabytes))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventDataW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hloghandle: Param0, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwdatabytes: u32, lpdatabytes: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogEventDataW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwdatabytes: u32, lpdatabytes: *mut u8);
        }
        RouterLogEventDataW(hloghandle.into_param().abi(), ::core::mem::transmute(dweventtype), ::core::mem::transmute(dwmessageid), ::core::mem::transmute(dwsubstringcount), ::core::mem::transmute(plpszsubstringarray), ::core::mem::transmute(dwdatabytes), ::core::mem::transmute(lpdatabytes))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hloghandle: Param0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: Param4) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogEventExA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PSTR);
        }
        RouterLogEventExA(hloghandle.into_param().abi(), ::core::mem::transmute(dweventtype), ::core::mem::transmute(dwerrorcode), ::core::mem::transmute(dwmessageid), ptszformat.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hloghandle: Param0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: Param4) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogEventExW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PWSTR);
        }
        RouterLogEventExW(hloghandle.into_param().abi(), ::core::mem::transmute(dweventtype), ::core::mem::transmute(dwerrorcode), ::core::mem::transmute(dwmessageid), ptszformat.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventStringA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hloghandle: Param0, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwerrorcode: u32, dwerrorindex: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogEventStringA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PSTR, dwerrorcode: u32, dwerrorindex: u32);
        }
        RouterLogEventStringA(hloghandle.into_param().abi(), ::core::mem::transmute(dweventtype), ::core::mem::transmute(dwmessageid), ::core::mem::transmute(dwsubstringcount), ::core::mem::transmute(plpszsubstringarray), ::core::mem::transmute(dwerrorcode), ::core::mem::transmute(dwerrorindex))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventStringW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hloghandle: Param0, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwerrorcode: u32, dwerrorindex: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogEventStringW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwerrorcode: u32, dwerrorindex: u32);
        }
        RouterLogEventStringW(hloghandle.into_param().abi(), ::core::mem::transmute(dweventtype), ::core::mem::transmute(dwmessageid), ::core::mem::transmute(dwsubstringcount), ::core::mem::transmute(plpszsubstringarray), ::core::mem::transmute(dwerrorcode), ::core::mem::transmute(dwerrorindex))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventValistExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hloghandle: Param0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: Param4, arglist: *mut i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogEventValistExA(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PSTR, arglist: *mut i8);
        }
        RouterLogEventValistExA(hloghandle.into_param().abi(), ::core::mem::transmute(dweventtype), ::core::mem::transmute(dwerrorcode), ::core::mem::transmute(dwmessageid), ptszformat.into_param().abi(), ::core::mem::transmute(arglist))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventValistExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hloghandle: Param0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: Param4, arglist: *mut i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogEventValistExW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: super::super::Foundation::PWSTR, arglist: *mut i8);
        }
        RouterLogEventValistExW(hloghandle.into_param().abi(), ::core::mem::transmute(dweventtype), ::core::mem::transmute(dwerrorcode), ::core::mem::transmute(dwmessageid), ptszformat.into_param().abi(), ::core::mem::transmute(arglist))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogEventW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hloghandle: Param0, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwerrorcode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogEventW(hloghandle: super::super::Foundation::HANDLE, dweventtype: u32, dwmessageid: u32, dwsubstringcount: u32, plpszsubstringarray: *const super::super::Foundation::PWSTR, dwerrorcode: u32);
        }
        RouterLogEventW(hloghandle.into_param().abi(), ::core::mem::transmute(dweventtype), ::core::mem::transmute(dwmessageid), ::core::mem::transmute(dwsubstringcount), ::core::mem::transmute(plpszsubstringarray), ::core::mem::transmute(dwerrorcode))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogRegisterA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszsource: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogRegisterA(lpszsource: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(RouterLogRegisterA(lpszsource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RouterLogRegisterW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszsource: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RouterLogRegisterW(lpszsource: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(RouterLogRegisterW(lpszsource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_100 {
    pub sv100_platform_id: u32,
    pub sv100_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_100 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_100 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_100 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_100 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_100>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_100 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_100 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1005 {
    pub sv1005_comment: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1005 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1005 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1005 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1005>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1005 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_101 {
    pub sv101_platform_id: u32,
    pub sv101_name: super::super::Foundation::PWSTR,
    pub sv101_version_major: u32,
    pub sv101_version_minor: u32,
    pub sv101_type: NET_SERVER_TYPE,
    pub sv101_comment: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_101 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_101 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_101 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_101 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_101>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_101 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_101 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1010 {
    pub sv1010_disc: i32,
}
impl ::core::marker::Copy for SERVER_INFO_1010 {}
impl ::core::clone::Clone for SERVER_INFO_1010 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1010 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1010 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1010>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1010 {}
impl ::core::default::Default for SERVER_INFO_1010 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1016 {
    pub sv1016_hidden: SERVER_INFO_HIDDEN,
}
impl ::core::marker::Copy for SERVER_INFO_1016 {}
impl ::core::clone::Clone for SERVER_INFO_1016 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1016 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1016 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1016>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1016 {}
impl ::core::default::Default for SERVER_INFO_1016 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1017 {
    pub sv1017_announce: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1017 {}
impl ::core::clone::Clone for SERVER_INFO_1017 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1017 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1017 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1017>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1017 {}
impl ::core::default::Default for SERVER_INFO_1017 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1018 {
    pub sv1018_anndelta: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1018 {}
impl ::core::clone::Clone for SERVER_INFO_1018 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1018 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1018 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1018>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1018 {}
impl ::core::default::Default for SERVER_INFO_1018 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_102 {
    pub sv102_platform_id: u32,
    pub sv102_name: super::super::Foundation::PWSTR,
    pub sv102_version_major: u32,
    pub sv102_version_minor: u32,
    pub sv102_type: NET_SERVER_TYPE,
    pub sv102_comment: super::super::Foundation::PWSTR,
    pub sv102_users: u32,
    pub sv102_disc: i32,
    pub sv102_hidden: SERVER_INFO_HIDDEN,
    pub sv102_announce: u32,
    pub sv102_anndelta: u32,
    pub sv102_licenses: u32,
    pub sv102_userpath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_102 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_102 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_102 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_102 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_102>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_102 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_102 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_103 {
    pub sv103_platform_id: u32,
    pub sv103_name: super::super::Foundation::PWSTR,
    pub sv103_version_major: u32,
    pub sv103_version_minor: u32,
    pub sv103_type: u32,
    pub sv103_comment: super::super::Foundation::PWSTR,
    pub sv103_users: u32,
    pub sv103_disc: i32,
    pub sv103_hidden: super::super::Foundation::BOOL,
    pub sv103_announce: u32,
    pub sv103_anndelta: u32,
    pub sv103_licenses: u32,
    pub sv103_userpath: super::super::Foundation::PWSTR,
    pub sv103_capabilities: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_103 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_103 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_103 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_103 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_103>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_103 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_103 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1107 {
    pub sv1107_users: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1107 {}
impl ::core::clone::Clone for SERVER_INFO_1107 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1107 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1107 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1107>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1107 {}
impl ::core::default::Default for SERVER_INFO_1107 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1501 {
    pub sv1501_sessopens: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1501 {}
impl ::core::clone::Clone for SERVER_INFO_1501 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1501 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1501 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1501>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1501 {}
impl ::core::default::Default for SERVER_INFO_1501 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1502 {
    pub sv1502_sessvcs: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1502 {}
impl ::core::clone::Clone for SERVER_INFO_1502 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1502 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1502 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1502>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1502 {}
impl ::core::default::Default for SERVER_INFO_1502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1503 {
    pub sv1503_opensearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1503 {}
impl ::core::clone::Clone for SERVER_INFO_1503 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1503 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1503 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1503>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1503 {}
impl ::core::default::Default for SERVER_INFO_1503 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1506 {
    pub sv1506_maxworkitems: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1506 {}
impl ::core::clone::Clone for SERVER_INFO_1506 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1506 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1506 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1506>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1506 {}
impl ::core::default::Default for SERVER_INFO_1506 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1509 {
    pub sv1509_maxrawbuflen: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1509 {}
impl ::core::clone::Clone for SERVER_INFO_1509 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1509 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1509 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1509>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1509 {}
impl ::core::default::Default for SERVER_INFO_1509 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1510 {
    pub sv1510_sessusers: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1510 {}
impl ::core::clone::Clone for SERVER_INFO_1510 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1510 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1510 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1510>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1510 {}
impl ::core::default::Default for SERVER_INFO_1510 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1511 {
    pub sv1511_sessconns: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1511 {}
impl ::core::clone::Clone for SERVER_INFO_1511 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1511 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1511 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1511>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1511 {}
impl ::core::default::Default for SERVER_INFO_1511 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1512 {
    pub sv1512_maxnonpagedmemoryusage: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1512 {}
impl ::core::clone::Clone for SERVER_INFO_1512 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1512 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1512 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1512>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1512 {}
impl ::core::default::Default for SERVER_INFO_1512 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1513 {
    pub sv1513_maxpagedmemoryusage: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1513 {}
impl ::core::clone::Clone for SERVER_INFO_1513 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1513 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1513 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1513>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1513 {}
impl ::core::default::Default for SERVER_INFO_1513 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1514 {
    pub sv1514_enablesoftcompat: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1514 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1514 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1514 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1514 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1514>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1514 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1514 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1515 {
    pub sv1515_enableforcedlogoff: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1515 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1515 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1515 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1515 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1515>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1515 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1515 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1516 {
    pub sv1516_timesource: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1516 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1516 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1516 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1516 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1516>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1516 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1516 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1518 {
    pub sv1518_lmannounce: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1518 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1518 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1518 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1518 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1518>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1518 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1518 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1520 {
    pub sv1520_maxcopyreadlen: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1520 {}
impl ::core::clone::Clone for SERVER_INFO_1520 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1520 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1520 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1520>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1520 {}
impl ::core::default::Default for SERVER_INFO_1520 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1521 {
    pub sv1521_maxcopywritelen: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1521 {}
impl ::core::clone::Clone for SERVER_INFO_1521 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1521 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1521 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1521>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1521 {}
impl ::core::default::Default for SERVER_INFO_1521 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1522 {
    pub sv1522_minkeepsearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1522 {}
impl ::core::clone::Clone for SERVER_INFO_1522 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1522 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1522 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1522>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1522 {}
impl ::core::default::Default for SERVER_INFO_1522 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1523 {
    pub sv1523_maxkeepsearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1523 {}
impl ::core::clone::Clone for SERVER_INFO_1523 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1523 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1523 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1523>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1523 {}
impl ::core::default::Default for SERVER_INFO_1523 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1524 {
    pub sv1524_minkeepcomplsearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1524 {}
impl ::core::clone::Clone for SERVER_INFO_1524 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1524 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1524 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1524>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1524 {}
impl ::core::default::Default for SERVER_INFO_1524 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1525 {
    pub sv1525_maxkeepcomplsearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1525 {}
impl ::core::clone::Clone for SERVER_INFO_1525 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1525 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1525 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1525>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1525 {}
impl ::core::default::Default for SERVER_INFO_1525 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1528 {
    pub sv1528_scavtimeout: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1528 {}
impl ::core::clone::Clone for SERVER_INFO_1528 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1528 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1528 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1528>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1528 {}
impl ::core::default::Default for SERVER_INFO_1528 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1529 {
    pub sv1529_minrcvqueue: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1529 {}
impl ::core::clone::Clone for SERVER_INFO_1529 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1529 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1529 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1529>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1529 {}
impl ::core::default::Default for SERVER_INFO_1529 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1530 {
    pub sv1530_minfreeworkitems: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1530 {}
impl ::core::clone::Clone for SERVER_INFO_1530 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1530 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1530 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1530>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1530 {}
impl ::core::default::Default for SERVER_INFO_1530 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1533 {
    pub sv1533_maxmpxct: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1533 {}
impl ::core::clone::Clone for SERVER_INFO_1533 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1533 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1533 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1533>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1533 {}
impl ::core::default::Default for SERVER_INFO_1533 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1534 {
    pub sv1534_oplockbreakwait: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1534 {}
impl ::core::clone::Clone for SERVER_INFO_1534 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1534 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1534 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1534>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1534 {}
impl ::core::default::Default for SERVER_INFO_1534 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1535 {
    pub sv1535_oplockbreakresponsewait: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1535 {}
impl ::core::clone::Clone for SERVER_INFO_1535 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1535 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1535 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1535>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1535 {}
impl ::core::default::Default for SERVER_INFO_1535 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1536 {
    pub sv1536_enableoplocks: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1536 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1536 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1536 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1536 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1536>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1536 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1536 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1537 {
    pub sv1537_enableoplockforceclose: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1537 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1537 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1537 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1537 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1537>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1537 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1537 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1538 {
    pub sv1538_enablefcbopens: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1538 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1538 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1538 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1538 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1538>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1538 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1538 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1539 {
    pub sv1539_enableraw: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1539 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1539 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1539 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1539 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1539>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1539 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1539 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1540 {
    pub sv1540_enablesharednetdrives: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1540 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1540 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1540 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1540 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1540>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1540 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1540 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1541 {
    pub sv1541_minfreeconnections: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1541 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1541 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1541 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1541 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1541>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1541 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1541 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1542 {
    pub sv1542_maxfreeconnections: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1542 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1542 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1542 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1542 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1542>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1542 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1542 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1543 {
    pub sv1543_initsesstable: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1543 {}
impl ::core::clone::Clone for SERVER_INFO_1543 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1543 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1543 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1543>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1543 {}
impl ::core::default::Default for SERVER_INFO_1543 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1544 {
    pub sv1544_initconntable: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1544 {}
impl ::core::clone::Clone for SERVER_INFO_1544 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1544 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1544 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1544>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1544 {}
impl ::core::default::Default for SERVER_INFO_1544 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1545 {
    pub sv1545_initfiletable: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1545 {}
impl ::core::clone::Clone for SERVER_INFO_1545 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1545 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1545 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1545>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1545 {}
impl ::core::default::Default for SERVER_INFO_1545 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1546 {
    pub sv1546_initsearchtable: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1546 {}
impl ::core::clone::Clone for SERVER_INFO_1546 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1546 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1546 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1546>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1546 {}
impl ::core::default::Default for SERVER_INFO_1546 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1547 {
    pub sv1547_alertschedule: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1547 {}
impl ::core::clone::Clone for SERVER_INFO_1547 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1547 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1547 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1547>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1547 {}
impl ::core::default::Default for SERVER_INFO_1547 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1548 {
    pub sv1548_errorthreshold: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1548 {}
impl ::core::clone::Clone for SERVER_INFO_1548 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1548 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1548 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1548>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1548 {}
impl ::core::default::Default for SERVER_INFO_1548 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1549 {
    pub sv1549_networkerrorthreshold: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1549 {}
impl ::core::clone::Clone for SERVER_INFO_1549 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1549 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1549 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1549>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1549 {}
impl ::core::default::Default for SERVER_INFO_1549 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1550 {
    pub sv1550_diskspacethreshold: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1550 {}
impl ::core::clone::Clone for SERVER_INFO_1550 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1550 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1550 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1550>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1550 {}
impl ::core::default::Default for SERVER_INFO_1550 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1552 {
    pub sv1552_maxlinkdelay: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1552 {}
impl ::core::clone::Clone for SERVER_INFO_1552 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1552 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1552 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1552>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1552 {}
impl ::core::default::Default for SERVER_INFO_1552 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1553 {
    pub sv1553_minlinkthroughput: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1553 {}
impl ::core::clone::Clone for SERVER_INFO_1553 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1553 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1553 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1553>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1553 {}
impl ::core::default::Default for SERVER_INFO_1553 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1554 {
    pub sv1554_linkinfovalidtime: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1554 {}
impl ::core::clone::Clone for SERVER_INFO_1554 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1554 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1554 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1554>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1554 {}
impl ::core::default::Default for SERVER_INFO_1554 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1555 {
    pub sv1555_scavqosinfoupdatetime: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1555 {}
impl ::core::clone::Clone for SERVER_INFO_1555 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1555 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1555 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1555>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1555 {}
impl ::core::default::Default for SERVER_INFO_1555 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1556 {
    pub sv1556_maxworkitemidletime: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1556 {}
impl ::core::clone::Clone for SERVER_INFO_1556 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1556 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1556 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1556>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1556 {}
impl ::core::default::Default for SERVER_INFO_1556 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1557 {
    pub sv1557_maxrawworkitems: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1557 {}
impl ::core::clone::Clone for SERVER_INFO_1557 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1557 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1557 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1557>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1557 {}
impl ::core::default::Default for SERVER_INFO_1557 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1560 {
    pub sv1560_producttype: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1560 {}
impl ::core::clone::Clone for SERVER_INFO_1560 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1560 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1560 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1560>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1560 {}
impl ::core::default::Default for SERVER_INFO_1560 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1561 {
    pub sv1561_serversize: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1561 {}
impl ::core::clone::Clone for SERVER_INFO_1561 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1561 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1561 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1561>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1561 {}
impl ::core::default::Default for SERVER_INFO_1561 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1562 {
    pub sv1562_connectionlessautodisc: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1562 {}
impl ::core::clone::Clone for SERVER_INFO_1562 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1562 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1562 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1562>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1562 {}
impl ::core::default::Default for SERVER_INFO_1562 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1563 {
    pub sv1563_sharingviolationretries: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1563 {}
impl ::core::clone::Clone for SERVER_INFO_1563 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1563 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1563 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1563>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1563 {}
impl ::core::default::Default for SERVER_INFO_1563 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1564 {
    pub sv1564_sharingviolationdelay: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1564 {}
impl ::core::clone::Clone for SERVER_INFO_1564 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1564 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1564 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1564>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1564 {}
impl ::core::default::Default for SERVER_INFO_1564 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1565 {
    pub sv1565_maxglobalopensearch: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1565 {}
impl ::core::clone::Clone for SERVER_INFO_1565 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1565 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1565 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1565>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1565 {}
impl ::core::default::Default for SERVER_INFO_1565 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1566 {
    pub sv1566_removeduplicatesearches: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1566 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1566 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1566 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1566 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1566>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1566 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1566 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1567 {
    pub sv1567_lockviolationretries: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1567 {}
impl ::core::clone::Clone for SERVER_INFO_1567 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1567 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1567 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1567>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1567 {}
impl ::core::default::Default for SERVER_INFO_1567 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1568 {
    pub sv1568_lockviolationoffset: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1568 {}
impl ::core::clone::Clone for SERVER_INFO_1568 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1568 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1568 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1568>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1568 {}
impl ::core::default::Default for SERVER_INFO_1568 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1569 {
    pub sv1569_lockviolationdelay: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1569 {}
impl ::core::clone::Clone for SERVER_INFO_1569 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1569 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1569 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1569>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1569 {}
impl ::core::default::Default for SERVER_INFO_1569 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1570 {
    pub sv1570_mdlreadswitchover: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1570 {}
impl ::core::clone::Clone for SERVER_INFO_1570 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1570 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1570 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1570>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1570 {}
impl ::core::default::Default for SERVER_INFO_1570 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1571 {
    pub sv1571_cachedopenlimit: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1571 {}
impl ::core::clone::Clone for SERVER_INFO_1571 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1571 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1571 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1571>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1571 {}
impl ::core::default::Default for SERVER_INFO_1571 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1572 {
    pub sv1572_criticalthreads: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1572 {}
impl ::core::clone::Clone for SERVER_INFO_1572 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1572 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1572 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1572>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1572 {}
impl ::core::default::Default for SERVER_INFO_1572 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1573 {
    pub sv1573_restrictnullsessaccess: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1573 {}
impl ::core::clone::Clone for SERVER_INFO_1573 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1573 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1573 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1573>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1573 {}
impl ::core::default::Default for SERVER_INFO_1573 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1574 {
    pub sv1574_enablewfw311directipx: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1574 {}
impl ::core::clone::Clone for SERVER_INFO_1574 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1574 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1574 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1574>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1574 {}
impl ::core::default::Default for SERVER_INFO_1574 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1575 {
    pub sv1575_otherqueueaffinity: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1575 {}
impl ::core::clone::Clone for SERVER_INFO_1575 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1575 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1575 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1575>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1575 {}
impl ::core::default::Default for SERVER_INFO_1575 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1576 {
    pub sv1576_queuesamplesecs: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1576 {}
impl ::core::clone::Clone for SERVER_INFO_1576 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1576 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1576 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1576>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1576 {}
impl ::core::default::Default for SERVER_INFO_1576 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1577 {
    pub sv1577_balancecount: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1577 {}
impl ::core::clone::Clone for SERVER_INFO_1577 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1577 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1577 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1577>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1577 {}
impl ::core::default::Default for SERVER_INFO_1577 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1578 {
    pub sv1578_preferredaffinity: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1578 {}
impl ::core::clone::Clone for SERVER_INFO_1578 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1578 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1578 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1578>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1578 {}
impl ::core::default::Default for SERVER_INFO_1578 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1579 {
    pub sv1579_maxfreerfcbs: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1579 {}
impl ::core::clone::Clone for SERVER_INFO_1579 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1579 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1579 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1579>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1579 {}
impl ::core::default::Default for SERVER_INFO_1579 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1580 {
    pub sv1580_maxfreemfcbs: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1580 {}
impl ::core::clone::Clone for SERVER_INFO_1580 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1580 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1580 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1580>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1580 {}
impl ::core::default::Default for SERVER_INFO_1580 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1581 {
    pub sv1581_maxfreemlcbs: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1581 {}
impl ::core::clone::Clone for SERVER_INFO_1581 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1581 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1581 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1581>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1581 {}
impl ::core::default::Default for SERVER_INFO_1581 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1582 {
    pub sv1582_maxfreepagedpoolchunks: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1582 {}
impl ::core::clone::Clone for SERVER_INFO_1582 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1582 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1582 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1582>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1582 {}
impl ::core::default::Default for SERVER_INFO_1582 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1583 {
    pub sv1583_minpagedpoolchunksize: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1583 {}
impl ::core::clone::Clone for SERVER_INFO_1583 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1583 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1583 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1583>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1583 {}
impl ::core::default::Default for SERVER_INFO_1583 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1584 {
    pub sv1584_maxpagedpoolchunksize: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1584 {}
impl ::core::clone::Clone for SERVER_INFO_1584 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1584 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1584 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1584>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1584 {}
impl ::core::default::Default for SERVER_INFO_1584 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1585 {
    pub sv1585_sendsfrompreferredprocessor: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1585 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1585 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1585 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1585 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1585>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1585 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1585 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1586 {
    pub sv1586_maxthreadsperqueue: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1586 {}
impl ::core::clone::Clone for SERVER_INFO_1586 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1586 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1586 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1586>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1586 {}
impl ::core::default::Default for SERVER_INFO_1586 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1587 {
    pub sv1587_cacheddirectorylimit: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1587 {}
impl ::core::clone::Clone for SERVER_INFO_1587 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1587 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1587 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1587>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1587 {}
impl ::core::default::Default for SERVER_INFO_1587 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1588 {
    pub sv1588_maxcopylength: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1588 {}
impl ::core::clone::Clone for SERVER_INFO_1588 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1588 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1588 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1588>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1588 {}
impl ::core::default::Default for SERVER_INFO_1588 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1590 {
    pub sv1590_enablecompression: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1590 {}
impl ::core::clone::Clone for SERVER_INFO_1590 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1590 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1590 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1590>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1590 {}
impl ::core::default::Default for SERVER_INFO_1590 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1591 {
    pub sv1591_autosharewks: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1591 {}
impl ::core::clone::Clone for SERVER_INFO_1591 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1591 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1591 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1591>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1591 {}
impl ::core::default::Default for SERVER_INFO_1591 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1592 {
    pub sv1592_autosharewks: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1592 {}
impl ::core::clone::Clone for SERVER_INFO_1592 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1592 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1592 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1592>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1592 {}
impl ::core::default::Default for SERVER_INFO_1592 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1593 {
    pub sv1593_enablesecuritysignature: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1593 {}
impl ::core::clone::Clone for SERVER_INFO_1593 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1593 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1593 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1593>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1593 {}
impl ::core::default::Default for SERVER_INFO_1593 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1594 {
    pub sv1594_requiresecuritysignature: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1594 {}
impl ::core::clone::Clone for SERVER_INFO_1594 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1594 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1594 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1594>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1594 {}
impl ::core::default::Default for SERVER_INFO_1594 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1595 {
    pub sv1595_minclientbuffersize: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1595 {}
impl ::core::clone::Clone for SERVER_INFO_1595 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1595 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1595 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1595>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1595 {}
impl ::core::default::Default for SERVER_INFO_1595 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1596 {
    pub sv1596_ConnectionNoSessionsTimeout: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1596 {}
impl ::core::clone::Clone for SERVER_INFO_1596 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1596 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1596 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1596>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1596 {}
impl ::core::default::Default for SERVER_INFO_1596 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1597 {
    pub sv1597_IdleThreadTimeOut: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1597 {}
impl ::core::clone::Clone for SERVER_INFO_1597 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1597 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1597 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1597>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1597 {}
impl ::core::default::Default for SERVER_INFO_1597 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1598 {
    pub sv1598_enableW9xsecuritysignature: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1598 {}
impl ::core::clone::Clone for SERVER_INFO_1598 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1598 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1598 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1598>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1598 {}
impl ::core::default::Default for SERVER_INFO_1598 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1599 {
    pub sv1598_enforcekerberosreauthentication: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1599 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1599 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1599 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1599 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1599>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1599 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1599 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1600 {
    pub sv1598_disabledos: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1600 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1600 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1600 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1600>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1600 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SERVER_INFO_1601 {
    pub sv1598_lowdiskspaceminimum: u32,
}
impl ::core::marker::Copy for SERVER_INFO_1601 {}
impl ::core::clone::Clone for SERVER_INFO_1601 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVER_INFO_1601 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVER_INFO_1601 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1601>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1601 {}
impl ::core::default::Default for SERVER_INFO_1601 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_1602 {
    pub sv_1598_disablestrictnamechecking: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_1602 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_1602 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_1602 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1602 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_1602>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1602 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1602 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_402 {
    pub sv402_ulist_mtime: u32,
    pub sv402_glist_mtime: u32,
    pub sv402_alist_mtime: u32,
    pub sv402_alerts: super::super::Foundation::PWSTR,
    pub sv402_security: SERVER_INFO_SECURITY,
    pub sv402_numadmin: u32,
    pub sv402_lanmask: u32,
    pub sv402_guestacct: super::super::Foundation::PWSTR,
    pub sv402_chdevs: u32,
    pub sv402_chdevq: u32,
    pub sv402_chdevjobs: u32,
    pub sv402_connections: u32,
    pub sv402_shares: u32,
    pub sv402_openfiles: u32,
    pub sv402_sessopens: u32,
    pub sv402_sessvcs: u32,
    pub sv402_sessreqs: u32,
    pub sv402_opensearch: u32,
    pub sv402_activelocks: u32,
    pub sv402_numreqbuf: u32,
    pub sv402_sizreqbuf: u32,
    pub sv402_numbigbuf: u32,
    pub sv402_numfiletasks: u32,
    pub sv402_alertsched: u32,
    pub sv402_erroralert: u32,
    pub sv402_logonalert: u32,
    pub sv402_accessalert: u32,
    pub sv402_diskalert: u32,
    pub sv402_netioalert: u32,
    pub sv402_maxauditsz: u32,
    pub sv402_srvheuristics: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_402 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_402 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_402 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_402 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_402>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_402 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_402 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_403 {
    pub sv403_ulist_mtime: u32,
    pub sv403_glist_mtime: u32,
    pub sv403_alist_mtime: u32,
    pub sv403_alerts: super::super::Foundation::PWSTR,
    pub sv403_security: SERVER_INFO_SECURITY,
    pub sv403_numadmin: u32,
    pub sv403_lanmask: u32,
    pub sv403_guestacct: super::super::Foundation::PWSTR,
    pub sv403_chdevs: u32,
    pub sv403_chdevq: u32,
    pub sv403_chdevjobs: u32,
    pub sv403_connections: u32,
    pub sv403_shares: u32,
    pub sv403_openfiles: u32,
    pub sv403_sessopens: u32,
    pub sv403_sessvcs: u32,
    pub sv403_sessreqs: u32,
    pub sv403_opensearch: u32,
    pub sv403_activelocks: u32,
    pub sv403_numreqbuf: u32,
    pub sv403_sizreqbuf: u32,
    pub sv403_numbigbuf: u32,
    pub sv403_numfiletasks: u32,
    pub sv403_alertsched: u32,
    pub sv403_erroralert: u32,
    pub sv403_logonalert: u32,
    pub sv403_accessalert: u32,
    pub sv403_diskalert: u32,
    pub sv403_netioalert: u32,
    pub sv403_maxauditsz: u32,
    pub sv403_srvheuristics: super::super::Foundation::PWSTR,
    pub sv403_auditedevents: u32,
    pub sv403_autoprofile: u32,
    pub sv403_autopath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_403 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_403 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_403 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_403 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_403>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_403 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_403 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_502 {
    pub sv502_sessopens: u32,
    pub sv502_sessvcs: u32,
    pub sv502_opensearch: u32,
    pub sv502_sizreqbuf: u32,
    pub sv502_initworkitems: u32,
    pub sv502_maxworkitems: u32,
    pub sv502_rawworkitems: u32,
    pub sv502_irpstacksize: u32,
    pub sv502_maxrawbuflen: u32,
    pub sv502_sessusers: u32,
    pub sv502_sessconns: u32,
    pub sv502_maxpagedmemoryusage: u32,
    pub sv502_maxnonpagedmemoryusage: u32,
    pub sv502_enablesoftcompat: super::super::Foundation::BOOL,
    pub sv502_enableforcedlogoff: super::super::Foundation::BOOL,
    pub sv502_timesource: super::super::Foundation::BOOL,
    pub sv502_acceptdownlevelapis: super::super::Foundation::BOOL,
    pub sv502_lmannounce: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_502 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_502 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_502>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_503 {
    pub sv503_sessopens: u32,
    pub sv503_sessvcs: u32,
    pub sv503_opensearch: u32,
    pub sv503_sizreqbuf: u32,
    pub sv503_initworkitems: u32,
    pub sv503_maxworkitems: u32,
    pub sv503_rawworkitems: u32,
    pub sv503_irpstacksize: u32,
    pub sv503_maxrawbuflen: u32,
    pub sv503_sessusers: u32,
    pub sv503_sessconns: u32,
    pub sv503_maxpagedmemoryusage: u32,
    pub sv503_maxnonpagedmemoryusage: u32,
    pub sv503_enablesoftcompat: super::super::Foundation::BOOL,
    pub sv503_enableforcedlogoff: super::super::Foundation::BOOL,
    pub sv503_timesource: super::super::Foundation::BOOL,
    pub sv503_acceptdownlevelapis: super::super::Foundation::BOOL,
    pub sv503_lmannounce: super::super::Foundation::BOOL,
    pub sv503_domain: super::super::Foundation::PWSTR,
    pub sv503_maxcopyreadlen: u32,
    pub sv503_maxcopywritelen: u32,
    pub sv503_minkeepsearch: u32,
    pub sv503_maxkeepsearch: u32,
    pub sv503_minkeepcomplsearch: u32,
    pub sv503_maxkeepcomplsearch: u32,
    pub sv503_threadcountadd: u32,
    pub sv503_numblockthreads: u32,
    pub sv503_scavtimeout: u32,
    pub sv503_minrcvqueue: u32,
    pub sv503_minfreeworkitems: u32,
    pub sv503_xactmemsize: u32,
    pub sv503_threadpriority: u32,
    pub sv503_maxmpxct: u32,
    pub sv503_oplockbreakwait: u32,
    pub sv503_oplockbreakresponsewait: u32,
    pub sv503_enableoplocks: super::super::Foundation::BOOL,
    pub sv503_enableoplockforceclose: super::super::Foundation::BOOL,
    pub sv503_enablefcbopens: super::super::Foundation::BOOL,
    pub sv503_enableraw: super::super::Foundation::BOOL,
    pub sv503_enablesharednetdrives: super::super::Foundation::BOOL,
    pub sv503_minfreeconnections: u32,
    pub sv503_maxfreeconnections: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_503 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_503 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_503 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_503 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_503>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_503 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_503 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_598 {
    pub sv598_maxrawworkitems: u32,
    pub sv598_maxthreadsperqueue: u32,
    pub sv598_producttype: u32,
    pub sv598_serversize: u32,
    pub sv598_connectionlessautodisc: u32,
    pub sv598_sharingviolationretries: u32,
    pub sv598_sharingviolationdelay: u32,
    pub sv598_maxglobalopensearch: u32,
    pub sv598_removeduplicatesearches: u32,
    pub sv598_lockviolationoffset: u32,
    pub sv598_lockviolationdelay: u32,
    pub sv598_mdlreadswitchover: u32,
    pub sv598_cachedopenlimit: u32,
    pub sv598_otherqueueaffinity: u32,
    pub sv598_restrictnullsessaccess: super::super::Foundation::BOOL,
    pub sv598_enablewfw311directipx: super::super::Foundation::BOOL,
    pub sv598_queuesamplesecs: u32,
    pub sv598_balancecount: u32,
    pub sv598_preferredaffinity: u32,
    pub sv598_maxfreerfcbs: u32,
    pub sv598_maxfreemfcbs: u32,
    pub sv598_maxfreelfcbs: u32,
    pub sv598_maxfreepagedpoolchunks: u32,
    pub sv598_minpagedpoolchunksize: u32,
    pub sv598_maxpagedpoolchunksize: u32,
    pub sv598_sendsfrompreferredprocessor: super::super::Foundation::BOOL,
    pub sv598_cacheddirectorylimit: u32,
    pub sv598_maxcopylength: u32,
    pub sv598_enablecompression: super::super::Foundation::BOOL,
    pub sv598_autosharewks: super::super::Foundation::BOOL,
    pub sv598_autoshareserver: super::super::Foundation::BOOL,
    pub sv598_enablesecuritysignature: super::super::Foundation::BOOL,
    pub sv598_requiresecuritysignature: super::super::Foundation::BOOL,
    pub sv598_minclientbuffersize: u32,
    pub sv598_serverguid: ::windows::core::GUID,
    pub sv598_ConnectionNoSessionsTimeout: u32,
    pub sv598_IdleThreadTimeOut: u32,
    pub sv598_enableW9xsecuritysignature: super::super::Foundation::BOOL,
    pub sv598_enforcekerberosreauthentication: super::super::Foundation::BOOL,
    pub sv598_disabledos: super::super::Foundation::BOOL,
    pub sv598_lowdiskspaceminimum: u32,
    pub sv598_disablestrictnamechecking: super::super::Foundation::BOOL,
    pub sv598_enableauthenticateusersharing: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_598 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_598 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_598 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_598 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_598>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_598 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_598 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_INFO_599 {
    pub sv599_sessopens: u32,
    pub sv599_sessvcs: u32,
    pub sv599_opensearch: u32,
    pub sv599_sizreqbuf: u32,
    pub sv599_initworkitems: u32,
    pub sv599_maxworkitems: u32,
    pub sv599_rawworkitems: u32,
    pub sv599_irpstacksize: u32,
    pub sv599_maxrawbuflen: u32,
    pub sv599_sessusers: u32,
    pub sv599_sessconns: u32,
    pub sv599_maxpagedmemoryusage: u32,
    pub sv599_maxnonpagedmemoryusage: u32,
    pub sv599_enablesoftcompat: super::super::Foundation::BOOL,
    pub sv599_enableforcedlogoff: super::super::Foundation::BOOL,
    pub sv599_timesource: super::super::Foundation::BOOL,
    pub sv599_acceptdownlevelapis: super::super::Foundation::BOOL,
    pub sv599_lmannounce: super::super::Foundation::BOOL,
    pub sv599_domain: super::super::Foundation::PWSTR,
    pub sv599_maxcopyreadlen: u32,
    pub sv599_maxcopywritelen: u32,
    pub sv599_minkeepsearch: u32,
    pub sv599_maxkeepsearch: u32,
    pub sv599_minkeepcomplsearch: u32,
    pub sv599_maxkeepcomplsearch: u32,
    pub sv599_threadcountadd: u32,
    pub sv599_numblockthreads: u32,
    pub sv599_scavtimeout: u32,
    pub sv599_minrcvqueue: u32,
    pub sv599_minfreeworkitems: u32,
    pub sv599_xactmemsize: u32,
    pub sv599_threadpriority: u32,
    pub sv599_maxmpxct: u32,
    pub sv599_oplockbreakwait: u32,
    pub sv599_oplockbreakresponsewait: u32,
    pub sv599_enableoplocks: super::super::Foundation::BOOL,
    pub sv599_enableoplockforceclose: super::super::Foundation::BOOL,
    pub sv599_enablefcbopens: super::super::Foundation::BOOL,
    pub sv599_enableraw: super::super::Foundation::BOOL,
    pub sv599_enablesharednetdrives: super::super::Foundation::BOOL,
    pub sv599_minfreeconnections: u32,
    pub sv599_maxfreeconnections: u32,
    pub sv599_initsesstable: u32,
    pub sv599_initconntable: u32,
    pub sv599_initfiletable: u32,
    pub sv599_initsearchtable: u32,
    pub sv599_alertschedule: u32,
    pub sv599_errorthreshold: u32,
    pub sv599_networkerrorthreshold: u32,
    pub sv599_diskspacethreshold: u32,
    pub sv599_reserved: u32,
    pub sv599_maxlinkdelay: u32,
    pub sv599_minlinkthroughput: u32,
    pub sv599_linkinfovalidtime: u32,
    pub sv599_scavqosinfoupdatetime: u32,
    pub sv599_maxworkitemidletime: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_INFO_599 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_INFO_599 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_INFO_599 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_599 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_INFO_599>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_599 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_599 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type SERVER_INFO_HIDDEN = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_VISIBLE: SERVER_INFO_HIDDEN = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_HIDDEN: SERVER_INFO_HIDDEN = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type SERVER_INFO_SECURITY = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SHARESECURITY: SERVER_INFO_SECURITY = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_USERSECURITY: SERVER_INFO_SECURITY = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_TRANSPORT_INFO_0 {
    pub svti0_numberofvcs: u32,
    pub svti0_transportname: super::super::Foundation::PWSTR,
    pub svti0_transportaddress: *mut u8,
    pub svti0_transportaddresslength: u32,
    pub svti0_networkaddress: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_TRANSPORT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_TRANSPORT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_TRANSPORT_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_TRANSPORT_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_TRANSPORT_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_TRANSPORT_INFO_1 {
    pub svti1_numberofvcs: u32,
    pub svti1_transportname: super::super::Foundation::PWSTR,
    pub svti1_transportaddress: *mut u8,
    pub svti1_transportaddresslength: u32,
    pub svti1_networkaddress: super::super::Foundation::PWSTR,
    pub svti1_domain: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_TRANSPORT_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_TRANSPORT_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_TRANSPORT_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_TRANSPORT_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_TRANSPORT_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_TRANSPORT_INFO_2 {
    pub svti2_numberofvcs: u32,
    pub svti2_transportname: super::super::Foundation::PWSTR,
    pub svti2_transportaddress: *mut u8,
    pub svti2_transportaddresslength: u32,
    pub svti2_networkaddress: super::super::Foundation::PWSTR,
    pub svti2_domain: super::super::Foundation::PWSTR,
    pub svti2_flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_TRANSPORT_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_TRANSPORT_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_TRANSPORT_INFO_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_TRANSPORT_INFO_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_TRANSPORT_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_TRANSPORT_INFO_3 {
    pub svti3_numberofvcs: u32,
    pub svti3_transportname: super::super::Foundation::PWSTR,
    pub svti3_transportaddress: *mut u8,
    pub svti3_transportaddresslength: u32,
    pub svti3_networkaddress: super::super::Foundation::PWSTR,
    pub svti3_domain: super::super::Foundation::PWSTR,
    pub svti3_flags: u32,
    pub svti3_passwordlength: u32,
    pub svti3_password: [u8; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_TRANSPORT_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_TRANSPORT_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVER_TRANSPORT_INFO_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVER_TRANSPORT_INFO_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_TRANSPORT_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE2_BASE: u32 = 5600u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_ACCOUNT_FLAG_ADD_AGAINST_RODC: i32 = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_ACCOUNT_FLAG_LINK_TO_HOST_ONLY: i32 = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_ACCOUNT_FLAG_REMOVE_OFFLINE: i32 = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_ACCOUNT_FLAG_UNLINK_FROM_HOST_ONLY: i32 = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_BASE: u32 = 3050u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_CCP_CHKPT_NUM: u32 = 255u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_CCP_NO_HINT: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_CCP_QUERY_HINT: u32 = 65536u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_CCP_WAIT_TIME: u32 = 65280u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_CTRL_CONTINUE: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_CTRL_INTERROGATE: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_CTRL_PAUSE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_CTRL_REDIR_COMM: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_CTRL_REDIR_DISK: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_CTRL_REDIR_PRINT: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_CTRL_UNINSTALL: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_INFO_0 {
    pub svci0_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_INFO_1 {
    pub svci1_name: super::super::Foundation::PWSTR,
    pub svci1_status: u32,
    pub svci1_code: u32,
    pub svci1_pid: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_INFO_2 {
    pub svci2_name: super::super::Foundation::PWSTR,
    pub svci2_status: u32,
    pub svci2_code: u32,
    pub svci2_pid: u32,
    pub svci2_text: super::super::Foundation::PWSTR,
    pub svci2_specific_error: u32,
    pub svci2_display_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_INFO_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_INFO_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_INSTALLED: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_INSTALL_PENDING: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_INSTALL_STATE: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_IP_CHKPT_NUM: u32 = 255u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_IP_NO_HINT: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_IP_QUERY_HINT: u32 = 65536u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_IP_WAITTIME_SHIFT: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_IP_WAIT_TIME: u32 = 65280u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_MAXTIME: u32 = 255u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_NOT_PAUSABLE: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_NOT_UNINSTALLABLE: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_NTIP_WAITTIME_SHIFT: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_NT_MAXTIME: u32 = 65535u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_PAUSABLE: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_PAUSE_STATE: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_REDIR_COMM_PAUSED: u32 = 1024u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_REDIR_DISK_PAUSED: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_REDIR_PAUSED: u32 = 1792u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_REDIR_PRINT_PAUSED: u32 = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_RESRV_MASK: u32 = 131071u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_AMBIGPARM: u32 = 3058u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_BADPARMVAL: u32 = 3051u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_CONFIG: u32 = 3055u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_CONFLPARM: u32 = 3063u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_DUPPARM: u32 = 3059u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_EXEC: u32 = 3061u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_FILE: u32 = 3064u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_INTERNAL: u32 = 3057u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_KILL: u32 = 3060u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_MISSPARM: u32 = 3052u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_ADDPAK: u32 = 3090u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_ANNOUNCE: u32 = 3083u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_DATABASE_ERROR: u32 = 5602u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_DISK: u32 = 3071u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_ERRLOG: u32 = 3088u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_FILES: u32 = 3079u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_FILE_UW: u32 = 3089u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_LANGROUP: u32 = 3081u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_LANROOT: u32 = 3075u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_LAZY: u32 = 3091u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_LOGS: u32 = 3080u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_LSA_MACHINE_ACCT: u32 = 5601u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_MEMORY: u32 = 3070u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_MSGNAME: u32 = 3082u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_NETLOGON_AUTH: u32 = 3098u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_NETLOGON_DC_CFLCT: u32 = 3097u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_NETLOGON_MPATH: u32 = 5600u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_NETLOGON_NO_DC: u32 = 3096u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_NULL: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_PROCESSES: u32 = 3073u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_REDIR: u32 = 3076u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_SECURITY: u32 = 3074u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_SEC_FILE_ERR: u32 = 3078u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_SERVER: u32 = 3077u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_SERVER_SEC_ERR: u32 = 3085u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_THREADS: u32 = 3072u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_UAS: u32 = 3084u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_UAS_INVALID_ROLE: u32 = 3095u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_UAS_MACHINE_ACCT: u32 = 3092u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_UAS_PROLOG: u32 = 3099u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_UAS_SERVERS_NMEMB: u32 = 3093u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_UAS_SERVERS_NOGRP: u32 = 3094u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_M_WKSTA: u32 = 3087u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_NORMAL: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_RESOURCE: u32 = 3054u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_SUBSERV: u32 = 3062u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_SYSTEM: u32 = 3056u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UIC_UNKPARM: u32 = 3053u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UNINSTALLABLE: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UNINSTALLED: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SERVICE_UNINSTALL_PENDING: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SESSION_CRYPT_KLEN: u32 = 21u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SESSION_PWLEN: u32 = 24u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SHPWLEN: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SMB_COMPRESSION_INFO {
    pub Switch: super::super::Foundation::BOOLEAN,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SMB_COMPRESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SMB_COMPRESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SMB_COMPRESSION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SMB_COMPRESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SMB_COMPRESSION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SMB_COMPRESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SMB_COMPRESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SMB_TREE_CONNECT_PARAMETERS {
    pub EABufferOffset: u32,
    pub EABufferLen: u32,
    pub CreateOptions: u32,
    pub TreeConnectAttributes: u32,
}
impl ::core::marker::Copy for SMB_TREE_CONNECT_PARAMETERS {}
impl ::core::clone::Clone for SMB_TREE_CONNECT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SMB_TREE_CONNECT_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SMB_TREE_CONNECT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SMB_TREE_CONNECT_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SMB_TREE_CONNECT_PARAMETERS {}
impl ::core::default::Default for SMB_TREE_CONNECT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for SMB_USE_OPTION_COMPRESSION_PARAMETERS {}
impl ::core::clone::Clone for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SMB_USE_OPTION_COMPRESSION_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SMB_USE_OPTION_COMPRESSION_PARAMETERS {}
impl ::core::default::Default for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SNLEN: u32 = 80u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SRV_HASH_GENERATION_ACTIVE: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SRV_SUPPORT_HASH_GENERATION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct STD_ALERT {
    pub alrt_timestamp: u32,
    pub alrt_eventname: [u16; 17],
    pub alrt_servicename: [u16; 81],
}
impl ::core::marker::Copy for STD_ALERT {}
impl ::core::clone::Clone for STD_ALERT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for STD_ALERT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STD_ALERT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STD_ALERT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STD_ALERT {}
impl ::core::default::Default for STD_ALERT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const STXTLEN: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SUPPORTS_ANY: i32 = -1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type SUPPORTS_BINDING_INTERFACE_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_LOWER: SUPPORTS_BINDING_INTERFACE_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const NCF_UPPER: SUPPORTS_BINDING_INTERFACE_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVAUD_BADNETLOGON: u32 = 384u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVAUD_BADSESSLOGON: u32 = 24u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVAUD_BADUSE: u32 = 6144u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVAUD_GOODNETLOGON: u32 = 96u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVAUD_GOODSESSLOGON: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVAUD_GOODUSE: u32 = 1536u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVAUD_LOGONLIM: u32 = 65536u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVAUD_PERMISSIONS: u32 = 16384u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVAUD_RESOURCE: u32 = 32768u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVAUD_SERVICE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVAUD_USERLIST: u32 = 8192u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVI1_NUM_ELEMENTS: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVI2_NUM_ELEMENTS: u32 = 40u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVI3_NUM_ELEMENTS: u32 = 44u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVTI2_CLUSTER_DNN_NAME: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVTI2_CLUSTER_NAME: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVTI2_REMAP_PIPE_NAMES: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVTI2_RESERVED1: u32 = 4096u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVTI2_RESERVED2: u32 = 8192u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVTI2_RESERVED3: u32 = 16384u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVTI2_SCOPED_NAME: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SVTI2_UNICODE_TRANSPORT_ADDRESS: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ACCEPTDOWNLEVELAPIS_PARMNUM: u32 = 517u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ACCESSALERT_PARMNUM: u32 = 40u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ACTIVELOCKS_PARMNUM: u32 = 419u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ALERTSCHEDULE_PARMNUM: u32 = 547u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ALERTSCHED_PARMNUM: u32 = 37u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ALERTS_PARMNUM: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ALIST_MTIME_PARMNUM: u32 = 403u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ANNDELTA_PARMNUM: u32 = 18u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ANNOUNCE_PARMNUM: u32 = 17u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_AUTOSHARESERVER_PARMNUM: u32 = 592u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_AUTOSHAREWKS_PARMNUM: u32 = 591u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_BALANCECOUNT_PARMNUM: u32 = 577u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_CACHEDDIRECTORYLIMIT_PARMNUM: u32 = 587u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_CACHEDOPENLIMIT_PARMNUM: u32 = 571u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_CHDEVJOBS_PARMNUM: u32 = 411u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_CHDEVQ_PARMNUM: u32 = 410u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_COMMENT_PARMNUM: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_CONNECTIONLESSAUTODISC_PARMNUM: u32 = 562u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_CONNECTIONNOSESSIONSTIMEOUT_PARMNUM: u32 = 596u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_CONNECTIONS_PARMNUM: u32 = 412u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_CRITICALTHREADS_PARMNUM: u32 = 572u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_DISABLEDOS_PARMNUM: u32 = 600u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_DISABLESTRICTNAMECHECKING_PARMNUM: u32 = 602u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_DISC_PARMNUM: u32 = 10u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_DISKALERT_PARMNUM: u32 = 41u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_DISKSPACETHRESHOLD_PARMNUM: u32 = 550u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_DOMAIN_PARMNUM: u32 = 519u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLEAUTHENTICATEUSERSHARING_PARMNUM: u32 = 603u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLECOMPRESSION_PARMNUM: u32 = 590u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLEFCBOPENS_PARMNUM: u32 = 538u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLEFORCEDLOGOFF_PARMNUM: u32 = 515u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLEOPLOCKFORCECLOSE_PARMNUM: u32 = 537u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLEOPLOCKS_PARMNUM: u32 = 536u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLERAW_PARMNUM: u32 = 539u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLESECURITYSIGNATURE_PARMNUM: u32 = 593u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLESHAREDNETDRIVES_PARMNUM: u32 = 540u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLESOFTCOMPAT_PARMNUM: u32 = 514u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLEW9XSECURITYSIGNATURE_PARMNUM: u32 = 598u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENABLEWFW311DIRECTIPX_PARMNUM: u32 = 574u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ENFORCEKERBEROSREAUTHENTICATION_PARMNUM: u32 = 599u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ERRORALERT_PARMNUM: u32 = 38u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ERRORTHRESHOLD_PARMNUM: u32 = 548u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_GLIST_MTIME_PARMNUM: u32 = 402u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_GUESTACC_PARMNUM: u32 = 408u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_HIDDEN_PARMNUM: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_IDLETHREADTIMEOUT_PARMNUM: u32 = 597u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_INITCONNTABLE_PARMNUM: u32 = 544u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_INITFILETABLE_PARMNUM: u32 = 545u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_INITSEARCHTABLE_PARMNUM: u32 = 546u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_INITSESSTABLE_PARMNUM: u32 = 543u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_INITWORKITEMS_PARMNUM: u32 = 505u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_IRPSTACKSIZE_PARMNUM: u32 = 508u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_LANMASK_PARMNUM: u32 = 407u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_LINKINFOVALIDTIME_PARMNUM: u32 = 554u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_LMANNOUNCE_PARMNUM: u32 = 518u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_LOCKVIOLATIONDELAY_PARMNUM: u32 = 569u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_LOCKVIOLATIONOFFSET_PARMNUM: u32 = 568u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_LOCKVIOLATIONRETRIES_PARMNUM: u32 = 567u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_LOGONALERT_PARMNUM: u32 = 39u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_LOWDISKSPACEMINIMUM_PARMNUM: u32 = 601u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXAUDITSZ_PARMNUM: u32 = 43u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXCOPYLENGTH_PARMNUM: u32 = 588u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXCOPYREADLEN_PARMNUM: u32 = 520u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXCOPYWRITELEN_PARMNUM: u32 = 521u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXFREECONNECTIONS_PARMNUM: u32 = 542u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXFREELFCBS_PARMNUM: u32 = 581u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXFREEMFCBS_PARMNUM: u32 = 580u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXFREEPAGEDPOOLCHUNKS_PARMNUM: u32 = 582u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXFREERFCBS_PARMNUM: u32 = 579u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXGLOBALOPENSEARCH_PARMNUM: u32 = 565u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXKEEPCOMPLSEARCH_PARMNUM: u32 = 525u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXKEEPSEARCH_PARMNUM: u32 = 523u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXLINKDELAY_PARMNUM: u32 = 552u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXMPXCT_PARMNUM: u32 = 533u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXNONPAGEDMEMORYUSAGE_PARMNUM: u32 = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXPAGEDMEMORYUSAGE_PARMNUM: u32 = 513u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXPAGEDPOOLCHUNKSIZE_PARMNUM: u32 = 584u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXRAWBUFLEN_PARMNUM: u32 = 509u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXRAWWORKITEMS_PARMNUM: u32 = 557u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXTHREADSPERQUEUE_PARMNUM: u32 = 586u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXWORKITEMIDLETIME_PARMNUM: u32 = 556u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAXWORKITEMS_PARMNUM: u32 = 506u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAX_CMD_LEN: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MAX_SRV_HEUR_LEN: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MDLREADSWITCHOVER_PARMNUM: u32 = 570u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MINCLIENTBUFFERSIZE_PARMNUM: u32 = 595u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MINFREECONNECTIONS_PARMNUM: u32 = 541u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MINFREEWORKITEMS_PARMNUM: u32 = 530u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MINKEEPCOMPLSEARCH_PARMNUM: u32 = 524u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MINKEEPSEARCH_PARMNUM: u32 = 522u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MINLINKTHROUGHPUT_PARMNUM: u32 = 553u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MINPAGEDPOOLCHUNKSIZE_PARMNUM: u32 = 583u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_MINRCVQUEUE_PARMNUM: u32 = 529u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_NAME_PARMNUM: u32 = 102u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_NETIOALERT_PARMNUM: u32 = 42u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_NETWORKERRORTHRESHOLD_PARMNUM: u32 = 549u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_NODISC: i32 = -1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_NUMADMIN_PARMNUM: u32 = 406u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_NUMBIGBUF_PARMNUM: u32 = 422u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_NUMBLOCKTHREADS_PARMNUM: u32 = 527u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_NUMFILETASKS_PARMNUM: u32 = 423u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_NUMREQBUF_PARMNUM: u32 = 420u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_OPENFILES_PARMNUM: u32 = 414u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_OPENSEARCH_PARMNUM: u32 = 503u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_OPLOCKBREAKRESPONSEWAIT_PARMNUM: u32 = 535u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_OPLOCKBREAKWAIT_PARMNUM: u32 = 534u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_OTHERQUEUEAFFINITY_PARMNUM: u32 = 575u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_PLATFORM_ID_NT: u32 = 500u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_PLATFORM_ID_OS2: u32 = 400u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_PLATFORM_ID_PARMNUM: u32 = 101u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_PREFERREDAFFINITY_PARMNUM: u32 = 578u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_PRODUCTTYPE_PARMNUM: u32 = 560u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_QUEUESAMPLESECS_PARMNUM: u32 = 576u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_RAWWORKITEMS_PARMNUM: u32 = 507u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_REMOVEDUPLICATESEARCHES_PARMNUM: u32 = 566u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_REQUIRESECURITYSIGNATURE_PARMNUM: u32 = 594u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_RESTRICTNULLSESSACCESS_PARMNUM: u32 = 573u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SCAVQOSINFOUPDATETIME_PARMNUM: u32 = 555u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SCAVTIMEOUT_PARMNUM: u32 = 528u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SECURITY_PARMNUM: u32 = 405u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SENDSFROMPREFERREDPROCESSOR_PARMNUM: u32 = 585u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SERVERSIZE_PARMNUM: u32 = 561u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SESSCONNS_PARMNUM: u32 = 511u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SESSOPENS_PARMNUM: u32 = 501u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SESSREQS_PARMNUM: u32 = 417u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SESSUSERS_PARMNUM: u32 = 510u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SESSVCS_PARMNUM: u32 = 502u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SHARES_PARMNUM: u32 = 413u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SHARINGVIOLATIONDELAY_PARMNUM: u32 = 564u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SHARINGVIOLATIONRETRIES_PARMNUM: u32 = 563u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SIZREQBUF_PARMNUM: u32 = 504u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_SRVHEURISTICS_PARMNUM: u32 = 431u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_THREADCOUNTADD_PARMNUM: u32 = 526u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_THREADPRIORITY_PARMNUM: u32 = 532u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TIMESOURCE_PARMNUM: u32 = 516u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_TYPE_PARMNUM: u32 = 105u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_ULIST_MTIME_PARMNUM: u32 = 401u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_USERPATH_PARMNUM: u32 = 112u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_USERS_PARMNUM: u32 = 107u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_USERS_PER_LICENSE: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_VERSION_MAJOR_PARMNUM: u32 = 103u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_VERSION_MINOR_PARMNUM: u32 = 104u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SV_XACTMEMSIZE_PARMNUM: u32 = 531u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SW_AUTOPROF_LOAD_MASK: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const SW_AUTOPROF_SAVE_MASK: u32 = 2u32;
pub const ServiceAccountPasswordGUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x262e99c9_6160_4871_acec_4e61736b6f21);
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetNetScheduleAccountInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszservername: Param0, pwszaccount: Param1, pwszpassword: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetNetScheduleAccountInformation(pwszservername: super::super::Foundation::PWSTR, pwszaccount: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        SetNetScheduleAccountInformation(pwszservername.into_param().abi(), pwszaccount.into_param().abi(), pwszpassword.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct TIME_OF_DAY_INFO {
    pub tod_elapsedt: u32,
    pub tod_msecs: u32,
    pub tod_hours: u32,
    pub tod_mins: u32,
    pub tod_secs: u32,
    pub tod_hunds: u32,
    pub tod_timezone: i32,
    pub tod_tinterval: u32,
    pub tod_day: u32,
    pub tod_month: u32,
    pub tod_year: u32,
    pub tod_weekday: u32,
}
impl ::core::marker::Copy for TIME_OF_DAY_INFO {}
impl ::core::clone::Clone for TIME_OF_DAY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TIME_OF_DAY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TIME_OF_DAY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TIME_OF_DAY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TIME_OF_DAY_INFO {}
impl ::core::default::Default for TIME_OF_DAY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const TITLE_SC_MESSAGE_BOX: i32 = -1073734795i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const TRACE_NO_STDINFO: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const TRACE_NO_SYNCH: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const TRACE_USE_CONSOLE: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const TRACE_USE_DATE: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const TRACE_USE_FILE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const TRACE_USE_MASK: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const TRACE_USE_MSEC: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRANSPORT_INFO {
    pub Type: TRANSPORT_TYPE,
    pub SkipCertificateCheck: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRANSPORT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRANSPORT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRANSPORT_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRANSPORT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRANSPORT_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRANSPORT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRANSPORT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const TRANSPORT_NAME_PARMNUM: u32 = 202u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const TRANSPORT_QUALITYOFSERVICE_PARMNUM: u32 = 201u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type TRANSPORT_TYPE = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UseTransportType_None: TRANSPORT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UseTransportType_Wsk: TRANSPORT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UseTransportType_Quic: TRANSPORT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn TraceDeregisterA(dwtraceid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceDeregisterA(dwtraceid: u32) -> u32;
        }
        ::core::mem::transmute(TraceDeregisterA(::core::mem::transmute(dwtraceid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn TraceDeregisterExA(dwtraceid: u32, dwflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceDeregisterExA(dwtraceid: u32, dwflags: u32) -> u32;
        }
        ::core::mem::transmute(TraceDeregisterExA(::core::mem::transmute(dwtraceid), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn TraceDeregisterExW(dwtraceid: u32, dwflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceDeregisterExW(dwtraceid: u32, dwflags: u32) -> u32;
        }
        ::core::mem::transmute(TraceDeregisterExW(::core::mem::transmute(dwtraceid), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
#[inline]
pub unsafe fn TraceDeregisterW(dwtraceid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceDeregisterW(dwtraceid: u32) -> u32;
        }
        ::core::mem::transmute(TraceDeregisterW(::core::mem::transmute(dwtraceid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceDumpExA<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: Param5, lpszprefix: Param6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceDumpExA(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: super::super::Foundation::BOOL, lpszprefix: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(TraceDumpExA(::core::mem::transmute(dwtraceid), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpbbytes), ::core::mem::transmute(dwbytecount), ::core::mem::transmute(dwgroupsize), baddressprefix.into_param().abi(), lpszprefix.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceDumpExW<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: Param5, lpszprefix: Param6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceDumpExW(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: super::super::Foundation::BOOL, lpszprefix: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(TraceDumpExW(::core::mem::transmute(dwtraceid), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpbbytes), ::core::mem::transmute(dwbytecount), ::core::mem::transmute(dwgroupsize), baddressprefix.into_param().abi(), lpszprefix.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceGetConsoleA(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceGetConsoleA(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(TraceGetConsoleA(::core::mem::transmute(dwtraceid), ::core::mem::transmute(lphconsole)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceGetConsoleW(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceGetConsoleW(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(TraceGetConsoleW(::core::mem::transmute(dwtraceid), ::core::mem::transmute(lphconsole)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TracePrintfA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(dwtraceid: u32, lpszformat: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TracePrintfA(dwtraceid: u32, lpszformat: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(TracePrintfA(::core::mem::transmute(dwtraceid), lpszformat.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TracePrintfExA<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(dwtraceid: u32, dwflags: u32, lpszformat: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TracePrintfExA(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(TracePrintfExA(::core::mem::transmute(dwtraceid), ::core::mem::transmute(dwflags), lpszformat.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TracePrintfExW<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dwtraceid: u32, dwflags: u32, lpszformat: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TracePrintfExW(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(TracePrintfExW(::core::mem::transmute(dwtraceid), ::core::mem::transmute(dwflags), lpszformat.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TracePrintfW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dwtraceid: u32, lpszformat: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TracePrintfW(dwtraceid: u32, lpszformat: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(TracePrintfW(::core::mem::transmute(dwtraceid), lpszformat.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TracePutsExA<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(dwtraceid: u32, dwflags: u32, lpszstring: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TracePutsExA(dwtraceid: u32, dwflags: u32, lpszstring: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(TracePutsExA(::core::mem::transmute(dwtraceid), ::core::mem::transmute(dwflags), lpszstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TracePutsExW<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dwtraceid: u32, dwflags: u32, lpszstring: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TracePutsExW(dwtraceid: u32, dwflags: u32, lpszstring: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(TracePutsExW(::core::mem::transmute(dwtraceid), ::core::mem::transmute(dwflags), lpszstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceRegisterExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszcallername: Param0, dwflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceRegisterExA(lpszcallername: super::super::Foundation::PSTR, dwflags: u32) -> u32;
        }
        ::core::mem::transmute(TraceRegisterExA(lpszcallername.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceRegisterExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszcallername: Param0, dwflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceRegisterExW(lpszcallername: super::super::Foundation::PWSTR, dwflags: u32) -> u32;
        }
        ::core::mem::transmute(TraceRegisterExW(lpszcallername.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceVprintfExA<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(dwtraceid: u32, dwflags: u32, lpszformat: Param2, arglist: *mut i8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceVprintfExA(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PSTR, arglist: *mut i8) -> u32;
        }
        ::core::mem::transmute(TraceVprintfExA(::core::mem::transmute(dwtraceid), ::core::mem::transmute(dwflags), lpszformat.into_param().abi(), ::core::mem::transmute(arglist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceVprintfExW<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dwtraceid: u32, dwflags: u32, lpszformat: Param2, arglist: *mut i8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceVprintfExW(dwtraceid: u32, dwflags: u32, lpszformat: super::super::Foundation::PWSTR, arglist: *mut i8) -> u32;
        }
        ::core::mem::transmute(TraceVprintfExW(::core::mem::transmute(dwtraceid), ::core::mem::transmute(dwflags), lpszformat.into_param().abi(), ::core::mem::transmute(arglist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_INTERDOMAIN_TRUST_ACCOUNT: u32 = 2048u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_MNS_LOGON_ACCOUNT: u32 = 131072u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_NORMAL_ACCOUNT: u32 = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_NO_AUTH_DATA_REQUIRED: u32 = 33554432u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_PARTIAL_SECRETS_ACCOUNT: u32 = 67108864u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_SERVER_TRUST_ACCOUNT: u32 = 8192u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_TEMP_DUPLICATE_ACCOUNT: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_USE_AES_KEYS: u32 = 134217728u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_WORKSTATION_TRUST_ACCOUNT: u32 = 4096u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UNCLEN: u32 = 17u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UNITS_PER_DAY: u32 = 24u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UNLEN: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UPPER_GET_HINT_MASK: u32 = 267386880u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UPPER_HINT_MASK: u32 = 65280u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type USER_ACCOUNT_FLAGS = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_SCRIPT: USER_ACCOUNT_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_ACCOUNTDISABLE: USER_ACCOUNT_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_HOMEDIR_REQUIRED: USER_ACCOUNT_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_PASSWD_NOTREQD: USER_ACCOUNT_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_PASSWD_CANT_CHANGE: USER_ACCOUNT_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_LOCKOUT: USER_ACCOUNT_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_DONT_EXPIRE_PASSWD: USER_ACCOUNT_FLAGS = 65536u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: USER_ACCOUNT_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_NOT_DELEGATED: USER_ACCOUNT_FLAGS = 1048576u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_SMARTCARD_REQUIRED: USER_ACCOUNT_FLAGS = 262144u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_USE_DES_KEY_ONLY: USER_ACCOUNT_FLAGS = 2097152u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_DONT_REQUIRE_PREAUTH: USER_ACCOUNT_FLAGS = 4194304u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_TRUSTED_FOR_DELEGATION: USER_ACCOUNT_FLAGS = 524288u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_PASSWORD_EXPIRED: USER_ACCOUNT_FLAGS = 8388608u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: USER_ACCOUNT_FLAGS = 16777216u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_ACCT_EXPIRES_PARMNUM: u32 = 17u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_AUTH_FLAGS_PARMNUM: u32 = 10u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_CODE_PAGE_PARMNUM: u32 = 25u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_COMMENT_PARMNUM: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_COUNTRY_CODE_PARMNUM: u32 = 24u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_FLAGS_PARMNUM: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_FULL_NAME_PARMNUM: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_HOME_DIR_DRIVE_PARMNUM: u32 = 53u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_HOME_DIR_PARMNUM: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_0 {
    pub usri0_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1 {
    pub usri1_name: super::super::Foundation::PWSTR,
    pub usri1_password: super::super::Foundation::PWSTR,
    pub usri1_password_age: u32,
    pub usri1_priv: USER_PRIV,
    pub usri1_home_dir: super::super::Foundation::PWSTR,
    pub usri1_comment: super::super::Foundation::PWSTR,
    pub usri1_flags: USER_ACCOUNT_FLAGS,
    pub usri1_script_path: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_10 {
    pub usri10_name: super::super::Foundation::PWSTR,
    pub usri10_comment: super::super::Foundation::PWSTR,
    pub usri10_usr_comment: super::super::Foundation::PWSTR,
    pub usri10_full_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_10 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_10 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_10>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1003 {
    pub usri1003_password: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1003 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1003 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1003 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1003 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1003>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1003 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1003 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_INFO_1005 {
    pub usri1005_priv: USER_PRIV,
}
impl ::core::marker::Copy for USER_INFO_1005 {}
impl ::core::clone::Clone for USER_INFO_1005 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_INFO_1005 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1005>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_INFO_1005 {}
impl ::core::default::Default for USER_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1006 {
    pub usri1006_home_dir: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1006 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1006 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1006 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1006 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1006>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1006 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1006 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1007 {
    pub usri1007_comment: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1007 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1007 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1007 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1007 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1007>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1007 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1007 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_INFO_1008 {
    pub usri1008_flags: USER_ACCOUNT_FLAGS,
}
impl ::core::marker::Copy for USER_INFO_1008 {}
impl ::core::clone::Clone for USER_INFO_1008 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_INFO_1008 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_INFO_1008 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1008>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_INFO_1008 {}
impl ::core::default::Default for USER_INFO_1008 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1009 {
    pub usri1009_script_path: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1009 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1009 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1009 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1009 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1009>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1009 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1009 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_INFO_1010 {
    pub usri1010_auth_flags: AF_OP,
}
impl ::core::marker::Copy for USER_INFO_1010 {}
impl ::core::clone::Clone for USER_INFO_1010 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_INFO_1010 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_INFO_1010 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1010>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_INFO_1010 {}
impl ::core::default::Default for USER_INFO_1010 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1011 {
    pub usri1011_full_name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1011 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1011 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1011 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1011 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1011>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1011 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1011 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1012 {
    pub usri1012_usr_comment: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1012 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1012 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1012 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1012 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1012>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1012 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1012 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1013 {
    pub usri1013_parms: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1013 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1013 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1013 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1013 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1013>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1013 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1013 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1014 {
    pub usri1014_workstations: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1014 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1014 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1014 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1014 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1014>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1014 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1014 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_INFO_1017 {
    pub usri1017_acct_expires: u32,
}
impl ::core::marker::Copy for USER_INFO_1017 {}
impl ::core::clone::Clone for USER_INFO_1017 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_INFO_1017 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_INFO_1017 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1017>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_INFO_1017 {}
impl ::core::default::Default for USER_INFO_1017 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_INFO_1018 {
    pub usri1018_max_storage: u32,
}
impl ::core::marker::Copy for USER_INFO_1018 {}
impl ::core::clone::Clone for USER_INFO_1018 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_INFO_1018 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_INFO_1018 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1018>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_INFO_1018 {}
impl ::core::default::Default for USER_INFO_1018 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_INFO_1020 {
    pub usri1020_units_per_week: u32,
    pub usri1020_logon_hours: *mut u8,
}
impl ::core::marker::Copy for USER_INFO_1020 {}
impl ::core::clone::Clone for USER_INFO_1020 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_INFO_1020 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_INFO_1020 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1020>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_INFO_1020 {}
impl ::core::default::Default for USER_INFO_1020 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1023 {
    pub usri1023_logon_server: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1023 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1023 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1023 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1023 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1023>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1023 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1023 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_INFO_1024 {
    pub usri1024_country_code: u32,
}
impl ::core::marker::Copy for USER_INFO_1024 {}
impl ::core::clone::Clone for USER_INFO_1024 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_INFO_1024 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_INFO_1024 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1024>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_INFO_1024 {}
impl ::core::default::Default for USER_INFO_1024 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_INFO_1025 {
    pub usri1025_code_page: u32,
}
impl ::core::marker::Copy for USER_INFO_1025 {}
impl ::core::clone::Clone for USER_INFO_1025 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_INFO_1025 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_INFO_1025 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1025>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_INFO_1025 {}
impl ::core::default::Default for USER_INFO_1025 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_INFO_1051 {
    pub usri1051_primary_group_id: u32,
}
impl ::core::marker::Copy for USER_INFO_1051 {}
impl ::core::clone::Clone for USER_INFO_1051 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_INFO_1051 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_INFO_1051 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1051>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_INFO_1051 {}
impl ::core::default::Default for USER_INFO_1051 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1052 {
    pub usri1052_profile: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1052 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1052 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1052 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1052 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1052>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1052 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1052 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_1053 {
    pub usri1053_home_dir_drive: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_1053 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_1053 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_1053 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_1053 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_1053>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_1053 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_1053 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_11 {
    pub usri11_name: super::super::Foundation::PWSTR,
    pub usri11_comment: super::super::Foundation::PWSTR,
    pub usri11_usr_comment: super::super::Foundation::PWSTR,
    pub usri11_full_name: super::super::Foundation::PWSTR,
    pub usri11_priv: USER_PRIV,
    pub usri11_auth_flags: AF_OP,
    pub usri11_password_age: u32,
    pub usri11_home_dir: super::super::Foundation::PWSTR,
    pub usri11_parms: super::super::Foundation::PWSTR,
    pub usri11_last_logon: u32,
    pub usri11_last_logoff: u32,
    pub usri11_bad_pw_count: u32,
    pub usri11_num_logons: u32,
    pub usri11_logon_server: super::super::Foundation::PWSTR,
    pub usri11_country_code: u32,
    pub usri11_workstations: super::super::Foundation::PWSTR,
    pub usri11_max_storage: u32,
    pub usri11_units_per_week: u32,
    pub usri11_logon_hours: *mut u8,
    pub usri11_code_page: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_11 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_11 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_11 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_11 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_11>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_11 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_11 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_2 {
    pub usri2_name: super::super::Foundation::PWSTR,
    pub usri2_password: super::super::Foundation::PWSTR,
    pub usri2_password_age: u32,
    pub usri2_priv: USER_PRIV,
    pub usri2_home_dir: super::super::Foundation::PWSTR,
    pub usri2_comment: super::super::Foundation::PWSTR,
    pub usri2_flags: USER_ACCOUNT_FLAGS,
    pub usri2_script_path: super::super::Foundation::PWSTR,
    pub usri2_auth_flags: AF_OP,
    pub usri2_full_name: super::super::Foundation::PWSTR,
    pub usri2_usr_comment: super::super::Foundation::PWSTR,
    pub usri2_parms: super::super::Foundation::PWSTR,
    pub usri2_workstations: super::super::Foundation::PWSTR,
    pub usri2_last_logon: u32,
    pub usri2_last_logoff: u32,
    pub usri2_acct_expires: u32,
    pub usri2_max_storage: u32,
    pub usri2_units_per_week: u32,
    pub usri2_logon_hours: *mut u8,
    pub usri2_bad_pw_count: u32,
    pub usri2_num_logons: u32,
    pub usri2_logon_server: super::super::Foundation::PWSTR,
    pub usri2_country_code: u32,
    pub usri2_code_page: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_20 {
    pub usri20_name: super::super::Foundation::PWSTR,
    pub usri20_full_name: super::super::Foundation::PWSTR,
    pub usri20_comment: super::super::Foundation::PWSTR,
    pub usri20_flags: USER_ACCOUNT_FLAGS,
    pub usri20_user_id: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_20 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_20 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_20 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_20 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_20>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_20 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_20 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_INFO_21 {
    pub usri21_password: [u8; 16],
}
impl ::core::marker::Copy for USER_INFO_21 {}
impl ::core::clone::Clone for USER_INFO_21 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_INFO_21 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_INFO_21 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_21>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_INFO_21 {}
impl ::core::default::Default for USER_INFO_21 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_22 {
    pub usri22_name: super::super::Foundation::PWSTR,
    pub usri22_password: [u8; 16],
    pub usri22_password_age: u32,
    pub usri22_priv: USER_PRIV,
    pub usri22_home_dir: super::super::Foundation::PWSTR,
    pub usri22_comment: super::super::Foundation::PWSTR,
    pub usri22_flags: USER_ACCOUNT_FLAGS,
    pub usri22_script_path: super::super::Foundation::PWSTR,
    pub usri22_auth_flags: AF_OP,
    pub usri22_full_name: super::super::Foundation::PWSTR,
    pub usri22_usr_comment: super::super::Foundation::PWSTR,
    pub usri22_parms: super::super::Foundation::PWSTR,
    pub usri22_workstations: super::super::Foundation::PWSTR,
    pub usri22_last_logon: u32,
    pub usri22_last_logoff: u32,
    pub usri22_acct_expires: u32,
    pub usri22_max_storage: u32,
    pub usri22_units_per_week: u32,
    pub usri22_logon_hours: *mut u8,
    pub usri22_bad_pw_count: u32,
    pub usri22_num_logons: u32,
    pub usri22_logon_server: super::super::Foundation::PWSTR,
    pub usri22_country_code: u32,
    pub usri22_code_page: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_22 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_22 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_22 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_22 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_22>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_22 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_22 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_23 {
    pub usri23_name: super::super::Foundation::PWSTR,
    pub usri23_full_name: super::super::Foundation::PWSTR,
    pub usri23_comment: super::super::Foundation::PWSTR,
    pub usri23_flags: USER_ACCOUNT_FLAGS,
    pub usri23_user_sid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_23 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_23 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_23 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_23 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_23>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_23 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_23 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_24 {
    pub usri24_internet_identity: super::super::Foundation::BOOL,
    pub usri24_flags: u32,
    pub usri24_internet_provider_name: super::super::Foundation::PWSTR,
    pub usri24_internet_principal_name: super::super::Foundation::PWSTR,
    pub usri24_user_sid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_24 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_24 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_24 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_24 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_24>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_24 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_24 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_3 {
    pub usri3_name: super::super::Foundation::PWSTR,
    pub usri3_password: super::super::Foundation::PWSTR,
    pub usri3_password_age: u32,
    pub usri3_priv: USER_PRIV,
    pub usri3_home_dir: super::super::Foundation::PWSTR,
    pub usri3_comment: super::super::Foundation::PWSTR,
    pub usri3_flags: USER_ACCOUNT_FLAGS,
    pub usri3_script_path: super::super::Foundation::PWSTR,
    pub usri3_auth_flags: AF_OP,
    pub usri3_full_name: super::super::Foundation::PWSTR,
    pub usri3_usr_comment: super::super::Foundation::PWSTR,
    pub usri3_parms: super::super::Foundation::PWSTR,
    pub usri3_workstations: super::super::Foundation::PWSTR,
    pub usri3_last_logon: u32,
    pub usri3_last_logoff: u32,
    pub usri3_acct_expires: u32,
    pub usri3_max_storage: u32,
    pub usri3_units_per_week: u32,
    pub usri3_logon_hours: *mut u8,
    pub usri3_bad_pw_count: u32,
    pub usri3_num_logons: u32,
    pub usri3_logon_server: super::super::Foundation::PWSTR,
    pub usri3_country_code: u32,
    pub usri3_code_page: u32,
    pub usri3_user_id: u32,
    pub usri3_primary_group_id: u32,
    pub usri3_profile: super::super::Foundation::PWSTR,
    pub usri3_home_dir_drive: super::super::Foundation::PWSTR,
    pub usri3_password_expired: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_INFO_4 {
    pub usri4_name: super::super::Foundation::PWSTR,
    pub usri4_password: super::super::Foundation::PWSTR,
    pub usri4_password_age: u32,
    pub usri4_priv: USER_PRIV,
    pub usri4_home_dir: super::super::Foundation::PWSTR,
    pub usri4_comment: super::super::Foundation::PWSTR,
    pub usri4_flags: USER_ACCOUNT_FLAGS,
    pub usri4_script_path: super::super::Foundation::PWSTR,
    pub usri4_auth_flags: AF_OP,
    pub usri4_full_name: super::super::Foundation::PWSTR,
    pub usri4_usr_comment: super::super::Foundation::PWSTR,
    pub usri4_parms: super::super::Foundation::PWSTR,
    pub usri4_workstations: super::super::Foundation::PWSTR,
    pub usri4_last_logon: u32,
    pub usri4_last_logoff: u32,
    pub usri4_acct_expires: u32,
    pub usri4_max_storage: u32,
    pub usri4_units_per_week: u32,
    pub usri4_logon_hours: *mut u8,
    pub usri4_bad_pw_count: u32,
    pub usri4_num_logons: u32,
    pub usri4_logon_server: super::super::Foundation::PWSTR,
    pub usri4_country_code: u32,
    pub usri4_code_page: u32,
    pub usri4_user_sid: super::super::Foundation::PSID,
    pub usri4_primary_group_id: u32,
    pub usri4_profile: super::super::Foundation::PWSTR,
    pub usri4_home_dir_drive: super::super::Foundation::PWSTR,
    pub usri4_password_expired: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_INFO_4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_INFO_4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_LAST_LOGOFF_PARMNUM: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_LAST_LOGON_PARMNUM: u32 = 15u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_LOGON_HOURS_PARMNUM: u32 = 20u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_LOGON_SERVER_PARMNUM: u32 = 23u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_MAX_STORAGE_PARMNUM: u32 = 18u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_MODALS_INFO_0 {
    pub usrmod0_min_passwd_len: u32,
    pub usrmod0_max_passwd_age: u32,
    pub usrmod0_min_passwd_age: u32,
    pub usrmod0_force_logoff: u32,
    pub usrmod0_password_hist_len: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_0 {}
impl ::core::clone::Clone for USER_MODALS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_MODALS_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MODALS_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_0 {}
impl ::core::default::Default for USER_MODALS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_MODALS_INFO_1 {
    pub usrmod1_role: u32,
    pub usrmod1_primary: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_MODALS_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_MODALS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_MODALS_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MODALS_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_MODALS_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_MODALS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_MODALS_INFO_1001 {
    pub usrmod1001_min_passwd_len: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1001 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1001 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_MODALS_INFO_1001 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1001 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MODALS_INFO_1001>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1001 {}
impl ::core::default::Default for USER_MODALS_INFO_1001 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_MODALS_INFO_1002 {
    pub usrmod1002_max_passwd_age: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1002 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1002 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_MODALS_INFO_1002 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MODALS_INFO_1002>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1002 {}
impl ::core::default::Default for USER_MODALS_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_MODALS_INFO_1003 {
    pub usrmod1003_min_passwd_age: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1003 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1003 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_MODALS_INFO_1003 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1003 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MODALS_INFO_1003>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1003 {}
impl ::core::default::Default for USER_MODALS_INFO_1003 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_MODALS_INFO_1004 {
    pub usrmod1004_force_logoff: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1004 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1004 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_MODALS_INFO_1004 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1004 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MODALS_INFO_1004>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1004 {}
impl ::core::default::Default for USER_MODALS_INFO_1004 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_MODALS_INFO_1005 {
    pub usrmod1005_password_hist_len: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1005 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1005 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_MODALS_INFO_1005 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MODALS_INFO_1005>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1005 {}
impl ::core::default::Default for USER_MODALS_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_MODALS_INFO_1006 {
    pub usrmod1006_role: USER_MODALS_ROLES,
}
impl ::core::marker::Copy for USER_MODALS_INFO_1006 {}
impl ::core::clone::Clone for USER_MODALS_INFO_1006 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_MODALS_INFO_1006 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1006 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MODALS_INFO_1006>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1006 {}
impl ::core::default::Default for USER_MODALS_INFO_1006 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_MODALS_INFO_1007 {
    pub usrmod1007_primary: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_MODALS_INFO_1007 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_MODALS_INFO_1007 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_MODALS_INFO_1007 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1007 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MODALS_INFO_1007>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_MODALS_INFO_1007 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_MODALS_INFO_1007 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USER_MODALS_INFO_2 {
    pub usrmod2_domain_name: super::super::Foundation::PWSTR,
    pub usrmod2_domain_id: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USER_MODALS_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USER_MODALS_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USER_MODALS_INFO_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_MODALS_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MODALS_INFO_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_MODALS_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_MODALS_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_MODALS_INFO_3 {
    pub usrmod3_lockout_duration: u32,
    pub usrmod3_lockout_observation_window: u32,
    pub usrmod3_lockout_threshold: u32,
}
impl ::core::marker::Copy for USER_MODALS_INFO_3 {}
impl ::core::clone::Clone for USER_MODALS_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_MODALS_INFO_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MODALS_INFO_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_3 {}
impl ::core::default::Default for USER_MODALS_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type USER_MODALS_ROLES = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UAS_ROLE_STANDALONE: USER_MODALS_ROLES = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UAS_ROLE_MEMBER: USER_MODALS_ROLES = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UAS_ROLE_BACKUP: USER_MODALS_ROLES = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const UAS_ROLE_PRIMARY: USER_MODALS_ROLES = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_NAME_PARMNUM: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_NUM_LOGONS_PARMNUM: u32 = 22u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USER_OTHER_INFO {
    pub alrtus_errcode: u32,
    pub alrtus_numstrings: u32,
}
impl ::core::marker::Copy for USER_OTHER_INFO {}
impl ::core::clone::Clone for USER_OTHER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USER_OTHER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_OTHER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_OTHER_INFO {}
impl ::core::default::Default for USER_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PAD_PW_COUNT_PARMNUM: u32 = 21u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PARMS_PARMNUM: u32 = 13u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PASSWORD_AGE_PARMNUM: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PASSWORD_PARMNUM: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PRIMARY_GROUP_PARMNUM: u32 = 51u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type USER_PRIV = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PRIV_GUEST: USER_PRIV = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PRIV_USER: USER_PRIV = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PRIV_ADMIN: USER_PRIV = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PRIV_MASK: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PRIV_PARMNUM: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PROFILE: u32 = 52u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_PROFILE_PARMNUM: u32 = 52u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_SCRIPT_PATH_PARMNUM: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_UNITS_PER_WEEK_PARMNUM: u32 = 19u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_USR_COMMENT_PARMNUM: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USER_WORKSTATIONS_PARMNUM: u32 = 14u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_ASGTYPE_PARMNUM: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_AUTHIDENTITY_PARMNUM: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_CHARDEV: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_CONN: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_DEFAULT_CREDENTIALS: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_DISCONN: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_DOMAINNAME_PARMNUM: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_FLAGS_PARMNUM: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_FLAG_GLOBAL_MAPPING: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USE_INFO_0 {
    pub ui0_local: super::super::Foundation::PWSTR,
    pub ui0_remote: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USE_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USE_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USE_INFO_1 {
    pub ui1_local: super::super::Foundation::PWSTR,
    pub ui1_remote: super::super::Foundation::PWSTR,
    pub ui1_password: super::super::Foundation::PWSTR,
    pub ui1_status: u32,
    pub ui1_asg_type: USE_INFO_ASG_TYPE,
    pub ui1_refcount: u32,
    pub ui1_usecount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USE_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USE_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USE_INFO_2 {
    pub ui2_local: super::super::Foundation::PWSTR,
    pub ui2_remote: super::super::Foundation::PWSTR,
    pub ui2_password: super::super::Foundation::PWSTR,
    pub ui2_status: u32,
    pub ui2_asg_type: USE_INFO_ASG_TYPE,
    pub ui2_refcount: u32,
    pub ui2_usecount: u32,
    pub ui2_username: super::super::Foundation::PWSTR,
    pub ui2_domainname: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USE_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USE_INFO_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USE_INFO_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USE_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USE_INFO_3 {
    pub ui3_ui2: USE_INFO_2,
    pub ui3_flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USE_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USE_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USE_INFO_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USE_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USE_INFO_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USE_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USE_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USE_INFO_4 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: u32,
    pub ui4_auth_identity: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USE_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USE_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USE_INFO_4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USE_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USE_INFO_4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USE_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USE_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USE_INFO_5 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: u32,
    pub ui4_auth_identity: *mut u8,
    pub ui5_security_descriptor_length: u32,
    pub ui5_security_descriptor: *mut u8,
    pub ui5_use_options_length: u32,
    pub ui5_use_options: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USE_INFO_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USE_INFO_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for USE_INFO_5 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USE_INFO_5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USE_INFO_5>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USE_INFO_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USE_INFO_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type USE_INFO_ASG_TYPE = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_WILDCARD: USE_INFO_ASG_TYPE = 4294967295u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_DISKDEV: USE_INFO_ASG_TYPE = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_SPOOLDEV: USE_INFO_ASG_TYPE = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_IPC: USE_INFO_ASG_TYPE = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_LOCAL_PARMNUM: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_NETERR: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_OK: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_OPTIONS_PARMNUM: u32 = 10u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {}
impl ::core::clone::Clone for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USE_OPTION_DEFERRED_CONNECTION_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {}
impl ::core::default::Default for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USE_OPTION_GENERIC {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for USE_OPTION_GENERIC {}
impl ::core::clone::Clone for USE_OPTION_GENERIC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USE_OPTION_GENERIC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USE_OPTION_GENERIC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USE_OPTION_GENERIC>()) == 0 }
    }
}
impl ::core::cmp::Eq for USE_OPTION_GENERIC {}
impl ::core::default::Default for USE_OPTION_GENERIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USE_OPTION_PROPERTIES {
    pub Tag: u32,
    pub pInfo: *mut ::core::ffi::c_void,
    pub Length: usize,
}
impl ::core::marker::Copy for USE_OPTION_PROPERTIES {}
impl ::core::clone::Clone for USE_OPTION_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USE_OPTION_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USE_OPTION_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USE_OPTION_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for USE_OPTION_PROPERTIES {}
impl ::core::default::Default for USE_OPTION_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct USE_OPTION_TRANSPORT_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for USE_OPTION_TRANSPORT_PARAMETERS {}
impl ::core::clone::Clone for USE_OPTION_TRANSPORT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for USE_OPTION_TRANSPORT_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USE_OPTION_TRANSPORT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USE_OPTION_TRANSPORT_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for USE_OPTION_TRANSPORT_PARAMETERS {}
impl ::core::default::Default for USE_OPTION_TRANSPORT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_PASSWORD_PARMNUM: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_PAUSED: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_RECONN: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_REMOTE_PARMNUM: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_SD_PARMNUM: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_SESSLOST: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_SPECIFIC_TRANSPORT: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const USE_USERNAME_PARMNUM: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const VALIDATED_LOGON: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const VALID_LOGOFF: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_BUFFERNAMEDPIPES_PARMNUM: u32 = 51u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_BUFFERREADONLYFILES_PARMNUM: u32 = 59u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_BUFFILESWITHDENYWRITE_PARMNUM: u32 = 58u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_CACHEFILETIMEOUT_PARMNUM: u32 = 47u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_CHARCOUNT_PARMNUM: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_CHARTIME_PARMNUM: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_CHARWAIT_PARMNUM: u32 = 10u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_COMPUTERNAME_PARMNUM: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_DORMANTFILELIMIT_PARMNUM: u32 = 46u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_ERRLOGSZ_PARMNUM: u32 = 27u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_FORCECORECREATEMODE_PARMNUM: u32 = 60u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_100 {
    pub wki100_platform_id: u32,
    pub wki100_computername: super::super::Foundation::PWSTR,
    pub wki100_langroup: super::super::Foundation::PWSTR,
    pub wki100_ver_major: u32,
    pub wki100_ver_minor: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_100 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_100 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_100 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_100 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_100>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_100 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_100 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_101 {
    pub wki101_platform_id: u32,
    pub wki101_computername: super::super::Foundation::PWSTR,
    pub wki101_langroup: super::super::Foundation::PWSTR,
    pub wki101_ver_major: u32,
    pub wki101_ver_minor: u32,
    pub wki101_lanroot: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_101 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_101 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_101 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_101 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_101>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_101 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_101 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1010 {
    pub wki1010_char_wait: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1010 {}
impl ::core::clone::Clone for WKSTA_INFO_1010 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1010 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1010 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1010>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1010 {}
impl ::core::default::Default for WKSTA_INFO_1010 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1011 {
    pub wki1011_collection_time: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1011 {}
impl ::core::clone::Clone for WKSTA_INFO_1011 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1011 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1011 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1011>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1011 {}
impl ::core::default::Default for WKSTA_INFO_1011 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1012 {
    pub wki1012_maximum_collection_count: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1012 {}
impl ::core::clone::Clone for WKSTA_INFO_1012 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1012 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1012 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1012>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1012 {}
impl ::core::default::Default for WKSTA_INFO_1012 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1013 {
    pub wki1013_keep_conn: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1013 {}
impl ::core::clone::Clone for WKSTA_INFO_1013 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1013 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1013 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1013>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1013 {}
impl ::core::default::Default for WKSTA_INFO_1013 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1018 {
    pub wki1018_sess_timeout: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1018 {}
impl ::core::clone::Clone for WKSTA_INFO_1018 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1018 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1018 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1018>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1018 {}
impl ::core::default::Default for WKSTA_INFO_1018 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_102 {
    pub wki102_platform_id: u32,
    pub wki102_computername: super::super::Foundation::PWSTR,
    pub wki102_langroup: super::super::Foundation::PWSTR,
    pub wki102_ver_major: u32,
    pub wki102_ver_minor: u32,
    pub wki102_lanroot: super::super::Foundation::PWSTR,
    pub wki102_logged_on_users: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_102 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_102 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_102 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_102 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_102>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_102 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_102 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1023 {
    pub wki1023_siz_char_buf: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1023 {}
impl ::core::clone::Clone for WKSTA_INFO_1023 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1023 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1023 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1023>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1023 {}
impl ::core::default::Default for WKSTA_INFO_1023 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1027 {
    pub wki1027_errlog_sz: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1027 {}
impl ::core::clone::Clone for WKSTA_INFO_1027 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1027 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1027 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1027>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1027 {}
impl ::core::default::Default for WKSTA_INFO_1027 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1028 {
    pub wki1028_print_buf_time: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1028 {}
impl ::core::clone::Clone for WKSTA_INFO_1028 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1028 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1028 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1028>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1028 {}
impl ::core::default::Default for WKSTA_INFO_1028 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1032 {
    pub wki1032_wrk_heuristics: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1032 {}
impl ::core::clone::Clone for WKSTA_INFO_1032 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1032 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1032 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1032>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1032 {}
impl ::core::default::Default for WKSTA_INFO_1032 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1033 {
    pub wki1033_max_threads: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1033 {}
impl ::core::clone::Clone for WKSTA_INFO_1033 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1033 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1033 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1033>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1033 {}
impl ::core::default::Default for WKSTA_INFO_1033 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1041 {
    pub wki1041_lock_quota: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1041 {}
impl ::core::clone::Clone for WKSTA_INFO_1041 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1041 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1041 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1041>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1041 {}
impl ::core::default::Default for WKSTA_INFO_1041 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1042 {
    pub wki1042_lock_increment: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1042 {}
impl ::core::clone::Clone for WKSTA_INFO_1042 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1042 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1042 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1042>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1042 {}
impl ::core::default::Default for WKSTA_INFO_1042 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1043 {
    pub wki1043_lock_maximum: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1043 {}
impl ::core::clone::Clone for WKSTA_INFO_1043 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1043 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1043 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1043>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1043 {}
impl ::core::default::Default for WKSTA_INFO_1043 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1044 {
    pub wki1044_pipe_increment: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1044 {}
impl ::core::clone::Clone for WKSTA_INFO_1044 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1044 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1044 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1044>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1044 {}
impl ::core::default::Default for WKSTA_INFO_1044 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1045 {
    pub wki1045_pipe_maximum: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1045 {}
impl ::core::clone::Clone for WKSTA_INFO_1045 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1045 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1045 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1045>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1045 {}
impl ::core::default::Default for WKSTA_INFO_1045 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1046 {
    pub wki1046_dormant_file_limit: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1046 {}
impl ::core::clone::Clone for WKSTA_INFO_1046 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1046 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1046 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1046>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1046 {}
impl ::core::default::Default for WKSTA_INFO_1046 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1047 {
    pub wki1047_cache_file_timeout: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1047 {}
impl ::core::clone::Clone for WKSTA_INFO_1047 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1047 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1047 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1047>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1047 {}
impl ::core::default::Default for WKSTA_INFO_1047 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1048 {
    pub wki1048_use_opportunistic_locking: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1048 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1048 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1048 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1048 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1048>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1048 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1048 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1049 {
    pub wki1049_use_unlock_behind: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1049 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1049 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1049 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1049 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1049>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1049 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1049 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1050 {
    pub wki1050_use_close_behind: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1050 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1050 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1050 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1050 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1050>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1050 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1050 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1051 {
    pub wki1051_buf_named_pipes: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1051 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1051 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1051 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1051 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1051>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1051 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1051 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1052 {
    pub wki1052_use_lock_read_unlock: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1052 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1052 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1052 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1052 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1052>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1052 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1052 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1053 {
    pub wki1053_utilize_nt_caching: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1053 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1053 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1053 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1053 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1053>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1053 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1053 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1054 {
    pub wki1054_use_raw_read: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1054 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1054 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1054 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1054 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1054>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1054 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1054 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1055 {
    pub wki1055_use_raw_write: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1055 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1055 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1055 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1055 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1055>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1055 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1055 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1056 {
    pub wki1056_use_write_raw_data: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1056 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1056 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1056 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1056 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1056>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1056 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1056 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1057 {
    pub wki1057_use_encryption: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1057 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1057 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1057 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1057 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1057>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1057 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1057 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1058 {
    pub wki1058_buf_files_deny_write: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1058 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1058 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1058 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1058 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1058>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1058 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1058 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1059 {
    pub wki1059_buf_read_only_files: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1059 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1059 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1059 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1059 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1059>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1059 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1059 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1060 {
    pub wki1060_force_core_create_mode: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1060 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1060 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1060 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1060 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1060>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1060 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1060 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_1061 {
    pub wki1061_use_512_byte_max_transfer: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_1061 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_1061 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_1061 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1061 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1061>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1061 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1061 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub struct WKSTA_INFO_1062 {
    pub wki1062_read_ahead_throughput: u32,
}
impl ::core::marker::Copy for WKSTA_INFO_1062 {}
impl ::core::clone::Clone for WKSTA_INFO_1062 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WKSTA_INFO_1062 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1062 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_1062>()) == 0 }
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1062 {}
impl ::core::default::Default for WKSTA_INFO_1062 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_302 {
    pub wki302_char_wait: u32,
    pub wki302_collection_time: u32,
    pub wki302_maximum_collection_count: u32,
    pub wki302_keep_conn: u32,
    pub wki302_keep_search: u32,
    pub wki302_max_cmds: u32,
    pub wki302_num_work_buf: u32,
    pub wki302_siz_work_buf: u32,
    pub wki302_max_wrk_cache: u32,
    pub wki302_sess_timeout: u32,
    pub wki302_siz_error: u32,
    pub wki302_num_alerts: u32,
    pub wki302_num_services: u32,
    pub wki302_errlog_sz: u32,
    pub wki302_print_buf_time: u32,
    pub wki302_num_char_buf: u32,
    pub wki302_siz_char_buf: u32,
    pub wki302_wrk_heuristics: super::super::Foundation::PWSTR,
    pub wki302_mailslots: u32,
    pub wki302_num_dgram_buf: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_302 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_302 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_302 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_302 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_302>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_302 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_302 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_402 {
    pub wki402_char_wait: u32,
    pub wki402_collection_time: u32,
    pub wki402_maximum_collection_count: u32,
    pub wki402_keep_conn: u32,
    pub wki402_keep_search: u32,
    pub wki402_max_cmds: u32,
    pub wki402_num_work_buf: u32,
    pub wki402_siz_work_buf: u32,
    pub wki402_max_wrk_cache: u32,
    pub wki402_sess_timeout: u32,
    pub wki402_siz_error: u32,
    pub wki402_num_alerts: u32,
    pub wki402_num_services: u32,
    pub wki402_errlog_sz: u32,
    pub wki402_print_buf_time: u32,
    pub wki402_num_char_buf: u32,
    pub wki402_siz_char_buf: u32,
    pub wki402_wrk_heuristics: super::super::Foundation::PWSTR,
    pub wki402_mailslots: u32,
    pub wki402_num_dgram_buf: u32,
    pub wki402_max_threads: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_402 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_402 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_402 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_402 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_402>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_402 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_402 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_INFO_502 {
    pub wki502_char_wait: u32,
    pub wki502_collection_time: u32,
    pub wki502_maximum_collection_count: u32,
    pub wki502_keep_conn: u32,
    pub wki502_max_cmds: u32,
    pub wki502_sess_timeout: u32,
    pub wki502_siz_char_buf: u32,
    pub wki502_max_threads: u32,
    pub wki502_lock_quota: u32,
    pub wki502_lock_increment: u32,
    pub wki502_lock_maximum: u32,
    pub wki502_pipe_increment: u32,
    pub wki502_pipe_maximum: u32,
    pub wki502_cache_file_timeout: u32,
    pub wki502_dormant_file_limit: u32,
    pub wki502_read_ahead_throughput: u32,
    pub wki502_num_mailslot_buffers: u32,
    pub wki502_num_srv_announce_buffers: u32,
    pub wki502_max_illegal_datagram_events: u32,
    pub wki502_illegal_datagram_event_reset_frequency: u32,
    pub wki502_log_election_packets: super::super::Foundation::BOOL,
    pub wki502_use_opportunistic_locking: super::super::Foundation::BOOL,
    pub wki502_use_unlock_behind: super::super::Foundation::BOOL,
    pub wki502_use_close_behind: super::super::Foundation::BOOL,
    pub wki502_buf_named_pipes: super::super::Foundation::BOOL,
    pub wki502_use_lock_read_unlock: super::super::Foundation::BOOL,
    pub wki502_utilize_nt_caching: super::super::Foundation::BOOL,
    pub wki502_use_raw_read: super::super::Foundation::BOOL,
    pub wki502_use_raw_write: super::super::Foundation::BOOL,
    pub wki502_use_write_raw_data: super::super::Foundation::BOOL,
    pub wki502_use_encryption: super::super::Foundation::BOOL,
    pub wki502_buf_files_deny_write: super::super::Foundation::BOOL,
    pub wki502_buf_read_only_files: super::super::Foundation::BOOL,
    pub wki502_force_core_create_mode: super::super::Foundation::BOOL,
    pub wki502_use_512_byte_max_transfer: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_INFO_502 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_INFO_502 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_INFO_502>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_KEEPCONN_PARMNUM: u32 = 13u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_KEEPSEARCH_PARMNUM: u32 = 14u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_LANGROUP_PARMNUM: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_LANROOT_PARMNUM: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_LOCKINCREMENT_PARMNUM: u32 = 42u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_LOCKMAXIMUM_PARMNUM: u32 = 43u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_LOCKQUOTA_PARMNUM: u32 = 41u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_LOGGED_ON_USERS_PARMNUM: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_LOGON_DOMAIN_PARMNUM: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_LOGON_SERVER_PARMNUM: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_MAILSLOTS_PARMNUM: u32 = 30u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_MAXCMDS_PARMNUM: u32 = 15u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_MAXTHREADS_PARMNUM: u32 = 33u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_MAXWRKCACHE_PARMNUM: u32 = 17u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_NUMALERTS_PARMNUM: u32 = 20u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_NUMCHARBUF_PARMNUM: u32 = 22u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_NUMDGRAMBUF_PARMNUM: u32 = 31u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_NUMSERVICES_PARMNUM: u32 = 21u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_NUMWORKBUF_PARMNUM: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_OTH_DOMAINS_PARMNUM: u32 = 101u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_PIPEINCREMENT_PARMNUM: u32 = 44u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_PIPEMAXIMUM_PARMNUM: u32 = 45u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_PLATFORM_ID_PARMNUM: u32 = 100u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_PRINTBUFTIME_PARMNUM: u32 = 28u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_READAHEADTHRUPUT_PARMNUM: u32 = 62u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_SESSTIMEOUT_PARMNUM: u32 = 18u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_SIZCHARBUF_PARMNUM: u32 = 23u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_SIZERROR_PARMNUM: u32 = 19u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_SIZWORKBUF_PARMNUM: u32 = 29u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_TRANSPORT_INFO_0 {
    pub wkti0_quality_of_service: u32,
    pub wkti0_number_of_vcs: u32,
    pub wkti0_transport_name: super::super::Foundation::PWSTR,
    pub wkti0_transport_address: super::super::Foundation::PWSTR,
    pub wkti0_wan_ish: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_TRANSPORT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_TRANSPORT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_TRANSPORT_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_TRANSPORT_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_TRANSPORT_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_TRANSPORT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_TRANSPORT_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_USE512BYTESMAXTRANSFER_PARMNUM: u32 = 61u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_USECLOSEBEHIND_PARMNUM: u32 = 50u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_USEENCRYPTION_PARMNUM: u32 = 57u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_USELOCKANDREADANDUNLOCK_PARMNUM: u32 = 52u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_USEOPPORTUNISTICLOCKING_PARMNUM: u32 = 48u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_USERAWREAD_PARMNUM: u32 = 54u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_USERAWWRITE_PARMNUM: u32 = 55u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_USER_INFO_0 {
    pub wkui0_username: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_USER_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_USER_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_USER_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_USER_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_USER_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_USER_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_USER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_USER_INFO_1 {
    pub wkui1_username: super::super::Foundation::PWSTR,
    pub wkui1_logon_domain: super::super::Foundation::PWSTR,
    pub wkui1_oth_domains: super::super::Foundation::PWSTR,
    pub wkui1_logon_server: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_USER_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_USER_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_USER_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_USER_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_USER_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_USER_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_USER_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WKSTA_USER_INFO_1101 {
    pub wkui1101_oth_domains: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WKSTA_USER_INFO_1101 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WKSTA_USER_INFO_1101 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WKSTA_USER_INFO_1101 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_USER_INFO_1101 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WKSTA_USER_INFO_1101>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_USER_INFO_1101 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_USER_INFO_1101 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_USEUNLOCKBEHIND_PARMNUM: u32 = 49u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_USEWRITERAWWITHDATA_PARMNUM: u32 = 56u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_UTILIZENTCACHING_PARMNUM: u32 = 53u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_VER_MAJOR_PARMNUM: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_VER_MINOR_PARMNUM: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WKSTA_WRKHEURISTICS_PARMNUM: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type WORKERFUNCTION = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_API_ERROR_FAILED_TO_LOAD_SCHEMA: u32 = 34u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_API_ERROR_FAILED_TO_LOAD_XML: u32 = 33u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_API_ERROR_INTERNAL: u32 = 36u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_API_ERROR_NOT_SUPPORTED: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_API_ERROR_XML_VALIDATION_FAILED: u32 = 35u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ALLOWED: u32 = 20u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ALLOWED_KEY_REQUIRED: u32 = 21u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ENABLED_KEY_PROVIDED: u32 = 22u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_EAP_METHOD_NOT_APPLICABLE: u32 = 24u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_EAP_METHOD_REQUIRED: u32 = 23u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_INVALID_AUTH_FOR_CONNECTION_TYPE: u32 = 15u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_INVALID_ENCRYPTION_FOR_AUTHMODE: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_KEY_INDEX_NOT_APPLICABLE: u32 = 19u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_KEY_INDEX_REQUIRED: u32 = 18u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_KEY_REQUIRED: u32 = 17u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_WPA_ENCRYPTION_NOT_SUPPORTED: u32 = 26u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_CONFIG_ERROR_WPA_NOT_SUPPORTED: u32 = 25u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_SET_ERROR_DUPLICATE_NETWORK: u32 = 27u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_SET_ERROR_MEMORY_ALLOCATION: u32 = 28u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_SET_ERROR_READING_1X_CONFIG: u32 = 29u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_SET_ERROR_WRITING_1X_CONFIG: u32 = 30u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_SET_ERROR_WRITING_WZC_CFG: u32 = 31u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_SUCCESS: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_1X_ENABLED: u32 = 10u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_AUTHENTICATION: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_BAD_KEY_INDEX: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_BAD_NETWORK_KEY: u32 = 14u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_BAD_SSID: u32 = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_BAD_VERSION: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_CONNECTION_TYPE: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_EAP_METHOD: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_ENCRYPTION: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_KEY_INDEX_RANGE: u32 = 13u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_KEY_PROVIDED_AUTOMATICALLY: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_NO_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_SSID_NOT_FOUND: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const WZC_PROFILE_XML_ERROR_UNSUPPORTED_VERSION: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub type tagRASCON_IPUI_FLAGS = i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_VPN: tagRASCON_IPUI_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_DEMAND_DIAL: tagRASCON_IPUI_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_NOT_ADMIN: tagRASCON_IPUI_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_USE_IPv4_STATICADDRESS: tagRASCON_IPUI_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_USE_IPv4_NAME_SERVERS: tagRASCON_IPUI_FLAGS = 16i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_USE_IPv4_REMOTE_GATEWAY: tagRASCON_IPUI_FLAGS = 32i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_USE_IPv4_EXPLICIT_METRIC: tagRASCON_IPUI_FLAGS = 64i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_USE_HEADER_COMPRESSION: tagRASCON_IPUI_FLAGS = 128i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_USE_DISABLE_REGISTER_DNS: tagRASCON_IPUI_FLAGS = 256i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_USE_PRIVATE_DNS_SUFFIX: tagRASCON_IPUI_FLAGS = 512i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_ENABLE_NBT: tagRASCON_IPUI_FLAGS = 1024i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_USE_IPv6_STATICADDRESS: tagRASCON_IPUI_FLAGS = 2048i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_USE_IPv6_NAME_SERVERS: tagRASCON_IPUI_FLAGS = 4096i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_USE_IPv6_REMOTE_GATEWAY: tagRASCON_IPUI_FLAGS = 8192i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_USE_IPv6_EXPLICIT_METRIC: tagRASCON_IPUI_FLAGS = 16384i32;
#[doc = "*Required features: 'Win32_NetworkManagement_NetManagement'*"]
pub const RCUIF_DISABLE_CLASS_BASED_ROUTE: tagRASCON_IPUI_FLAGS = 32768i32;
