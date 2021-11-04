#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Security_AppLocker")]
pub mod AppLocker;
#[cfg(feature = "Win32_Security_Authentication")]
pub mod Authentication;
#[cfg(feature = "Win32_Security_Authorization")]
pub mod Authorization;
#[cfg(feature = "Win32_Security_ConfigurationSnapin")]
pub mod ConfigurationSnapin;
#[cfg(feature = "Win32_Security_Credentials")]
pub mod Credentials;
#[cfg(feature = "Win32_Security_Cryptography")]
pub mod Cryptography;
#[cfg(feature = "Win32_Security_DiagnosticDataQuery")]
pub mod DiagnosticDataQuery;
#[cfg(feature = "Win32_Security_DirectoryServices")]
pub mod DirectoryServices;
#[cfg(feature = "Win32_Security_EnterpriseData")]
pub mod EnterpriseData;
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
pub mod ExtensibleAuthenticationProtocol;
#[cfg(feature = "Win32_Security_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_Security_LicenseProtection")]
pub mod LicenseProtection;
#[cfg(feature = "Win32_Security_NetworkAccessProtection")]
pub mod NetworkAccessProtection;
#[cfg(feature = "Win32_Security_Tpm")]
pub mod Tpm;
#[cfg(feature = "Win32_Security_WinTrust")]
pub mod WinTrust;
#[cfg(feature = "Win32_Security_WinWlx")]
pub mod WinWlx;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACCESS_ALLOWED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ACCESS_ALLOWED_ACE {}
impl ::std::default::Default for ACCESS_ALLOWED_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACCESS_ALLOWED_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACCESS_ALLOWED_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_ALLOWED_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_ALLOWED_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_ALLOWED_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACCESS_ALLOWED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ACCESS_ALLOWED_CALLBACK_ACE {}
impl ::std::default::Default for ACCESS_ALLOWED_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACCESS_ALLOWED_CALLBACK_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACCESS_ALLOWED_CALLBACK_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_ALLOWED_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_ALLOWED_CALLBACK_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_ALLOWED_CALLBACK_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::runtime::GUID,
    pub InheritedObjectType: ::windows::runtime::GUID,
    pub SidStart: u32,
}
impl ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {}
impl ::std::default::Default for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACCESS_ALLOWED_CALLBACK_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACCESS_ALLOWED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::runtime::GUID,
    pub InheritedObjectType: ::windows::runtime::GUID,
    pub SidStart: u32,
}
impl ACCESS_ALLOWED_OBJECT_ACE {}
impl ::std::default::Default for ACCESS_ALLOWED_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACCESS_ALLOWED_OBJECT_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACCESS_ALLOWED_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_ALLOWED_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_ALLOWED_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_ALLOWED_OBJECT_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACCESS_DENIED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ACCESS_DENIED_ACE {}
impl ::std::default::Default for ACCESS_DENIED_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACCESS_DENIED_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACCESS_DENIED_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_DENIED_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_DENIED_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_DENIED_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACCESS_DENIED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ACCESS_DENIED_CALLBACK_ACE {}
impl ::std::default::Default for ACCESS_DENIED_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACCESS_DENIED_CALLBACK_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACCESS_DENIED_CALLBACK_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_DENIED_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_DENIED_CALLBACK_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_DENIED_CALLBACK_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::runtime::GUID,
    pub InheritedObjectType: ::windows::runtime::GUID,
    pub SidStart: u32,
}
impl ACCESS_DENIED_CALLBACK_OBJECT_ACE {}
impl ::std::default::Default for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACCESS_DENIED_CALLBACK_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_DENIED_CALLBACK_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACCESS_DENIED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::runtime::GUID,
    pub InheritedObjectType: ::windows::runtime::GUID,
    pub SidStart: u32,
}
impl ACCESS_DENIED_OBJECT_ACE {}
impl ::std::default::Default for ACCESS_DENIED_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACCESS_DENIED_OBJECT_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACCESS_DENIED_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_DENIED_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_DENIED_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_DENIED_OBJECT_ACE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ACE_FLAGS(pub u32);
pub const CONTAINER_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(2u32);
pub const FAILED_ACCESS_ACE_FLAG: ACE_FLAGS = ACE_FLAGS(128u32);
pub const INHERIT_ONLY_ACE: ACE_FLAGS = ACE_FLAGS(8u32);
pub const INHERITED_ACE: ACE_FLAGS = ACE_FLAGS(16u32);
pub const NO_PROPAGATE_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(4u32);
pub const OBJECT_INHERIT_ACE: ACE_FLAGS = ACE_FLAGS(1u32);
pub const SUCCESSFUL_ACCESS_ACE_FLAG: ACE_FLAGS = ACE_FLAGS(64u32);
pub const SUB_CONTAINERS_AND_OBJECTS_INHERIT: ACE_FLAGS = ACE_FLAGS(3u32);
pub const SUB_CONTAINERS_ONLY_INHERIT: ACE_FLAGS = ACE_FLAGS(2u32);
pub const SUB_OBJECTS_ONLY_INHERIT: ACE_FLAGS = ACE_FLAGS(1u32);
pub const INHERIT_NO_PROPAGATE: ACE_FLAGS = ACE_FLAGS(4u32);
pub const INHERIT_ONLY: ACE_FLAGS = ACE_FLAGS(8u32);
pub const NO_INHERITANCE: ACE_FLAGS = ACE_FLAGS(0u32);
pub const INHERIT_ONLY_ACE_: ACE_FLAGS = ACE_FLAGS(8u32);
impl ::std::convert::From<u32> for ACE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACE_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for ACE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ACE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ACE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ACE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACE_HEADER {
    pub AceType: u8,
    pub AceFlags: u8,
    pub AceSize: u16,
}
impl ACE_HEADER {}
impl ::std::default::Default for ACE_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACE_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACE_HEADER").field("AceType", &self.AceType).field("AceFlags", &self.AceFlags).field("AceSize", &self.AceSize).finish()
    }
}
impl ::std::cmp::PartialEq for ACE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.AceType == other.AceType && self.AceFlags == other.AceFlags && self.AceSize == other.AceSize
    }
}
impl ::std::cmp::Eq for ACE_HEADER {}
unsafe impl ::windows::runtime::Abi for ACE_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ACE_REVISION(pub u32);
pub const ACL_REVISION: ACE_REVISION = ACE_REVISION(2u32);
pub const ACL_REVISION_DS: ACE_REVISION = ACE_REVISION(4u32);
impl ::std::convert::From<u32> for ACE_REVISION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACE_REVISION {
    type Abi = Self;
}
impl ::std::ops::BitOr for ACE_REVISION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ACE_REVISION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ACE_REVISION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ACE_REVISION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ACE_REVISION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACL {
    pub AclRevision: u8,
    pub Sbz1: u8,
    pub AclSize: u16,
    pub AceCount: u16,
    pub Sbz2: u16,
}
impl ACL {}
impl ::std::default::Default for ACL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACL").field("AclRevision", &self.AclRevision).field("Sbz1", &self.Sbz1).field("AclSize", &self.AclSize).field("AceCount", &self.AceCount).field("Sbz2", &self.Sbz2).finish()
    }
}
impl ::std::cmp::PartialEq for ACL {
    fn eq(&self, other: &Self) -> bool {
        self.AclRevision == other.AclRevision && self.Sbz1 == other.Sbz1 && self.AclSize == other.AclSize && self.AceCount == other.AceCount && self.Sbz2 == other.Sbz2
    }
}
impl ::std::cmp::Eq for ACL {}
unsafe impl ::windows::runtime::Abi for ACL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ACL_INFORMATION_CLASS(pub i32);
pub const AclRevisionInformation: ACL_INFORMATION_CLASS = ACL_INFORMATION_CLASS(1i32);
pub const AclSizeInformation: ACL_INFORMATION_CLASS = ACL_INFORMATION_CLASS(2i32);
impl ::std::convert::From<i32> for ACL_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACL_INFORMATION_CLASS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACL_REVISION_INFORMATION {
    pub AclRevision: u32,
}
impl ACL_REVISION_INFORMATION {}
impl ::std::default::Default for ACL_REVISION_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACL_REVISION_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACL_REVISION_INFORMATION").field("AclRevision", &self.AclRevision).finish()
    }
}
impl ::std::cmp::PartialEq for ACL_REVISION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AclRevision == other.AclRevision
    }
}
impl ::std::cmp::Eq for ACL_REVISION_INFORMATION {}
unsafe impl ::windows::runtime::Abi for ACL_REVISION_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct ACL_SIZE_INFORMATION {
    pub AceCount: u32,
    pub AclBytesInUse: u32,
    pub AclBytesFree: u32,
}
impl ACL_SIZE_INFORMATION {}
impl ::std::default::Default for ACL_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACL_SIZE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACL_SIZE_INFORMATION").field("AceCount", &self.AceCount).field("AclBytesInUse", &self.AclBytesInUse).field("AclBytesFree", &self.AclBytesFree).finish()
    }
}
impl ::std::cmp::PartialEq for ACL_SIZE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AceCount == other.AceCount && self.AclBytesInUse == other.AclBytesInUse && self.AclBytesFree == other.AclBytesFree
    }
}
impl ::std::cmp::Eq for ACL_SIZE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for ACL_SIZE_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDIT_EVENT_TYPE(pub i32);
pub const AuditEventObjectAccess: AUDIT_EVENT_TYPE = AUDIT_EVENT_TYPE(0i32);
pub const AuditEventDirectoryServiceAccess: AUDIT_EVENT_TYPE = AUDIT_EVENT_TYPE(1i32);
impl ::std::convert::From<i32> for AUDIT_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AUDIT_EVENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheck<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(psecuritydescriptor: *const SECURITY_DESCRIPTOR, clienttoken: Param1, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: *mut PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AccessCheck(psecuritydescriptor: *const SECURITY_DESCRIPTOR, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: *mut PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AccessCheck(
            ::std::mem::transmute(psecuritydescriptor),
            clienttoken.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(genericmapping),
            ::std::mem::transmute(privilegeset),
            ::std::mem::transmute(privilegesetlength),
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(accessstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckAndAuditAlarmA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param7: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    objecttypename: Param2,
    objectname: Param3,
    securitydescriptor: *const SECURITY_DESCRIPTOR,
    desiredaccess: u32,
    genericmapping: *const GENERIC_MAPPING,
    objectcreation: Param7,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
    pfgenerateonclose: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AccessCheckAndAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::std::ffi::c_void, objecttypename: super::Foundation::PSTR, objectname: super::Foundation::PSTR, securitydescriptor: *const SECURITY_DESCRIPTOR, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: super::Foundation::BOOL, grantedaccess: *mut u32, accessstatus: *mut i32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AccessCheckAndAuditAlarmA(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            objecttypename.into_param().abi(),
            objectname.into_param().abi(),
            ::std::mem::transmute(securitydescriptor),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(genericmapping),
            objectcreation.into_param().abi(),
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(accessstatus),
            ::std::mem::transmute(pfgenerateonclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckAndAuditAlarmW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param7: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    objecttypename: Param2,
    objectname: Param3,
    securitydescriptor: *const SECURITY_DESCRIPTOR,
    desiredaccess: u32,
    genericmapping: *const GENERIC_MAPPING,
    objectcreation: Param7,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
    pfgenerateonclose: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AccessCheckAndAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::std::ffi::c_void, objecttypename: super::Foundation::PWSTR, objectname: super::Foundation::PWSTR, securitydescriptor: *const SECURITY_DESCRIPTOR, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: super::Foundation::BOOL, grantedaccess: *mut u32, accessstatus: *mut i32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AccessCheckAndAuditAlarmW(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            objecttypename.into_param().abi(),
            objectname.into_param().abi(),
            ::std::mem::transmute(securitydescriptor),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(genericmapping),
            objectcreation.into_param().abi(),
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(accessstatus),
            ::std::mem::transmute(pfgenerateonclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByType<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
    principalselfsid: Param1,
    clienttoken: Param2,
    desiredaccess: u32,
    objecttypelist: *mut OBJECT_TYPE_LIST,
    objecttypelistlength: u32,
    genericmapping: *const GENERIC_MAPPING,
    privilegeset: *mut PRIVILEGE_SET,
    privilegesetlength: *mut u32,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AccessCheckByType(psecuritydescriptor: *const SECURITY_DESCRIPTOR, principalselfsid: super::Foundation::PSID, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, objecttypelist: *mut OBJECT_TYPE_LIST, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: *mut PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AccessCheckByType(
            ::std::mem::transmute(psecuritydescriptor),
            principalselfsid.into_param().abi(),
            clienttoken.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(objecttypelist),
            ::std::mem::transmute(objecttypelistlength),
            ::std::mem::transmute(genericmapping),
            ::std::mem::transmute(privilegeset),
            ::std::mem::transmute(privilegesetlength),
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(accessstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeAndAuditAlarmA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param12: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    objecttypename: Param2,
    objectname: Param3,
    securitydescriptor: *const SECURITY_DESCRIPTOR,
    principalselfsid: Param5,
    desiredaccess: u32,
    audittype: AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: *mut OBJECT_TYPE_LIST,
    objecttypelistlength: u32,
    genericmapping: *const GENERIC_MAPPING,
    objectcreation: Param12,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
    pfgenerateonclose: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AccessCheckByTypeAndAuditAlarmA(
                subsystemname: super::Foundation::PSTR,
                handleid: *const ::std::ffi::c_void,
                objecttypename: super::Foundation::PSTR,
                objectname: super::Foundation::PSTR,
                securitydescriptor: *const SECURITY_DESCRIPTOR,
                principalselfsid: super::Foundation::PSID,
                desiredaccess: u32,
                audittype: AUDIT_EVENT_TYPE,
                flags: u32,
                objecttypelist: *mut OBJECT_TYPE_LIST,
                objecttypelistlength: u32,
                genericmapping: *const GENERIC_MAPPING,
                objectcreation: super::Foundation::BOOL,
                grantedaccess: *mut u32,
                accessstatus: *mut i32,
                pfgenerateonclose: *mut i32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AccessCheckByTypeAndAuditAlarmA(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            objecttypename.into_param().abi(),
            objectname.into_param().abi(),
            ::std::mem::transmute(securitydescriptor),
            principalselfsid.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(audittype),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(objecttypelist),
            ::std::mem::transmute(objecttypelistlength),
            ::std::mem::transmute(genericmapping),
            objectcreation.into_param().abi(),
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(accessstatus),
            ::std::mem::transmute(pfgenerateonclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeAndAuditAlarmW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param12: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    objecttypename: Param2,
    objectname: Param3,
    securitydescriptor: *const SECURITY_DESCRIPTOR,
    principalselfsid: Param5,
    desiredaccess: u32,
    audittype: AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: *mut OBJECT_TYPE_LIST,
    objecttypelistlength: u32,
    genericmapping: *const GENERIC_MAPPING,
    objectcreation: Param12,
    grantedaccess: *mut u32,
    accessstatus: *mut i32,
    pfgenerateonclose: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AccessCheckByTypeAndAuditAlarmW(
                subsystemname: super::Foundation::PWSTR,
                handleid: *const ::std::ffi::c_void,
                objecttypename: super::Foundation::PWSTR,
                objectname: super::Foundation::PWSTR,
                securitydescriptor: *const SECURITY_DESCRIPTOR,
                principalselfsid: super::Foundation::PSID,
                desiredaccess: u32,
                audittype: AUDIT_EVENT_TYPE,
                flags: u32,
                objecttypelist: *mut OBJECT_TYPE_LIST,
                objecttypelistlength: u32,
                genericmapping: *const GENERIC_MAPPING,
                objectcreation: super::Foundation::BOOL,
                grantedaccess: *mut u32,
                accessstatus: *mut i32,
                pfgenerateonclose: *mut i32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AccessCheckByTypeAndAuditAlarmW(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            objecttypename.into_param().abi(),
            objectname.into_param().abi(),
            ::std::mem::transmute(securitydescriptor),
            principalselfsid.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(audittype),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(objecttypelist),
            ::std::mem::transmute(objecttypelistlength),
            ::std::mem::transmute(genericmapping),
            objectcreation.into_param().abi(),
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(accessstatus),
            ::std::mem::transmute(pfgenerateonclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeResultList<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
    principalselfsid: Param1,
    clienttoken: Param2,
    desiredaccess: u32,
    objecttypelist: *mut OBJECT_TYPE_LIST,
    objecttypelistlength: u32,
    genericmapping: *const GENERIC_MAPPING,
    privilegeset: *mut PRIVILEGE_SET,
    privilegesetlength: *mut u32,
    grantedaccesslist: *mut u32,
    accessstatuslist: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AccessCheckByTypeResultList(psecuritydescriptor: *const SECURITY_DESCRIPTOR, principalselfsid: super::Foundation::PSID, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, objecttypelist: *mut OBJECT_TYPE_LIST, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: *mut PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccesslist: *mut u32, accessstatuslist: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AccessCheckByTypeResultList(
            ::std::mem::transmute(psecuritydescriptor),
            principalselfsid.into_param().abi(),
            clienttoken.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(objecttypelist),
            ::std::mem::transmute(objecttypelistlength),
            ::std::mem::transmute(genericmapping),
            ::std::mem::transmute(privilegeset),
            ::std::mem::transmute(privilegesetlength),
            ::std::mem::transmute(grantedaccesslist),
            ::std::mem::transmute(accessstatuslist),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param12: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    objecttypename: Param2,
    objectname: Param3,
    securitydescriptor: *const SECURITY_DESCRIPTOR,
    principalselfsid: Param5,
    desiredaccess: u32,
    audittype: AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: *mut OBJECT_TYPE_LIST,
    objecttypelistlength: u32,
    genericmapping: *const GENERIC_MAPPING,
    objectcreation: Param12,
    grantedaccess: *mut u32,
    accessstatuslist: *mut u32,
    pfgenerateonclose: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AccessCheckByTypeResultListAndAuditAlarmA(
                subsystemname: super::Foundation::PSTR,
                handleid: *const ::std::ffi::c_void,
                objecttypename: super::Foundation::PSTR,
                objectname: super::Foundation::PSTR,
                securitydescriptor: *const SECURITY_DESCRIPTOR,
                principalselfsid: super::Foundation::PSID,
                desiredaccess: u32,
                audittype: AUDIT_EVENT_TYPE,
                flags: u32,
                objecttypelist: *mut OBJECT_TYPE_LIST,
                objecttypelistlength: u32,
                genericmapping: *const GENERIC_MAPPING,
                objectcreation: super::Foundation::BOOL,
                grantedaccess: *mut u32,
                accessstatuslist: *mut u32,
                pfgenerateonclose: *mut i32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AccessCheckByTypeResultListAndAuditAlarmA(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            objecttypename.into_param().abi(),
            objectname.into_param().abi(),
            ::std::mem::transmute(securitydescriptor),
            principalselfsid.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(audittype),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(objecttypelist),
            ::std::mem::transmute(objecttypelistlength),
            ::std::mem::transmute(genericmapping),
            objectcreation.into_param().abi(),
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(accessstatuslist),
            ::std::mem::transmute(pfgenerateonclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmByHandleA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param13: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    clienttoken: Param2,
    objecttypename: Param3,
    objectname: Param4,
    securitydescriptor: *const SECURITY_DESCRIPTOR,
    principalselfsid: Param6,
    desiredaccess: u32,
    audittype: AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: *mut OBJECT_TYPE_LIST,
    objecttypelistlength: u32,
    genericmapping: *const GENERIC_MAPPING,
    objectcreation: Param13,
    grantedaccess: *mut u32,
    accessstatuslist: *mut u32,
    pfgenerateonclose: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AccessCheckByTypeResultListAndAuditAlarmByHandleA(
                subsystemname: super::Foundation::PSTR,
                handleid: *const ::std::ffi::c_void,
                clienttoken: super::Foundation::HANDLE,
                objecttypename: super::Foundation::PSTR,
                objectname: super::Foundation::PSTR,
                securitydescriptor: *const SECURITY_DESCRIPTOR,
                principalselfsid: super::Foundation::PSID,
                desiredaccess: u32,
                audittype: AUDIT_EVENT_TYPE,
                flags: u32,
                objecttypelist: *mut OBJECT_TYPE_LIST,
                objecttypelistlength: u32,
                genericmapping: *const GENERIC_MAPPING,
                objectcreation: super::Foundation::BOOL,
                grantedaccess: *mut u32,
                accessstatuslist: *mut u32,
                pfgenerateonclose: *mut i32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AccessCheckByTypeResultListAndAuditAlarmByHandleA(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            clienttoken.into_param().abi(),
            objecttypename.into_param().abi(),
            objectname.into_param().abi(),
            ::std::mem::transmute(securitydescriptor),
            principalselfsid.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(audittype),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(objecttypelist),
            ::std::mem::transmute(objecttypelistlength),
            ::std::mem::transmute(genericmapping),
            objectcreation.into_param().abi(),
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(accessstatuslist),
            ::std::mem::transmute(pfgenerateonclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmByHandleW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param13: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    clienttoken: Param2,
    objecttypename: Param3,
    objectname: Param4,
    securitydescriptor: *const SECURITY_DESCRIPTOR,
    principalselfsid: Param6,
    desiredaccess: u32,
    audittype: AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: *mut OBJECT_TYPE_LIST,
    objecttypelistlength: u32,
    genericmapping: *const GENERIC_MAPPING,
    objectcreation: Param13,
    grantedaccesslist: *mut u32,
    accessstatuslist: *mut u32,
    pfgenerateonclose: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(
                subsystemname: super::Foundation::PWSTR,
                handleid: *const ::std::ffi::c_void,
                clienttoken: super::Foundation::HANDLE,
                objecttypename: super::Foundation::PWSTR,
                objectname: super::Foundation::PWSTR,
                securitydescriptor: *const SECURITY_DESCRIPTOR,
                principalselfsid: super::Foundation::PSID,
                desiredaccess: u32,
                audittype: AUDIT_EVENT_TYPE,
                flags: u32,
                objecttypelist: *mut OBJECT_TYPE_LIST,
                objecttypelistlength: u32,
                genericmapping: *const GENERIC_MAPPING,
                objectcreation: super::Foundation::BOOL,
                grantedaccesslist: *mut u32,
                accessstatuslist: *mut u32,
                pfgenerateonclose: *mut i32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AccessCheckByTypeResultListAndAuditAlarmByHandleW(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            clienttoken.into_param().abi(),
            objecttypename.into_param().abi(),
            objectname.into_param().abi(),
            ::std::mem::transmute(securitydescriptor),
            principalselfsid.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(audittype),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(objecttypelist),
            ::std::mem::transmute(objecttypelistlength),
            ::std::mem::transmute(genericmapping),
            objectcreation.into_param().abi(),
            ::std::mem::transmute(grantedaccesslist),
            ::std::mem::transmute(accessstatuslist),
            ::std::mem::transmute(pfgenerateonclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param12: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    objecttypename: Param2,
    objectname: Param3,
    securitydescriptor: *const SECURITY_DESCRIPTOR,
    principalselfsid: Param5,
    desiredaccess: u32,
    audittype: AUDIT_EVENT_TYPE,
    flags: u32,
    objecttypelist: *mut OBJECT_TYPE_LIST,
    objecttypelistlength: u32,
    genericmapping: *const GENERIC_MAPPING,
    objectcreation: Param12,
    grantedaccesslist: *mut u32,
    accessstatuslist: *mut u32,
    pfgenerateonclose: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AccessCheckByTypeResultListAndAuditAlarmW(
                subsystemname: super::Foundation::PWSTR,
                handleid: *const ::std::ffi::c_void,
                objecttypename: super::Foundation::PWSTR,
                objectname: super::Foundation::PWSTR,
                securitydescriptor: *const SECURITY_DESCRIPTOR,
                principalselfsid: super::Foundation::PSID,
                desiredaccess: u32,
                audittype: AUDIT_EVENT_TYPE,
                flags: u32,
                objecttypelist: *mut OBJECT_TYPE_LIST,
                objecttypelistlength: u32,
                genericmapping: *const GENERIC_MAPPING,
                objectcreation: super::Foundation::BOOL,
                grantedaccesslist: *mut u32,
                accessstatuslist: *mut u32,
                pfgenerateonclose: *mut i32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AccessCheckByTypeResultListAndAuditAlarmW(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            objecttypename.into_param().abi(),
            objectname.into_param().abi(),
            ::std::mem::transmute(securitydescriptor),
            principalselfsid.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(audittype),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(objecttypelist),
            ::std::mem::transmute(objecttypelistlength),
            ::std::mem::transmute(genericmapping),
            objectcreation.into_param().abi(),
            ::std::mem::transmute(grantedaccesslist),
            ::std::mem::transmute(accessstatuslist),
            ::std::mem::transmute(pfgenerateonclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessAllowedAce<'a, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(pacl: *mut ACL, dwacerevision: u32, accessmask: u32, psid: Param3) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessAllowedAce(pacl: *mut ACL, dwacerevision: u32, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessAllowedAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(accessmask), psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessAllowedAceEx<'a, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: Param4) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessAllowedAceEx(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessAllowedAceEx(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(aceflags), ::std::mem::transmute(accessmask), psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessAllowedObjectAce<'a, Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: *const ::windows::runtime::GUID, inheritedobjecttypeguid: *const ::windows::runtime::GUID, psid: Param6) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessAllowedObjectAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: *const ::windows::runtime::GUID, inheritedobjecttypeguid: *const ::windows::runtime::GUID, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessAllowedObjectAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(aceflags), ::std::mem::transmute(accessmask), ::std::mem::transmute(objecttypeguid), ::std::mem::transmute(inheritedobjecttypeguid), psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessDeniedAce<'a, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(pacl: *mut ACL, dwacerevision: u32, accessmask: u32, psid: Param3) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessDeniedAce(pacl: *mut ACL, dwacerevision: u32, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessDeniedAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(accessmask), psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessDeniedAceEx<'a, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: Param4) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessDeniedAceEx(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessDeniedAceEx(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(aceflags), ::std::mem::transmute(accessmask), psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAccessDeniedObjectAce<'a, Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: *const ::windows::runtime::GUID, inheritedobjecttypeguid: *const ::windows::runtime::GUID, psid: Param6) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessDeniedObjectAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: *const ::windows::runtime::GUID, inheritedobjecttypeguid: *const ::windows::runtime::GUID, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessDeniedObjectAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(aceflags), ::std::mem::transmute(accessmask), ::std::mem::transmute(objecttypeguid), ::std::mem::transmute(inheritedobjecttypeguid), psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAce(pacl: *mut ACL, dwacerevision: u32, dwstartingaceindex: u32, pacelist: *const ::std::ffi::c_void, nacelistlength: u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAce(pacl: *mut ACL, dwacerevision: u32, dwstartingaceindex: u32, pacelist: *const ::std::ffi::c_void, nacelistlength: u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(dwstartingaceindex), ::std::mem::transmute(pacelist), ::std::mem::transmute(nacelistlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAuditAccessAce<'a, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(pacl: *mut ACL, dwacerevision: u32, dwaccessmask: u32, psid: Param3, bauditsuccess: Param4, bauditfailure: Param5) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAuditAccessAce(pacl: *mut ACL, dwacerevision: u32, dwaccessmask: u32, psid: super::Foundation::PSID, bauditsuccess: super::Foundation::BOOL, bauditfailure: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAuditAccessAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(dwaccessmask), psid.into_param().abi(), bauditsuccess.into_param().abi(), bauditfailure.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAuditAccessAceEx<'a, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>, Param6: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, dwaccessmask: u32, psid: Param4, bauditsuccess: Param5, bauditfailure: Param6) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAuditAccessAceEx(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, dwaccessmask: u32, psid: super::Foundation::PSID, bauditsuccess: super::Foundation::BOOL, bauditfailure: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAuditAccessAceEx(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(aceflags), ::std::mem::transmute(dwaccessmask), psid.into_param().abi(), bauditsuccess.into_param().abi(), bauditfailure.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAuditAccessObjectAce<'a, Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param7: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>, Param8: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(
    pacl: *mut ACL,
    dwacerevision: u32,
    aceflags: ACE_FLAGS,
    accessmask: u32,
    objecttypeguid: *const ::windows::runtime::GUID,
    inheritedobjecttypeguid: *const ::windows::runtime::GUID,
    psid: Param6,
    bauditsuccess: Param7,
    bauditfailure: Param8,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAuditAccessObjectAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: *const ::windows::runtime::GUID, inheritedobjecttypeguid: *const ::windows::runtime::GUID, psid: super::Foundation::PSID, bauditsuccess: super::Foundation::BOOL, bauditfailure: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAuditAccessObjectAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(aceflags),
            ::std::mem::transmute(accessmask),
            ::std::mem::transmute(objecttypeguid),
            ::std::mem::transmute(inheritedobjecttypeguid),
            psid.into_param().abi(),
            bauditsuccess.into_param().abi(),
            bauditfailure.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddConditionalAce<'a, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, acetype: u8, accessmask: u32, psid: Param5, conditionstr: Param6, returnlength: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddConditionalAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, acetype: u8, accessmask: u32, psid: super::Foundation::PSID, conditionstr: super::Foundation::PWSTR, returnlength: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddConditionalAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(aceflags), ::std::mem::transmute(acetype), ::std::mem::transmute(accessmask), psid.into_param().abi(), conditionstr.into_param().abi(), ::std::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddMandatoryAce<'a, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, mandatorypolicy: u32, plabelsid: Param4) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddMandatoryAce(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, mandatorypolicy: u32, plabelsid: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddMandatoryAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(aceflags), ::std::mem::transmute(mandatorypolicy), plabelsid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddResourceAttributeAce<'a, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: Param4, pattributeinfo: *const CLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddResourceAttributeAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID, pattributeinfo: *const CLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddResourceAttributeAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(aceflags), ::std::mem::transmute(accessmask), psid.into_param().abi(), ::std::mem::transmute(pattributeinfo), ::std::mem::transmute(preturnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddScopedPolicyIDAce<'a, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: Param4) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddScopedPolicyIDAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddScopedPolicyIDAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwacerevision), ::std::mem::transmute(aceflags), ::std::mem::transmute(accessmask), psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdjustTokenGroups<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(tokenhandle: Param0, resettodefault: Param1, newstate: *const TOKEN_GROUPS, bufferlength: u32, previousstate: *mut TOKEN_GROUPS, returnlength: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdjustTokenGroups(tokenhandle: super::Foundation::HANDLE, resettodefault: super::Foundation::BOOL, newstate: *const TOKEN_GROUPS, bufferlength: u32, previousstate: *mut TOKEN_GROUPS, returnlength: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AdjustTokenGroups(tokenhandle.into_param().abi(), resettodefault.into_param().abi(), ::std::mem::transmute(newstate), ::std::mem::transmute(bufferlength), ::std::mem::transmute(previousstate), ::std::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdjustTokenPrivileges<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(tokenhandle: Param0, disableallprivileges: Param1, newstate: *const TOKEN_PRIVILEGES, bufferlength: u32, previousstate: *mut TOKEN_PRIVILEGES, returnlength: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdjustTokenPrivileges(tokenhandle: super::Foundation::HANDLE, disableallprivileges: super::Foundation::BOOL, newstate: *const TOKEN_PRIVILEGES, bufferlength: u32, previousstate: *mut TOKEN_PRIVILEGES, returnlength: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AdjustTokenPrivileges(tokenhandle.into_param().abi(), disableallprivileges.into_param().abi(), ::std::mem::transmute(newstate), ::std::mem::transmute(bufferlength), ::std::mem::transmute(previousstate), ::std::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateAndInitializeSid(pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8, nsubauthority0: u32, nsubauthority1: u32, nsubauthority2: u32, nsubauthority3: u32, nsubauthority4: u32, nsubauthority5: u32, nsubauthority6: u32, nsubauthority7: u32, psid: *mut super::Foundation::PSID) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocateAndInitializeSid(pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8, nsubauthority0: u32, nsubauthority1: u32, nsubauthority2: u32, nsubauthority3: u32, nsubauthority4: u32, nsubauthority5: u32, nsubauthority6: u32, nsubauthority7: u32, psid: *mut super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AllocateAndInitializeSid(
            ::std::mem::transmute(pidentifierauthority),
            ::std::mem::transmute(nsubauthoritycount),
            ::std::mem::transmute(nsubauthority0),
            ::std::mem::transmute(nsubauthority1),
            ::std::mem::transmute(nsubauthority2),
            ::std::mem::transmute(nsubauthority3),
            ::std::mem::transmute(nsubauthority4),
            ::std::mem::transmute(nsubauthority5),
            ::std::mem::transmute(nsubauthority6),
            ::std::mem::transmute(nsubauthority7),
            ::std::mem::transmute(psid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateLocallyUniqueId(luid: *mut super::Foundation::LUID) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocateLocallyUniqueId(luid: *mut super::Foundation::LUID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AllocateLocallyUniqueId(::std::mem::transmute(luid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AreAllAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AreAllAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AreAllAccessesGranted(::std::mem::transmute(grantedaccess), ::std::mem::transmute(desiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AreAnyAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AreAnyAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AreAnyAccessesGranted(::std::mem::transmute(grantedaccess), ::std::mem::transmute(desiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0,
}
#[cfg(feature = "Win32_Foundation")]
impl CLAIM_SECURITY_ATTRIBUTES_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: *mut CLAIM_SECURITY_ATTRIBUTE_V1,
}
#[cfg(feature = "Win32_Foundation")]
impl CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CLAIM_SECURITY_ATTRIBUTE_FLAGS(pub u32);
pub const CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(1u32);
pub const CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(2u32);
pub const CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(4u32);
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(8u32);
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(16u32);
pub const CLAIM_SECURITY_ATTRIBUTE_MANDATORY: CLAIM_SECURITY_ATTRIBUTE_FLAGS = CLAIM_SECURITY_ATTRIBUTE_FLAGS(32u32);
impl ::std::convert::From<u32> for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub Name: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE").field("Version", &self.Version).field("Name", &self.Name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut ::std::ffi::c_void,
    pub ValueLength: u32,
}
impl CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl ::std::default::Default for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE").field("pValue", &self.pValue).field("ValueLength", &self.ValueLength).finish()
    }
}
impl ::std::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.pValue == other.pValue && self.ValueLength == other.ValueLength
    }
}
impl ::std::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    pub Name: u32,
    pub ValueType: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE,
    pub Reserved: u16,
    pub Flags: CLAIM_SECURITY_ATTRIBUTE_FLAGS,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0,
}
impl CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {}
impl ::std::default::Default for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {}
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub union CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    pub pInt64: [u32; 1],
    pub pUint64: [u32; 1],
    pub ppString: [u32; 1],
    pub pFqbn: [u32; 1],
    pub pOctetString: [u32; 1],
}
impl CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {}
impl ::std::default::Default for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {}
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct CLAIM_SECURITY_ATTRIBUTE_V1 {
    pub Name: super::Foundation::PWSTR,
    pub ValueType: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE,
    pub Reserved: u16,
    pub Flags: u32,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_V1_0,
}
#[cfg(feature = "Win32_Foundation")]
impl CLAIM_SECURITY_ATTRIBUTE_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_V1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTE_V1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: *mut i64,
    pub pUint64: *mut u64,
    pub ppString: *mut super::Foundation::PWSTR,
    pub pFqbn: *mut CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: *mut CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl CLAIM_SECURITY_ATTRIBUTE_V1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CLAIM_SECURITY_ATTRIBUTE_V1_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(pub u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(1u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(2u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(3u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(16u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(4u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_SID: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(5u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(6u16);
impl ::std::convert::From<u16> for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CREATE_RESTRICTED_TOKEN_FLAGS(pub u32);
pub const DISABLE_MAX_PRIVILEGE: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(1u32);
pub const SANDBOX_INERT: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(2u32);
pub const LUA_TOKEN: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(4u32);
pub const WRITE_RESTRICTED: CREATE_RESTRICTED_TOKEN_FLAGS = CREATE_RESTRICTED_TOKEN_FLAGS(8u32);
impl ::std::convert::From<u32> for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CREATE_RESTRICTED_TOKEN_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CREATE_RESTRICTED_TOKEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Security`*"]
pub const CVT_SECONDS: u32 = 1u32;
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckTokenCapability<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(tokenhandle: Param0, capabilitysidtocheck: Param1, hascapability: *mut super::Foundation::BOOL) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckTokenCapability(tokenhandle: super::Foundation::HANDLE, capabilitysidtocheck: super::Foundation::PSID, hascapability: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckTokenCapability(tokenhandle.into_param().abi(), capabilitysidtocheck.into_param().abi(), ::std::mem::transmute(hascapability)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckTokenMembership<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(tokenhandle: Param0, sidtocheck: Param1, ismember: *mut super::Foundation::BOOL) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckTokenMembership(tokenhandle: super::Foundation::HANDLE, sidtocheck: super::Foundation::PSID, ismember: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckTokenMembership(tokenhandle.into_param().abi(), sidtocheck.into_param().abi(), ::std::mem::transmute(ismember)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckTokenMembershipEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(tokenhandle: Param0, sidtocheck: Param1, flags: u32, ismember: *mut super::Foundation::BOOL) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckTokenMembershipEx(tokenhandle: super::Foundation::HANDLE, sidtocheck: super::Foundation::PSID, flags: u32, ismember: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckTokenMembershipEx(tokenhandle.into_param().abi(), sidtocheck.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(ismember)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertToAutoInheritPrivateObjectSecurity<'a, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::BOOLEAN>>(parentdescriptor: *const SECURITY_DESCRIPTOR, currentsecuritydescriptor: *const SECURITY_DESCRIPTOR, newsecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, objecttype: *const ::windows::runtime::GUID, isdirectoryobject: Param4, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor: *const SECURITY_DESCRIPTOR, currentsecuritydescriptor: *const SECURITY_DESCRIPTOR, newsecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, objecttype: *const ::windows::runtime::GUID, isdirectoryobject: super::Foundation::BOOLEAN, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ConvertToAutoInheritPrivateObjectSecurity(::std::mem::transmute(parentdescriptor), ::std::mem::transmute(currentsecuritydescriptor), ::std::mem::transmute(newsecuritydescriptor), ::std::mem::transmute(objecttype), isdirectoryobject.into_param().abi(), ::std::mem::transmute(genericmapping)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CopySid<'a, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(ndestinationsidlength: u32, pdestinationsid: super::Foundation::PSID, psourcesid: Param2) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopySid(ndestinationsidlength: u32, pdestinationsid: super::Foundation::PSID, psourcesid: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CopySid(::std::mem::transmute(ndestinationsidlength), ::std::mem::transmute(pdestinationsid), psourcesid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePrivateObjectSecurity<'a, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(parentdescriptor: *const SECURITY_DESCRIPTOR, creatordescriptor: *const SECURITY_DESCRIPTOR, newdescriptor: *mut *mut SECURITY_DESCRIPTOR, isdirectoryobject: Param3, token: Param4, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePrivateObjectSecurity(parentdescriptor: *const SECURITY_DESCRIPTOR, creatordescriptor: *const SECURITY_DESCRIPTOR, newdescriptor: *mut *mut SECURITY_DESCRIPTOR, isdirectoryobject: super::Foundation::BOOL, token: super::Foundation::HANDLE, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreatePrivateObjectSecurity(::std::mem::transmute(parentdescriptor), ::std::mem::transmute(creatordescriptor), ::std::mem::transmute(newdescriptor), isdirectoryobject.into_param().abi(), token.into_param().abi(), ::std::mem::transmute(genericmapping)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePrivateObjectSecurityEx<'a, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>, Param6: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(
    parentdescriptor: *const SECURITY_DESCRIPTOR,
    creatordescriptor: *const SECURITY_DESCRIPTOR,
    newdescriptor: *mut *mut SECURITY_DESCRIPTOR,
    objecttype: *const ::windows::runtime::GUID,
    iscontainerobject: Param4,
    autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS,
    token: Param6,
    genericmapping: *const GENERIC_MAPPING,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePrivateObjectSecurityEx(parentdescriptor: *const SECURITY_DESCRIPTOR, creatordescriptor: *const SECURITY_DESCRIPTOR, newdescriptor: *mut *mut SECURITY_DESCRIPTOR, objecttype: *const ::windows::runtime::GUID, iscontainerobject: super::Foundation::BOOL, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, token: super::Foundation::HANDLE, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreatePrivateObjectSecurityEx(
            ::std::mem::transmute(parentdescriptor),
            ::std::mem::transmute(creatordescriptor),
            ::std::mem::transmute(newdescriptor),
            ::std::mem::transmute(objecttype),
            iscontainerobject.into_param().abi(),
            ::std::mem::transmute(autoinheritflags),
            token.into_param().abi(),
            ::std::mem::transmute(genericmapping),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePrivateObjectSecurityWithMultipleInheritance<'a, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>, Param7: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(
    parentdescriptor: *const SECURITY_DESCRIPTOR,
    creatordescriptor: *const SECURITY_DESCRIPTOR,
    newdescriptor: *mut *mut SECURITY_DESCRIPTOR,
    objecttypes: *const *const ::windows::runtime::GUID,
    guidcount: u32,
    iscontainerobject: Param5,
    autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS,
    token: Param7,
    genericmapping: *const GENERIC_MAPPING,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor: *const SECURITY_DESCRIPTOR, creatordescriptor: *const SECURITY_DESCRIPTOR, newdescriptor: *mut *mut SECURITY_DESCRIPTOR, objecttypes: *const *const ::windows::runtime::GUID, guidcount: u32, iscontainerobject: super::Foundation::BOOL, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, token: super::Foundation::HANDLE, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreatePrivateObjectSecurityWithMultipleInheritance(
            ::std::mem::transmute(parentdescriptor),
            ::std::mem::transmute(creatordescriptor),
            ::std::mem::transmute(newdescriptor),
            ::std::mem::transmute(objecttypes),
            ::std::mem::transmute(guidcount),
            iscontainerobject.into_param().abi(),
            ::std::mem::transmute(autoinheritflags),
            token.into_param().abi(),
            ::std::mem::transmute(genericmapping),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateRestrictedToken<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(existingtokenhandle: Param0, flags: CREATE_RESTRICTED_TOKEN_FLAGS, disablesidcount: u32, sidstodisable: *const SID_AND_ATTRIBUTES, deleteprivilegecount: u32, privilegestodelete: *const LUID_AND_ATTRIBUTES, restrictedsidcount: u32, sidstorestrict: *const SID_AND_ATTRIBUTES, newtokenhandle: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRestrictedToken(existingtokenhandle: super::Foundation::HANDLE, flags: CREATE_RESTRICTED_TOKEN_FLAGS, disablesidcount: u32, sidstodisable: *const SID_AND_ATTRIBUTES, deleteprivilegecount: u32, privilegestodelete: *const LUID_AND_ATTRIBUTES, restrictedsidcount: u32, sidstorestrict: *const SID_AND_ATTRIBUTES, newtokenhandle: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateRestrictedToken(
            existingtokenhandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(disablesidcount),
            ::std::mem::transmute(sidstodisable),
            ::std::mem::transmute(deleteprivilegecount),
            ::std::mem::transmute(privilegestodelete),
            ::std::mem::transmute(restrictedsidcount),
            ::std::mem::transmute(sidstorestrict),
            ::std::mem::transmute(newtokenhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateWellKnownSid<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(wellknownsidtype: WELL_KNOWN_SID_TYPE, domainsid: Param1, psid: super::Foundation::PSID, cbsid: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWellKnownSid(wellknownsidtype: WELL_KNOWN_SID_TYPE, domainsid: super::Foundation::PSID, psid: super::Foundation::PSID, cbsid: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateWellKnownSid(::std::mem::transmute(wellknownsidtype), domainsid.into_param().abi(), ::std::mem::transmute(psid), ::std::mem::transmute(cbsid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteAce(pacl: *mut ACL, dwaceindex: u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteAce(pacl: *mut ACL, dwaceindex: u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwaceindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveCapabilitySidsFromName<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(capname: Param0, capabilitygroupsids: *mut *mut super::Foundation::PSID, capabilitygroupsidcount: *mut u32, capabilitysids: *mut *mut super::Foundation::PSID, capabilitysidcount: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeriveCapabilitySidsFromName(capname: super::Foundation::PWSTR, capabilitygroupsids: *mut *mut super::Foundation::PSID, capabilitygroupsidcount: *mut u32, capabilitysids: *mut *mut super::Foundation::PSID, capabilitysidcount: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeriveCapabilitySidsFromName(capname.into_param().abi(), ::std::mem::transmute(capabilitygroupsids), ::std::mem::transmute(capabilitygroupsidcount), ::std::mem::transmute(capabilitysids), ::std::mem::transmute(capabilitysidcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyPrivateObjectSecurity(objectdescriptor: *const *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyPrivateObjectSecurity(objectdescriptor: *const *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(DestroyPrivateObjectSecurity(::std::mem::transmute(objectdescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DuplicateToken<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(existingtokenhandle: Param0, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DuplicateToken(existingtokenhandle: super::Foundation::HANDLE, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(DuplicateToken(existingtokenhandle.into_param().abi(), ::std::mem::transmute(impersonationlevel), ::std::mem::transmute(duplicatetokenhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DuplicateTokenEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(hexistingtoken: Param0, dwdesiredaccess: TOKEN_ACCESS_MASK, lptokenattributes: *const SECURITY_ATTRIBUTES, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, tokentype: TOKEN_TYPE, phnewtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DuplicateTokenEx(hexistingtoken: super::Foundation::HANDLE, dwdesiredaccess: TOKEN_ACCESS_MASK, lptokenattributes: *const SECURITY_ATTRIBUTES, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, tokentype: TOKEN_TYPE, phnewtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(DuplicateTokenEx(hexistingtoken.into_param().abi(), ::std::mem::transmute(dwdesiredaccess), ::std::mem::transmute(lptokenattributes), ::std::mem::transmute(impersonationlevel), ::std::mem::transmute(tokentype), ::std::mem::transmute(phnewtoken)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENUM_PERIOD(pub i32);
pub const ENUM_PERIOD_INVALID: ENUM_PERIOD = ENUM_PERIOD(-1i32);
pub const ENUM_PERIOD_SECONDS: ENUM_PERIOD = ENUM_PERIOD(0i32);
pub const ENUM_PERIOD_MINUTES: ENUM_PERIOD = ENUM_PERIOD(1i32);
pub const ENUM_PERIOD_HOURS: ENUM_PERIOD = ENUM_PERIOD(2i32);
pub const ENUM_PERIOD_DAYS: ENUM_PERIOD = ENUM_PERIOD(3i32);
pub const ENUM_PERIOD_WEEKS: ENUM_PERIOD = ENUM_PERIOD(4i32);
pub const ENUM_PERIOD_MONTHS: ENUM_PERIOD = ENUM_PERIOD(5i32);
pub const ENUM_PERIOD_YEARS: ENUM_PERIOD = ENUM_PERIOD(6i32);
impl ::std::convert::From<i32> for ENUM_PERIOD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ENUM_PERIOD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EqualDomainSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(psid1: Param0, psid2: Param1, pfequal: *mut super::Foundation::BOOL) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EqualDomainSid(psid1: super::Foundation::PSID, psid2: super::Foundation::PSID, pfequal: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(EqualDomainSid(psid1.into_param().abi(), psid2.into_param().abi(), ::std::mem::transmute(pfequal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EqualPrefixSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(psid1: Param0, psid2: Param1) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EqualPrefixSid(psid1: super::Foundation::PSID, psid2: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(EqualPrefixSid(psid1.into_param().abi(), psid2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EqualSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(psid1: Param0, psid2: Param1) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EqualSid(psid1: super::Foundation::PSID, psid2: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(EqualSid(psid1.into_param().abi(), psid2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstFreeAce(pacl: *const ACL, pace: *mut *mut ::std::ffi::c_void) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstFreeAce(pacl: *const ACL, pace: *mut *mut ::std::ffi::c_void) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindFirstFreeAce(::std::mem::transmute(pacl), ::std::mem::transmute(pace)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(psid: Param0) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeSid(psid: super::Foundation::PSID) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(FreeSid(psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct GENERIC_MAPPING {
    pub GenericRead: u32,
    pub GenericWrite: u32,
    pub GenericExecute: u32,
    pub GenericAll: u32,
}
impl GENERIC_MAPPING {}
impl ::std::default::Default for GENERIC_MAPPING {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GENERIC_MAPPING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GENERIC_MAPPING").field("GenericRead", &self.GenericRead).field("GenericWrite", &self.GenericWrite).field("GenericExecute", &self.GenericExecute).field("GenericAll", &self.GenericAll).finish()
    }
}
impl ::std::cmp::PartialEq for GENERIC_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.GenericRead == other.GenericRead && self.GenericWrite == other.GenericWrite && self.GenericExecute == other.GenericExecute && self.GenericAll == other.GenericAll
    }
}
impl ::std::cmp::Eq for GENERIC_MAPPING {}
unsafe impl ::windows::runtime::Abi for GENERIC_MAPPING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAce(pacl: *const ACL, dwaceindex: u32, pace: *mut *mut ::std::ffi::c_void) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAce(pacl: *const ACL, dwaceindex: u32, pace: *mut *mut ::std::ffi::c_void) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetAce(::std::mem::transmute(pacl), ::std::mem::transmute(dwaceindex), ::std::mem::transmute(pace)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAclInformation(pacl: *const ACL, paclinformation: *mut ::std::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAclInformation(pacl: *const ACL, paclinformation: *mut ::std::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetAclInformation(::std::mem::transmute(pacl), ::std::mem::transmute(paclinformation), ::std::mem::transmute(naclinformationlength), ::std::mem::transmute(dwaclinformationclass)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppContainerAce(acl: *const ACL, startingaceindex: u32, appcontainerace: *mut *mut ::std::ffi::c_void, appcontaineraceindex: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerAce(acl: *const ACL, startingaceindex: u32, appcontainerace: *mut *mut ::std::ffi::c_void, appcontaineraceindex: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetAppContainerAce(::std::mem::transmute(acl), ::std::mem::transmute(startingaceindex), ::std::mem::transmute(appcontainerace), ::std::mem::transmute(appcontaineraceindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCachedSigningLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(file: Param0, flags: *mut u32, signinglevel: *mut u32, thumbprint: *mut u8, thumbprintsize: *mut u32, thumbprintalgorithm: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCachedSigningLevel(file: super::Foundation::HANDLE, flags: *mut u32, signinglevel: *mut u32, thumbprint: *mut u8, thumbprintsize: *mut u32, thumbprintalgorithm: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCachedSigningLevel(file.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(signinglevel), ::std::mem::transmute(thumbprint), ::std::mem::transmute(thumbprintsize), ::std::mem::transmute(thumbprintalgorithm)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileSecurityA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>>(lpfilename: Param0, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileSecurityA(lpfilename: super::Foundation::PSTR, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileSecurityA(lpfilename.into_param().abi(), ::std::mem::transmute(requestedinformation), ::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(nlength), ::std::mem::transmute(lpnlengthneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileSecurityW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(lpfilename: Param0, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileSecurityW(lpfilename: super::Foundation::PWSTR, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileSecurityW(lpfilename.into_param().abi(), ::std::mem::transmute(requestedinformation), ::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(nlength), ::std::mem::transmute(lpnlengthneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetKernelObjectSecurity<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(handle: Param0, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKernelObjectSecurity(handle: super::Foundation::HANDLE, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetKernelObjectSecurity(handle.into_param().abi(), ::std::mem::transmute(requestedinformation), ::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(nlength), ::std::mem::transmute(lpnlengthneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLengthSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(psid: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLengthSid(psid: super::Foundation::PSID) -> u32;
        }
        ::std::mem::transmute(GetLengthSid(psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateObjectSecurity(objectdescriptor: *const SECURITY_DESCRIPTOR, securityinformation: u32, resultantdescriptor: *mut SECURITY_DESCRIPTOR, descriptorlength: u32, returnlength: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateObjectSecurity(objectdescriptor: *const SECURITY_DESCRIPTOR, securityinformation: u32, resultantdescriptor: *mut SECURITY_DESCRIPTOR, descriptorlength: u32, returnlength: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPrivateObjectSecurity(::std::mem::transmute(objectdescriptor), ::std::mem::transmute(securityinformation), ::std::mem::transmute(resultantdescriptor), ::std::mem::transmute(descriptorlength), ::std::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorControl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, pcontrol: *mut u16, lpdwrevision: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorControl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, pcontrol: *mut u16, lpdwrevision: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSecurityDescriptorControl(::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(pcontrol), ::std::mem::transmute(lpdwrevision)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorDacl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, lpbdaclpresent: *mut i32, pdacl: *mut *mut ACL, lpbdacldefaulted: *mut i32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorDacl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, lpbdaclpresent: *mut i32, pdacl: *mut *mut ACL, lpbdacldefaulted: *mut i32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSecurityDescriptorDacl(::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(lpbdaclpresent), ::std::mem::transmute(pdacl), ::std::mem::transmute(lpbdacldefaulted)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorGroup(psecuritydescriptor: *const SECURITY_DESCRIPTOR, pgroup: *mut super::Foundation::PSID, lpbgroupdefaulted: *mut i32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorGroup(psecuritydescriptor: *const SECURITY_DESCRIPTOR, pgroup: *mut super::Foundation::PSID, lpbgroupdefaulted: *mut i32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSecurityDescriptorGroup(::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(pgroup), ::std::mem::transmute(lpbgroupdefaulted)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorLength(psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorLength(psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> u32;
        }
        ::std::mem::transmute(GetSecurityDescriptorLength(::std::mem::transmute(psecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorOwner(psecuritydescriptor: *const SECURITY_DESCRIPTOR, powner: *mut super::Foundation::PSID, lpbownerdefaulted: *mut i32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorOwner(psecuritydescriptor: *const SECURITY_DESCRIPTOR, powner: *mut super::Foundation::PSID, lpbownerdefaulted: *mut i32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSecurityDescriptorOwner(::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(powner), ::std::mem::transmute(lpbownerdefaulted)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorRMControl(securitydescriptor: *const SECURITY_DESCRIPTOR, rmcontrol: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorRMControl(securitydescriptor: *const SECURITY_DESCRIPTOR, rmcontrol: *mut u8) -> u32;
        }
        ::std::mem::transmute(GetSecurityDescriptorRMControl(::std::mem::transmute(securitydescriptor), ::std::mem::transmute(rmcontrol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSecurityDescriptorSacl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, lpbsaclpresent: *mut i32, psacl: *mut *mut ACL, lpbsacldefaulted: *mut i32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorSacl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, lpbsaclpresent: *mut i32, psacl: *mut *mut ACL, lpbsacldefaulted: *mut i32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSecurityDescriptorSacl(::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(lpbsaclpresent), ::std::mem::transmute(psacl), ::std::mem::transmute(lpbsacldefaulted)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSidIdentifierAuthority<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(psid: Param0) -> *mut SID_IDENTIFIER_AUTHORITY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSidIdentifierAuthority(psid: super::Foundation::PSID) -> *mut SID_IDENTIFIER_AUTHORITY;
        }
        ::std::mem::transmute(GetSidIdentifierAuthority(psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`*"]
#[inline]
pub unsafe fn GetSidLengthRequired(nsubauthoritycount: u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSidLengthRequired(nsubauthoritycount: u8) -> u32;
        }
        ::std::mem::transmute(GetSidLengthRequired(::std::mem::transmute(nsubauthoritycount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSidSubAuthority<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(psid: Param0, nsubauthority: u32) -> *mut u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSidSubAuthority(psid: super::Foundation::PSID, nsubauthority: u32) -> *mut u32;
        }
        ::std::mem::transmute(GetSidSubAuthority(psid.into_param().abi(), ::std::mem::transmute(nsubauthority)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSidSubAuthorityCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(psid: Param0) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSidSubAuthorityCount(psid: super::Foundation::PSID) -> *mut u8;
        }
        ::std::mem::transmute(GetSidSubAuthorityCount(psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTokenInformation<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(tokenhandle: Param0, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: *mut ::std::ffi::c_void, tokeninformationlength: u32, returnlength: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTokenInformation(tokenhandle: super::Foundation::HANDLE, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: *mut ::std::ffi::c_void, tokeninformationlength: u32, returnlength: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetTokenInformation(tokenhandle.into_param().abi(), ::std::mem::transmute(tokeninformationclass), ::std::mem::transmute(tokeninformation), ::std::mem::transmute(tokeninformationlength), ::std::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserObjectSecurity<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(hobj: Param0, psirequested: *const u32, psid: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUserObjectSecurity(hobj: super::Foundation::HANDLE, psirequested: *const u32, psid: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetUserObjectSecurity(hobj.into_param().abi(), ::std::mem::transmute(psirequested), ::std::mem::transmute(psid), ::std::mem::transmute(nlength), ::std::mem::transmute(lpnlengthneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowsAccountDomainSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(psid: Param0, pdomainsid: super::Foundation::PSID, cbdomainsid: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowsAccountDomainSid(psid: super::Foundation::PSID, pdomainsid: super::Foundation::PSID, cbdomainsid: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetWindowsAccountDomainSid(psid.into_param().abi(), ::std::mem::transmute(pdomainsid), ::std::mem::transmute(cbdomainsid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HDIAGNOSTIC_DATA_QUERY_SESSION(pub isize);
impl ::std::default::Default for HDIAGNOSTIC_DATA_QUERY_SESSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HDIAGNOSTIC_DATA_QUERY_SESSION {}
unsafe impl ::windows::runtime::Abi for HDIAGNOSTIC_DATA_QUERY_SESSION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION(pub isize);
impl ::std::default::Default for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {}
unsafe impl ::windows::runtime::Abi for HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION(pub isize);
impl ::std::default::Default for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {}
unsafe impl ::windows::runtime::Abi for HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HDIAGNOSTIC_EVENT_TAG_DESCRIPTION(pub isize);
impl ::std::default::Default for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {}
unsafe impl ::windows::runtime::Abi for HDIAGNOSTIC_EVENT_TAG_DESCRIPTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HDIAGNOSTIC_RECORD(pub isize);
impl ::std::default::Default for HDIAGNOSTIC_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HDIAGNOSTIC_RECORD {}
unsafe impl ::windows::runtime::Abi for HDIAGNOSTIC_RECORD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HDIAGNOSTIC_REPORT(pub isize);
impl ::std::default::Default for HDIAGNOSTIC_REPORT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HDIAGNOSTIC_REPORT {}
unsafe impl ::windows::runtime::Abi for HDIAGNOSTIC_REPORT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImpersonateAnonymousToken<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(threadhandle: Param0) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImpersonateAnonymousToken(threadhandle: super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImpersonateAnonymousToken(threadhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImpersonateLoggedOnUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(htoken: Param0) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImpersonateLoggedOnUser(htoken: super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImpersonateLoggedOnUser(htoken.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImpersonateSelf(impersonationlevel: SECURITY_IMPERSONATION_LEVEL) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImpersonateSelf(impersonationlevel: SECURITY_IMPERSONATION_LEVEL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImpersonateSelf(::std::mem::transmute(impersonationlevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeAcl(pacl: *mut ACL, nacllength: u32, dwaclrevision: u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeAcl(pacl: *mut ACL, nacllength: u32, dwaclrevision: u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeAcl(::std::mem::transmute(pacl), ::std::mem::transmute(nacllength), ::std::mem::transmute(dwaclrevision)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeSecurityDescriptor(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, dwrevision: u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeSecurityDescriptor(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, dwrevision: u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeSecurityDescriptor(::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(dwrevision)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeSid(sid: super::Foundation::PSID, pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeSid(sid: super::Foundation::PSID, pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeSid(::std::mem::transmute(sid), ::std::mem::transmute(pidentifierauthority), ::std::mem::transmute(nsubauthoritycount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsTokenRestricted<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(tokenhandle: Param0) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsTokenRestricted(tokenhandle: super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsTokenRestricted(tokenhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsValidAcl(pacl: *const ACL) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsValidAcl(pacl: *const ACL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsValidAcl(::std::mem::transmute(pacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsValidSecurityDescriptor(psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsValidSecurityDescriptor(psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsValidSecurityDescriptor(::std::mem::transmute(psecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsValidSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(psid: Param0) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsValidSid(psid: super::Foundation::PSID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsValidSid(psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWellKnownSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(psid: Param0, wellknownsidtype: WELL_KNOWN_SID_TYPE) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsWellKnownSid(psid: super::Foundation::PSID, wellknownsidtype: WELL_KNOWN_SID_TYPE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsWellKnownSid(psid.into_param().abi(), ::std::mem::transmute(wellknownsidtype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct LLFILETIME {
    pub Anonymous: LLFILETIME_0,
}
#[cfg(feature = "Win32_Foundation")]
impl LLFILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LLFILETIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LLFILETIME {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LLFILETIME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LLFILETIME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union LLFILETIME_0 {
    pub ll: i64,
    pub ft: super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl LLFILETIME_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LLFILETIME_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LLFILETIME_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LLFILETIME_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LLFILETIME_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LOGON32_LOGON(pub u32);
pub const LOGON32_LOGON_BATCH: LOGON32_LOGON = LOGON32_LOGON(4u32);
pub const LOGON32_LOGON_INTERACTIVE: LOGON32_LOGON = LOGON32_LOGON(2u32);
pub const LOGON32_LOGON_NETWORK: LOGON32_LOGON = LOGON32_LOGON(3u32);
pub const LOGON32_LOGON_NETWORK_CLEARTEXT: LOGON32_LOGON = LOGON32_LOGON(8u32);
pub const LOGON32_LOGON_NEW_CREDENTIALS: LOGON32_LOGON = LOGON32_LOGON(9u32);
pub const LOGON32_LOGON_SERVICE: LOGON32_LOGON = LOGON32_LOGON(5u32);
pub const LOGON32_LOGON_UNLOCK: LOGON32_LOGON = LOGON32_LOGON(7u32);
impl ::std::convert::From<u32> for LOGON32_LOGON {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LOGON32_LOGON {
    type Abi = Self;
}
impl ::std::ops::BitOr for LOGON32_LOGON {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LOGON32_LOGON {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LOGON32_LOGON {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LOGON32_LOGON {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LOGON32_LOGON {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LOGON32_PROVIDER(pub u32);
pub const LOGON32_PROVIDER_DEFAULT: LOGON32_PROVIDER = LOGON32_PROVIDER(0u32);
pub const LOGON32_PROVIDER_WINNT50: LOGON32_PROVIDER = LOGON32_PROVIDER(3u32);
pub const LOGON32_PROVIDER_WINNT40: LOGON32_PROVIDER = LOGON32_PROVIDER(2u32);
impl ::std::convert::From<u32> for LOGON32_PROVIDER {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LOGON32_PROVIDER {
    type Abi = Self;
}
impl ::std::ops::BitOr for LOGON32_PROVIDER {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LOGON32_PROVIDER {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LOGON32_PROVIDER {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LOGON32_PROVIDER {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LOGON32_PROVIDER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: super::Foundation::LUID,
    pub Attributes: TOKEN_PRIVILEGES_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl LUID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LUID_AND_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LUID_AND_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LUID_AND_ATTRIBUTES").field("Luid", &self.Luid).field("Attributes", &self.Attributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LUID_AND_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Luid == other.Luid && self.Attributes == other.Attributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LUID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LUID_AND_ATTRIBUTES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogonUserA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>>(lpszusername: Param0, lpszdomain: Param1, lpszpassword: Param2, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogonUserA(lpszusername: super::Foundation::PSTR, lpszdomain: super::Foundation::PSTR, lpszpassword: super::Foundation::PSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LogonUserA(lpszusername.into_param().abi(), lpszdomain.into_param().abi(), lpszpassword.into_param().abi(), ::std::mem::transmute(dwlogontype), ::std::mem::transmute(dwlogonprovider), ::std::mem::transmute(phtoken)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogonUserExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>>(
    lpszusername: Param0,
    lpszdomain: Param1,
    lpszpassword: Param2,
    dwlogontype: LOGON32_LOGON,
    dwlogonprovider: LOGON32_PROVIDER,
    phtoken: *mut super::Foundation::HANDLE,
    pplogonsid: *mut super::Foundation::PSID,
    ppprofilebuffer: *mut *mut ::std::ffi::c_void,
    pdwprofilelength: *mut u32,
    pquotalimits: *mut QUOTA_LIMITS,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogonUserExA(lpszusername: super::Foundation::PSTR, lpszdomain: super::Foundation::PSTR, lpszpassword: super::Foundation::PSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE, pplogonsid: *mut super::Foundation::PSID, ppprofilebuffer: *mut *mut ::std::ffi::c_void, pdwprofilelength: *mut u32, pquotalimits: *mut QUOTA_LIMITS) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LogonUserExA(
            lpszusername.into_param().abi(),
            lpszdomain.into_param().abi(),
            lpszpassword.into_param().abi(),
            ::std::mem::transmute(dwlogontype),
            ::std::mem::transmute(dwlogonprovider),
            ::std::mem::transmute(phtoken),
            ::std::mem::transmute(pplogonsid),
            ::std::mem::transmute(ppprofilebuffer),
            ::std::mem::transmute(pdwprofilelength),
            ::std::mem::transmute(pquotalimits),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogonUserExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(
    lpszusername: Param0,
    lpszdomain: Param1,
    lpszpassword: Param2,
    dwlogontype: LOGON32_LOGON,
    dwlogonprovider: LOGON32_PROVIDER,
    phtoken: *mut super::Foundation::HANDLE,
    pplogonsid: *mut super::Foundation::PSID,
    ppprofilebuffer: *mut *mut ::std::ffi::c_void,
    pdwprofilelength: *mut u32,
    pquotalimits: *mut QUOTA_LIMITS,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogonUserExW(lpszusername: super::Foundation::PWSTR, lpszdomain: super::Foundation::PWSTR, lpszpassword: super::Foundation::PWSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE, pplogonsid: *mut super::Foundation::PSID, ppprofilebuffer: *mut *mut ::std::ffi::c_void, pdwprofilelength: *mut u32, pquotalimits: *mut QUOTA_LIMITS) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LogonUserExW(
            lpszusername.into_param().abi(),
            lpszdomain.into_param().abi(),
            lpszpassword.into_param().abi(),
            ::std::mem::transmute(dwlogontype),
            ::std::mem::transmute(dwlogonprovider),
            ::std::mem::transmute(phtoken),
            ::std::mem::transmute(pplogonsid),
            ::std::mem::transmute(ppprofilebuffer),
            ::std::mem::transmute(pdwprofilelength),
            ::std::mem::transmute(pquotalimits),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogonUserW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(lpszusername: Param0, lpszdomain: Param1, lpszpassword: Param2, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogonUserW(lpszusername: super::Foundation::PWSTR, lpszdomain: super::Foundation::PWSTR, lpszpassword: super::Foundation::PWSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LogonUserW(lpszusername.into_param().abi(), lpszdomain.into_param().abi(), lpszpassword.into_param().abi(), ::std::mem::transmute(dwlogontype), ::std::mem::transmute(dwlogonprovider), ::std::mem::transmute(phtoken)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupAccountNameA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>>(lpsystemname: Param0, lpaccountname: Param1, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: super::Foundation::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupAccountNameA(lpsystemname: super::Foundation::PSTR, lpaccountname: super::Foundation::PSTR, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: super::Foundation::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupAccountNameA(lpsystemname.into_param().abi(), lpaccountname.into_param().abi(), ::std::mem::transmute(sid), ::std::mem::transmute(cbsid), ::std::mem::transmute(referenceddomainname), ::std::mem::transmute(cchreferenceddomainname), ::std::mem::transmute(peuse)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupAccountNameW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(lpsystemname: Param0, lpaccountname: Param1, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: super::Foundation::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupAccountNameW(lpsystemname: super::Foundation::PWSTR, lpaccountname: super::Foundation::PWSTR, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: super::Foundation::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupAccountNameW(lpsystemname.into_param().abi(), lpaccountname.into_param().abi(), ::std::mem::transmute(sid), ::std::mem::transmute(cbsid), ::std::mem::transmute(referenceddomainname), ::std::mem::transmute(cchreferenceddomainname), ::std::mem::transmute(peuse)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupAccountSidA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(lpsystemname: Param0, sid: Param1, name: super::Foundation::PSTR, cchname: *mut u32, referenceddomainname: super::Foundation::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupAccountSidA(lpsystemname: super::Foundation::PSTR, sid: super::Foundation::PSID, name: super::Foundation::PSTR, cchname: *mut u32, referenceddomainname: super::Foundation::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupAccountSidA(lpsystemname.into_param().abi(), sid.into_param().abi(), ::std::mem::transmute(name), ::std::mem::transmute(cchname), ::std::mem::transmute(referenceddomainname), ::std::mem::transmute(cchreferenceddomainname), ::std::mem::transmute(peuse)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupAccountSidW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(lpsystemname: Param0, sid: Param1, name: super::Foundation::PWSTR, cchname: *mut u32, referenceddomainname: super::Foundation::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupAccountSidW(lpsystemname: super::Foundation::PWSTR, sid: super::Foundation::PSID, name: super::Foundation::PWSTR, cchname: *mut u32, referenceddomainname: super::Foundation::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupAccountSidW(lpsystemname.into_param().abi(), sid.into_param().abi(), ::std::mem::transmute(name), ::std::mem::transmute(cchname), ::std::mem::transmute(referenceddomainname), ::std::mem::transmute(cchreferenceddomainname), ::std::mem::transmute(peuse)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeDisplayNameA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>>(lpsystemname: Param0, lpname: Param1, lpdisplayname: super::Foundation::PSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeDisplayNameA(lpsystemname: super::Foundation::PSTR, lpname: super::Foundation::PSTR, lpdisplayname: super::Foundation::PSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeDisplayNameA(lpsystemname.into_param().abi(), lpname.into_param().abi(), ::std::mem::transmute(lpdisplayname), ::std::mem::transmute(cchdisplayname), ::std::mem::transmute(lplanguageid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeDisplayNameW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(lpsystemname: Param0, lpname: Param1, lpdisplayname: super::Foundation::PWSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeDisplayNameW(lpsystemname: super::Foundation::PWSTR, lpname: super::Foundation::PWSTR, lpdisplayname: super::Foundation::PWSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeDisplayNameW(lpsystemname.into_param().abi(), lpname.into_param().abi(), ::std::mem::transmute(lpdisplayname), ::std::mem::transmute(cchdisplayname), ::std::mem::transmute(lplanguageid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeNameA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>>(lpsystemname: Param0, lpluid: *const super::Foundation::LUID, lpname: super::Foundation::PSTR, cchname: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeNameA(lpsystemname: super::Foundation::PSTR, lpluid: *const super::Foundation::LUID, lpname: super::Foundation::PSTR, cchname: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeNameA(lpsystemname.into_param().abi(), ::std::mem::transmute(lpluid), ::std::mem::transmute(lpname), ::std::mem::transmute(cchname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeNameW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(lpsystemname: Param0, lpluid: *const super::Foundation::LUID, lpname: super::Foundation::PWSTR, cchname: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeNameW(lpsystemname: super::Foundation::PWSTR, lpluid: *const super::Foundation::LUID, lpname: super::Foundation::PWSTR, cchname: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeNameW(lpsystemname.into_param().abi(), ::std::mem::transmute(lpluid), ::std::mem::transmute(lpname), ::std::mem::transmute(cchname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeValueA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>>(lpsystemname: Param0, lpname: Param1, lpluid: *mut super::Foundation::LUID) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeValueA(lpsystemname: super::Foundation::PSTR, lpname: super::Foundation::PSTR, lpluid: *mut super::Foundation::LUID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeValueA(lpsystemname.into_param().abi(), lpname.into_param().abi(), ::std::mem::transmute(lpluid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LookupPrivilegeValueW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(lpsystemname: Param0, lpname: Param1, lpluid: *mut super::Foundation::LUID) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeValueW(lpsystemname: super::Foundation::PWSTR, lpname: super::Foundation::PWSTR, lpluid: *mut super::Foundation::LUID) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeValueW(lpsystemname.into_param().abi(), lpname.into_param().abi(), ::std::mem::transmute(lpluid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MANDATORY_LEVEL(pub i32);
pub const MandatoryLevelUntrusted: MANDATORY_LEVEL = MANDATORY_LEVEL(0i32);
pub const MandatoryLevelLow: MANDATORY_LEVEL = MANDATORY_LEVEL(1i32);
pub const MandatoryLevelMedium: MANDATORY_LEVEL = MANDATORY_LEVEL(2i32);
pub const MandatoryLevelHigh: MANDATORY_LEVEL = MANDATORY_LEVEL(3i32);
pub const MandatoryLevelSystem: MANDATORY_LEVEL = MANDATORY_LEVEL(4i32);
pub const MandatoryLevelSecureProcess: MANDATORY_LEVEL = MANDATORY_LEVEL(5i32);
pub const MandatoryLevelCount: MANDATORY_LEVEL = MANDATORY_LEVEL(6i32);
impl ::std::convert::From<i32> for MANDATORY_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MANDATORY_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MakeAbsoluteSD(pselfrelativesecuritydescriptor: *const SECURITY_DESCRIPTOR, pabsolutesecuritydescriptor: *mut SECURITY_DESCRIPTOR, lpdwabsolutesecuritydescriptorsize: *mut u32, pdacl: *mut ACL, lpdwdaclsize: *mut u32, psacl: *mut ACL, lpdwsaclsize: *mut u32, powner: super::Foundation::PSID, lpdwownersize: *mut u32, pprimarygroup: super::Foundation::PSID, lpdwprimarygroupsize: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MakeAbsoluteSD(pselfrelativesecuritydescriptor: *const SECURITY_DESCRIPTOR, pabsolutesecuritydescriptor: *mut SECURITY_DESCRIPTOR, lpdwabsolutesecuritydescriptorsize: *mut u32, pdacl: *mut ACL, lpdwdaclsize: *mut u32, psacl: *mut ACL, lpdwsaclsize: *mut u32, powner: super::Foundation::PSID, lpdwownersize: *mut u32, pprimarygroup: super::Foundation::PSID, lpdwprimarygroupsize: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(MakeAbsoluteSD(
            ::std::mem::transmute(pselfrelativesecuritydescriptor),
            ::std::mem::transmute(pabsolutesecuritydescriptor),
            ::std::mem::transmute(lpdwabsolutesecuritydescriptorsize),
            ::std::mem::transmute(pdacl),
            ::std::mem::transmute(lpdwdaclsize),
            ::std::mem::transmute(psacl),
            ::std::mem::transmute(lpdwsaclsize),
            ::std::mem::transmute(powner),
            ::std::mem::transmute(lpdwownersize),
            ::std::mem::transmute(pprimarygroup),
            ::std::mem::transmute(lpdwprimarygroupsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MakeSelfRelativeSD(pabsolutesecuritydescriptor: *const SECURITY_DESCRIPTOR, pselfrelativesecuritydescriptor: *mut SECURITY_DESCRIPTOR, lpdwbufferlength: *mut u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MakeSelfRelativeSD(pabsolutesecuritydescriptor: *const SECURITY_DESCRIPTOR, pselfrelativesecuritydescriptor: *mut SECURITY_DESCRIPTOR, lpdwbufferlength: *mut u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(MakeSelfRelativeSD(::std::mem::transmute(pabsolutesecuritydescriptor), ::std::mem::transmute(pselfrelativesecuritydescriptor), ::std::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`*"]
#[inline]
pub unsafe fn MapGenericMask(accessmask: *mut u32, genericmapping: *const GENERIC_MAPPING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapGenericMask(accessmask: *mut u32, genericmapping: *const GENERIC_MAPPING);
        }
        ::std::mem::transmute(MapGenericMask(::std::mem::transmute(accessmask), ::std::mem::transmute(genericmapping)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct NCRYPT_DESCRIPTOR_HANDLE(pub isize);
impl ::std::default::Default for NCRYPT_DESCRIPTOR_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for NCRYPT_DESCRIPTOR_HANDLE {}
unsafe impl ::windows::runtime::Abi for NCRYPT_DESCRIPTOR_HANDLE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct NCRYPT_STREAM_HANDLE(pub isize);
impl ::std::default::Default for NCRYPT_STREAM_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for NCRYPT_STREAM_HANDLE {}
unsafe impl ::windows::runtime::Abi for NCRYPT_STREAM_HANDLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OBJECT_SECURITY_INFORMATION(pub u32);
pub const ATTRIBUTE_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(32u32);
pub const BACKUP_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(65536u32);
pub const DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(4u32);
pub const GROUP_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(2u32);
pub const LABEL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(16u32);
pub const OWNER_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(1u32);
pub const PROTECTED_DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(2147483648u32);
pub const PROTECTED_SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(1073741824u32);
pub const SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(8u32);
pub const SCOPE_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(64u32);
pub const UNPROTECTED_DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(536870912u32);
pub const UNPROTECTED_SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = OBJECT_SECURITY_INFORMATION(268435456u32);
impl ::std::convert::From<u32> for OBJECT_SECURITY_INFORMATION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OBJECT_SECURITY_INFORMATION {
    type Abi = Self;
}
impl ::std::ops::BitOr for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for OBJECT_SECURITY_INFORMATION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for OBJECT_SECURITY_INFORMATION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for OBJECT_SECURITY_INFORMATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct OBJECT_TYPE_LIST {
    pub Level: u16,
    pub Sbz: u16,
    pub ObjectType: *mut ::windows::runtime::GUID,
}
impl OBJECT_TYPE_LIST {}
impl ::std::default::Default for OBJECT_TYPE_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OBJECT_TYPE_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OBJECT_TYPE_LIST").field("Level", &self.Level).field("Sbz", &self.Sbz).field("ObjectType", &self.ObjectType).finish()
    }
}
impl ::std::cmp::PartialEq for OBJECT_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level && self.Sbz == other.Sbz && self.ObjectType == other.ObjectType
    }
}
impl ::std::cmp::Eq for OBJECT_TYPE_LIST {}
unsafe impl ::windows::runtime::Abi for OBJECT_TYPE_LIST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectCloseAuditAlarmA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(subsystemname: Param0, handleid: *const ::std::ffi::c_void, generateonclose: Param2) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectCloseAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::std::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectCloseAuditAlarmA(subsystemname.into_param().abi(), ::std::mem::transmute(handleid), generateonclose.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectCloseAuditAlarmW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(subsystemname: Param0, handleid: *const ::std::ffi::c_void, generateonclose: Param2) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectCloseAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::std::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectCloseAuditAlarmW(subsystemname.into_param().abi(), ::std::mem::transmute(handleid), generateonclose.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectDeleteAuditAlarmA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(subsystemname: Param0, handleid: *const ::std::ffi::c_void, generateonclose: Param2) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectDeleteAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::std::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectDeleteAuditAlarmA(subsystemname.into_param().abi(), ::std::mem::transmute(handleid), generateonclose.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectDeleteAuditAlarmW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(subsystemname: Param0, handleid: *const ::std::ffi::c_void, generateonclose: Param2) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectDeleteAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::std::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectDeleteAuditAlarmW(subsystemname.into_param().abi(), ::std::mem::transmute(handleid), generateonclose.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectOpenAuditAlarmA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param9: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>, Param10: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    objecttypename: Param2,
    objectname: Param3,
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
    clienttoken: Param5,
    desiredaccess: u32,
    grantedaccess: u32,
    privileges: *const PRIVILEGE_SET,
    objectcreation: Param9,
    accessgranted: Param10,
    generateonclose: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectOpenAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::std::ffi::c_void, objecttypename: super::Foundation::PSTR, objectname: super::Foundation::PSTR, psecuritydescriptor: *const SECURITY_DESCRIPTOR, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, grantedaccess: u32, privileges: *const PRIVILEGE_SET, objectcreation: super::Foundation::BOOL, accessgranted: super::Foundation::BOOL, generateonclose: *mut i32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectOpenAuditAlarmA(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            objecttypename.into_param().abi(),
            objectname.into_param().abi(),
            ::std::mem::transmute(psecuritydescriptor),
            clienttoken.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(privileges),
            objectcreation.into_param().abi(),
            accessgranted.into_param().abi(),
            ::std::mem::transmute(generateonclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectOpenAuditAlarmW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param9: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>, Param10: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    objecttypename: Param2,
    objectname: Param3,
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
    clienttoken: Param5,
    desiredaccess: u32,
    grantedaccess: u32,
    privileges: *const PRIVILEGE_SET,
    objectcreation: Param9,
    accessgranted: Param10,
    generateonclose: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectOpenAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::std::ffi::c_void, objecttypename: super::Foundation::PWSTR, objectname: super::Foundation::PWSTR, psecuritydescriptor: *const SECURITY_DESCRIPTOR, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, grantedaccess: u32, privileges: *const PRIVILEGE_SET, objectcreation: super::Foundation::BOOL, accessgranted: super::Foundation::BOOL, generateonclose: *mut i32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectOpenAuditAlarmW(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            objecttypename.into_param().abi(),
            objectname.into_param().abi(),
            ::std::mem::transmute(psecuritydescriptor),
            clienttoken.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(privileges),
            objectcreation.into_param().abi(),
            accessgranted.into_param().abi(),
            ::std::mem::transmute(generateonclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectPrivilegeAuditAlarmA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(subsystemname: Param0, handleid: *const ::std::ffi::c_void, clienttoken: Param2, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: Param5) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectPrivilegeAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::std::ffi::c_void, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectPrivilegeAuditAlarmA(subsystemname.into_param().abi(), ::std::mem::transmute(handleid), clienttoken.into_param().abi(), ::std::mem::transmute(desiredaccess), ::std::mem::transmute(privileges), accessgranted.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObjectPrivilegeAuditAlarmW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(subsystemname: Param0, handleid: *const ::std::ffi::c_void, clienttoken: Param2, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: Param5) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectPrivilegeAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::std::ffi::c_void, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectPrivilegeAuditAlarmW(subsystemname.into_param().abi(), ::std::mem::transmute(handleid), clienttoken.into_param().abi(), ::std::mem::transmute(desiredaccess), ::std::mem::transmute(privileges), accessgranted.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_CALL_PACKAGE_UNTRUSTED = unsafe extern "system" fn(clientrequest: *const *const ::std::ffi::c_void, protocolsubmitbuffer: *const ::std::ffi::c_void, clientbufferbase: *const ::std::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut ::std::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::Foundation::NTSTATUS;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct PRIVILEGE_SET {
    pub PrivilegeCount: u32,
    pub Control: u32,
    pub Privilege: [LUID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl PRIVILEGE_SET {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PRIVILEGE_SET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PRIVILEGE_SET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRIVILEGE_SET").field("PrivilegeCount", &self.PrivilegeCount).field("Control", &self.Control).field("Privilege", &self.Privilege).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PRIVILEGE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.PrivilegeCount == other.PrivilegeCount && self.Control == other.Control && self.Privilege == other.Privilege
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PRIVILEGE_SET {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PRIVILEGE_SET {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrivilegeCheck<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(clienttoken: Param0, requiredprivileges: *mut PRIVILEGE_SET, pfresult: *mut i32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrivilegeCheck(clienttoken: super::Foundation::HANDLE, requiredprivileges: *mut PRIVILEGE_SET, pfresult: *mut i32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrivilegeCheck(clienttoken.into_param().abi(), ::std::mem::transmute(requiredprivileges), ::std::mem::transmute(pfresult)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrivilegedServiceAuditAlarmA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(subsystemname: Param0, servicename: Param1, clienttoken: Param2, privileges: *const PRIVILEGE_SET, accessgranted: Param4) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrivilegedServiceAuditAlarmA(subsystemname: super::Foundation::PSTR, servicename: super::Foundation::PSTR, clienttoken: super::Foundation::HANDLE, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrivilegedServiceAuditAlarmA(subsystemname.into_param().abi(), servicename.into_param().abi(), clienttoken.into_param().abi(), ::std::mem::transmute(privileges), accessgranted.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrivilegedServiceAuditAlarmW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(subsystemname: Param0, servicename: Param1, clienttoken: Param2, privileges: *const PRIVILEGE_SET, accessgranted: Param4) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrivilegedServiceAuditAlarmW(subsystemname: super::Foundation::PWSTR, servicename: super::Foundation::PWSTR, clienttoken: super::Foundation::HANDLE, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrivilegedServiceAuditAlarmW(subsystemname.into_param().abi(), servicename.into_param().abi(), clienttoken.into_param().abi(), ::std::mem::transmute(privileges), accessgranted.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct QUOTA_LIMITS {
    pub PagedPoolLimit: usize,
    pub NonPagedPoolLimit: usize,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub PagefileLimit: usize,
    pub TimeLimit: i64,
}
impl QUOTA_LIMITS {}
impl ::std::default::Default for QUOTA_LIMITS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for QUOTA_LIMITS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("QUOTA_LIMITS")
            .field("PagedPoolLimit", &self.PagedPoolLimit)
            .field("NonPagedPoolLimit", &self.NonPagedPoolLimit)
            .field("MinimumWorkingSetSize", &self.MinimumWorkingSetSize)
            .field("MaximumWorkingSetSize", &self.MaximumWorkingSetSize)
            .field("PagefileLimit", &self.PagefileLimit)
            .field("TimeLimit", &self.TimeLimit)
            .finish()
    }
}
impl ::std::cmp::PartialEq for QUOTA_LIMITS {
    fn eq(&self, other: &Self) -> bool {
        self.PagedPoolLimit == other.PagedPoolLimit && self.NonPagedPoolLimit == other.NonPagedPoolLimit && self.MinimumWorkingSetSize == other.MinimumWorkingSetSize && self.MaximumWorkingSetSize == other.MaximumWorkingSetSize && self.PagefileLimit == other.PagefileLimit && self.TimeLimit == other.TimeLimit
    }
}
impl ::std::cmp::Eq for QUOTA_LIMITS {}
unsafe impl ::windows::runtime::Abi for QUOTA_LIMITS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[inline]
pub unsafe fn QuerySecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QuerySecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32);
        }
        ::std::mem::transmute(QuerySecurityAccessMask(::std::mem::transmute(securityinformation), ::std::mem::transmute(desiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RevertToSelf() -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RevertToSelf() -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(RevertToSelf())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlConvertSidToUnicodeString<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOLEAN>>(unicodestring: *mut super::Foundation::UNICODE_STRING, sid: Param1, allocatedestinationstring: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlConvertSidToUnicodeString(unicodestring: *mut super::Foundation::UNICODE_STRING, sid: super::Foundation::PSID, allocatedestinationstring: super::Foundation::BOOLEAN) -> super::Foundation::NTSTATUS;
        }
        RtlConvertSidToUnicodeString(::std::mem::transmute(unicodestring), sid.into_param().abi(), allocatedestinationstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct SAFER_LEVEL_HANDLE(pub isize);
impl ::std::default::Default for SAFER_LEVEL_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for SAFER_LEVEL_HANDLE {}
unsafe impl ::windows::runtime::Abi for SAFER_LEVEL_HANDLE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct SC_HANDLE(pub isize);
impl ::std::default::Default for SC_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for SC_HANDLE {}
unsafe impl ::windows::runtime::Abi for SC_HANDLE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut ::std::ffi::c_void,
    pub bInheritHandle: super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl SECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SECURITY_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SECURITY_ATTRIBUTES").field("nLength", &self.nLength).field("lpSecurityDescriptor", &self.lpSecurityDescriptor).field("bInheritHandle", &self.bInheritHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.nLength == other.nLength && self.lpSecurityDescriptor == other.lpSecurityDescriptor && self.bInheritHandle == other.bInheritHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SECURITY_ATTRIBUTES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SECURITY_AUTO_INHERIT_FLAGS(pub u32);
pub const SEF_AVOID_OWNER_CHECK: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(16u32);
pub const SEF_AVOID_OWNER_RESTRICTION: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(4096u32);
pub const SEF_AVOID_PRIVILEGE_CHECK: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(8u32);
pub const SEF_DACL_AUTO_INHERIT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(1u32);
pub const SEF_DEFAULT_DESCRIPTOR_FOR_OBJECT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(4u32);
pub const SEF_DEFAULT_GROUP_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(64u32);
pub const SEF_DEFAULT_OWNER_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(32u32);
pub const SEF_MACL_NO_EXECUTE_UP: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(1024u32);
pub const SEF_MACL_NO_READ_UP: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(512u32);
pub const SEF_MACL_NO_WRITE_UP: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(256u32);
pub const SEF_SACL_AUTO_INHERIT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(2u32);
impl ::std::convert::From<u32> for SECURITY_AUTO_INHERIT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SECURITY_AUTO_INHERIT_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SECURITY_AUTO_INHERIT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SECURITY_AUTO_INHERIT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SECURITY_AUTO_INHERIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct SECURITY_CAPABILITIES {
    pub AppContainerSid: super::Foundation::PSID,
    pub Capabilities: *mut SID_AND_ATTRIBUTES,
    pub CapabilityCount: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SECURITY_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SECURITY_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SECURITY_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SECURITY_CAPABILITIES").field("AppContainerSid", &self.AppContainerSid).field("Capabilities", &self.Capabilities).field("CapabilityCount", &self.CapabilityCount).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SECURITY_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.AppContainerSid == other.AppContainerSid && self.Capabilities == other.Capabilities && self.CapabilityCount == other.CapabilityCount && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SECURITY_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SECURITY_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct SECURITY_DESCRIPTOR {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: u16,
    pub Owner: super::Foundation::PSID,
    pub Group: super::Foundation::PSID,
    pub Sacl: *mut ACL,
    pub Dacl: *mut ACL,
}
#[cfg(feature = "Win32_Foundation")]
impl SECURITY_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SECURITY_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SECURITY_DESCRIPTOR").field("Revision", &self.Revision).field("Sbz1", &self.Sbz1).field("Control", &self.Control).field("Owner", &self.Owner).field("Group", &self.Group).field("Sacl", &self.Sacl).field("Dacl", &self.Dacl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SECURITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.Sbz1 == other.Sbz1 && self.Control == other.Control && self.Owner == other.Owner && self.Group == other.Group && self.Sacl == other.Sacl && self.Dacl == other.Dacl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SECURITY_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SECURITY_DESCRIPTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SECURITY_IMPERSONATION_LEVEL(pub i32);
pub const SecurityAnonymous: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(0i32);
pub const SecurityIdentification: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(1i32);
pub const SecurityImpersonation: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(2i32);
pub const SecurityDelegation: SECURITY_IMPERSONATION_LEVEL = SECURITY_IMPERSONATION_LEVEL(3i32);
impl ::std::convert::From<i32> for SECURITY_IMPERSONATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SECURITY_IMPERSONATION_LEVEL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct SECURITY_QUALITY_OF_SERVICE {
    pub Length: u32,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub ContextTrackingMode: u8,
    pub EffectiveOnly: super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl SECURITY_QUALITY_OF_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SECURITY_QUALITY_OF_SERVICE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SECURITY_QUALITY_OF_SERVICE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SECURITY_QUALITY_OF_SERVICE").field("Length", &self.Length).field("ImpersonationLevel", &self.ImpersonationLevel).field("ContextTrackingMode", &self.ContextTrackingMode).field("EffectiveOnly", &self.EffectiveOnly).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SECURITY_QUALITY_OF_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.ImpersonationLevel == other.ImpersonationLevel && self.ContextTrackingMode == other.ContextTrackingMode && self.EffectiveOnly == other.EffectiveOnly
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SECURITY_QUALITY_OF_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SECURITY_QUALITY_OF_SERVICE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
pub type SEC_THREAD_START = unsafe extern "system" fn(lpthreadparameter: *mut ::std::ffi::c_void) -> u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SID {
    pub Revision: u8,
    pub SubAuthorityCount: u8,
    pub IdentifierAuthority: SID_IDENTIFIER_AUTHORITY,
    pub SubAuthority: [u32; 1],
}
impl SID {}
impl ::std::default::Default for SID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SID").field("Revision", &self.Revision).field("SubAuthorityCount", &self.SubAuthorityCount).field("IdentifierAuthority", &self.IdentifierAuthority).field("SubAuthority", &self.SubAuthority).finish()
    }
}
impl ::std::cmp::PartialEq for SID {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.SubAuthorityCount == other.SubAuthorityCount && self.IdentifierAuthority == other.IdentifierAuthority && self.SubAuthority == other.SubAuthority
    }
}
impl ::std::cmp::Eq for SID {}
unsafe impl ::windows::runtime::Abi for SID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct SID_AND_ATTRIBUTES {
    pub Sid: super::Foundation::PSID,
    pub Attributes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SID_AND_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SID_AND_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SID_AND_ATTRIBUTES").field("Sid", &self.Sid).field("Attributes", &self.Attributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SID_AND_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Sid == other.Sid && self.Attributes == other.Attributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SID_AND_ATTRIBUTES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct SID_AND_ATTRIBUTES_HASH {
    pub SidCount: u32,
    pub SidAttr: *mut SID_AND_ATTRIBUTES,
    pub Hash: [usize; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl SID_AND_ATTRIBUTES_HASH {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SID_AND_ATTRIBUTES_HASH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SID_AND_ATTRIBUTES_HASH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SID_AND_ATTRIBUTES_HASH").field("SidCount", &self.SidCount).field("SidAttr", &self.SidAttr).field("Hash", &self.Hash).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SID_AND_ATTRIBUTES_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.SidCount == other.SidCount && self.SidAttr == other.SidAttr && self.Hash == other.Hash
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SID_AND_ATTRIBUTES_HASH {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SID_AND_ATTRIBUTES_HASH {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SID_IDENTIFIER_AUTHORITY {
    pub Value: [u8; 6],
}
impl SID_IDENTIFIER_AUTHORITY {}
impl ::std::default::Default for SID_IDENTIFIER_AUTHORITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SID_IDENTIFIER_AUTHORITY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SID_IDENTIFIER_AUTHORITY").field("Value", &self.Value).finish()
    }
}
impl ::std::cmp::PartialEq for SID_IDENTIFIER_AUTHORITY {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::std::cmp::Eq for SID_IDENTIFIER_AUTHORITY {}
unsafe impl ::windows::runtime::Abi for SID_IDENTIFIER_AUTHORITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SID_NAME_USE(pub i32);
pub const SidTypeUser: SID_NAME_USE = SID_NAME_USE(1i32);
pub const SidTypeGroup: SID_NAME_USE = SID_NAME_USE(2i32);
pub const SidTypeDomain: SID_NAME_USE = SID_NAME_USE(3i32);
pub const SidTypeAlias: SID_NAME_USE = SID_NAME_USE(4i32);
pub const SidTypeWellKnownGroup: SID_NAME_USE = SID_NAME_USE(5i32);
pub const SidTypeDeletedAccount: SID_NAME_USE = SID_NAME_USE(6i32);
pub const SidTypeInvalid: SID_NAME_USE = SID_NAME_USE(7i32);
pub const SidTypeUnknown: SID_NAME_USE = SID_NAME_USE(8i32);
pub const SidTypeComputer: SID_NAME_USE = SID_NAME_USE(9i32);
pub const SidTypeLabel: SID_NAME_USE = SID_NAME_USE(10i32);
pub const SidTypeLogonSession: SID_NAME_USE = SID_NAME_USE(11i32);
impl ::std::convert::From<i32> for SID_NAME_USE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SID_NAME_USE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SYSTEM_ALARM_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl SYSTEM_ALARM_ACE {}
impl ::std::default::Default for SYSTEM_ALARM_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_ALARM_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_ALARM_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_ALARM_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_ALARM_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_ALARM_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SYSTEM_ALARM_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl SYSTEM_ALARM_CALLBACK_ACE {}
impl ::std::default::Default for SYSTEM_ALARM_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_ALARM_CALLBACK_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_ALARM_CALLBACK_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_ALARM_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_ALARM_CALLBACK_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_ALARM_CALLBACK_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::runtime::GUID,
    pub InheritedObjectType: ::windows::runtime::GUID,
    pub SidStart: u32,
}
impl SYSTEM_ALARM_CALLBACK_OBJECT_ACE {}
impl ::std::default::Default for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_ALARM_CALLBACK_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SYSTEM_ALARM_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: u32,
    pub ObjectType: ::windows::runtime::GUID,
    pub InheritedObjectType: ::windows::runtime::GUID,
    pub SidStart: u32,
}
impl SYSTEM_ALARM_OBJECT_ACE {}
impl ::std::default::Default for SYSTEM_ALARM_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_ALARM_OBJECT_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_ALARM_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_ALARM_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_ALARM_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_ALARM_OBJECT_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SYSTEM_AUDIT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl SYSTEM_AUDIT_ACE {}
impl ::std::default::Default for SYSTEM_AUDIT_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_AUDIT_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_AUDIT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_AUDIT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_AUDIT_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_AUDIT_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SYSTEM_AUDIT_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl SYSTEM_AUDIT_CALLBACK_ACE {}
impl ::std::default::Default for SYSTEM_AUDIT_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_AUDIT_CALLBACK_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_AUDIT_CALLBACK_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_AUDIT_CALLBACK_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_AUDIT_CALLBACK_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_AUDIT_CALLBACK_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::runtime::GUID,
    pub InheritedObjectType: ::windows::runtime::GUID,
    pub SidStart: u32,
}
impl SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {}
impl ::std::default::Default for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_AUDIT_CALLBACK_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SYSTEM_AUDIT_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows::runtime::GUID,
    pub InheritedObjectType: ::windows::runtime::GUID,
    pub SidStart: u32,
}
impl SYSTEM_AUDIT_OBJECT_ACE {}
impl ::std::default::Default for SYSTEM_AUDIT_OBJECT_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_AUDIT_OBJECT_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_AUDIT_OBJECT_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("Flags", &self.Flags).field("ObjectType", &self.ObjectType).field("InheritedObjectType", &self.InheritedObjectType).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_AUDIT_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.Flags == other.Flags && self.ObjectType == other.ObjectType && self.InheritedObjectType == other.InheritedObjectType && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_AUDIT_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_AUDIT_OBJECT_ACE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYSTEM_AUDIT_OBJECT_ACE_FLAGS(pub u32);
pub const ACE_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS = SYSTEM_AUDIT_OBJECT_ACE_FLAGS(1u32);
pub const ACE_INHERITED_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS = SYSTEM_AUDIT_OBJECT_ACE_FLAGS(2u32);
impl ::std::convert::From<u32> for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SYSTEM_MANDATORY_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl SYSTEM_MANDATORY_LABEL_ACE {}
impl ::std::default::Default for SYSTEM_MANDATORY_LABEL_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_MANDATORY_LABEL_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_MANDATORY_LABEL_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_MANDATORY_LABEL_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_MANDATORY_LABEL_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_MANDATORY_LABEL_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl SYSTEM_RESOURCE_ATTRIBUTE_ACE {}
impl ::std::default::Default for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_RESOURCE_ATTRIBUTE_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_RESOURCE_ATTRIBUTE_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct SYSTEM_SCOPED_POLICY_ID_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl SYSTEM_SCOPED_POLICY_ID_ACE {}
impl ::std::default::Default for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_SCOPED_POLICY_ID_ACE").field("Header", &self.Header).field("Mask", &self.Mask).field("SidStart", &self.SidStart).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Mask == other.Mask && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_SCOPED_POLICY_ID_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_SCOPED_POLICY_ID_ACE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetAclInformation(pacl: *mut ACL, paclinformation: *const ::std::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetAclInformation(pacl: *mut ACL, paclinformation: *const ::std::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetAclInformation(::std::mem::transmute(pacl), ::std::mem::transmute(paclinformation), ::std::mem::transmute(naclinformationlength), ::std::mem::transmute(dwaclinformationclass)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCachedSigningLevel<'a, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(sourcefiles: *const super::Foundation::HANDLE, sourcefilecount: u32, flags: u32, targetfile: Param3) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCachedSigningLevel(sourcefiles: *const super::Foundation::HANDLE, sourcefilecount: u32, flags: u32, targetfile: super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetCachedSigningLevel(::std::mem::transmute(sourcefiles), ::std::mem::transmute(sourcefilecount), ::std::mem::transmute(flags), targetfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileSecurityA<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>>(lpfilename: Param0, securityinformation: u32, psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileSecurityA(lpfilename: super::Foundation::PSTR, securityinformation: u32, psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileSecurityA(lpfilename.into_param().abi(), ::std::mem::transmute(securityinformation), ::std::mem::transmute(psecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFileSecurityW<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>>(lpfilename: Param0, securityinformation: u32, psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileSecurityW(lpfilename: super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileSecurityW(lpfilename.into_param().abi(), ::std::mem::transmute(securityinformation), ::std::mem::transmute(psecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetKernelObjectSecurity<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(handle: Param0, securityinformation: u32, securitydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetKernelObjectSecurity(handle: super::Foundation::HANDLE, securityinformation: u32, securitydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetKernelObjectSecurity(handle.into_param().abi(), ::std::mem::transmute(securityinformation), ::std::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPrivateObjectSecurity<'a, Param4: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(securityinformation: u32, modificationdescriptor: *const SECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, genericmapping: *const GENERIC_MAPPING, token: Param4) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPrivateObjectSecurity(securityinformation: u32, modificationdescriptor: *const SECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, genericmapping: *const GENERIC_MAPPING, token: super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetPrivateObjectSecurity(::std::mem::transmute(securityinformation), ::std::mem::transmute(modificationdescriptor), ::std::mem::transmute(objectssecuritydescriptor), ::std::mem::transmute(genericmapping), token.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPrivateObjectSecurityEx<'a, Param5: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(securityinformation: u32, modificationdescriptor: *const SECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, genericmapping: *const GENERIC_MAPPING, token: Param5) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPrivateObjectSecurityEx(securityinformation: u32, modificationdescriptor: *const SECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, genericmapping: *const GENERIC_MAPPING, token: super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetPrivateObjectSecurityEx(::std::mem::transmute(securityinformation), ::std::mem::transmute(modificationdescriptor), ::std::mem::transmute(objectssecuritydescriptor), ::std::mem::transmute(autoinheritflags), ::std::mem::transmute(genericmapping), token.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`*"]
#[inline]
pub unsafe fn SetSecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32);
        }
        ::std::mem::transmute(SetSecurityAccessMask(::std::mem::transmute(securityinformation), ::std::mem::transmute(desiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityDescriptorControl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, controlbitsofinterest: u16, controlbitstoset: u16) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorControl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, controlbitsofinterest: u16, controlbitstoset: u16) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSecurityDescriptorControl(::std::mem::transmute(psecuritydescriptor), ::std::mem::transmute(controlbitsofinterest), ::std::mem::transmute(controlbitstoset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityDescriptorDacl<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, bdaclpresent: Param1, pdacl: *const ACL, bdacldefaulted: Param3) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorDacl(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, bdaclpresent: super::Foundation::BOOL, pdacl: *const ACL, bdacldefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSecurityDescriptorDacl(::std::mem::transmute(psecuritydescriptor), bdaclpresent.into_param().abi(), ::std::mem::transmute(pdacl), bdacldefaulted.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityDescriptorGroup<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, pgroup: Param1, bgroupdefaulted: Param2) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorGroup(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, pgroup: super::Foundation::PSID, bgroupdefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSecurityDescriptorGroup(::std::mem::transmute(psecuritydescriptor), pgroup.into_param().abi(), bgroupdefaulted.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityDescriptorOwner<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, powner: Param1, bownerdefaulted: Param2) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorOwner(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, powner: super::Foundation::PSID, bownerdefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSecurityDescriptorOwner(::std::mem::transmute(psecuritydescriptor), powner.into_param().abi(), bownerdefaulted.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityDescriptorRMControl(securitydescriptor: *mut SECURITY_DESCRIPTOR, rmcontrol: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorRMControl(securitydescriptor: *mut SECURITY_DESCRIPTOR, rmcontrol: *const u8) -> u32;
        }
        ::std::mem::transmute(SetSecurityDescriptorRMControl(::std::mem::transmute(securitydescriptor), ::std::mem::transmute(rmcontrol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSecurityDescriptorSacl<'a, Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>>(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, bsaclpresent: Param1, psacl: *const ACL, bsacldefaulted: Param3) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorSacl(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, bsaclpresent: super::Foundation::BOOL, psacl: *const ACL, bsacldefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSecurityDescriptorSacl(::std::mem::transmute(psecuritydescriptor), bsaclpresent.into_param().abi(), ::std::mem::transmute(psacl), bsacldefaulted.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTokenInformation<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(tokenhandle: Param0, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: *const ::std::ffi::c_void, tokeninformationlength: u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTokenInformation(tokenhandle: super::Foundation::HANDLE, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: *const ::std::ffi::c_void, tokeninformationlength: u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetTokenInformation(tokenhandle.into_param().abi(), ::std::mem::transmute(tokeninformationclass), ::std::mem::transmute(tokeninformation), ::std::mem::transmute(tokeninformationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUserObjectSecurity<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>>(hobj: Param0, psirequested: *const OBJECT_SECURITY_INFORMATION, psid: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUserObjectSecurity(hobj: super::Foundation::HANDLE, psirequested: *const OBJECT_SECURITY_INFORMATION, psid: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetUserObjectSecurity(hobj.into_param().abi(), ::std::mem::transmute(psirequested), ::std::mem::transmute(psid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_ACCESS_INFORMATION {
    pub SidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub RestrictedSidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub Privileges: *mut TOKEN_PRIVILEGES,
    pub AuthenticationId: super::Foundation::LUID,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub MandatoryPolicy: TOKEN_MANDATORY_POLICY,
    pub Flags: u32,
    pub AppContainerNumber: u32,
    pub PackageSid: super::Foundation::PSID,
    pub CapabilitiesHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub TrustLevelSid: super::Foundation::PSID,
    pub SecurityAttributes: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_ACCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_ACCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_ACCESS_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_ACCESS_INFORMATION")
            .field("SidHash", &self.SidHash)
            .field("RestrictedSidHash", &self.RestrictedSidHash)
            .field("Privileges", &self.Privileges)
            .field("AuthenticationId", &self.AuthenticationId)
            .field("TokenType", &self.TokenType)
            .field("ImpersonationLevel", &self.ImpersonationLevel)
            .field("MandatoryPolicy", &self.MandatoryPolicy)
            .field("Flags", &self.Flags)
            .field("AppContainerNumber", &self.AppContainerNumber)
            .field("PackageSid", &self.PackageSid)
            .field("CapabilitiesHash", &self.CapabilitiesHash)
            .field("TrustLevelSid", &self.TrustLevelSid)
            .field("SecurityAttributes", &self.SecurityAttributes)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_ACCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SidHash == other.SidHash
            && self.RestrictedSidHash == other.RestrictedSidHash
            && self.Privileges == other.Privileges
            && self.AuthenticationId == other.AuthenticationId
            && self.TokenType == other.TokenType
            && self.ImpersonationLevel == other.ImpersonationLevel
            && self.MandatoryPolicy == other.MandatoryPolicy
            && self.Flags == other.Flags
            && self.AppContainerNumber == other.AppContainerNumber
            && self.PackageSid == other.PackageSid
            && self.CapabilitiesHash == other.CapabilitiesHash
            && self.TrustLevelSid == other.TrustLevelSid
            && self.SecurityAttributes == other.SecurityAttributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_ACCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_ACCESS_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TOKEN_ACCESS_MASK(pub u32);
pub const TOKEN_DELETE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(65536u32);
pub const TOKEN_READ_CONTROL: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(131072u32);
pub const TOKEN_WRITE_DAC: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(262144u32);
pub const TOKEN_WRITE_OWNER: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(524288u32);
pub const TOKEN_ACCESS_SYSTEM_SECURITY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(16777216u32);
pub const TOKEN_ASSIGN_PRIMARY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(1u32);
pub const TOKEN_DUPLICATE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(2u32);
pub const TOKEN_IMPERSONATE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(4u32);
pub const TOKEN_QUERY: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(8u32);
pub const TOKEN_QUERY_SOURCE: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(16u32);
pub const TOKEN_ADJUST_PRIVILEGES: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(32u32);
pub const TOKEN_ADJUST_GROUPS: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(64u32);
pub const TOKEN_ADJUST_DEFAULT: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(128u32);
pub const TOKEN_ADJUST_SESSIONID: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(256u32);
pub const TOKEN_ALL_ACCESS: TOKEN_ACCESS_MASK = TOKEN_ACCESS_MASK(983295u32);
impl ::std::convert::From<u32> for TOKEN_ACCESS_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOKEN_ACCESS_MASK {
    type Abi = Self;
}
impl ::std::ops::BitOr for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TOKEN_ACCESS_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TOKEN_ACCESS_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TOKEN_ACCESS_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_APPCONTAINER_INFORMATION {
    pub TokenAppContainer: super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_APPCONTAINER_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_APPCONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_APPCONTAINER_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_APPCONTAINER_INFORMATION").field("TokenAppContainer", &self.TokenAppContainer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_APPCONTAINER_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TokenAppContainer == other.TokenAppContainer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_APPCONTAINER_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_APPCONTAINER_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct TOKEN_AUDIT_POLICY {
    pub PerUserPolicy: [u8; 30],
}
impl TOKEN_AUDIT_POLICY {}
impl ::std::default::Default for TOKEN_AUDIT_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TOKEN_AUDIT_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_AUDIT_POLICY").field("PerUserPolicy", &self.PerUserPolicy).finish()
    }
}
impl ::std::cmp::PartialEq for TOKEN_AUDIT_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.PerUserPolicy == other.PerUserPolicy
    }
}
impl ::std::cmp::Eq for TOKEN_AUDIT_POLICY {}
unsafe impl ::windows::runtime::Abi for TOKEN_AUDIT_POLICY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_CONTROL {
    pub TokenId: super::Foundation::LUID,
    pub AuthenticationId: super::Foundation::LUID,
    pub ModifiedId: super::Foundation::LUID,
    pub TokenSource: TOKEN_SOURCE,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_CONTROL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_CONTROL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_CONTROL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_CONTROL").field("TokenId", &self.TokenId).field("AuthenticationId", &self.AuthenticationId).field("ModifiedId", &self.ModifiedId).field("TokenSource", &self.TokenSource).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.TokenId == other.TokenId && self.AuthenticationId == other.AuthenticationId && self.ModifiedId == other.ModifiedId && self.TokenSource == other.TokenSource
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_CONTROL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_CONTROL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct TOKEN_DEFAULT_DACL {
    pub DefaultDacl: *mut ACL,
}
impl TOKEN_DEFAULT_DACL {}
impl ::std::default::Default for TOKEN_DEFAULT_DACL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TOKEN_DEFAULT_DACL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_DEFAULT_DACL").field("DefaultDacl", &self.DefaultDacl).finish()
    }
}
impl ::std::cmp::PartialEq for TOKEN_DEFAULT_DACL {
    fn eq(&self, other: &Self) -> bool {
        self.DefaultDacl == other.DefaultDacl
    }
}
impl ::std::cmp::Eq for TOKEN_DEFAULT_DACL {}
unsafe impl ::windows::runtime::Abi for TOKEN_DEFAULT_DACL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct TOKEN_DEVICE_CLAIMS {
    pub DeviceClaims: *mut ::std::ffi::c_void,
}
impl TOKEN_DEVICE_CLAIMS {}
impl ::std::default::Default for TOKEN_DEVICE_CLAIMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TOKEN_DEVICE_CLAIMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_DEVICE_CLAIMS").field("DeviceClaims", &self.DeviceClaims).finish()
    }
}
impl ::std::cmp::PartialEq for TOKEN_DEVICE_CLAIMS {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceClaims == other.DeviceClaims
    }
}
impl ::std::cmp::Eq for TOKEN_DEVICE_CLAIMS {}
unsafe impl ::windows::runtime::Abi for TOKEN_DEVICE_CLAIMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct TOKEN_ELEVATION {
    pub TokenIsElevated: u32,
}
impl TOKEN_ELEVATION {}
impl ::std::default::Default for TOKEN_ELEVATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TOKEN_ELEVATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_ELEVATION").field("TokenIsElevated", &self.TokenIsElevated).finish()
    }
}
impl ::std::cmp::PartialEq for TOKEN_ELEVATION {
    fn eq(&self, other: &Self) -> bool {
        self.TokenIsElevated == other.TokenIsElevated
    }
}
impl ::std::cmp::Eq for TOKEN_ELEVATION {}
unsafe impl ::windows::runtime::Abi for TOKEN_ELEVATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TOKEN_ELEVATION_TYPE(pub i32);
pub const TokenElevationTypeDefault: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(1i32);
pub const TokenElevationTypeFull: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(2i32);
pub const TokenElevationTypeLimited: TOKEN_ELEVATION_TYPE = TOKEN_ELEVATION_TYPE(3i32);
impl ::std::convert::From<i32> for TOKEN_ELEVATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOKEN_ELEVATION_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_GROUPS {
    pub GroupCount: u32,
    pub Groups: [SID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_GROUPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_GROUPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_GROUPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_GROUPS").field("GroupCount", &self.GroupCount).field("Groups", &self.Groups).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_GROUPS {
    fn eq(&self, other: &Self) -> bool {
        self.GroupCount == other.GroupCount && self.Groups == other.Groups
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_GROUPS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_GROUPS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_GROUPS_AND_PRIVILEGES {
    pub SidCount: u32,
    pub SidLength: u32,
    pub Sids: *mut SID_AND_ATTRIBUTES,
    pub RestrictedSidCount: u32,
    pub RestrictedSidLength: u32,
    pub RestrictedSids: *mut SID_AND_ATTRIBUTES,
    pub PrivilegeCount: u32,
    pub PrivilegeLength: u32,
    pub Privileges: *mut LUID_AND_ATTRIBUTES,
    pub AuthenticationId: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_GROUPS_AND_PRIVILEGES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_GROUPS_AND_PRIVILEGES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_GROUPS_AND_PRIVILEGES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_GROUPS_AND_PRIVILEGES")
            .field("SidCount", &self.SidCount)
            .field("SidLength", &self.SidLength)
            .field("Sids", &self.Sids)
            .field("RestrictedSidCount", &self.RestrictedSidCount)
            .field("RestrictedSidLength", &self.RestrictedSidLength)
            .field("RestrictedSids", &self.RestrictedSids)
            .field("PrivilegeCount", &self.PrivilegeCount)
            .field("PrivilegeLength", &self.PrivilegeLength)
            .field("Privileges", &self.Privileges)
            .field("AuthenticationId", &self.AuthenticationId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_GROUPS_AND_PRIVILEGES {
    fn eq(&self, other: &Self) -> bool {
        self.SidCount == other.SidCount && self.SidLength == other.SidLength && self.Sids == other.Sids && self.RestrictedSidCount == other.RestrictedSidCount && self.RestrictedSidLength == other.RestrictedSidLength && self.RestrictedSids == other.RestrictedSids && self.PrivilegeCount == other.PrivilegeCount && self.PrivilegeLength == other.PrivilegeLength && self.Privileges == other.Privileges && self.AuthenticationId == other.AuthenticationId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_GROUPS_AND_PRIVILEGES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_GROUPS_AND_PRIVILEGES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TOKEN_INFORMATION_CLASS(pub i32);
pub const TokenUser: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(1i32);
pub const TokenGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(2i32);
pub const TokenPrivileges: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(3i32);
pub const TokenOwner: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(4i32);
pub const TokenPrimaryGroup: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(5i32);
pub const TokenDefaultDacl: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(6i32);
pub const TokenSource: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(7i32);
pub const TokenType: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(8i32);
pub const TokenImpersonationLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(9i32);
pub const TokenStatistics: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(10i32);
pub const TokenRestrictedSids: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(11i32);
pub const TokenSessionId: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(12i32);
pub const TokenGroupsAndPrivileges: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(13i32);
pub const TokenSessionReference: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(14i32);
pub const TokenSandBoxInert: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(15i32);
pub const TokenAuditPolicy: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(16i32);
pub const TokenOrigin: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(17i32);
pub const TokenElevationType: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(18i32);
pub const TokenLinkedToken: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(19i32);
pub const TokenElevation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(20i32);
pub const TokenHasRestrictions: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(21i32);
pub const TokenAccessInformation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(22i32);
pub const TokenVirtualizationAllowed: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(23i32);
pub const TokenVirtualizationEnabled: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(24i32);
pub const TokenIntegrityLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(25i32);
pub const TokenUIAccess: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(26i32);
pub const TokenMandatoryPolicy: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(27i32);
pub const TokenLogonSid: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(28i32);
pub const TokenIsAppContainer: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(29i32);
pub const TokenCapabilities: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(30i32);
pub const TokenAppContainerSid: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(31i32);
pub const TokenAppContainerNumber: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(32i32);
pub const TokenUserClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(33i32);
pub const TokenDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(34i32);
pub const TokenRestrictedUserClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(35i32);
pub const TokenRestrictedDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(36i32);
pub const TokenDeviceGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(37i32);
pub const TokenRestrictedDeviceGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(38i32);
pub const TokenSecurityAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(39i32);
pub const TokenIsRestricted: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(40i32);
pub const TokenProcessTrustLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(41i32);
pub const TokenPrivateNameSpace: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(42i32);
pub const TokenSingletonAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(43i32);
pub const TokenBnoIsolation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(44i32);
pub const TokenChildProcessFlags: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(45i32);
pub const TokenIsLessPrivilegedAppContainer: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(46i32);
pub const TokenIsSandboxed: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(47i32);
pub const MaxTokenInfoClass: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(48i32);
impl ::std::convert::From<i32> for TOKEN_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOKEN_INFORMATION_CLASS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_LINKED_TOKEN {
    pub LinkedToken: super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_LINKED_TOKEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_LINKED_TOKEN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_LINKED_TOKEN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_LINKED_TOKEN").field("LinkedToken", &self.LinkedToken).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_LINKED_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.LinkedToken == other.LinkedToken
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_LINKED_TOKEN {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_LINKED_TOKEN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_MANDATORY_LABEL {
    pub Label: SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_MANDATORY_LABEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_MANDATORY_LABEL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_MANDATORY_LABEL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_MANDATORY_LABEL").field("Label", &self.Label).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_MANDATORY_LABEL {
    fn eq(&self, other: &Self) -> bool {
        self.Label == other.Label
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_MANDATORY_LABEL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_MANDATORY_LABEL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct TOKEN_MANDATORY_POLICY {
    pub Policy: TOKEN_MANDATORY_POLICY_ID,
}
impl TOKEN_MANDATORY_POLICY {}
impl ::std::default::Default for TOKEN_MANDATORY_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TOKEN_MANDATORY_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_MANDATORY_POLICY").field("Policy", &self.Policy).finish()
    }
}
impl ::std::cmp::PartialEq for TOKEN_MANDATORY_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Policy == other.Policy
    }
}
impl ::std::cmp::Eq for TOKEN_MANDATORY_POLICY {}
unsafe impl ::windows::runtime::Abi for TOKEN_MANDATORY_POLICY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TOKEN_MANDATORY_POLICY_ID(pub u32);
pub const TOKEN_MANDATORY_POLICY_OFF: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(0u32);
pub const TOKEN_MANDATORY_POLICY_NO_WRITE_UP: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(1u32);
pub const TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(2u32);
pub const TOKEN_MANDATORY_POLICY_VALID_MASK: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(3u32);
impl ::std::convert::From<u32> for TOKEN_MANDATORY_POLICY_ID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOKEN_MANDATORY_POLICY_ID {
    type Abi = Self;
}
impl ::std::ops::BitOr for TOKEN_MANDATORY_POLICY_ID {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TOKEN_MANDATORY_POLICY_ID {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TOKEN_MANDATORY_POLICY_ID {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TOKEN_MANDATORY_POLICY_ID {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TOKEN_MANDATORY_POLICY_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_ORIGIN {
    pub OriginatingLogonSession: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_ORIGIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_ORIGIN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_ORIGIN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_ORIGIN").field("OriginatingLogonSession", &self.OriginatingLogonSession).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_ORIGIN {
    fn eq(&self, other: &Self) -> bool {
        self.OriginatingLogonSession == other.OriginatingLogonSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_ORIGIN {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_ORIGIN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_OWNER {
    pub Owner: super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_OWNER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_OWNER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_OWNER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_OWNER").field("Owner", &self.Owner).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_OWNER {
    fn eq(&self, other: &Self) -> bool {
        self.Owner == other.Owner
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_OWNER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_OWNER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_PRIMARY_GROUP {
    pub PrimaryGroup: super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_PRIMARY_GROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_PRIMARY_GROUP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_PRIMARY_GROUP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_PRIMARY_GROUP").field("PrimaryGroup", &self.PrimaryGroup).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_PRIMARY_GROUP {
    fn eq(&self, other: &Self) -> bool {
        self.PrimaryGroup == other.PrimaryGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_PRIMARY_GROUP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_PRIMARY_GROUP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: u32,
    pub Privileges: [LUID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_PRIVILEGES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_PRIVILEGES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_PRIVILEGES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_PRIVILEGES").field("PrivilegeCount", &self.PrivilegeCount).field("Privileges", &self.Privileges).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_PRIVILEGES {
    fn eq(&self, other: &Self) -> bool {
        self.PrivilegeCount == other.PrivilegeCount && self.Privileges == other.Privileges
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_PRIVILEGES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_PRIVILEGES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TOKEN_PRIVILEGES_ATTRIBUTES(pub u32);
pub const SE_PRIVILEGE_ENABLED: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(2u32);
pub const SE_PRIVILEGE_ENABLED_BY_DEFAULT: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(1u32);
pub const SE_PRIVILEGE_REMOVED: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(4u32);
pub const SE_PRIVILEGE_USED_FOR_ACCESS: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(2147483648u32);
impl ::std::convert::From<u32> for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Abi = Self;
}
impl ::std::ops::BitOr for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_SOURCE {
    pub SourceName: [super::Foundation::CHAR; 8],
    pub SourceIdentifier: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_SOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_SOURCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_SOURCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_SOURCE").field("SourceName", &self.SourceName).field("SourceIdentifier", &self.SourceIdentifier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.SourceName == other.SourceName && self.SourceIdentifier == other.SourceIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_SOURCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_SOURCE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_STATISTICS {
    pub TokenId: super::Foundation::LUID,
    pub AuthenticationId: super::Foundation::LUID,
    pub ExpirationTime: i64,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub DynamicCharged: u32,
    pub DynamicAvailable: u32,
    pub GroupCount: u32,
    pub PrivilegeCount: u32,
    pub ModifiedId: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_STATISTICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_STATISTICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_STATISTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_STATISTICS")
            .field("TokenId", &self.TokenId)
            .field("AuthenticationId", &self.AuthenticationId)
            .field("ExpirationTime", &self.ExpirationTime)
            .field("TokenType", &self.TokenType)
            .field("ImpersonationLevel", &self.ImpersonationLevel)
            .field("DynamicCharged", &self.DynamicCharged)
            .field("DynamicAvailable", &self.DynamicAvailable)
            .field("GroupCount", &self.GroupCount)
            .field("PrivilegeCount", &self.PrivilegeCount)
            .field("ModifiedId", &self.ModifiedId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.TokenId == other.TokenId && self.AuthenticationId == other.AuthenticationId && self.ExpirationTime == other.ExpirationTime && self.TokenType == other.TokenType && self.ImpersonationLevel == other.ImpersonationLevel && self.DynamicCharged == other.DynamicCharged && self.DynamicAvailable == other.DynamicAvailable && self.GroupCount == other.GroupCount && self.PrivilegeCount == other.PrivilegeCount && self.ModifiedId == other.ModifiedId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_STATISTICS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_STATISTICS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TOKEN_TYPE(pub i32);
pub const TokenPrimary: TOKEN_TYPE = TOKEN_TYPE(1i32);
pub const TokenImpersonation: TOKEN_TYPE = TOKEN_TYPE(2i32);
impl ::std::convert::From<i32> for TOKEN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOKEN_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security`, `Win32_Foundation`*"]
pub struct TOKEN_USER {
    pub User: SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_USER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOKEN_USER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOKEN_USER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_USER").field("User", &self.User).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOKEN_USER {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOKEN_USER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOKEN_USER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security`*"]
pub struct TOKEN_USER_CLAIMS {
    pub UserClaims: *mut ::std::ffi::c_void,
}
impl TOKEN_USER_CLAIMS {}
impl ::std::default::Default for TOKEN_USER_CLAIMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TOKEN_USER_CLAIMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_USER_CLAIMS").field("UserClaims", &self.UserClaims).finish()
    }
}
impl ::std::cmp::PartialEq for TOKEN_USER_CLAIMS {
    fn eq(&self, other: &Self) -> bool {
        self.UserClaims == other.UserClaims
    }
}
impl ::std::cmp::Eq for TOKEN_USER_CLAIMS {}
unsafe impl ::windows::runtime::Abi for TOKEN_USER_CLAIMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WELL_KNOWN_SID_TYPE(pub i32);
pub const WinNullSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(0i32);
pub const WinWorldSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(1i32);
pub const WinLocalSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(2i32);
pub const WinCreatorOwnerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(3i32);
pub const WinCreatorGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(4i32);
pub const WinCreatorOwnerServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(5i32);
pub const WinCreatorGroupServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(6i32);
pub const WinNtAuthoritySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(7i32);
pub const WinDialupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(8i32);
pub const WinNetworkSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(9i32);
pub const WinBatchSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(10i32);
pub const WinInteractiveSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(11i32);
pub const WinServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(12i32);
pub const WinAnonymousSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(13i32);
pub const WinProxySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(14i32);
pub const WinEnterpriseControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(15i32);
pub const WinSelfSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(16i32);
pub const WinAuthenticatedUserSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(17i32);
pub const WinRestrictedCodeSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(18i32);
pub const WinTerminalServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(19i32);
pub const WinRemoteLogonIdSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(20i32);
pub const WinLogonIdsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(21i32);
pub const WinLocalSystemSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(22i32);
pub const WinLocalServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(23i32);
pub const WinNetworkServiceSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(24i32);
pub const WinBuiltinDomainSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(25i32);
pub const WinBuiltinAdministratorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(26i32);
pub const WinBuiltinUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(27i32);
pub const WinBuiltinGuestsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(28i32);
pub const WinBuiltinPowerUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(29i32);
pub const WinBuiltinAccountOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(30i32);
pub const WinBuiltinSystemOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(31i32);
pub const WinBuiltinPrintOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(32i32);
pub const WinBuiltinBackupOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(33i32);
pub const WinBuiltinReplicatorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(34i32);
pub const WinBuiltinPreWindows2000CompatibleAccessSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(35i32);
pub const WinBuiltinRemoteDesktopUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(36i32);
pub const WinBuiltinNetworkConfigurationOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(37i32);
pub const WinAccountAdministratorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(38i32);
pub const WinAccountGuestSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(39i32);
pub const WinAccountKrbtgtSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(40i32);
pub const WinAccountDomainAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(41i32);
pub const WinAccountDomainUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(42i32);
pub const WinAccountDomainGuestsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(43i32);
pub const WinAccountComputersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(44i32);
pub const WinAccountControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(45i32);
pub const WinAccountCertAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(46i32);
pub const WinAccountSchemaAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(47i32);
pub const WinAccountEnterpriseAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(48i32);
pub const WinAccountPolicyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(49i32);
pub const WinAccountRasAndIasServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(50i32);
pub const WinNTLMAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(51i32);
pub const WinDigestAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(52i32);
pub const WinSChannelAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(53i32);
pub const WinThisOrganizationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(54i32);
pub const WinOtherOrganizationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(55i32);
pub const WinBuiltinIncomingForestTrustBuildersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(56i32);
pub const WinBuiltinPerfMonitoringUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(57i32);
pub const WinBuiltinPerfLoggingUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(58i32);
pub const WinBuiltinAuthorizationAccessSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(59i32);
pub const WinBuiltinTerminalServerLicenseServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(60i32);
pub const WinBuiltinDCOMUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(61i32);
pub const WinBuiltinIUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(62i32);
pub const WinIUserSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(63i32);
pub const WinBuiltinCryptoOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(64i32);
pub const WinUntrustedLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(65i32);
pub const WinLowLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(66i32);
pub const WinMediumLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(67i32);
pub const WinHighLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(68i32);
pub const WinSystemLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(69i32);
pub const WinWriteRestrictedCodeSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(70i32);
pub const WinCreatorOwnerRightsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(71i32);
pub const WinCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(72i32);
pub const WinNonCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(73i32);
pub const WinEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(74i32);
pub const WinAccountReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(75i32);
pub const WinBuiltinEventLogReadersGroup: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(76i32);
pub const WinNewEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(77i32);
pub const WinBuiltinCertSvcDComAccessGroup: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(78i32);
pub const WinMediumPlusLabelSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(79i32);
pub const WinLocalLogonSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(80i32);
pub const WinConsoleLogonSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(81i32);
pub const WinThisOrganizationCertificateSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(82i32);
pub const WinApplicationPackageAuthoritySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(83i32);
pub const WinBuiltinAnyPackageSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(84i32);
pub const WinCapabilityInternetClientSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(85i32);
pub const WinCapabilityInternetClientServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(86i32);
pub const WinCapabilityPrivateNetworkClientServerSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(87i32);
pub const WinCapabilityPicturesLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(88i32);
pub const WinCapabilityVideosLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(89i32);
pub const WinCapabilityMusicLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(90i32);
pub const WinCapabilityDocumentsLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(91i32);
pub const WinCapabilitySharedUserCertificatesSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(92i32);
pub const WinCapabilityEnterpriseAuthenticationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(93i32);
pub const WinCapabilityRemovableStorageSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(94i32);
pub const WinBuiltinRDSRemoteAccessServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(95i32);
pub const WinBuiltinRDSEndpointServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(96i32);
pub const WinBuiltinRDSManagementServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(97i32);
pub const WinUserModeDriversSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(98i32);
pub const WinBuiltinHyperVAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(99i32);
pub const WinAccountCloneableControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(100i32);
pub const WinBuiltinAccessControlAssistanceOperatorsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(101i32);
pub const WinBuiltinRemoteManagementUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(102i32);
pub const WinAuthenticationAuthorityAssertedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(103i32);
pub const WinAuthenticationServiceAssertedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(104i32);
pub const WinLocalAccountSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(105i32);
pub const WinLocalAccountAndAdministratorSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(106i32);
pub const WinAccountProtectedUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(107i32);
pub const WinCapabilityAppointmentsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(108i32);
pub const WinCapabilityContactsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(109i32);
pub const WinAccountDefaultSystemManagedSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(110i32);
pub const WinBuiltinDefaultSystemManagedGroupSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(111i32);
pub const WinBuiltinStorageReplicaAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(112i32);
pub const WinAccountKeyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(113i32);
pub const WinAccountEnterpriseKeyAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(114i32);
pub const WinAuthenticationKeyTrustSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(115i32);
pub const WinAuthenticationKeyPropertyMFASid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(116i32);
pub const WinAuthenticationKeyPropertyAttestationSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(117i32);
pub const WinAuthenticationFreshKeyAuthSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(118i32);
pub const WinBuiltinDeviceOwnersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(119i32);
impl ::std::convert::From<i32> for WELL_KNOWN_SID_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WELL_KNOWN_SID_TYPE {
    type Abi = Self;
}
