#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Security_Authentication")]
pub mod Authentication;
#[cfg(feature = "Win32_Security_Authorization")]
pub mod Authorization;
#[cfg(feature = "Win32_Security_Credentials")]
pub mod Credentials;
#[cfg(feature = "Win32_Security_Cryptography")]
pub mod Cryptography;
#[cfg(feature = "Win32_Security_EnterpriseData")]
pub mod EnterpriseData;
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
pub mod ExtensibleAuthenticationProtocol;
#[cfg(feature = "Win32_Security_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_Security_NetworkAccessProtection")]
pub mod NetworkAccessProtection;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("ACCESS_ALLOWED_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("ACCESS_ALLOWED_CALLBACK_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("ACCESS_ALLOWED_CALLBACK_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("ACCESS_ALLOWED_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_ALLOWED_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_ALLOWED_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_ALLOWED_OBJECT_ACE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("ACCESS_DENIED_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("ACCESS_DENIED_CALLBACK_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("ACCESS_DENIED_CALLBACK_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_DENIED_CALLBACK_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("ACCESS_DENIED_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACCESS_DENIED_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for ACCESS_DENIED_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for ACCESS_DENIED_OBJECT_ACE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
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
        fmt.debug_struct("ACE_HEADER")
            .field("AceType", &self.AceType)
            .field("AceFlags", &self.AceFlags)
            .field("AceSize", &self.AceSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.AceType == other.AceType
            && self.AceFlags == other.AceFlags
            && self.AceSize == other.AceSize
    }
}
impl ::std::cmp::Eq for ACE_HEADER {}
unsafe impl ::windows::runtime::Abi for ACE_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
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
        fmt.debug_struct("ACL")
            .field("AclRevision", &self.AclRevision)
            .field("Sbz1", &self.Sbz1)
            .field("AclSize", &self.AclSize)
            .field("AceCount", &self.AceCount)
            .field("Sbz2", &self.Sbz2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACL {
    fn eq(&self, other: &Self) -> bool {
        self.AclRevision == other.AclRevision
            && self.Sbz1 == other.Sbz1
            && self.AclSize == other.AclSize
            && self.AceCount == other.AceCount
            && self.Sbz2 == other.Sbz2
    }
}
impl ::std::cmp::Eq for ACL {}
unsafe impl ::windows::runtime::Abi for ACL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("ACL_REVISION_INFORMATION")
            .field("AclRevision", &self.AclRevision)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("ACL_SIZE_INFORMATION")
            .field("AceCount", &self.AceCount)
            .field("AclBytesInUse", &self.AclBytesInUse)
            .field("AclBytesFree", &self.AclBytesFree)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACL_SIZE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AceCount == other.AceCount
            && self.AclBytesInUse == other.AclBytesInUse
            && self.AclBytesFree == other.AclBytesFree
    }
}
impl ::std::cmp::Eq for ACL_SIZE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for ACL_SIZE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn AccessCheck<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
    clienttoken: Param1,
    desiredaccess: u32,
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
            fn AccessCheck(
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
                clienttoken: super::Foundation::HANDLE,
                desiredaccess: u32,
                genericmapping: *const GENERIC_MAPPING,
                privilegeset: *mut PRIVILEGE_SET,
                privilegesetlength: *mut u32,
                grantedaccess: *mut u32,
                accessstatus: *mut i32,
            ) -> super::Foundation::BOOL;
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AccessCheckAndAuditAlarmA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param7: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
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
            fn AccessCheckAndAuditAlarmA(
                subsystemname: super::Foundation::PSTR,
                handleid: *const ::std::ffi::c_void,
                objecttypename: super::Foundation::PSTR,
                objectname: super::Foundation::PSTR,
                securitydescriptor: *const SECURITY_DESCRIPTOR,
                desiredaccess: u32,
                genericmapping: *const GENERIC_MAPPING,
                objectcreation: super::Foundation::BOOL,
                grantedaccess: *mut u32,
                accessstatus: *mut i32,
                pfgenerateonclose: *mut i32,
            ) -> super::Foundation::BOOL;
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AccessCheckAndAuditAlarmW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param7: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
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
            fn AccessCheckAndAuditAlarmW(
                subsystemname: super::Foundation::PWSTR,
                handleid: *const ::std::ffi::c_void,
                objecttypename: super::Foundation::PWSTR,
                objectname: super::Foundation::PWSTR,
                securitydescriptor: *const SECURITY_DESCRIPTOR,
                desiredaccess: u32,
                genericmapping: *const GENERIC_MAPPING,
                objectcreation: super::Foundation::BOOL,
                grantedaccess: *mut u32,
                accessstatus: *mut i32,
                pfgenerateonclose: *mut i32,
            ) -> super::Foundation::BOOL;
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn AccessCheckByType<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
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
            fn AccessCheckByType(
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
                principalselfsid: super::Foundation::PSID,
                clienttoken: super::Foundation::HANDLE,
                desiredaccess: u32,
                objecttypelist: *mut OBJECT_TYPE_LIST,
                objecttypelistlength: u32,
                genericmapping: *const GENERIC_MAPPING,
                privilegeset: *mut PRIVILEGE_SET,
                privilegesetlength: *mut u32,
                grantedaccess: *mut u32,
                accessstatus: *mut i32,
            ) -> super::Foundation::BOOL;
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AccessCheckByTypeAndAuditAlarmA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param12: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AccessCheckByTypeAndAuditAlarmW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param12: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn AccessCheckByTypeResultList<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
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
            fn AccessCheckByTypeResultList(
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
                principalselfsid: super::Foundation::PSID,
                clienttoken: super::Foundation::HANDLE,
                desiredaccess: u32,
                objecttypelist: *mut OBJECT_TYPE_LIST,
                objecttypelistlength: u32,
                genericmapping: *const GENERIC_MAPPING,
                privilegeset: *mut PRIVILEGE_SET,
                privilegesetlength: *mut u32,
                grantedaccesslist: *mut u32,
                accessstatuslist: *mut u32,
            ) -> super::Foundation::BOOL;
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param12: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmByHandleA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param13: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmByHandleW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param13: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param12: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddAccessAllowedAce<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    pacl: *mut ACL,
    dwacerevision: u32,
    accessmask: u32,
    psid: Param3,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessAllowedAce(
                pacl: *mut ACL,
                dwacerevision: u32,
                accessmask: u32,
                psid: super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessAllowedAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(accessmask),
            psid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddAccessAllowedAceEx<
    'a,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    pacl: *mut ACL,
    dwacerevision: u32,
    aceflags: ACE_FLAGS,
    accessmask: u32,
    psid: Param4,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessAllowedAceEx(
                pacl: *mut ACL,
                dwacerevision: u32,
                aceflags: ACE_FLAGS,
                accessmask: u32,
                psid: super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessAllowedAceEx(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(aceflags),
            ::std::mem::transmute(accessmask),
            psid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddAccessAllowedObjectAce<
    'a,
    Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    pacl: *mut ACL,
    dwacerevision: u32,
    aceflags: ACE_FLAGS,
    accessmask: u32,
    objecttypeguid: *const ::windows::runtime::GUID,
    inheritedobjecttypeguid: *const ::windows::runtime::GUID,
    psid: Param6,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessAllowedObjectAce(
                pacl: *mut ACL,
                dwacerevision: u32,
                aceflags: ACE_FLAGS,
                accessmask: u32,
                objecttypeguid: *const ::windows::runtime::GUID,
                inheritedobjecttypeguid: *const ::windows::runtime::GUID,
                psid: super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessAllowedObjectAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(aceflags),
            ::std::mem::transmute(accessmask),
            ::std::mem::transmute(objecttypeguid),
            ::std::mem::transmute(inheritedobjecttypeguid),
            psid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddAccessDeniedAce<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    pacl: *mut ACL,
    dwacerevision: u32,
    accessmask: u32,
    psid: Param3,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessDeniedAce(
                pacl: *mut ACL,
                dwacerevision: u32,
                accessmask: u32,
                psid: super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessDeniedAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(accessmask),
            psid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddAccessDeniedAceEx<
    'a,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    pacl: *mut ACL,
    dwacerevision: u32,
    aceflags: ACE_FLAGS,
    accessmask: u32,
    psid: Param4,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessDeniedAceEx(
                pacl: *mut ACL,
                dwacerevision: u32,
                aceflags: ACE_FLAGS,
                accessmask: u32,
                psid: super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessDeniedAceEx(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(aceflags),
            ::std::mem::transmute(accessmask),
            psid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddAccessDeniedObjectAce<
    'a,
    Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    pacl: *mut ACL,
    dwacerevision: u32,
    aceflags: ACE_FLAGS,
    accessmask: u32,
    objecttypeguid: *const ::windows::runtime::GUID,
    inheritedobjecttypeguid: *const ::windows::runtime::GUID,
    psid: Param6,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAccessDeniedObjectAce(
                pacl: *mut ACL,
                dwacerevision: u32,
                aceflags: ACE_FLAGS,
                accessmask: u32,
                objecttypeguid: *const ::windows::runtime::GUID,
                inheritedobjecttypeguid: *const ::windows::runtime::GUID,
                psid: super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAccessDeniedObjectAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(aceflags),
            ::std::mem::transmute(accessmask),
            ::std::mem::transmute(objecttypeguid),
            ::std::mem::transmute(inheritedobjecttypeguid),
            psid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddAce(
    pacl: *mut ACL,
    dwacerevision: u32,
    dwstartingaceindex: u32,
    pacelist: *const ::std::ffi::c_void,
    nacelistlength: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAce(
                pacl: *mut ACL,
                dwacerevision: u32,
                dwstartingaceindex: u32,
                pacelist: *const ::std::ffi::c_void,
                nacelistlength: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(dwstartingaceindex),
            ::std::mem::transmute(pacelist),
            ::std::mem::transmute(nacelistlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddAuditAccessAce<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    pacl: *mut ACL,
    dwacerevision: u32,
    dwaccessmask: u32,
    psid: Param3,
    bauditsuccess: Param4,
    bauditfailure: Param5,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAuditAccessAce(
                pacl: *mut ACL,
                dwacerevision: u32,
                dwaccessmask: u32,
                psid: super::Foundation::PSID,
                bauditsuccess: super::Foundation::BOOL,
                bauditfailure: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAuditAccessAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(dwaccessmask),
            psid.into_param().abi(),
            bauditsuccess.into_param().abi(),
            bauditfailure.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddAuditAccessAceEx<
    'a,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
    Param6: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    pacl: *mut ACL,
    dwacerevision: u32,
    aceflags: ACE_FLAGS,
    dwaccessmask: u32,
    psid: Param4,
    bauditsuccess: Param5,
    bauditfailure: Param6,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAuditAccessAceEx(
                pacl: *mut ACL,
                dwacerevision: u32,
                aceflags: ACE_FLAGS,
                dwaccessmask: u32,
                psid: super::Foundation::PSID,
                bauditsuccess: super::Foundation::BOOL,
                bauditfailure: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddAuditAccessAceEx(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(aceflags),
            ::std::mem::transmute(dwaccessmask),
            psid.into_param().abi(),
            bauditsuccess.into_param().abi(),
            bauditfailure.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddAuditAccessObjectAce<
    'a,
    Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param7: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
    Param8: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
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
            fn AddAuditAccessObjectAce(
                pacl: *mut ACL,
                dwacerevision: u32,
                aceflags: ACE_FLAGS,
                accessmask: u32,
                objecttypeguid: *const ::windows::runtime::GUID,
                inheritedobjecttypeguid: *const ::windows::runtime::GUID,
                psid: super::Foundation::PSID,
                bauditsuccess: super::Foundation::BOOL,
                bauditfailure: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddConditionalAce<
    'a,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param6: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    pacl: *mut ACL,
    dwacerevision: u32,
    aceflags: ACE_FLAGS,
    acetype: u8,
    accessmask: u32,
    psid: Param5,
    conditionstr: Param6,
    returnlength: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddConditionalAce(
                pacl: *mut ACL,
                dwacerevision: u32,
                aceflags: ACE_FLAGS,
                acetype: u8,
                accessmask: u32,
                psid: super::Foundation::PSID,
                conditionstr: super::Foundation::PWSTR,
                returnlength: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddConditionalAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(aceflags),
            ::std::mem::transmute(acetype),
            ::std::mem::transmute(accessmask),
            psid.into_param().abi(),
            conditionstr.into_param().abi(),
            ::std::mem::transmute(returnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddMandatoryAce<
    'a,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    pacl: *mut ACL,
    dwacerevision: ACE_REVISION,
    aceflags: ACE_FLAGS,
    mandatorypolicy: u32,
    plabelsid: Param4,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddMandatoryAce(
                pacl: *mut ACL,
                dwacerevision: ACE_REVISION,
                aceflags: ACE_FLAGS,
                mandatorypolicy: u32,
                plabelsid: super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddMandatoryAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(aceflags),
            ::std::mem::transmute(mandatorypolicy),
            plabelsid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddResourceAttributeAce<
    'a,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    pacl: *mut ACL,
    dwacerevision: u32,
    aceflags: ACE_FLAGS,
    accessmask: u32,
    psid: Param4,
    pattributeinfo: *const CLAIM_SECURITY_ATTRIBUTES_INFORMATION,
    preturnlength: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddResourceAttributeAce(
                pacl: *mut ACL,
                dwacerevision: u32,
                aceflags: ACE_FLAGS,
                accessmask: u32,
                psid: super::Foundation::PSID,
                pattributeinfo: *const CLAIM_SECURITY_ATTRIBUTES_INFORMATION,
                preturnlength: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddResourceAttributeAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(aceflags),
            ::std::mem::transmute(accessmask),
            psid.into_param().abi(),
            ::std::mem::transmute(pattributeinfo),
            ::std::mem::transmute(preturnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddScopedPolicyIDAce<
    'a,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    pacl: *mut ACL,
    dwacerevision: u32,
    aceflags: ACE_FLAGS,
    accessmask: u32,
    psid: Param4,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddScopedPolicyIDAce(
                pacl: *mut ACL,
                dwacerevision: u32,
                aceflags: ACE_FLAGS,
                accessmask: u32,
                psid: super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddScopedPolicyIDAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwacerevision),
            ::std::mem::transmute(aceflags),
            ::std::mem::transmute(accessmask),
            psid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AdjustTokenGroups<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    tokenhandle: Param0,
    resettodefault: Param1,
    newstate: *const TOKEN_GROUPS,
    bufferlength: u32,
    previousstate: *mut TOKEN_GROUPS,
    returnlength: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdjustTokenGroups(
                tokenhandle: super::Foundation::HANDLE,
                resettodefault: super::Foundation::BOOL,
                newstate: *const TOKEN_GROUPS,
                bufferlength: u32,
                previousstate: *mut TOKEN_GROUPS,
                returnlength: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AdjustTokenGroups(
            tokenhandle.into_param().abi(),
            resettodefault.into_param().abi(),
            ::std::mem::transmute(newstate),
            ::std::mem::transmute(bufferlength),
            ::std::mem::transmute(previousstate),
            ::std::mem::transmute(returnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn AdjustTokenPrivileges<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    tokenhandle: Param0,
    disableallprivileges: Param1,
    newstate: *const TOKEN_PRIVILEGES,
    bufferlength: u32,
    previousstate: *mut TOKEN_PRIVILEGES,
    returnlength: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdjustTokenPrivileges(
                tokenhandle: super::Foundation::HANDLE,
                disableallprivileges: super::Foundation::BOOL,
                newstate: *const TOKEN_PRIVILEGES,
                bufferlength: u32,
                previousstate: *mut TOKEN_PRIVILEGES,
                returnlength: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AdjustTokenPrivileges(
            tokenhandle.into_param().abi(),
            disableallprivileges.into_param().abi(),
            ::std::mem::transmute(newstate),
            ::std::mem::transmute(bufferlength),
            ::std::mem::transmute(previousstate),
            ::std::mem::transmute(returnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AllocateAndInitializeSid(
    pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY,
    nsubauthoritycount: u8,
    nsubauthority0: u32,
    nsubauthority1: u32,
    nsubauthority2: u32,
    nsubauthority3: u32,
    nsubauthority4: u32,
    nsubauthority5: u32,
    nsubauthority6: u32,
    nsubauthority7: u32,
    psid: *mut super::Foundation::PSID,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocateAndInitializeSid(
                pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY,
                nsubauthoritycount: u8,
                nsubauthority0: u32,
                nsubauthority1: u32,
                nsubauthority2: u32,
                nsubauthority3: u32,
                nsubauthority4: u32,
                nsubauthority5: u32,
                nsubauthority6: u32,
                nsubauthority7: u32,
                psid: *mut super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn AllocateLocallyUniqueId(
    luid: *mut super::System::SystemServices::LUID,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocateLocallyUniqueId(
                luid: *mut super::System::SystemServices::LUID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AllocateLocallyUniqueId(::std::mem::transmute(luid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AreAllAccessesGranted(
    grantedaccess: u32,
    desiredaccess: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AreAllAccessesGranted(
                grantedaccess: u32,
                desiredaccess: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AreAllAccessesGranted(
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(desiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AreAnyAccessesGranted(
    grantedaccess: u32,
    desiredaccess: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AreAnyAccessesGranted(
                grantedaccess: u32,
                desiredaccess: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(AreAnyAccessesGranted(
            ::std::mem::transmute(grantedaccess),
            ::std::mem::transmute(desiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CASetupProperty(pub i32);
pub const ENUM_SETUPPROP_INVALID: CASetupProperty = CASetupProperty(-1i32);
pub const ENUM_SETUPPROP_CATYPE: CASetupProperty = CASetupProperty(0i32);
pub const ENUM_SETUPPROP_CAKEYINFORMATION: CASetupProperty = CASetupProperty(1i32);
pub const ENUM_SETUPPROP_INTERACTIVE: CASetupProperty = CASetupProperty(2i32);
pub const ENUM_SETUPPROP_CANAME: CASetupProperty = CASetupProperty(3i32);
pub const ENUM_SETUPPROP_CADSSUFFIX: CASetupProperty = CASetupProperty(4i32);
pub const ENUM_SETUPPROP_VALIDITYPERIOD: CASetupProperty = CASetupProperty(5i32);
pub const ENUM_SETUPPROP_VALIDITYPERIODUNIT: CASetupProperty = CASetupProperty(6i32);
pub const ENUM_SETUPPROP_EXPIRATIONDATE: CASetupProperty = CASetupProperty(7i32);
pub const ENUM_SETUPPROP_PRESERVEDATABASE: CASetupProperty = CASetupProperty(8i32);
pub const ENUM_SETUPPROP_DATABASEDIRECTORY: CASetupProperty = CASetupProperty(9i32);
pub const ENUM_SETUPPROP_LOGDIRECTORY: CASetupProperty = CASetupProperty(10i32);
pub const ENUM_SETUPPROP_SHAREDFOLDER: CASetupProperty = CASetupProperty(11i32);
pub const ENUM_SETUPPROP_PARENTCAMACHINE: CASetupProperty = CASetupProperty(12i32);
pub const ENUM_SETUPPROP_PARENTCANAME: CASetupProperty = CASetupProperty(13i32);
pub const ENUM_SETUPPROP_REQUESTFILE: CASetupProperty = CASetupProperty(14i32);
pub const ENUM_SETUPPROP_WEBCAMACHINE: CASetupProperty = CASetupProperty(15i32);
pub const ENUM_SETUPPROP_WEBCANAME: CASetupProperty = CASetupProperty(16i32);
impl ::std::convert::From<i32> for CASetupProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CASetupProperty {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CATALOG_INFO {
    pub cbStruct: u32,
    pub wszCatalogFile: [u16; 260],
}
impl CATALOG_INFO {}
impl ::std::default::Default for CATALOG_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CATALOG_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CATALOG_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("wszCatalogFile", &self.wszCatalogFile)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CATALOG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.wszCatalogFile == other.wszCatalogFile
    }
}
impl ::std::cmp::Eq for CATALOG_INFO {}
unsafe impl ::windows::runtime::Abi for CATALOG_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CAT_MEMBERINFO {
    pub pwszSubjGuid: super::Foundation::PWSTR,
    pub dwCertVersion: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CAT_MEMBERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CAT_MEMBERINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CAT_MEMBERINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAT_MEMBERINFO")
            .field("pwszSubjGuid", &self.pwszSubjGuid)
            .field("dwCertVersion", &self.dwCertVersion)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CAT_MEMBERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszSubjGuid == other.pwszSubjGuid && self.dwCertVersion == other.dwCertVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CAT_MEMBERINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CAT_MEMBERINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CAT_MEMBERINFO2 {
    pub SubjectGuid: ::windows::runtime::GUID,
    pub dwCertVersion: u32,
}
impl CAT_MEMBERINFO2 {}
impl ::std::default::Default for CAT_MEMBERINFO2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAT_MEMBERINFO2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAT_MEMBERINFO2")
            .field("SubjectGuid", &self.SubjectGuid)
            .field("dwCertVersion", &self.dwCertVersion)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CAT_MEMBERINFO2 {
    fn eq(&self, other: &Self) -> bool {
        self.SubjectGuid == other.SubjectGuid && self.dwCertVersion == other.dwCertVersion
    }
}
impl ::std::cmp::Eq for CAT_MEMBERINFO2 {}
unsafe impl ::windows::runtime::Abi for CAT_MEMBERINFO2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct CAT_NAMEVALUE {
    pub pwszTag: super::Foundation::PWSTR,
    pub fdwFlags: u32,
    pub Value: Cryptography::Core::CRYPTOAPI_BLOB,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl CAT_NAMEVALUE {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for CAT_NAMEVALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for CAT_NAMEVALUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAT_NAMEVALUE")
            .field("pwszTag", &self.pwszTag)
            .field("fdwFlags", &self.fdwFlags)
            .field("Value", &self.Value)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for CAT_NAMEVALUE {
    fn eq(&self, other: &Self) -> bool {
        self.pwszTag == other.pwszTag
            && self.fdwFlags == other.fdwFlags
            && self.Value == other.Value
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for CAT_NAMEVALUE {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for CAT_NAMEVALUE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CCertSrvSetup: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2518620175,
    62812,
    16701,
    [169, 179, 125, 42, 244, 216, 228, 47],
);
pub const CCertSrvSetupKeyInformation: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        943143174,
        21555,
        17971,
        [176, 251, 41, 183, 231, 130, 98, 225],
    );
pub const CCertificateEnrollmentPolicyServerSetup: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2950887986,
        16817,
        17821,
        [165, 222, 73, 173, 216, 167, 33, 130],
    );
pub const CCertificateEnrollmentServerSetup: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2567107516,
        34991,
        19704,
        [174, 98, 113, 64, 83, 21, 82, 182],
    );
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CEPSetupProperty(pub i32);
pub const ENUM_CEPSETUPPROP_AUTHENTICATION: CEPSetupProperty = CEPSetupProperty(0i32);
pub const ENUM_CEPSETUPPROP_SSLCERTHASH: CEPSetupProperty = CEPSetupProperty(1i32);
pub const ENUM_CEPSETUPPROP_URL: CEPSetupProperty = CEPSetupProperty(2i32);
pub const ENUM_CEPSETUPPROP_KEYBASED_RENEWAL: CEPSetupProperty = CEPSetupProperty(3i32);
impl ::std::convert::From<i32> for CEPSetupProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CEPSetupProperty {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CESSetupProperty(pub i32);
pub const ENUM_CESSETUPPROP_USE_IISAPPPOOLIDENTITY: CESSetupProperty = CESSetupProperty(0i32);
pub const ENUM_CESSETUPPROP_CACONFIG: CESSetupProperty = CESSetupProperty(1i32);
pub const ENUM_CESSETUPPROP_AUTHENTICATION: CESSetupProperty = CESSetupProperty(2i32);
pub const ENUM_CESSETUPPROP_SSLCERTHASH: CESSetupProperty = CESSetupProperty(3i32);
pub const ENUM_CESSETUPPROP_URL: CESSetupProperty = CESSetupProperty(4i32);
pub const ENUM_CESSETUPPROP_RENEWALONLY: CESSetupProperty = CESSetupProperty(5i32);
pub const ENUM_CESSETUPPROP_ALLOW_KEYBASED_RENEWAL: CESSetupProperty = CESSetupProperty(6i32);
impl ::std::convert::From<i32> for CESSetupProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CESSetupProperty {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    type DefaultType = Self;
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
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CLAIM_SECURITY_ATTRIBUTE_FLAGS(pub u32);
pub const CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(1u32);
pub const CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(2u32);
pub const CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(4u32);
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(8u32);
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(16u32);
pub const CLAIM_SECURITY_ATTRIBUTE_MANDATORY: CLAIM_SECURITY_ATTRIBUTE_FLAGS =
    CLAIM_SECURITY_ATTRIBUTE_FLAGS(32u32);
impl ::std::convert::From<u32> for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
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
        fmt.debug_struct("CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE")
            .field("Version", &self.Version)
            .field("Name", &self.Name)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE")
            .field("pValue", &self.pValue)
            .field("ValueLength", &self.ValueLength)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    type DefaultType = Self;
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
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(pub u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(1u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(2u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(3u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(16u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(4u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_SID: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(5u16);
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE =
    CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE(6u16);
impl ::std::convert::From<u16> for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CMSCEPSetup: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2857327618,
    36476,
    18884,
    [148, 250, 103, 165, 204, 94, 173, 180],
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct CONFIG_CI_PROV_INFO {
    pub cbSize: u32,
    pub dwPolicies: u32,
    pub pPolicies: *mut Cryptography::Core::CRYPTOAPI_BLOB,
    pub result: CONFIG_CI_PROV_INFO_RESULT,
    pub dwScenario: u32,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl CONFIG_CI_PROV_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for CONFIG_CI_PROV_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for CONFIG_CI_PROV_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONFIG_CI_PROV_INFO")
            .field("cbSize", &self.cbSize)
            .field("dwPolicies", &self.dwPolicies)
            .field("pPolicies", &self.pPolicies)
            .field("result", &self.result)
            .field("dwScenario", &self.dwScenario)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for CONFIG_CI_PROV_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwPolicies == other.dwPolicies
            && self.pPolicies == other.pPolicies
            && self.result == other.result
            && self.dwScenario == other.dwScenario
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for CONFIG_CI_PROV_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for CONFIG_CI_PROV_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONFIG_CI_PROV_INFO_RESULT {
    pub hr: ::windows::runtime::HRESULT,
    pub dwResult: u32,
    pub dwPolicyIndex: u32,
    pub fIsExplicitDeny: super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl CONFIG_CI_PROV_INFO_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONFIG_CI_PROV_INFO_RESULT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CONFIG_CI_PROV_INFO_RESULT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONFIG_CI_PROV_INFO_RESULT")
            .field("hr", &self.hr)
            .field("dwResult", &self.dwResult)
            .field("dwPolicyIndex", &self.dwPolicyIndex)
            .field("fIsExplicitDeny", &self.fIsExplicitDeny)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONFIG_CI_PROV_INFO_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr
            && self.dwResult == other.dwResult
            && self.dwPolicyIndex == other.dwPolicyIndex
            && self.fIsExplicitDeny == other.fIsExplicitDeny
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONFIG_CI_PROV_INFO_RESULT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONFIG_CI_PROV_INFO_RESULT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CREATE_RESTRICTED_TOKEN_FLAGS(pub u32);
pub const DISABLE_MAX_PRIVILEGE: CREATE_RESTRICTED_TOKEN_FLAGS =
    CREATE_RESTRICTED_TOKEN_FLAGS(1u32);
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
    type DefaultType = Self;
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTCATATTRIBUTE {
    pub cbStruct: u32,
    pub pwszReferenceTag: super::Foundation::PWSTR,
    pub dwAttrTypeAndAction: u32,
    pub cbValue: u32,
    pub pbValue: *mut u8,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPTCATATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPTCATATTRIBUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPTCATATTRIBUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPTCATATTRIBUTE")
            .field("cbStruct", &self.cbStruct)
            .field("pwszReferenceTag", &self.pwszReferenceTag)
            .field("dwAttrTypeAndAction", &self.dwAttrTypeAndAction)
            .field("cbValue", &self.cbValue)
            .field("pbValue", &self.pbValue)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPTCATATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pwszReferenceTag == other.pwszReferenceTag
            && self.dwAttrTypeAndAction == other.dwAttrTypeAndAction
            && self.cbValue == other.cbValue
            && self.pbValue == other.pbValue
            && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPTCATATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPTCATATTRIBUTE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTCATCDF {
    pub cbStruct: u32,
    pub hFile: super::Foundation::HANDLE,
    pub dwCurFilePos: u32,
    pub dwLastMemberOffset: u32,
    pub fEOF: super::Foundation::BOOL,
    pub pwszResultDir: super::Foundation::PWSTR,
    pub hCATStore: super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPTCATCDF {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPTCATCDF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPTCATCDF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPTCATCDF")
            .field("cbStruct", &self.cbStruct)
            .field("hFile", &self.hFile)
            .field("dwCurFilePos", &self.dwCurFilePos)
            .field("dwLastMemberOffset", &self.dwLastMemberOffset)
            .field("fEOF", &self.fEOF)
            .field("pwszResultDir", &self.pwszResultDir)
            .field("hCATStore", &self.hCATStore)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPTCATCDF {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.hFile == other.hFile
            && self.dwCurFilePos == other.dwCurFilePos
            && self.dwLastMemberOffset == other.dwLastMemberOffset
            && self.fEOF == other.fEOF
            && self.pwszResultDir == other.pwszResultDir
            && self.hCATStore == other.hCATStore
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPTCATCDF {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPTCATCDF {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct CRYPTCATMEMBER {
    pub cbStruct: u32,
    pub pwszReferenceTag: super::Foundation::PWSTR,
    pub pwszFileName: super::Foundation::PWSTR,
    pub gSubjectType: ::windows::runtime::GUID,
    pub fdwMemberFlags: u32,
    pub pIndirectData: *mut SIP_INDIRECT_DATA,
    pub dwCertVersion: u32,
    pub dwReserved: u32,
    pub hReserved: super::Foundation::HANDLE,
    pub sEncodedIndirectData: Cryptography::Core::CRYPTOAPI_BLOB,
    pub sEncodedMemberInfo: Cryptography::Core::CRYPTOAPI_BLOB,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl CRYPTCATMEMBER {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for CRYPTCATMEMBER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for CRYPTCATMEMBER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPTCATMEMBER")
            .field("cbStruct", &self.cbStruct)
            .field("pwszReferenceTag", &self.pwszReferenceTag)
            .field("pwszFileName", &self.pwszFileName)
            .field("gSubjectType", &self.gSubjectType)
            .field("fdwMemberFlags", &self.fdwMemberFlags)
            .field("pIndirectData", &self.pIndirectData)
            .field("dwCertVersion", &self.dwCertVersion)
            .field("dwReserved", &self.dwReserved)
            .field("hReserved", &self.hReserved)
            .field("sEncodedIndirectData", &self.sEncodedIndirectData)
            .field("sEncodedMemberInfo", &self.sEncodedMemberInfo)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for CRYPTCATMEMBER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pwszReferenceTag == other.pwszReferenceTag
            && self.pwszFileName == other.pwszFileName
            && self.gSubjectType == other.gSubjectType
            && self.fdwMemberFlags == other.fdwMemberFlags
            && self.pIndirectData == other.pIndirectData
            && self.dwCertVersion == other.dwCertVersion
            && self.dwReserved == other.dwReserved
            && self.hReserved == other.hReserved
            && self.sEncodedIndirectData == other.sEncodedIndirectData
            && self.sEncodedMemberInfo == other.sEncodedMemberInfo
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for CRYPTCATMEMBER {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for CRYPTCATMEMBER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTCATSTORE {
    pub cbStruct: u32,
    pub dwPublicVersion: u32,
    pub pwszP7File: super::Foundation::PWSTR,
    pub hProv: usize,
    pub dwEncodingType: u32,
    pub fdwStoreFlags: CRYPTCAT_OPEN_FLAGS,
    pub hReserved: super::Foundation::HANDLE,
    pub hAttrs: super::Foundation::HANDLE,
    pub hCryptMsg: *mut ::std::ffi::c_void,
    pub hSorted: super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPTCATSTORE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPTCATSTORE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPTCATSTORE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPTCATSTORE")
            .field("cbStruct", &self.cbStruct)
            .field("dwPublicVersion", &self.dwPublicVersion)
            .field("pwszP7File", &self.pwszP7File)
            .field("hProv", &self.hProv)
            .field("dwEncodingType", &self.dwEncodingType)
            .field("fdwStoreFlags", &self.fdwStoreFlags)
            .field("hReserved", &self.hReserved)
            .field("hAttrs", &self.hAttrs)
            .field("hCryptMsg", &self.hCryptMsg)
            .field("hSorted", &self.hSorted)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPTCATSTORE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwPublicVersion == other.dwPublicVersion
            && self.pwszP7File == other.pwszP7File
            && self.hProv == other.hProv
            && self.dwEncodingType == other.dwEncodingType
            && self.fdwStoreFlags == other.fdwStoreFlags
            && self.hReserved == other.hReserved
            && self.hAttrs == other.hAttrs
            && self.hCryptMsg == other.hCryptMsg
            && self.hSorted == other.hSorted
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPTCATSTORE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPTCATSTORE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CRYPTCAT_ADDCATALOG_HARDLINK: u32 = 1u32;
pub const CRYPTCAT_ADDCATALOG_NONE: u32 = 0u32;
pub const CRYPTCAT_ATTR_AUTHENTICATED: u32 = 268435456u32;
pub const CRYPTCAT_ATTR_DATAASCII: u32 = 65536u32;
pub const CRYPTCAT_ATTR_DATABASE64: u32 = 131072u32;
pub const CRYPTCAT_ATTR_DATAREPLACE: u32 = 262144u32;
pub const CRYPTCAT_ATTR_NAMEASCII: u32 = 1u32;
pub const CRYPTCAT_ATTR_NAMEOBJID: u32 = 2u32;
pub const CRYPTCAT_ATTR_NO_AUTO_COMPAT_ENTRY: u32 = 16777216u32;
pub const CRYPTCAT_ATTR_UNAUTHENTICATED: u32 = 536870912u32;
pub const CRYPTCAT_E_AREA_ATTRIBUTE: u32 = 131072u32;
pub const CRYPTCAT_E_AREA_HEADER: u32 = 0u32;
pub const CRYPTCAT_E_AREA_MEMBER: u32 = 65536u32;
pub const CRYPTCAT_E_CDF_ATTR_TOOFEWVALUES: u32 = 131074u32;
pub const CRYPTCAT_E_CDF_ATTR_TYPECOMBO: u32 = 131076u32;
pub const CRYPTCAT_E_CDF_BAD_GUID_CONV: u32 = 131073u32;
pub const CRYPTCAT_E_CDF_DUPLICATE: u32 = 2u32;
pub const CRYPTCAT_E_CDF_MEMBER_FILENOTFOUND: u32 = 65540u32;
pub const CRYPTCAT_E_CDF_MEMBER_FILE_PATH: u32 = 65537u32;
pub const CRYPTCAT_E_CDF_MEMBER_INDIRECTDATA: u32 = 65538u32;
pub const CRYPTCAT_E_CDF_TAGNOTFOUND: u32 = 4u32;
pub const CRYPTCAT_E_CDF_UNSUPPORTED: u32 = 1u32;
pub const CRYPTCAT_MAX_MEMBERTAG: u32 = 64u32;
pub const CRYPTCAT_MEMBER_SORTED: u32 = 1073741824u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CRYPTCAT_OPEN_FLAGS(pub u32);
pub const CRYPTCAT_OPEN_ALWAYS: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(2u32);
pub const CRYPTCAT_OPEN_CREATENEW: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(1u32);
pub const CRYPTCAT_OPEN_EXISTING: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(4u32);
pub const CRYPTCAT_OPEN_EXCLUDE_PAGE_HASHES: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(65536u32);
pub const CRYPTCAT_OPEN_INCLUDE_PAGE_HASHES: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(131072u32);
pub const CRYPTCAT_OPEN_VERIFYSIGHASH: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(268435456u32);
pub const CRYPTCAT_OPEN_NO_CONTENT_HCRYPTMSG: CRYPTCAT_OPEN_FLAGS =
    CRYPTCAT_OPEN_FLAGS(536870912u32);
pub const CRYPTCAT_OPEN_SORTED: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(1073741824u32);
pub const CRYPTCAT_OPEN_FLAGS_MASK: CRYPTCAT_OPEN_FLAGS = CRYPTCAT_OPEN_FLAGS(4294901760u32);
impl ::std::convert::From<u32> for CRYPTCAT_OPEN_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRYPTCAT_OPEN_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CRYPTCAT_OPEN_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CRYPTCAT_OPEN_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CRYPTCAT_OPEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CRYPTCAT_VERSION(pub u32);
pub const CRYPTCAT_VERSION_1: CRYPTCAT_VERSION = CRYPTCAT_VERSION(256u32);
pub const CRYPTCAT_VERSION_2: CRYPTCAT_VERSION = CRYPTCAT_VERSION(512u32);
impl ::std::convert::From<u32> for CRYPTCAT_VERSION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CRYPTCAT_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CRYPTCAT_VERSION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CRYPTCAT_VERSION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CRYPTCAT_VERSION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CRYPTCAT_VERSION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CRYPTCAT_VERSION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CRYPTPROTECTMEMORY_BLOCK_SIZE: u32 = 16u32;
pub const CRYPTPROTECTMEMORY_CROSS_PROCESS: u32 = 1u32;
pub const CRYPTPROTECTMEMORY_SAME_LOGON: u32 = 2u32;
pub const CRYPTPROTECTMEMORY_SAME_PROCESS: u32 = 0u32;
pub const CRYPTPROTECT_AUDIT: u32 = 16u32;
pub const CRYPTPROTECT_CRED_REGENERATE: u32 = 128u32;
pub const CRYPTPROTECT_CRED_SYNC: u32 = 8u32;
pub const CRYPTPROTECT_FIRST_RESERVED_FLAGVAL: u32 = 268435455u32;
pub const CRYPTPROTECT_LAST_RESERVED_FLAGVAL: u32 = 4294967295u32;
pub const CRYPTPROTECT_LOCAL_MACHINE: u32 = 4u32;
pub const CRYPTPROTECT_NO_RECOVERY: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPTPROTECT_PROMPTSTRUCT {
    pub cbSize: u32,
    pub dwPromptFlags: u32,
    pub hwndApp: super::Foundation::HWND,
    pub szPrompt: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPTPROTECT_PROMPTSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPTPROTECT_PROMPTSTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPTPROTECT_PROMPTSTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPTPROTECT_PROMPTSTRUCT")
            .field("cbSize", &self.cbSize)
            .field("dwPromptFlags", &self.dwPromptFlags)
            .field("hwndApp", &self.hwndApp)
            .field("szPrompt", &self.szPrompt)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPTPROTECT_PROMPTSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwPromptFlags == other.dwPromptFlags
            && self.hwndApp == other.hwndApp
            && self.szPrompt == other.szPrompt
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPTPROTECT_PROMPTSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPTPROTECT_PROMPTSTRUCT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CRYPTPROTECT_PROMPT_ON_PROTECT: u32 = 2u32;
pub const CRYPTPROTECT_PROMPT_ON_UNPROTECT: u32 = 1u32;
pub const CRYPTPROTECT_PROMPT_REQUIRE_STRONG: u32 = 16u32;
pub const CRYPTPROTECT_PROMPT_RESERVED: u32 = 4u32;
pub const CRYPTPROTECT_PROMPT_STRONG: u32 = 8u32;
pub const CRYPTPROTECT_UI_FORBIDDEN: u32 = 1u32;
pub const CRYPTPROTECT_VERIFY_PROTECTION: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct CRYPT_PROVIDER_CERT {
    pub cbStruct: u32,
    pub pCert: *mut Cryptography::Core::CERT_CONTEXT,
    pub fCommercial: super::Foundation::BOOL,
    pub fTrustedRoot: super::Foundation::BOOL,
    pub fSelfSigned: super::Foundation::BOOL,
    pub fTestCert: super::Foundation::BOOL,
    pub dwRevokedReason: u32,
    pub dwConfidence: u32,
    pub dwError: u32,
    pub pTrustListContext: *mut Cryptography::Core::CTL_CONTEXT,
    pub fTrustListSignerCert: super::Foundation::BOOL,
    pub pCtlContext: *mut Cryptography::Core::CTL_CONTEXT,
    pub dwCtlError: u32,
    pub fIsCyclic: super::Foundation::BOOL,
    pub pChainElement: *mut Cryptography::Core::CERT_CHAIN_ELEMENT,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl CRYPT_PROVIDER_CERT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for CRYPT_PROVIDER_CERT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for CRYPT_PROVIDER_CERT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_CERT")
            .field("cbStruct", &self.cbStruct)
            .field("pCert", &self.pCert)
            .field("fCommercial", &self.fCommercial)
            .field("fTrustedRoot", &self.fTrustedRoot)
            .field("fSelfSigned", &self.fSelfSigned)
            .field("fTestCert", &self.fTestCert)
            .field("dwRevokedReason", &self.dwRevokedReason)
            .field("dwConfidence", &self.dwConfidence)
            .field("dwError", &self.dwError)
            .field("pTrustListContext", &self.pTrustListContext)
            .field("fTrustListSignerCert", &self.fTrustListSignerCert)
            .field("pCtlContext", &self.pCtlContext)
            .field("dwCtlError", &self.dwCtlError)
            .field("fIsCyclic", &self.fIsCyclic)
            .field("pChainElement", &self.pChainElement)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_CERT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pCert == other.pCert
            && self.fCommercial == other.fCommercial
            && self.fTrustedRoot == other.fTrustedRoot
            && self.fSelfSigned == other.fSelfSigned
            && self.fTestCert == other.fTestCert
            && self.dwRevokedReason == other.dwRevokedReason
            && self.dwConfidence == other.dwConfidence
            && self.dwError == other.dwError
            && self.pTrustListContext == other.pTrustListContext
            && self.fTrustListSignerCert == other.fTrustListSignerCert
            && self.pCtlContext == other.pCtlContext
            && self.dwCtlError == other.dwCtlError
            && self.fIsCyclic == other.fIsCyclic
            && self.pChainElement == other.pChainElement
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_CERT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_CERT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct CRYPT_PROVIDER_DATA {
    pub cbStruct: u32,
    pub pWintrustData: *mut WINTRUST_DATA,
    pub fOpenedFile: super::Foundation::BOOL,
    pub hWndParent: super::Foundation::HWND,
    pub pgActionID: *mut ::windows::runtime::GUID,
    pub hProv: usize,
    pub dwError: u32,
    pub dwRegSecuritySettings: u32,
    pub dwRegPolicySettings: u32,
    pub psPfns: *mut CRYPT_PROVIDER_FUNCTIONS,
    pub cdwTrustStepErrors: u32,
    pub padwTrustStepErrors: *mut u32,
    pub chStores: u32,
    pub pahStores: *mut *mut ::std::ffi::c_void,
    pub dwEncoding: u32,
    pub hMsg: *mut ::std::ffi::c_void,
    pub csSigners: u32,
    pub pasSigners: *mut CRYPT_PROVIDER_SGNR,
    pub csProvPrivData: u32,
    pub pasProvPrivData: *mut CRYPT_PROVIDER_PRIVDATA,
    pub dwSubjectChoice: u32,
    pub Anonymous: CRYPT_PROVIDER_DATA_0,
    pub pszUsageOID: super::Foundation::PSTR,
    pub fRecallWithState: super::Foundation::BOOL,
    pub sftSystemTime: super::Foundation::FILETIME,
    pub pszCTLSignerUsageOID: super::Foundation::PSTR,
    pub dwProvFlags: u32,
    pub dwFinalError: u32,
    pub pRequestUsage: *mut Cryptography::Core::CERT_USAGE_MATCH,
    pub dwTrustPubSettings: u32,
    pub dwUIStateFlags: u32,
    pub pSigState: *mut CRYPT_PROVIDER_SIGSTATE,
    pub pSigSettings: *mut WINTRUST_SIGNATURE_SETTINGS,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl CRYPT_PROVIDER_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for CRYPT_PROVIDER_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub union CRYPT_PROVIDER_DATA_0 {
    pub pPDSip: *mut PROVDATA_SIP,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl CRYPT_PROVIDER_DATA_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for CRYPT_PROVIDER_DATA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_DATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_DATA_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_DATA_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CRYPT_PROVIDER_DEFUSAGE {
    pub cbStruct: u32,
    pub gActionID: ::windows::runtime::GUID,
    pub pDefPolicyCallbackData: *mut ::std::ffi::c_void,
    pub pDefSIPClientData: *mut ::std::ffi::c_void,
}
impl CRYPT_PROVIDER_DEFUSAGE {}
impl ::std::default::Default for CRYPT_PROVIDER_DEFUSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CRYPT_PROVIDER_DEFUSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_DEFUSAGE")
            .field("cbStruct", &self.cbStruct)
            .field("gActionID", &self.gActionID)
            .field("pDefPolicyCallbackData", &self.pDefPolicyCallbackData)
            .field("pDefSIPClientData", &self.pDefSIPClientData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_DEFUSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.gActionID == other.gActionID
            && self.pDefPolicyCallbackData == other.pDefPolicyCallbackData
            && self.pDefSIPClientData == other.pDefSIPClientData
    }
}
impl ::std::cmp::Eq for CRYPT_PROVIDER_DEFUSAGE {}
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_DEFUSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct CRYPT_PROVIDER_FUNCTIONS {
    pub cbStruct: u32,
    pub pfnAlloc: ::std::option::Option<PFN_CPD_MEM_ALLOC>,
    pub pfnFree: ::std::option::Option<PFN_CPD_MEM_FREE>,
    pub pfnAddStore2Chain: ::std::option::Option<PFN_CPD_ADD_STORE>,
    pub pfnAddSgnr2Chain: ::std::option::Option<PFN_CPD_ADD_SGNR>,
    pub pfnAddCert2Chain: ::std::option::Option<PFN_CPD_ADD_CERT>,
    pub pfnAddPrivData2Chain: ::std::option::Option<PFN_CPD_ADD_PRIVDATA>,
    pub pfnInitialize: ::std::option::Option<PFN_PROVIDER_INIT_CALL>,
    pub pfnObjectTrust: ::std::option::Option<PFN_PROVIDER_OBJTRUST_CALL>,
    pub pfnSignatureTrust: ::std::option::Option<PFN_PROVIDER_SIGTRUST_CALL>,
    pub pfnCertificateTrust: ::std::option::Option<PFN_PROVIDER_CERTTRUST_CALL>,
    pub pfnFinalPolicy: ::std::option::Option<PFN_PROVIDER_FINALPOLICY_CALL>,
    pub pfnCertCheckPolicy: ::std::option::Option<PFN_PROVIDER_CERTCHKPOLICY_CALL>,
    pub pfnTestFinalPolicy: ::std::option::Option<PFN_PROVIDER_TESTFINALPOLICY_CALL>,
    pub psUIpfns: *mut CRYPT_PROVUI_FUNCS,
    pub pfnCleanupPolicy: ::std::option::Option<PFN_PROVIDER_CLEANUP_CALL>,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl CRYPT_PROVIDER_FUNCTIONS {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for CRYPT_PROVIDER_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for CRYPT_PROVIDER_FUNCTIONS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_FUNCTIONS")
            .field("cbStruct", &self.cbStruct)
            .field("psUIpfns", &self.psUIpfns)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_FUNCTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pfnAlloc.map(|f| f as usize) == other.pfnAlloc.map(|f| f as usize)
            && self.pfnFree.map(|f| f as usize) == other.pfnFree.map(|f| f as usize)
            && self.pfnAddStore2Chain.map(|f| f as usize)
                == other.pfnAddStore2Chain.map(|f| f as usize)
            && self.pfnAddSgnr2Chain.map(|f| f as usize)
                == other.pfnAddSgnr2Chain.map(|f| f as usize)
            && self.pfnAddCert2Chain.map(|f| f as usize)
                == other.pfnAddCert2Chain.map(|f| f as usize)
            && self.pfnAddPrivData2Chain.map(|f| f as usize)
                == other.pfnAddPrivData2Chain.map(|f| f as usize)
            && self.pfnInitialize.map(|f| f as usize) == other.pfnInitialize.map(|f| f as usize)
            && self.pfnObjectTrust.map(|f| f as usize) == other.pfnObjectTrust.map(|f| f as usize)
            && self.pfnSignatureTrust.map(|f| f as usize)
                == other.pfnSignatureTrust.map(|f| f as usize)
            && self.pfnCertificateTrust.map(|f| f as usize)
                == other.pfnCertificateTrust.map(|f| f as usize)
            && self.pfnFinalPolicy.map(|f| f as usize) == other.pfnFinalPolicy.map(|f| f as usize)
            && self.pfnCertCheckPolicy.map(|f| f as usize)
                == other.pfnCertCheckPolicy.map(|f| f as usize)
            && self.pfnTestFinalPolicy.map(|f| f as usize)
                == other.pfnTestFinalPolicy.map(|f| f as usize)
            && self.psUIpfns == other.psUIpfns
            && self.pfnCleanupPolicy.map(|f| f as usize)
                == other.pfnCleanupPolicy.map(|f| f as usize)
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_FUNCTIONS {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_FUNCTIONS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CRYPT_PROVIDER_PRIVDATA {
    pub cbStruct: u32,
    pub gProviderID: ::windows::runtime::GUID,
    pub cbProvData: u32,
    pub pvProvData: *mut ::std::ffi::c_void,
}
impl CRYPT_PROVIDER_PRIVDATA {}
impl ::std::default::Default for CRYPT_PROVIDER_PRIVDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CRYPT_PROVIDER_PRIVDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_PRIVDATA")
            .field("cbStruct", &self.cbStruct)
            .field("gProviderID", &self.gProviderID)
            .field("cbProvData", &self.cbProvData)
            .field("pvProvData", &self.pvProvData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_PRIVDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.gProviderID == other.gProviderID
            && self.cbProvData == other.cbProvData
            && self.pvProvData == other.pvProvData
    }
}
impl ::std::cmp::Eq for CRYPT_PROVIDER_PRIVDATA {}
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_PRIVDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PROVIDER_REGDEFUSAGE {
    pub cbStruct: u32,
    pub pgActionID: *mut ::windows::runtime::GUID,
    pub pwszDllName: super::Foundation::PWSTR,
    pub pwszLoadCallbackDataFunctionName: super::Foundation::PSTR,
    pub pwszFreeCallbackDataFunctionName: super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPT_PROVIDER_REGDEFUSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPT_PROVIDER_REGDEFUSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPT_PROVIDER_REGDEFUSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_REGDEFUSAGE")
            .field("cbStruct", &self.cbStruct)
            .field("pgActionID", &self.pgActionID)
            .field("pwszDllName", &self.pwszDllName)
            .field(
                "pwszLoadCallbackDataFunctionName",
                &self.pwszLoadCallbackDataFunctionName,
            )
            .field(
                "pwszFreeCallbackDataFunctionName",
                &self.pwszFreeCallbackDataFunctionName,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_REGDEFUSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pgActionID == other.pgActionID
            && self.pwszDllName == other.pwszDllName
            && self.pwszLoadCallbackDataFunctionName == other.pwszLoadCallbackDataFunctionName
            && self.pwszFreeCallbackDataFunctionName == other.pwszFreeCallbackDataFunctionName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPT_PROVIDER_REGDEFUSAGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_REGDEFUSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct CRYPT_PROVIDER_SGNR {
    pub cbStruct: u32,
    pub sftVerifyAsOf: super::Foundation::FILETIME,
    pub csCertChain: u32,
    pub pasCertChain: *mut CRYPT_PROVIDER_CERT,
    pub dwSignerType: u32,
    pub psSigner: *mut Cryptography::Core::CMSG_SIGNER_INFO,
    pub dwError: u32,
    pub csCounterSigners: u32,
    pub pasCounterSigners: *mut CRYPT_PROVIDER_SGNR,
    pub pChainContext: *mut Cryptography::Core::CERT_CHAIN_CONTEXT,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl CRYPT_PROVIDER_SGNR {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for CRYPT_PROVIDER_SGNR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for CRYPT_PROVIDER_SGNR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_SGNR")
            .field("cbStruct", &self.cbStruct)
            .field("sftVerifyAsOf", &self.sftVerifyAsOf)
            .field("csCertChain", &self.csCertChain)
            .field("pasCertChain", &self.pasCertChain)
            .field("dwSignerType", &self.dwSignerType)
            .field("psSigner", &self.psSigner)
            .field("dwError", &self.dwError)
            .field("csCounterSigners", &self.csCounterSigners)
            .field("pasCounterSigners", &self.pasCounterSigners)
            .field("pChainContext", &self.pChainContext)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_SGNR {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.sftVerifyAsOf == other.sftVerifyAsOf
            && self.csCertChain == other.csCertChain
            && self.pasCertChain == other.pasCertChain
            && self.dwSignerType == other.dwSignerType
            && self.psSigner == other.psSigner
            && self.dwError == other.dwError
            && self.csCounterSigners == other.csCounterSigners
            && self.pasCounterSigners == other.pasCounterSigners
            && self.pChainContext == other.pChainContext
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_SGNR {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_SGNR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct CRYPT_PROVIDER_SIGSTATE {
    pub cbStruct: u32,
    pub rhSecondarySigs: *mut *mut ::std::ffi::c_void,
    pub hPrimarySig: *mut ::std::ffi::c_void,
    pub fFirstAttemptMade: super::Foundation::BOOL,
    pub fNoMoreSigs: super::Foundation::BOOL,
    pub cSecondarySigs: u32,
    pub dwCurrentIndex: u32,
    pub fSupportMultiSig: super::Foundation::BOOL,
    pub dwCryptoPolicySupport: u32,
    pub iAttemptCount: u32,
    pub fCheckedSealing: super::Foundation::BOOL,
    pub pSealingSignature: *mut SEALING_SIGNATURE_ATTRIBUTE,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl CRYPT_PROVIDER_SIGSTATE {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for CRYPT_PROVIDER_SIGSTATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for CRYPT_PROVIDER_SIGSTATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVIDER_SIGSTATE")
            .field("cbStruct", &self.cbStruct)
            .field("rhSecondarySigs", &self.rhSecondarySigs)
            .field("hPrimarySig", &self.hPrimarySig)
            .field("fFirstAttemptMade", &self.fFirstAttemptMade)
            .field("fNoMoreSigs", &self.fNoMoreSigs)
            .field("cSecondarySigs", &self.cSecondarySigs)
            .field("dwCurrentIndex", &self.dwCurrentIndex)
            .field("fSupportMultiSig", &self.fSupportMultiSig)
            .field("dwCryptoPolicySupport", &self.dwCryptoPolicySupport)
            .field("iAttemptCount", &self.iAttemptCount)
            .field("fCheckedSealing", &self.fCheckedSealing)
            .field("pSealingSignature", &self.pSealingSignature)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for CRYPT_PROVIDER_SIGSTATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.rhSecondarySigs == other.rhSecondarySigs
            && self.hPrimarySig == other.hPrimarySig
            && self.fFirstAttemptMade == other.fFirstAttemptMade
            && self.fNoMoreSigs == other.fNoMoreSigs
            && self.cSecondarySigs == other.cSecondarySigs
            && self.dwCurrentIndex == other.dwCurrentIndex
            && self.fSupportMultiSig == other.fSupportMultiSig
            && self.dwCryptoPolicySupport == other.dwCryptoPolicySupport
            && self.iAttemptCount == other.iAttemptCount
            && self.fCheckedSealing == other.fCheckedSealing
            && self.pSealingSignature == other.pSealingSignature
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for CRYPT_PROVIDER_SIGSTATE {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVIDER_SIGSTATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PROVUI_DATA {
    pub cbStruct: u32,
    pub dwFinalError: u32,
    pub pYesButtonText: super::Foundation::PWSTR,
    pub pNoButtonText: super::Foundation::PWSTR,
    pub pMoreInfoButtonText: super::Foundation::PWSTR,
    pub pAdvancedLinkText: super::Foundation::PWSTR,
    pub pCopyActionText: super::Foundation::PWSTR,
    pub pCopyActionTextNoTS: super::Foundation::PWSTR,
    pub pCopyActionTextNotSigned: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPT_PROVUI_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPT_PROVUI_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPT_PROVUI_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVUI_DATA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFinalError", &self.dwFinalError)
            .field("pYesButtonText", &self.pYesButtonText)
            .field("pNoButtonText", &self.pNoButtonText)
            .field("pMoreInfoButtonText", &self.pMoreInfoButtonText)
            .field("pAdvancedLinkText", &self.pAdvancedLinkText)
            .field("pCopyActionText", &self.pCopyActionText)
            .field("pCopyActionTextNoTS", &self.pCopyActionTextNoTS)
            .field("pCopyActionTextNotSigned", &self.pCopyActionTextNotSigned)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPT_PROVUI_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwFinalError == other.dwFinalError
            && self.pYesButtonText == other.pYesButtonText
            && self.pNoButtonText == other.pNoButtonText
            && self.pMoreInfoButtonText == other.pMoreInfoButtonText
            && self.pAdvancedLinkText == other.pAdvancedLinkText
            && self.pCopyActionText == other.pCopyActionText
            && self.pCopyActionTextNoTS == other.pCopyActionTextNoTS
            && self.pCopyActionTextNotSigned == other.pCopyActionTextNotSigned
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPT_PROVUI_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVUI_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct CRYPT_PROVUI_FUNCS {
    pub cbStruct: u32,
    pub psUIData: *mut CRYPT_PROVUI_DATA,
    pub pfnOnMoreInfoClick: ::std::option::Option<PFN_PROVUI_CALL>,
    pub pfnOnMoreInfoClickDefault: ::std::option::Option<PFN_PROVUI_CALL>,
    pub pfnOnAdvancedClick: ::std::option::Option<PFN_PROVUI_CALL>,
    pub pfnOnAdvancedClickDefault: ::std::option::Option<PFN_PROVUI_CALL>,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl CRYPT_PROVUI_FUNCS {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for CRYPT_PROVUI_FUNCS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for CRYPT_PROVUI_FUNCS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_PROVUI_FUNCS")
            .field("cbStruct", &self.cbStruct)
            .field("psUIData", &self.psUIData)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for CRYPT_PROVUI_FUNCS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.psUIData == other.psUIData
            && self.pfnOnMoreInfoClick.map(|f| f as usize)
                == other.pfnOnMoreInfoClick.map(|f| f as usize)
            && self.pfnOnMoreInfoClickDefault.map(|f| f as usize)
                == other.pfnOnMoreInfoClickDefault.map(|f| f as usize)
            && self.pfnOnAdvancedClick.map(|f| f as usize)
                == other.pfnOnAdvancedClick.map(|f| f as usize)
            && self.pfnOnAdvancedClickDefault.map(|f| f as usize)
                == other.pfnOnAdvancedClickDefault.map(|f| f as usize)
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for CRYPT_PROVUI_FUNCS {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for CRYPT_PROVUI_FUNCS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_REGISTER_ACTIONID {
    pub cbStruct: u32,
    pub sInitProvider: CRYPT_TRUST_REG_ENTRY,
    pub sObjectProvider: CRYPT_TRUST_REG_ENTRY,
    pub sSignatureProvider: CRYPT_TRUST_REG_ENTRY,
    pub sCertificateProvider: CRYPT_TRUST_REG_ENTRY,
    pub sCertificatePolicyProvider: CRYPT_TRUST_REG_ENTRY,
    pub sFinalPolicyProvider: CRYPT_TRUST_REG_ENTRY,
    pub sTestPolicyProvider: CRYPT_TRUST_REG_ENTRY,
    pub sCleanupProvider: CRYPT_TRUST_REG_ENTRY,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPT_REGISTER_ACTIONID {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPT_REGISTER_ACTIONID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPT_REGISTER_ACTIONID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_REGISTER_ACTIONID")
            .field("cbStruct", &self.cbStruct)
            .field("sInitProvider", &self.sInitProvider)
            .field("sObjectProvider", &self.sObjectProvider)
            .field("sSignatureProvider", &self.sSignatureProvider)
            .field("sCertificateProvider", &self.sCertificateProvider)
            .field(
                "sCertificatePolicyProvider",
                &self.sCertificatePolicyProvider,
            )
            .field("sFinalPolicyProvider", &self.sFinalPolicyProvider)
            .field("sTestPolicyProvider", &self.sTestPolicyProvider)
            .field("sCleanupProvider", &self.sCleanupProvider)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPT_REGISTER_ACTIONID {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.sInitProvider == other.sInitProvider
            && self.sObjectProvider == other.sObjectProvider
            && self.sSignatureProvider == other.sSignatureProvider
            && self.sCertificateProvider == other.sCertificateProvider
            && self.sCertificatePolicyProvider == other.sCertificatePolicyProvider
            && self.sFinalPolicyProvider == other.sFinalPolicyProvider
            && self.sTestPolicyProvider == other.sTestPolicyProvider
            && self.sCleanupProvider == other.sCleanupProvider
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPT_REGISTER_ACTIONID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPT_REGISTER_ACTIONID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TRUST_REG_ENTRY {
    pub cbStruct: u32,
    pub pwszDLLName: super::Foundation::PWSTR,
    pub pwszFunctionName: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CRYPT_TRUST_REG_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CRYPT_TRUST_REG_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CRYPT_TRUST_REG_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CRYPT_TRUST_REG_ENTRY")
            .field("cbStruct", &self.cbStruct)
            .field("pwszDLLName", &self.pwszDLLName)
            .field("pwszFunctionName", &self.pwszFunctionName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CRYPT_TRUST_REG_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pwszDLLName == other.pwszDLLName
            && self.pwszFunctionName == other.pwszFunctionName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CRYPT_TRUST_REG_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CRYPT_TRUST_REG_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CVT_SECONDS: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CheckTokenCapability<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    tokenhandle: Param0,
    capabilitysidtocheck: Param1,
    hascapability: *mut super::Foundation::BOOL,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckTokenCapability(
                tokenhandle: super::Foundation::HANDLE,
                capabilitysidtocheck: super::Foundation::PSID,
                hascapability: *mut super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckTokenCapability(
            tokenhandle.into_param().abi(),
            capabilitysidtocheck.into_param().abi(),
            ::std::mem::transmute(hascapability),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CheckTokenMembership<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    tokenhandle: Param0,
    sidtocheck: Param1,
    ismember: *mut super::Foundation::BOOL,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckTokenMembership(
                tokenhandle: super::Foundation::HANDLE,
                sidtocheck: super::Foundation::PSID,
                ismember: *mut super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckTokenMembership(
            tokenhandle.into_param().abi(),
            sidtocheck.into_param().abi(),
            ::std::mem::transmute(ismember),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CheckTokenMembershipEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    tokenhandle: Param0,
    sidtocheck: Param1,
    flags: u32,
    ismember: *mut super::Foundation::BOOL,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckTokenMembershipEx(
                tokenhandle: super::Foundation::HANDLE,
                sidtocheck: super::Foundation::PSID,
                flags: u32,
                ismember: *mut super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckTokenMembershipEx(
            tokenhandle.into_param().abi(),
            sidtocheck.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(ismember),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ConvertToAutoInheritPrivateObjectSecurity<
    'a,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::BOOLEAN>,
>(
    parentdescriptor: *const SECURITY_DESCRIPTOR,
    currentsecuritydescriptor: *const SECURITY_DESCRIPTOR,
    newsecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR,
    objecttype: *const ::windows::runtime::GUID,
    isdirectoryobject: Param4,
    genericmapping: *const GENERIC_MAPPING,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertToAutoInheritPrivateObjectSecurity(
                parentdescriptor: *const SECURITY_DESCRIPTOR,
                currentsecuritydescriptor: *const SECURITY_DESCRIPTOR,
                newsecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR,
                objecttype: *const ::windows::runtime::GUID,
                isdirectoryobject: super::Foundation::BOOLEAN,
                genericmapping: *const GENERIC_MAPPING,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ConvertToAutoInheritPrivateObjectSecurity(
            ::std::mem::transmute(parentdescriptor),
            ::std::mem::transmute(currentsecuritydescriptor),
            ::std::mem::transmute(newsecuritydescriptor),
            ::std::mem::transmute(objecttype),
            isdirectoryobject.into_param().abi(),
            ::std::mem::transmute(genericmapping),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CopySid<'a, Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(
    ndestinationsidlength: u32,
    pdestinationsid: super::Foundation::PSID,
    psourcesid: Param2,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopySid(
                ndestinationsidlength: u32,
                pdestinationsid: super::Foundation::PSID,
                psourcesid: super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CopySid(
            ::std::mem::transmute(ndestinationsidlength),
            ::std::mem::transmute(pdestinationsid),
            psourcesid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreatePrivateObjectSecurity<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    parentdescriptor: *const SECURITY_DESCRIPTOR,
    creatordescriptor: *const SECURITY_DESCRIPTOR,
    newdescriptor: *mut *mut SECURITY_DESCRIPTOR,
    isdirectoryobject: Param3,
    token: Param4,
    genericmapping: *const GENERIC_MAPPING,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePrivateObjectSecurity(
                parentdescriptor: *const SECURITY_DESCRIPTOR,
                creatordescriptor: *const SECURITY_DESCRIPTOR,
                newdescriptor: *mut *mut SECURITY_DESCRIPTOR,
                isdirectoryobject: super::Foundation::BOOL,
                token: super::Foundation::HANDLE,
                genericmapping: *const GENERIC_MAPPING,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreatePrivateObjectSecurity(
            ::std::mem::transmute(parentdescriptor),
            ::std::mem::transmute(creatordescriptor),
            ::std::mem::transmute(newdescriptor),
            isdirectoryobject.into_param().abi(),
            token.into_param().abi(),
            ::std::mem::transmute(genericmapping),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreatePrivateObjectSecurityEx<
    'a,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
    Param6: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
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
            fn CreatePrivateObjectSecurityEx(
                parentdescriptor: *const SECURITY_DESCRIPTOR,
                creatordescriptor: *const SECURITY_DESCRIPTOR,
                newdescriptor: *mut *mut SECURITY_DESCRIPTOR,
                objecttype: *const ::windows::runtime::GUID,
                iscontainerobject: super::Foundation::BOOL,
                autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS,
                token: super::Foundation::HANDLE,
                genericmapping: *const GENERIC_MAPPING,
            ) -> super::Foundation::BOOL;
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreatePrivateObjectSecurityWithMultipleInheritance<
    'a,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
    Param7: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
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
            fn CreatePrivateObjectSecurityWithMultipleInheritance(
                parentdescriptor: *const SECURITY_DESCRIPTOR,
                creatordescriptor: *const SECURITY_DESCRIPTOR,
                newdescriptor: *mut *mut SECURITY_DESCRIPTOR,
                objecttypes: *const *const ::windows::runtime::GUID,
                guidcount: u32,
                iscontainerobject: super::Foundation::BOOL,
                autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS,
                token: super::Foundation::HANDLE,
                genericmapping: *const GENERIC_MAPPING,
            ) -> super::Foundation::BOOL;
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn CreateRestrictedToken<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    existingtokenhandle: Param0,
    flags: CREATE_RESTRICTED_TOKEN_FLAGS,
    disablesidcount: u32,
    sidstodisable: *const SID_AND_ATTRIBUTES,
    deleteprivilegecount: u32,
    privilegestodelete: *const LUID_AND_ATTRIBUTES,
    restrictedsidcount: u32,
    sidstorestrict: *const SID_AND_ATTRIBUTES,
    newtokenhandle: *mut super::Foundation::HANDLE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRestrictedToken(
                existingtokenhandle: super::Foundation::HANDLE,
                flags: CREATE_RESTRICTED_TOKEN_FLAGS,
                disablesidcount: u32,
                sidstodisable: *const SID_AND_ATTRIBUTES,
                deleteprivilegecount: u32,
                privilegestodelete: *const LUID_AND_ATTRIBUTES,
                restrictedsidcount: u32,
                sidstorestrict: *const SID_AND_ATTRIBUTES,
                newtokenhandle: *mut super::Foundation::HANDLE,
            ) -> super::Foundation::BOOL;
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateWellKnownSid<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    wellknownsidtype: WELL_KNOWN_SID_TYPE,
    domainsid: Param1,
    psid: super::Foundation::PSID,
    cbsid: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWellKnownSid(
                wellknownsidtype: WELL_KNOWN_SID_TYPE,
                domainsid: super::Foundation::PSID,
                psid: super::Foundation::PSID,
                cbsid: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateWellKnownSid(
            ::std::mem::transmute(wellknownsidtype),
            domainsid.into_param().abi(),
            ::std::mem::transmute(psid),
            ::std::mem::transmute(cbsid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATAdminAcquireContext(
    phcatadmin: *mut isize,
    pgsubsystem: *const ::windows::runtime::GUID,
    dwflags: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminAcquireContext(
                phcatadmin: *mut isize,
                pgsubsystem: *const ::windows::runtime::GUID,
                dwflags: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminAcquireContext(
            ::std::mem::transmute(phcatadmin),
            ::std::mem::transmute(pgsubsystem),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptCATAdminAcquireContext2<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    phcatadmin: *mut isize,
    pgsubsystem: *const ::windows::runtime::GUID,
    pwszhashalgorithm: Param2,
    pstronghashpolicy: *const Cryptography::Core::CERT_STRONG_SIGN_PARA,
    dwflags: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminAcquireContext2(
                phcatadmin: *mut isize,
                pgsubsystem: *const ::windows::runtime::GUID,
                pwszhashalgorithm: super::Foundation::PWSTR,
                pstronghashpolicy: *const Cryptography::Core::CERT_STRONG_SIGN_PARA,
                dwflags: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminAcquireContext2(
            ::std::mem::transmute(phcatadmin),
            ::std::mem::transmute(pgsubsystem),
            pwszhashalgorithm.into_param().abi(),
            ::std::mem::transmute(pstronghashpolicy),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATAdminAddCatalog<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hcatadmin: isize,
    pwszcatalogfile: Param1,
    pwszselectbasename: Param2,
    dwflags: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminAddCatalog(
                hcatadmin: isize,
                pwszcatalogfile: super::Foundation::PWSTR,
                pwszselectbasename: super::Foundation::PWSTR,
                dwflags: u32,
            ) -> isize;
        }
        ::std::mem::transmute(CryptCATAdminAddCatalog(
            ::std::mem::transmute(hcatadmin),
            pwszcatalogfile.into_param().abi(),
            pwszselectbasename.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hfile: Param0,
    pcbhash: *mut u32,
    pbhash: *mut u8,
    dwflags: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminCalcHashFromFileHandle(
                hfile: super::Foundation::HANDLE,
                pcbhash: *mut u32,
                pbhash: *mut u8,
                dwflags: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminCalcHashFromFileHandle(
            hfile.into_param().abi(),
            ::std::mem::transmute(pcbhash),
            ::std::mem::transmute(pbhash),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATAdminCalcHashFromFileHandle2<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hcatadmin: isize,
    hfile: Param1,
    pcbhash: *mut u32,
    pbhash: *mut u8,
    dwflags: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminCalcHashFromFileHandle2(
                hcatadmin: isize,
                hfile: super::Foundation::HANDLE,
                pcbhash: *mut u32,
                pbhash: *mut u8,
                dwflags: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminCalcHashFromFileHandle2(
            ::std::mem::transmute(hcatadmin),
            hfile.into_param().abi(),
            ::std::mem::transmute(pcbhash),
            ::std::mem::transmute(pbhash),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn CryptCATAdminEnumCatalogFromHash(
    hcatadmin: isize,
    pbhash: *const u8,
    cbhash: u32,
    dwflags: u32,
    phprevcatinfo: *mut isize,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminEnumCatalogFromHash(
                hcatadmin: isize,
                pbhash: *const u8,
                cbhash: u32,
                dwflags: u32,
                phprevcatinfo: *mut isize,
            ) -> isize;
        }
        ::std::mem::transmute(CryptCATAdminEnumCatalogFromHash(
            ::std::mem::transmute(hcatadmin),
            ::std::mem::transmute(pbhash),
            ::std::mem::transmute(cbhash),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(phprevcatinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATAdminPauseServiceForBackup<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    dwflags: u32,
    fresume: Param1,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminPauseServiceForBackup(
                dwflags: u32,
                fresume: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminPauseServiceForBackup(
            ::std::mem::transmute(dwflags),
            fresume.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATAdminReleaseCatalogContext(
    hcatadmin: isize,
    hcatinfo: isize,
    dwflags: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminReleaseCatalogContext(
                hcatadmin: isize,
                hcatinfo: isize,
                dwflags: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminReleaseCatalogContext(
            ::std::mem::transmute(hcatadmin),
            ::std::mem::transmute(hcatinfo),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATAdminReleaseContext(
    hcatadmin: isize,
    dwflags: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminReleaseContext(
                hcatadmin: isize,
                dwflags: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminReleaseContext(
            ::std::mem::transmute(hcatadmin),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATAdminRemoveCatalog<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hcatadmin: isize,
    pwszcatalogfile: Param1,
    dwflags: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminRemoveCatalog(
                hcatadmin: isize,
                pwszcatalogfile: super::Foundation::PWSTR,
                dwflags: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminRemoveCatalog(
            ::std::mem::transmute(hcatadmin),
            pwszcatalogfile.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATAdminResolveCatalogPath<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hcatadmin: isize,
    pwszcatalogfile: Param1,
    pscatinfo: *mut CATALOG_INFO,
    dwflags: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAdminResolveCatalogPath(
                hcatadmin: isize,
                pwszcatalogfile: super::Foundation::PWSTR,
                pscatinfo: *mut CATALOG_INFO,
                dwflags: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATAdminResolveCatalogPath(
            ::std::mem::transmute(hcatadmin),
            pwszcatalogfile.into_param().abi(),
            ::std::mem::transmute(pscatinfo),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptCATAllocSortedMemberInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hcatalog: Param0,
    pwszreferencetag: Param1,
) -> *mut CRYPTCATMEMBER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATAllocSortedMemberInfo(
                hcatalog: super::Foundation::HANDLE,
                pwszreferencetag: super::Foundation::PWSTR,
            ) -> *mut CRYPTCATMEMBER;
        }
        ::std::mem::transmute(CryptCATAllocSortedMemberInfo(
            hcatalog.into_param().abi(),
            pwszreferencetag.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATCDFClose(pcdf: *mut CRYPTCATCDF) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCDFClose(pcdf: *mut CRYPTCATCDF) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATCDFClose(::std::mem::transmute(pcdf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptCATCDFEnumAttributes(
    pcdf: *mut CRYPTCATCDF,
    pmember: *mut CRYPTCATMEMBER,
    pprevattr: *mut CRYPTCATATTRIBUTE,
    pfnparseerror: ::std::option::Option<PFN_CDF_PARSE_ERROR_CALLBACK>,
) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCDFEnumAttributes(
                pcdf: *mut CRYPTCATCDF,
                pmember: *mut CRYPTCATMEMBER,
                pprevattr: *mut CRYPTCATATTRIBUTE,
                pfnparseerror: ::windows::runtime::RawPtr,
            ) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATCDFEnumAttributes(
            ::std::mem::transmute(pcdf),
            ::std::mem::transmute(pmember),
            ::std::mem::transmute(pprevattr),
            ::std::mem::transmute(pfnparseerror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATCDFEnumCatAttributes(
    pcdf: *mut CRYPTCATCDF,
    pprevattr: *mut CRYPTCATATTRIBUTE,
    pfnparseerror: ::std::option::Option<PFN_CDF_PARSE_ERROR_CALLBACK>,
) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCDFEnumCatAttributes(
                pcdf: *mut CRYPTCATCDF,
                pprevattr: *mut CRYPTCATATTRIBUTE,
                pfnparseerror: ::windows::runtime::RawPtr,
            ) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATCDFEnumCatAttributes(
            ::std::mem::transmute(pcdf),
            ::std::mem::transmute(pprevattr),
            ::std::mem::transmute(pfnparseerror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptCATCDFEnumMembers(
    pcdf: *mut CRYPTCATCDF,
    pprevmember: *mut CRYPTCATMEMBER,
    pfnparseerror: ::std::option::Option<PFN_CDF_PARSE_ERROR_CALLBACK>,
) -> *mut CRYPTCATMEMBER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCDFEnumMembers(
                pcdf: *mut CRYPTCATCDF,
                pprevmember: *mut CRYPTCATMEMBER,
                pfnparseerror: ::windows::runtime::RawPtr,
            ) -> *mut CRYPTCATMEMBER;
        }
        ::std::mem::transmute(CryptCATCDFEnumMembers(
            ::std::mem::transmute(pcdf),
            ::std::mem::transmute(pprevmember),
            ::std::mem::transmute(pfnparseerror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATCDFOpen<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    pwszfilepath: Param0,
    pfnparseerror: ::std::option::Option<PFN_CDF_PARSE_ERROR_CALLBACK>,
) -> *mut CRYPTCATCDF {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCDFOpen(
                pwszfilepath: super::Foundation::PWSTR,
                pfnparseerror: ::windows::runtime::RawPtr,
            ) -> *mut CRYPTCATCDF;
        }
        ::std::mem::transmute(CryptCATCDFOpen(
            pwszfilepath.into_param().abi(),
            ::std::mem::transmute(pfnparseerror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATCatalogInfoFromContext(
    hcatinfo: isize,
    pscatinfo: *mut CATALOG_INFO,
    dwflags: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATCatalogInfoFromContext(
                hcatinfo: isize,
                pscatinfo: *mut CATALOG_INFO,
                dwflags: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATCatalogInfoFromContext(
            ::std::mem::transmute(hcatinfo),
            ::std::mem::transmute(pscatinfo),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATClose<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hcatalog: Param0,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATClose(hcatalog: super::Foundation::HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATClose(hcatalog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptCATEnumerateAttr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hcatalog: Param0,
    pcatmember: *mut CRYPTCATMEMBER,
    pprevattr: *mut CRYPTCATATTRIBUTE,
) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATEnumerateAttr(
                hcatalog: super::Foundation::HANDLE,
                pcatmember: *mut CRYPTCATMEMBER,
                pprevattr: *mut CRYPTCATATTRIBUTE,
            ) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATEnumerateAttr(
            hcatalog.into_param().abi(),
            ::std::mem::transmute(pcatmember),
            ::std::mem::transmute(pprevattr),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATEnumerateCatAttr<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hcatalog: Param0,
    pprevattr: *mut CRYPTCATATTRIBUTE,
) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATEnumerateCatAttr(
                hcatalog: super::Foundation::HANDLE,
                pprevattr: *mut CRYPTCATATTRIBUTE,
            ) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATEnumerateCatAttr(
            hcatalog.into_param().abi(),
            ::std::mem::transmute(pprevattr),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptCATEnumerateMember<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hcatalog: Param0,
    pprevmember: *mut CRYPTCATMEMBER,
) -> *mut CRYPTCATMEMBER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATEnumerateMember(
                hcatalog: super::Foundation::HANDLE,
                pprevmember: *mut CRYPTCATMEMBER,
            ) -> *mut CRYPTCATMEMBER;
        }
        ::std::mem::transmute(CryptCATEnumerateMember(
            hcatalog.into_param().abi(),
            ::std::mem::transmute(pprevmember),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptCATFreeSortedMemberInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hcatalog: Param0,
    pcatmember: *mut CRYPTCATMEMBER,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATFreeSortedMemberInfo(
                hcatalog: super::Foundation::HANDLE,
                pcatmember: *mut CRYPTCATMEMBER,
            );
        }
        ::std::mem::transmute(CryptCATFreeSortedMemberInfo(
            hcatalog.into_param().abi(),
            ::std::mem::transmute(pcatmember),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptCATGetAttrInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hcatalog: Param0,
    pcatmember: *mut CRYPTCATMEMBER,
    pwszreferencetag: Param2,
) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATGetAttrInfo(
                hcatalog: super::Foundation::HANDLE,
                pcatmember: *mut CRYPTCATMEMBER,
                pwszreferencetag: super::Foundation::PWSTR,
            ) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATGetAttrInfo(
            hcatalog.into_param().abi(),
            ::std::mem::transmute(pcatmember),
            pwszreferencetag.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATGetCatAttrInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hcatalog: Param0,
    pwszreferencetag: Param1,
) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATGetCatAttrInfo(
                hcatalog: super::Foundation::HANDLE,
                pwszreferencetag: super::Foundation::PWSTR,
            ) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATGetCatAttrInfo(
            hcatalog.into_param().abi(),
            pwszreferencetag.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptCATGetMemberInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hcatalog: Param0,
    pwszreferencetag: Param1,
) -> *mut CRYPTCATMEMBER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATGetMemberInfo(
                hcatalog: super::Foundation::HANDLE,
                pwszreferencetag: super::Foundation::PWSTR,
            ) -> *mut CRYPTCATMEMBER;
        }
        ::std::mem::transmute(CryptCATGetMemberInfo(
            hcatalog.into_param().abi(),
            pwszreferencetag.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATHandleFromStore(pcatstore: *mut CRYPTCATSTORE) -> super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATHandleFromStore(pcatstore: *mut CRYPTCATSTORE) -> super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CryptCATHandleFromStore(::std::mem::transmute(pcatstore)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATOpen<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    pwszfilename: Param0,
    fdwopenflags: CRYPTCAT_OPEN_FLAGS,
    hprov: usize,
    dwpublicversion: CRYPTCAT_VERSION,
    dwencodingtype: u32,
) -> super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATOpen(
                pwszfilename: super::Foundation::PWSTR,
                fdwopenflags: CRYPTCAT_OPEN_FLAGS,
                hprov: usize,
                dwpublicversion: CRYPTCAT_VERSION,
                dwencodingtype: u32,
            ) -> super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CryptCATOpen(
            pwszfilename.into_param().abi(),
            ::std::mem::transmute(fdwopenflags),
            ::std::mem::transmute(hprov),
            ::std::mem::transmute(dwpublicversion),
            ::std::mem::transmute(dwencodingtype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATPersistStore<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hcatalog: Param0,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATPersistStore(hcatalog: super::Foundation::HANDLE)
                -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptCATPersistStore(hcatalog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptCATPutAttrInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hcatalog: Param0,
    pcatmember: *mut CRYPTCATMEMBER,
    pwszreferencetag: Param2,
    dwattrtypeandaction: u32,
    cbdata: u32,
    pbdata: *mut u8,
) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATPutAttrInfo(
                hcatalog: super::Foundation::HANDLE,
                pcatmember: *mut CRYPTCATMEMBER,
                pwszreferencetag: super::Foundation::PWSTR,
                dwattrtypeandaction: u32,
                cbdata: u32,
                pbdata: *mut u8,
            ) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATPutAttrInfo(
            hcatalog.into_param().abi(),
            ::std::mem::transmute(pcatmember),
            pwszreferencetag.into_param().abi(),
            ::std::mem::transmute(dwattrtypeandaction),
            ::std::mem::transmute(cbdata),
            ::std::mem::transmute(pbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATPutCatAttrInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hcatalog: Param0,
    pwszreferencetag: Param1,
    dwattrtypeandaction: u32,
    cbdata: u32,
    pbdata: *mut u8,
) -> *mut CRYPTCATATTRIBUTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATPutCatAttrInfo(
                hcatalog: super::Foundation::HANDLE,
                pwszreferencetag: super::Foundation::PWSTR,
                dwattrtypeandaction: u32,
                cbdata: u32,
                pbdata: *mut u8,
            ) -> *mut CRYPTCATATTRIBUTE;
        }
        ::std::mem::transmute(CryptCATPutCatAttrInfo(
            hcatalog.into_param().abi(),
            pwszreferencetag.into_param().abi(),
            ::std::mem::transmute(dwattrtypeandaction),
            ::std::mem::transmute(cbdata),
            ::std::mem::transmute(pbdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptCATPutMemberInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hcatalog: Param0,
    pwszfilename: Param1,
    pwszreferencetag: Param2,
    pgsubjecttype: *mut ::windows::runtime::GUID,
    dwcertversion: u32,
    cbsipindirectdata: u32,
    pbsipindirectdata: *mut u8,
) -> *mut CRYPTCATMEMBER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATPutMemberInfo(
                hcatalog: super::Foundation::HANDLE,
                pwszfilename: super::Foundation::PWSTR,
                pwszreferencetag: super::Foundation::PWSTR,
                pgsubjecttype: *mut ::windows::runtime::GUID,
                dwcertversion: u32,
                cbsipindirectdata: u32,
                pbsipindirectdata: *mut u8,
            ) -> *mut CRYPTCATMEMBER;
        }
        ::std::mem::transmute(CryptCATPutMemberInfo(
            hcatalog.into_param().abi(),
            pwszfilename.into_param().abi(),
            pwszreferencetag.into_param().abi(),
            ::std::mem::transmute(pgsubjecttype),
            ::std::mem::transmute(dwcertversion),
            ::std::mem::transmute(cbsipindirectdata),
            ::std::mem::transmute(pbsipindirectdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptCATStoreFromHandle<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hcatalog: Param0,
) -> *mut CRYPTCATSTORE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptCATStoreFromHandle(hcatalog: super::Foundation::HANDLE) -> *mut CRYPTCATSTORE;
        }
        ::std::mem::transmute(CryptCATStoreFromHandle(hcatalog.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptProtectData<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    pdatain: *const Cryptography::Core::CRYPTOAPI_BLOB,
    szdatadescr: Param1,
    poptionalentropy: *const Cryptography::Core::CRYPTOAPI_BLOB,
    pvreserved: *mut ::std::ffi::c_void,
    ppromptstruct: *const CRYPTPROTECT_PROMPTSTRUCT,
    dwflags: u32,
    pdataout: *mut Cryptography::Core::CRYPTOAPI_BLOB,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptProtectData(
                pdatain: *const Cryptography::Core::CRYPTOAPI_BLOB,
                szdatadescr: super::Foundation::PWSTR,
                poptionalentropy: *const Cryptography::Core::CRYPTOAPI_BLOB,
                pvreserved: *mut ::std::ffi::c_void,
                ppromptstruct: *const CRYPTPROTECT_PROMPTSTRUCT,
                dwflags: u32,
                pdataout: *mut Cryptography::Core::CRYPTOAPI_BLOB,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptProtectData(
            ::std::mem::transmute(pdatain),
            szdatadescr.into_param().abi(),
            ::std::mem::transmute(poptionalentropy),
            ::std::mem::transmute(pvreserved),
            ::std::mem::transmute(ppromptstruct),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pdataout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptProtectMemory(
    pdatain: *mut ::std::ffi::c_void,
    cbdatain: u32,
    dwflags: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptProtectMemory(
                pdatain: *mut ::std::ffi::c_void,
                cbdatain: u32,
                dwflags: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptProtectMemory(
            ::std::mem::transmute(pdatain),
            ::std::mem::transmute(cbdatain),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptSIPAddProvider(psnewprov: *mut SIP_ADD_NEWPROVIDER) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPAddProvider(psnewprov: *mut SIP_ADD_NEWPROVIDER) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPAddProvider(::std::mem::transmute(psnewprov)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptSIPCreateIndirectData(
    psubjectinfo: *mut SIP_SUBJECTINFO,
    pcbindirectdata: *mut u32,
    pindirectdata: *mut SIP_INDIRECT_DATA,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPCreateIndirectData(
                psubjectinfo: *mut SIP_SUBJECTINFO,
                pcbindirectdata: *mut u32,
                pindirectdata: *mut SIP_INDIRECT_DATA,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPCreateIndirectData(
            ::std::mem::transmute(psubjectinfo),
            ::std::mem::transmute(pcbindirectdata),
            ::std::mem::transmute(pindirectdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptSIPGetCaps(
    psubjinfo: *const SIP_SUBJECTINFO,
    pcaps: *mut SIP_CAP_SET_V3,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPGetCaps(
                psubjinfo: *const SIP_SUBJECTINFO,
                pcaps: *mut SIP_CAP_SET_V3,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPGetCaps(
            ::std::mem::transmute(psubjinfo),
            ::std::mem::transmute(pcaps),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptSIPGetSealedDigest(
    psubjectinfo: *const SIP_SUBJECTINFO,
    psig: *const u8,
    dwsig: u32,
    pbdigest: *mut u8,
    pcbdigest: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPGetSealedDigest(
                psubjectinfo: *const SIP_SUBJECTINFO,
                psig: *const u8,
                dwsig: u32,
                pbdigest: *mut u8,
                pcbdigest: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPGetSealedDigest(
            ::std::mem::transmute(psubjectinfo),
            ::std::mem::transmute(psig),
            ::std::mem::transmute(dwsig),
            ::std::mem::transmute(pbdigest),
            ::std::mem::transmute(pcbdigest),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptSIPGetSignedDataMsg(
    psubjectinfo: *mut SIP_SUBJECTINFO,
    pdwencodingtype: *mut Cryptography::Core::CERT_QUERY_ENCODING_TYPE,
    dwindex: u32,
    pcbsigneddatamsg: *mut u32,
    pbsigneddatamsg: *mut u8,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPGetSignedDataMsg(
                psubjectinfo: *mut SIP_SUBJECTINFO,
                pdwencodingtype: *mut Cryptography::Core::CERT_QUERY_ENCODING_TYPE,
                dwindex: u32,
                pcbsigneddatamsg: *mut u32,
                pbsigneddatamsg: *mut u8,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPGetSignedDataMsg(
            ::std::mem::transmute(psubjectinfo),
            ::std::mem::transmute(pdwencodingtype),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(pcbsigneddatamsg),
            ::std::mem::transmute(pbsigneddatamsg),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptSIPLoad(
    pgsubject: *const ::windows::runtime::GUID,
    dwflags: u32,
    psipdispatch: *mut SIP_DISPATCH_INFO,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPLoad(
                pgsubject: *const ::windows::runtime::GUID,
                dwflags: u32,
                psipdispatch: *mut ::std::mem::ManuallyDrop<SIP_DISPATCH_INFO>,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPLoad(
            ::std::mem::transmute(pgsubject),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(psipdispatch),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptSIPPutSignedDataMsg(
    psubjectinfo: *mut SIP_SUBJECTINFO,
    dwencodingtype: Cryptography::Core::CERT_QUERY_ENCODING_TYPE,
    pdwindex: *mut u32,
    cbsigneddatamsg: u32,
    pbsigneddatamsg: *mut u8,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPPutSignedDataMsg(
                psubjectinfo: *mut SIP_SUBJECTINFO,
                dwencodingtype: Cryptography::Core::CERT_QUERY_ENCODING_TYPE,
                pdwindex: *mut u32,
                cbsigneddatamsg: u32,
                pbsigneddatamsg: *mut u8,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPPutSignedDataMsg(
            ::std::mem::transmute(psubjectinfo),
            ::std::mem::transmute(dwencodingtype),
            ::std::mem::transmute(pdwindex),
            ::std::mem::transmute(cbsigneddatamsg),
            ::std::mem::transmute(pbsigneddatamsg),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptSIPRemoveProvider(
    pgprov: *mut ::windows::runtime::GUID,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRemoveProvider(
                pgprov: *mut ::windows::runtime::GUID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPRemoveProvider(::std::mem::transmute(pgprov)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptSIPRemoveSignedDataMsg(
    psubjectinfo: *mut SIP_SUBJECTINFO,
    dwindex: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRemoveSignedDataMsg(
                psubjectinfo: *mut SIP_SUBJECTINFO,
                dwindex: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPRemoveSignedDataMsg(
            ::std::mem::transmute(psubjectinfo),
            ::std::mem::transmute(dwindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptSIPRetrieveSubjectGuid<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    filename: Param0,
    hfilein: Param1,
    pgsubject: *mut ::windows::runtime::GUID,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRetrieveSubjectGuid(
                filename: super::Foundation::PWSTR,
                hfilein: super::Foundation::HANDLE,
                pgsubject: *mut ::windows::runtime::GUID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPRetrieveSubjectGuid(
            filename.into_param().abi(),
            hfilein.into_param().abi(),
            ::std::mem::transmute(pgsubject),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptSIPRetrieveSubjectGuidForCatalogFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    filename: Param0,
    hfilein: Param1,
    pgsubject: *mut ::windows::runtime::GUID,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPRetrieveSubjectGuidForCatalogFile(
                filename: super::Foundation::PWSTR,
                hfilein: super::Foundation::HANDLE,
                pgsubject: *mut ::windows::runtime::GUID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPRetrieveSubjectGuidForCatalogFile(
            filename.into_param().abi(),
            hfilein.into_param().abi(),
            ::std::mem::transmute(pgsubject),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptSIPVerifyIndirectData(
    psubjectinfo: *mut SIP_SUBJECTINFO,
    pindirectdata: *mut SIP_INDIRECT_DATA,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptSIPVerifyIndirectData(
                psubjectinfo: *mut SIP_SUBJECTINFO,
                pindirectdata: *mut SIP_INDIRECT_DATA,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptSIPVerifyIndirectData(
            ::std::mem::transmute(psubjectinfo),
            ::std::mem::transmute(pindirectdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn CryptUnprotectData(
    pdatain: *const Cryptography::Core::CRYPTOAPI_BLOB,
    ppszdatadescr: *mut super::Foundation::PWSTR,
    poptionalentropy: *const Cryptography::Core::CRYPTOAPI_BLOB,
    pvreserved: *mut ::std::ffi::c_void,
    ppromptstruct: *const CRYPTPROTECT_PROMPTSTRUCT,
    dwflags: u32,
    pdataout: *mut Cryptography::Core::CRYPTOAPI_BLOB,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUnprotectData(
                pdatain: *const Cryptography::Core::CRYPTOAPI_BLOB,
                ppszdatadescr: *mut super::Foundation::PWSTR,
                poptionalentropy: *const Cryptography::Core::CRYPTOAPI_BLOB,
                pvreserved: *mut ::std::ffi::c_void,
                ppromptstruct: *const CRYPTPROTECT_PROMPTSTRUCT,
                dwflags: u32,
                pdataout: *mut Cryptography::Core::CRYPTOAPI_BLOB,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptUnprotectData(
            ::std::mem::transmute(pdatain),
            ::std::mem::transmute(ppszdatadescr),
            ::std::mem::transmute(poptionalentropy),
            ::std::mem::transmute(pvreserved),
            ::std::mem::transmute(ppromptstruct),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pdataout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptUnprotectMemory(
    pdatain: *mut ::std::ffi::c_void,
    cbdatain: u32,
    dwflags: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUnprotectMemory(
                pdatain: *mut ::std::ffi::c_void,
                cbdatain: u32,
                dwflags: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptUnprotectMemory(
            ::std::mem::transmute(pdatain),
            ::std::mem::transmute(cbdatain),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CryptUpdateProtectedState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    poldsid: Param0,
    pwszoldpassword: Param1,
    dwflags: u32,
    pdwsuccesscount: *mut u32,
    pdwfailurecount: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CryptUpdateProtectedState(
                poldsid: super::Foundation::PSID,
                pwszoldpassword: super::Foundation::PWSTR,
                dwflags: u32,
                pdwsuccesscount: *mut u32,
                pdwfailurecount: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(CryptUpdateProtectedState(
            poldsid.into_param().abi(),
            pwszoldpassword.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pdwsuccesscount),
            ::std::mem::transmute(pdwfailurecount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    pub moduleName: super::Foundation::PWSTR,
    pub friendlyModuleName: super::Foundation::PWSTR,
    pub eventCount: u32,
    pub uploadSizeBytes: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_EVENT_BINARY_STATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_BINARY_STATS")
            .field("moduleName", &self.moduleName)
            .field("friendlyModuleName", &self.friendlyModuleName)
            .field("eventCount", &self.eventCount)
            .field("uploadSizeBytes", &self.uploadSizeBytes)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.moduleName == other.moduleName
            && self.friendlyModuleName == other.friendlyModuleName
            && self.eventCount == other.eventCount
            && self.uploadSizeBytes == other.uploadSizeBytes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    pub id: i32,
    pub name: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    pub name: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION")
            .field("name", &self.name)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    pub privacyTag: i32,
    pub name: super::Foundation::PWSTR,
    pub description: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION")
            .field("privacyTag", &self.privacyTag)
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.privacyTag == other.privacyTag
            && self.name == other.name
            && self.description == other.description
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    pub privacyTag: i32,
    pub eventCount: u32,
}
impl DIAGNOSTIC_DATA_EVENT_TAG_STATS {}
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_TAG_STATS")
            .field("privacyTag", &self.privacyTag)
            .field("eventCount", &self.eventCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.privacyTag == other.privacyTag && self.eventCount == other.eventCount
    }
}
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TAG_STATS {}
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    pub hoursOfHistoryToKeep: u32,
    pub maxStoreMegabytes: u32,
    pub requestedMaxStoreMegabytes: u32,
}
impl DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {}
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION")
            .field("hoursOfHistoryToKeep", &self.hoursOfHistoryToKeep)
            .field("maxStoreMegabytes", &self.maxStoreMegabytes)
            .field(
                "requestedMaxStoreMegabytes",
                &self.requestedMaxStoreMegabytes,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.hoursOfHistoryToKeep == other.hoursOfHistoryToKeep
            && self.maxStoreMegabytes == other.maxStoreMegabytes
            && self.requestedMaxStoreMegabytes == other.requestedMaxStoreMegabytes
    }
}
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {}
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DIAGNOSTIC_DATA_GENERAL_STATS {
    pub optInLevel: u32,
    pub transcriptSizeBytes: u64,
    pub oldestEventTimestamp: u64,
    pub totalEventCountLast24Hours: u32,
    pub averageDailyEvents: f32,
}
impl DIAGNOSTIC_DATA_GENERAL_STATS {}
impl ::std::default::Default for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_GENERAL_STATS")
            .field("optInLevel", &self.optInLevel)
            .field("transcriptSizeBytes", &self.transcriptSizeBytes)
            .field("oldestEventTimestamp", &self.oldestEventTimestamp)
            .field(
                "totalEventCountLast24Hours",
                &self.totalEventCountLast24Hours,
            )
            .field("averageDailyEvents", &self.averageDailyEvents)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.optInLevel == other.optInLevel
            && self.transcriptSizeBytes == other.transcriptSizeBytes
            && self.oldestEventTimestamp == other.oldestEventTimestamp
            && self.totalEventCountLast24Hours == other.totalEventCountLast24Hours
            && self.averageDailyEvents == other.averageDailyEvents
    }
}
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_GENERAL_STATS {}
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_GENERAL_STATS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_RECORD {
    pub rowId: i64,
    pub timestamp: u64,
    pub eventKeywords: u64,
    pub fullEventName: super::Foundation::PWSTR,
    pub providerGroupGuid: super::Foundation::PWSTR,
    pub producerName: super::Foundation::PWSTR,
    pub privacyTags: *mut i32,
    pub privacyTagCount: u32,
    pub categoryIds: *mut i32,
    pub categoryIdCount: u32,
    pub isCoreData: super::Foundation::BOOL,
    pub extra1: super::Foundation::PWSTR,
    pub extra2: super::Foundation::PWSTR,
    pub extra3: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_RECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_RECORD")
            .field("rowId", &self.rowId)
            .field("timestamp", &self.timestamp)
            .field("eventKeywords", &self.eventKeywords)
            .field("fullEventName", &self.fullEventName)
            .field("providerGroupGuid", &self.providerGroupGuid)
            .field("producerName", &self.producerName)
            .field("privacyTags", &self.privacyTags)
            .field("privacyTagCount", &self.privacyTagCount)
            .field("categoryIds", &self.categoryIds)
            .field("categoryIdCount", &self.categoryIdCount)
            .field("isCoreData", &self.isCoreData)
            .field("extra1", &self.extra1)
            .field("extra2", &self.extra2)
            .field("extra3", &self.extra3)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.rowId == other.rowId
            && self.timestamp == other.timestamp
            && self.eventKeywords == other.eventKeywords
            && self.fullEventName == other.fullEventName
            && self.providerGroupGuid == other.providerGroupGuid
            && self.producerName == other.producerName
            && self.privacyTags == other.privacyTags
            && self.privacyTagCount == other.privacyTagCount
            && self.categoryIds == other.categoryIds
            && self.categoryIdCount == other.categoryIdCount
            && self.isCoreData == other.isCoreData
            && self.extra1 == other.extra1
            && self.extra2 == other.extra2
            && self.extra3 == other.extra3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_RECORD {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_RECORD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    pub producerNames: *mut super::Foundation::PWSTR,
    pub producerNameCount: u32,
    pub textToMatch: super::Foundation::PWSTR,
    pub categoryIds: *mut i32,
    pub categoryIdCount: u32,
    pub privacyTags: *mut i32,
    pub privacyTagCount: u32,
    pub coreDataOnly: super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_SEARCH_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_SEARCH_CRITERIA")
            .field("producerNames", &self.producerNames)
            .field("producerNameCount", &self.producerNameCount)
            .field("textToMatch", &self.textToMatch)
            .field("categoryIds", &self.categoryIds)
            .field("categoryIdCount", &self.categoryIdCount)
            .field("privacyTags", &self.privacyTags)
            .field("privacyTagCount", &self.privacyTagCount)
            .field("coreDataOnly", &self.coreDataOnly)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn eq(&self, other: &Self) -> bool {
        self.producerNames == other.producerNames
            && self.producerNameCount == other.producerNameCount
            && self.textToMatch == other.textToMatch
            && self.categoryIds == other.categoryIds
            && self.categoryIdCount == other.categoryIdCount
            && self.privacyTags == other.privacyTags
            && self.privacyTagCount == other.privacyTagCount
            && self.coreDataOnly == other.coreDataOnly
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_SEARCH_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_REPORT_DATA {
    pub signature: DIAGNOSTIC_REPORT_SIGNATURE,
    pub bucketId: ::windows::runtime::GUID,
    pub reportId: ::windows::runtime::GUID,
    pub creationTime: super::Foundation::FILETIME,
    pub sizeInBytes: u64,
    pub cabId: super::Foundation::PWSTR,
    pub reportStatus: u32,
    pub reportIntegratorId: ::windows::runtime::GUID,
    pub fileNames: *mut super::Foundation::PWSTR,
    pub fileCount: u32,
    pub friendlyEventName: super::Foundation::PWSTR,
    pub applicationName: super::Foundation::PWSTR,
    pub applicationPath: super::Foundation::PWSTR,
    pub description: super::Foundation::PWSTR,
    pub bucketIdString: super::Foundation::PWSTR,
    pub legacyBucketId: u64,
    pub reportKey: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_REPORT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_REPORT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_REPORT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_REPORT_DATA")
            .field("signature", &self.signature)
            .field("bucketId", &self.bucketId)
            .field("reportId", &self.reportId)
            .field("creationTime", &self.creationTime)
            .field("sizeInBytes", &self.sizeInBytes)
            .field("cabId", &self.cabId)
            .field("reportStatus", &self.reportStatus)
            .field("reportIntegratorId", &self.reportIntegratorId)
            .field("fileNames", &self.fileNames)
            .field("fileCount", &self.fileCount)
            .field("friendlyEventName", &self.friendlyEventName)
            .field("applicationName", &self.applicationName)
            .field("applicationPath", &self.applicationPath)
            .field("description", &self.description)
            .field("bucketIdString", &self.bucketIdString)
            .field("legacyBucketId", &self.legacyBucketId)
            .field("reportKey", &self.reportKey)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_REPORT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature
            && self.bucketId == other.bucketId
            && self.reportId == other.reportId
            && self.creationTime == other.creationTime
            && self.sizeInBytes == other.sizeInBytes
            && self.cabId == other.cabId
            && self.reportStatus == other.reportStatus
            && self.reportIntegratorId == other.reportIntegratorId
            && self.fileNames == other.fileNames
            && self.fileCount == other.fileCount
            && self.friendlyEventName == other.friendlyEventName
            && self.applicationName == other.applicationName
            && self.applicationPath == other.applicationPath
            && self.description == other.description
            && self.bucketIdString == other.bucketIdString
            && self.legacyBucketId == other.legacyBucketId
            && self.reportKey == other.reportKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_REPORT_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_REPORT_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DIAGNOSTIC_REPORT_PARAMETER {
    pub name: [u16; 129],
    pub value: [u16; 260],
}
impl DIAGNOSTIC_REPORT_PARAMETER {}
impl ::std::default::Default for DIAGNOSTIC_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DIAGNOSTIC_REPORT_PARAMETER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_REPORT_PARAMETER")
            .field("name", &self.name)
            .field("value", &self.value)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DIAGNOSTIC_REPORT_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl ::std::cmp::Eq for DIAGNOSTIC_REPORT_PARAMETER {}
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_REPORT_PARAMETER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DIAGNOSTIC_REPORT_SIGNATURE {
    pub eventName: [u16; 65],
    pub parameters: [DIAGNOSTIC_REPORT_PARAMETER; 10],
}
impl DIAGNOSTIC_REPORT_SIGNATURE {}
impl ::std::default::Default for DIAGNOSTIC_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DIAGNOSTIC_REPORT_SIGNATURE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_REPORT_SIGNATURE")
            .field("eventName", &self.eventName)
            .field("parameters", &self.parameters)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DIAGNOSTIC_REPORT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.eventName == other.eventName && self.parameters == other.parameters
    }
}
impl ::std::cmp::Eq for DIAGNOSTIC_REPORT_SIGNATURE {}
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_REPORT_SIGNATURE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct DRIVER_VER_INFO {
    pub cbStruct: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
    pub dwPlatform: u32,
    pub dwVersion: u32,
    pub wszVersion: [u16; 260],
    pub wszSignedBy: [u16; 260],
    pub pcSignerCertContext: *mut Cryptography::Core::CERT_CONTEXT,
    pub sOSVersionLow: DRIVER_VER_MAJORMINOR,
    pub sOSVersionHigh: DRIVER_VER_MAJORMINOR,
    pub dwBuildNumberLow: u32,
    pub dwBuildNumberHigh: u32,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl DRIVER_VER_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for DRIVER_VER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for DRIVER_VER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRIVER_VER_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwPlatform", &self.dwPlatform)
            .field("dwVersion", &self.dwVersion)
            .field("wszVersion", &self.wszVersion)
            .field("wszSignedBy", &self.wszSignedBy)
            .field("pcSignerCertContext", &self.pcSignerCertContext)
            .field("sOSVersionLow", &self.sOSVersionLow)
            .field("sOSVersionHigh", &self.sOSVersionHigh)
            .field("dwBuildNumberLow", &self.dwBuildNumberLow)
            .field("dwBuildNumberHigh", &self.dwBuildNumberHigh)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for DRIVER_VER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwPlatform == other.dwPlatform
            && self.dwVersion == other.dwVersion
            && self.wszVersion == other.wszVersion
            && self.wszSignedBy == other.wszSignedBy
            && self.pcSignerCertContext == other.pcSignerCertContext
            && self.sOSVersionLow == other.sOSVersionLow
            && self.sOSVersionHigh == other.sOSVersionHigh
            && self.dwBuildNumberLow == other.dwBuildNumberLow
            && self.dwBuildNumberHigh == other.dwBuildNumberHigh
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for DRIVER_VER_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for DRIVER_VER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DRIVER_VER_MAJORMINOR {
    pub dwMajor: u32,
    pub dwMinor: u32,
}
impl DRIVER_VER_MAJORMINOR {}
impl ::std::default::Default for DRIVER_VER_MAJORMINOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DRIVER_VER_MAJORMINOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRIVER_VER_MAJORMINOR")
            .field("dwMajor", &self.dwMajor)
            .field("dwMinor", &self.dwMinor)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DRIVER_VER_MAJORMINOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwMajor == other.dwMajor && self.dwMinor == other.dwMinor
    }
}
impl ::std::cmp::Eq for DRIVER_VER_MAJORMINOR {}
unsafe impl ::windows::runtime::Abi for DRIVER_VER_MAJORMINOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
pub unsafe fn DSCreateISecurityInfoObject<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::Foundation::LPARAM>,
>(
    pwszobjectpath: Param0,
    pwszobjectclass: Param1,
    dwflags: u32,
    ppsi: *mut ::std::option::Option<Authorization::ISecurityInformation>,
    pfnreadsd: ::std::option::Option<PFNREADOBJECTSECURITY>,
    pfnwritesd: ::std::option::Option<PFNWRITEOBJECTSECURITY>,
    lpcontext: Param6,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSCreateISecurityInfoObject(
                pwszobjectpath: super::Foundation::PWSTR,
                pwszobjectclass: super::Foundation::PWSTR,
                dwflags: u32,
                ppsi: *mut ::windows::runtime::RawPtr,
                pfnreadsd: ::windows::runtime::RawPtr,
                pfnwritesd: ::windows::runtime::RawPtr,
                lpcontext: super::Foundation::LPARAM,
            ) -> ::windows::runtime::HRESULT;
        }
        DSCreateISecurityInfoObject(
            pwszobjectpath.into_param().abi(),
            pwszobjectclass.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(ppsi),
            ::std::mem::transmute(pfnreadsd),
            ::std::mem::transmute(pfnwritesd),
            lpcontext.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
pub unsafe fn DSCreateISecurityInfoObjectEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param9: ::windows::runtime::IntoParam<'a, super::Foundation::LPARAM>,
>(
    pwszobjectpath: Param0,
    pwszobjectclass: Param1,
    pwszserver: Param2,
    pwszusername: Param3,
    pwszpassword: Param4,
    dwflags: u32,
    ppsi: *mut ::std::option::Option<Authorization::ISecurityInformation>,
    pfnreadsd: ::std::option::Option<PFNREADOBJECTSECURITY>,
    pfnwritesd: ::std::option::Option<PFNWRITEOBJECTSECURITY>,
    lpcontext: Param9,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSCreateISecurityInfoObjectEx(
                pwszobjectpath: super::Foundation::PWSTR,
                pwszobjectclass: super::Foundation::PWSTR,
                pwszserver: super::Foundation::PWSTR,
                pwszusername: super::Foundation::PWSTR,
                pwszpassword: super::Foundation::PWSTR,
                dwflags: u32,
                ppsi: *mut ::windows::runtime::RawPtr,
                pfnreadsd: ::windows::runtime::RawPtr,
                pfnwritesd: ::windows::runtime::RawPtr,
                lpcontext: super::Foundation::LPARAM,
            ) -> ::windows::runtime::HRESULT;
        }
        DSCreateISecurityInfoObjectEx(
            pwszobjectpath.into_param().abi(),
            pwszobjectclass.into_param().abi(),
            pwszserver.into_param().abi(),
            pwszusername.into_param().abi(),
            pwszpassword.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(ppsi),
            ::std::mem::transmute(pfnreadsd),
            ::std::mem::transmute(pfnwritesd),
            lpcontext.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub unsafe fn DSCreateSecurityPage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::Foundation::LPARAM>,
>(
    pwszobjectpath: Param0,
    pwszobjectclass: Param1,
    dwflags: u32,
    phpage: *mut super::UI::Controls::HPROPSHEETPAGE,
    pfnreadsd: ::std::option::Option<PFNREADOBJECTSECURITY>,
    pfnwritesd: ::std::option::Option<PFNWRITEOBJECTSECURITY>,
    lpcontext: Param6,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSCreateSecurityPage(
                pwszobjectpath: super::Foundation::PWSTR,
                pwszobjectclass: super::Foundation::PWSTR,
                dwflags: u32,
                phpage: *mut super::UI::Controls::HPROPSHEETPAGE,
                pfnreadsd: ::windows::runtime::RawPtr,
                pfnwritesd: ::windows::runtime::RawPtr,
                lpcontext: super::Foundation::LPARAM,
            ) -> ::windows::runtime::HRESULT;
        }
        DSCreateSecurityPage(
            pwszobjectpath.into_param().abi(),
            pwszobjectclass.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(phpage),
            ::std::mem::transmute(pfnreadsd),
            ::std::mem::transmute(pfnwritesd),
            lpcontext.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DSEditSecurity<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HWND>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param7: ::windows::runtime::IntoParam<'a, super::Foundation::LPARAM>,
>(
    hwndowner: Param0,
    pwszobjectpath: Param1,
    pwszobjectclass: Param2,
    dwflags: u32,
    pwszcaption: Param4,
    pfnreadsd: ::std::option::Option<PFNREADOBJECTSECURITY>,
    pfnwritesd: ::std::option::Option<PFNWRITEOBJECTSECURITY>,
    lpcontext: Param7,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSEditSecurity(
                hwndowner: super::Foundation::HWND,
                pwszobjectpath: super::Foundation::PWSTR,
                pwszobjectclass: super::Foundation::PWSTR,
                dwflags: u32,
                pwszcaption: super::Foundation::PWSTR,
                pfnreadsd: ::windows::runtime::RawPtr,
                pfnwritesd: ::windows::runtime::RawPtr,
                lpcontext: super::Foundation::LPARAM,
            ) -> ::windows::runtime::HRESULT;
        }
        DSEditSecurity(
            hwndowner.into_param().abi(),
            pwszobjectpath.into_param().abi(),
            pwszobjectclass.into_param().abi(),
            ::std::mem::transmute(dwflags),
            pwszcaption.into_param().abi(),
            ::std::mem::transmute(pfnreadsd),
            ::std::mem::transmute(pfnwritesd),
            lpcontext.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DSSI_IS_ROOT: u32 = 16u32;
pub const DSSI_NO_ACCESS_CHECK: u32 = 2u32;
pub const DSSI_NO_EDIT_OWNER: u32 = 8u32;
pub const DSSI_NO_EDIT_SACL: u32 = 4u32;
pub const DSSI_NO_FILTER: u32 = 32u32;
pub const DSSI_NO_READONLY_MESSAGE: u32 = 64u32;
pub const DSSI_READ_ONLY: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DdqAccessLevel(pub i32);
pub const NoData: DdqAccessLevel = DdqAccessLevel(0i32);
pub const CurrentUserData: DdqAccessLevel = DdqAccessLevel(1i32);
pub const AllUserData: DdqAccessLevel = DdqAccessLevel(2i32);
impl ::std::convert::From<i32> for DdqAccessLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DdqAccessLevel {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn DdqCancelDiagnosticRecordOperation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCancelDiagnosticRecordOperation(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
            ) -> ::windows::runtime::HRESULT;
        }
        DdqCancelDiagnosticRecordOperation(hsession.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqCloseSession<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCloseSession(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
            ) -> ::windows::runtime::HRESULT;
        }
        DdqCloseSession(hsession.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqCreateSession(
    accesslevel: DdqAccessLevel,
) -> ::windows::runtime::Result<HDIAGNOSTIC_DATA_QUERY_SESSION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCreateSession(
                accesslevel: DdqAccessLevel,
                hsession: *mut HDIAGNOSTIC_DATA_QUERY_SESSION,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HDIAGNOSTIC_DATA_QUERY_SESSION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqCreateSession(::std::mem::transmute(accesslevel), &mut result__)
            .from_abi::<HDIAGNOSTIC_DATA_QUERY_SESSION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqExtractDiagnosticReport<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hsession: Param0,
    reportstoretype: u32,
    reportkey: Param2,
    destinationpath: Param3,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqExtractDiagnosticReport(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                reportstoretype: u32,
                reportkey: super::Foundation::PWSTR,
                destinationpath: super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        DdqExtractDiagnosticReport(
            hsession.into_param().abi(),
            ::std::mem::transmute(reportstoretype),
            reportkey.into_param().abi(),
            destinationpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqFreeDiagnosticRecordLocaleTags<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>,
>(
    htagdescription: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordLocaleTags(
                htagdescription: HDIAGNOSTIC_EVENT_TAG_DESCRIPTION,
            ) -> ::windows::runtime::HRESULT;
        }
        DdqFreeDiagnosticRecordLocaleTags(htagdescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqFreeDiagnosticRecordPage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_RECORD>,
>(
    hrecord: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordPage(
                hrecord: HDIAGNOSTIC_RECORD,
            ) -> ::windows::runtime::HRESULT;
        }
        DdqFreeDiagnosticRecordPage(hrecord.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqFreeDiagnosticRecordProducerCategories<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>,
>(
    hcategorydescription: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordProducerCategories(
                hcategorydescription: HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION,
            ) -> ::windows::runtime::HRESULT;
        }
        DdqFreeDiagnosticRecordProducerCategories(hcategorydescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqFreeDiagnosticRecordProducers<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>,
>(
    hproducerdescription: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordProducers(
                hproducerdescription: HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION,
            ) -> ::windows::runtime::HRESULT;
        }
        DdqFreeDiagnosticRecordProducers(hproducerdescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqFreeDiagnosticReport<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_REPORT>,
>(
    hreport: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticReport(hreport: HDIAGNOSTIC_REPORT) -> ::windows::runtime::HRESULT;
        }
        DdqFreeDiagnosticReport(hreport.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqGetDiagnosticDataAccessLevelAllowed() -> ::windows::runtime::Result<DdqAccessLevel>
{
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticDataAccessLevelAllowed(
                accesslevel: *mut DdqAccessLevel,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DdqAccessLevel as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticDataAccessLevelAllowed(&mut result__).from_abi::<DdqAccessLevel>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordAtIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_RECORD>,
>(
    hrecord: Param0,
    index: u32,
) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_RECORD> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordAtIndex(
                hrecord: HDIAGNOSTIC_RECORD,
                index: u32,
                record: *mut DIAGNOSTIC_DATA_RECORD,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DIAGNOSTIC_DATA_RECORD as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqGetDiagnosticRecordAtIndex(
            hrecord.into_param().abi(),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<DIAGNOSTIC_DATA_RECORD>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordBinaryDistribution<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
    producernames: *const super::Foundation::PWSTR,
    producernamecount: u32,
    topnbinaries: u32,
    binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS,
    statcount: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordBinaryDistribution(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                producernames: *const super::Foundation::PWSTR,
                producernamecount: u32,
                topnbinaries: u32,
                binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS,
                statcount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        DdqGetDiagnosticRecordBinaryDistribution(
            hsession.into_param().abi(),
            ::std::mem::transmute(producernames),
            ::std::mem::transmute(producernamecount),
            ::std::mem::transmute(topnbinaries),
            ::std::mem::transmute(binarystats),
            ::std::mem::transmute(statcount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordCategoryAtIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>,
>(
    hcategorydescription: Param0,
    index: u32,
) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCategoryAtIndex(
                hcategorydescription: HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION,
                index: u32,
                categorydescription: *mut DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__ : < DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        DdqGetDiagnosticRecordCategoryAtIndex(
            hcategorydescription.into_param().abi(),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqGetDiagnosticRecordCategoryCount<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>,
>(
    hcategorydescription: Param0,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCategoryCount(
                hcategorydescription: HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION,
                categorydescriptioncount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordCategoryCount(hcategorydescription.into_param().abi(), &mut result__)
            .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqGetDiagnosticRecordCount<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_RECORD>,
>(
    hrecord: Param0,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCount(
                hrecord: HDIAGNOSTIC_RECORD,
                recordcount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordCount(hrecord.into_param().abi(), &mut result__)
            .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagAtIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>,
>(
    htagdescription: Param0,
    index: u32,
) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTagAtIndex(
                htagdescription: HDIAGNOSTIC_EVENT_TAG_DESCRIPTION,
                index: u32,
                tagdescription: *mut DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTagAtIndex(
            htagdescription.into_param().abi(),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqGetDiagnosticRecordLocaleTagCount<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>,
>(
    htagdescription: Param0,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTagCount(
                htagdescription: HDIAGNOSTIC_EVENT_TAG_DESCRIPTION,
                tagdescriptioncount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTagCount(htagdescription.into_param().abi(), &mut result__)
            .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordLocaleTags<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hsession: Param0,
    locale: Param1,
) -> ::windows::runtime::Result<HDIAGNOSTIC_EVENT_TAG_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTags(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                locale: super::Foundation::PWSTR,
                htagdescription: *mut HDIAGNOSTIC_EVENT_TAG_DESCRIPTION,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HDIAGNOSTIC_EVENT_TAG_DESCRIPTION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTags(
            hsession.into_param().abi(),
            locale.into_param().abi(),
            &mut result__,
        )
        .from_abi::<HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordPage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
    searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA,
    offset: u32,
    pagerecordcount: u32,
    baserowid: i64,
) -> ::windows::runtime::Result<HDIAGNOSTIC_RECORD> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordPage(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA,
                offset: u32,
                pagerecordcount: u32,
                baserowid: i64,
                hrecord: *mut HDIAGNOSTIC_RECORD,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HDIAGNOSTIC_RECORD as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqGetDiagnosticRecordPage(
            hsession.into_param().abi(),
            ::std::mem::transmute(searchcriteria),
            ::std::mem::transmute(offset),
            ::std::mem::transmute(pagerecordcount),
            ::std::mem::transmute(baserowid),
            &mut result__,
        )
        .from_abi::<HDIAGNOSTIC_RECORD>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordPayload<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
    rowid: i64,
) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordPayload(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                rowid: i64,
                payload: *mut super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqGetDiagnosticRecordPayload(
            hsession.into_param().abi(),
            ::std::mem::transmute(rowid),
            &mut result__,
        )
        .from_abi::<super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordProducerAtIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>,
>(
    hproducerdescription: Param0,
    index: u32,
) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerAtIndex(
                hproducerdescription: HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION,
                index: u32,
                producerdescription: *mut DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__ : < DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        DdqGetDiagnosticRecordProducerAtIndex(
            hproducerdescription.into_param().abi(),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordProducerCategories<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hsession: Param0,
    producername: Param1,
) -> ::windows::runtime::Result<HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerCategories(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                producername: super::Foundation::PWSTR,
                hcategorydescription: *mut HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqGetDiagnosticRecordProducerCategories(
            hsession.into_param().abi(),
            producername.into_param().abi(),
            &mut result__,
        )
        .from_abi::<HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqGetDiagnosticRecordProducerCount<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>,
>(
    hproducerdescription: Param0,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerCount(
                hproducerdescription: HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION,
                producerdescriptioncount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordProducerCount(hproducerdescription.into_param().abi(), &mut result__)
            .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqGetDiagnosticRecordProducers<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
) -> ::windows::runtime::Result<HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducers(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                hproducerdescription: *mut HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqGetDiagnosticRecordProducers(hsession.into_param().abi(), &mut result__)
            .from_abi::<HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordStats<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
    searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA,
    recordcount: *mut u32,
    minrowid: *mut i64,
    maxrowid: *mut i64,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordStats(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA,
                recordcount: *mut u32,
                minrowid: *mut i64,
                maxrowid: *mut i64,
            ) -> ::windows::runtime::HRESULT;
        }
        DdqGetDiagnosticRecordStats(
            hsession.into_param().abi(),
            ::std::mem::transmute(searchcriteria),
            ::std::mem::transmute(recordcount),
            ::std::mem::transmute(minrowid),
            ::std::mem::transmute(maxrowid),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordSummary<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
    producernames: *const super::Foundation::PWSTR,
    producernamecount: u32,
) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_GENERAL_STATS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordSummary(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                producernames: *const super::Foundation::PWSTR,
                producernamecount: u32,
                generalstats: *mut DIAGNOSTIC_DATA_GENERAL_STATS,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DIAGNOSTIC_DATA_GENERAL_STATS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqGetDiagnosticRecordSummary(
            hsession.into_param().abi(),
            ::std::mem::transmute(producernames),
            ::std::mem::transmute(producernamecount),
            &mut result__,
        )
        .from_abi::<DIAGNOSTIC_DATA_GENERAL_STATS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticRecordTagDistribution<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
    producernames: *const super::Foundation::PWSTR,
    producernamecount: u32,
    tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS,
    statcount: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordTagDistribution(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                producernames: *const super::Foundation::PWSTR,
                producernamecount: u32,
                tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS,
                statcount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        DdqGetDiagnosticRecordTagDistribution(
            hsession.into_param().abi(),
            ::std::mem::transmute(producernames),
            ::std::mem::transmute(producernamecount),
            ::std::mem::transmute(tagstats),
            ::std::mem::transmute(statcount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqGetDiagnosticReport<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
    reportstoretype: u32,
) -> ::windows::runtime::Result<HDIAGNOSTIC_REPORT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReport(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                reportstoretype: u32,
                hreport: *mut HDIAGNOSTIC_REPORT,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HDIAGNOSTIC_REPORT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqGetDiagnosticReport(
            hsession.into_param().abi(),
            ::std::mem::transmute(reportstoretype),
            &mut result__,
        )
        .from_abi::<HDIAGNOSTIC_REPORT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqGetDiagnosticReportAtIndex<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_REPORT>,
>(
    hreport: Param0,
    index: u32,
) -> ::windows::runtime::Result<DIAGNOSTIC_REPORT_DATA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportAtIndex(
                hreport: HDIAGNOSTIC_REPORT,
                index: u32,
                report: *mut DIAGNOSTIC_REPORT_DATA,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DIAGNOSTIC_REPORT_DATA as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqGetDiagnosticReportAtIndex(
            hreport.into_param().abi(),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<DIAGNOSTIC_REPORT_DATA>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqGetDiagnosticReportCount<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_REPORT>,
>(
    hreport: Param0,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportCount(
                hreport: HDIAGNOSTIC_REPORT,
                reportcount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticReportCount(hreport.into_param().abi(), &mut result__)
            .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqGetDiagnosticReportStoreReportCount<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
    reportstoretype: u32,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportStoreReportCount(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                reportstoretype: u32,
                reportcount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticReportStoreReportCount(
            hsession.into_param().abi(),
            ::std::mem::transmute(reportstoretype),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqGetSessionAccessLevel<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
) -> ::windows::runtime::Result<DdqAccessLevel> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetSessionAccessLevel(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                accesslevel: *mut DdqAccessLevel,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DdqAccessLevel as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetSessionAccessLevel(hsession.into_param().abi(), &mut result__)
            .from_abi::<DdqAccessLevel>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqGetTranscriptConfiguration<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetTranscriptConfiguration(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                currentconfig: *mut DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__ : < DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        DdqGetTranscriptConfiguration(hsession.into_param().abi(), &mut result__)
            .from_abi::<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DdqIsDiagnosticRecordSampledIn<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hsession: Param0,
    providergroup: *const ::windows::runtime::GUID,
    providerid: *const ::windows::runtime::GUID,
    providername: Param3,
    eventid: *const u32,
    eventname: Param5,
    eventversion: *const u32,
    eventkeywords: *const u64,
) -> ::windows::runtime::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqIsDiagnosticRecordSampledIn(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                providergroup: *const ::windows::runtime::GUID,
                providerid: *const ::windows::runtime::GUID,
                providername: super::Foundation::PWSTR,
                eventid: *const u32,
                eventname: super::Foundation::PWSTR,
                eventversion: *const u32,
                eventkeywords: *const u64,
                issampledin: *mut super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DdqIsDiagnosticRecordSampledIn(
            hsession.into_param().abi(),
            ::std::mem::transmute(providergroup),
            ::std::mem::transmute(providerid),
            providername.into_param().abi(),
            ::std::mem::transmute(eventid),
            eventname.into_param().abi(),
            ::std::mem::transmute(eventversion),
            ::std::mem::transmute(eventkeywords),
            &mut result__,
        )
        .from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DdqSetTranscriptConfiguration<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDIAGNOSTIC_DATA_QUERY_SESSION>,
>(
    hsession: Param0,
    desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqSetTranscriptConfiguration(
                hsession: HDIAGNOSTIC_DATA_QUERY_SESSION,
                desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION,
            ) -> ::windows::runtime::HRESULT;
        }
        DdqSetTranscriptConfiguration(
            hsession.into_param().abi(),
            ::std::mem::transmute(desiredconfig),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DeleteAce(pacl: *mut ACL, dwaceindex: u32) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteAce(pacl: *mut ACL, dwaceindex: u32) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwaceindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DeriveCapabilitySidsFromName<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    capname: Param0,
    capabilitygroupsids: *mut *mut super::Foundation::PSID,
    capabilitygroupsidcount: *mut u32,
    capabilitysids: *mut *mut super::Foundation::PSID,
    capabilitysidcount: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeriveCapabilitySidsFromName(
                capname: super::Foundation::PWSTR,
                capabilitygroupsids: *mut *mut super::Foundation::PSID,
                capabilitygroupsidcount: *mut u32,
                capabilitysids: *mut *mut super::Foundation::PSID,
                capabilitysidcount: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeriveCapabilitySidsFromName(
            capname.into_param().abi(),
            ::std::mem::transmute(capabilitygroupsids),
            ::std::mem::transmute(capabilitygroupsidcount),
            ::std::mem::transmute(capabilitysids),
            ::std::mem::transmute(capabilitysidcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DestroyPrivateObjectSecurity(
    objectdescriptor: *const *const SECURITY_DESCRIPTOR,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyPrivateObjectSecurity(
                objectdescriptor: *const *const SECURITY_DESCRIPTOR,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(DestroyPrivateObjectSecurity(::std::mem::transmute(
            objectdescriptor,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DuplicateToken<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    existingtokenhandle: Param0,
    impersonationlevel: SECURITY_IMPERSONATION_LEVEL,
    duplicatetokenhandle: *mut super::Foundation::HANDLE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DuplicateToken(
                existingtokenhandle: super::Foundation::HANDLE,
                impersonationlevel: SECURITY_IMPERSONATION_LEVEL,
                duplicatetokenhandle: *mut super::Foundation::HANDLE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(DuplicateToken(
            existingtokenhandle.into_param().abi(),
            ::std::mem::transmute(impersonationlevel),
            ::std::mem::transmute(duplicatetokenhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DuplicateTokenEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hexistingtoken: Param0,
    dwdesiredaccess: TOKEN_ACCESS_MASK,
    lptokenattributes: *const SECURITY_ATTRIBUTES,
    impersonationlevel: SECURITY_IMPERSONATION_LEVEL,
    tokentype: TOKEN_TYPE,
    phnewtoken: *mut super::Foundation::HANDLE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DuplicateTokenEx(
                hexistingtoken: super::Foundation::HANDLE,
                dwdesiredaccess: TOKEN_ACCESS_MASK,
                lptokenattributes: *const SECURITY_ATTRIBUTES,
                impersonationlevel: SECURITY_IMPERSONATION_LEVEL,
                tokentype: TOKEN_TYPE,
                phnewtoken: *mut super::Foundation::HANDLE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(DuplicateTokenEx(
            hexistingtoken.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(lptokenattributes),
            ::std::mem::transmute(impersonationlevel),
            ::std::mem::transmute(tokentype),
            ::std::mem::transmute(phnewtoken),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EqualDomainSid<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    psid1: Param0,
    psid2: Param1,
    pfequal: *mut super::Foundation::BOOL,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EqualDomainSid(
                psid1: super::Foundation::PSID,
                psid2: super::Foundation::PSID,
                pfequal: *mut super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(EqualDomainSid(
            psid1.into_param().abi(),
            psid2.into_param().abi(),
            ::std::mem::transmute(pfequal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EqualPrefixSid<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    psid1: Param0,
    psid2: Param1,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EqualPrefixSid(
                psid1: super::Foundation::PSID,
                psid2: super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(EqualPrefixSid(
            psid1.into_param().abi(),
            psid2.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EqualSid<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    psid1: Param0,
    psid2: Param1,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EqualSid(
                psid1: super::Foundation::PSID,
                psid2: super::Foundation::PSID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(EqualSid(psid1.into_param().abi(), psid2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FindFirstFreeAce(
    pacl: *const ACL,
    pace: *mut *mut ::std::ffi::c_void,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstFreeAce(
                pacl: *const ACL,
                pace: *mut *mut ::std::ffi::c_void,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindFirstFreeAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(pace),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn FreeSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(
    psid: Param0,
) -> *mut ::std::ffi::c_void {
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
        fmt.debug_struct("GENERIC_MAPPING")
            .field("GenericRead", &self.GenericRead)
            .field("GenericWrite", &self.GenericWrite)
            .field("GenericExecute", &self.GenericExecute)
            .field("GenericAll", &self.GenericAll)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GENERIC_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.GenericRead == other.GenericRead
            && self.GenericWrite == other.GenericWrite
            && self.GenericExecute == other.GenericExecute
            && self.GenericAll == other.GenericAll
    }
}
impl ::std::cmp::Eq for GENERIC_MAPPING {}
unsafe impl ::windows::runtime::Abi for GENERIC_MAPPING {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetAce(
    pacl: *const ACL,
    dwaceindex: u32,
    pace: *mut *mut ::std::ffi::c_void,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAce(
                pacl: *const ACL,
                dwaceindex: u32,
                pace: *mut *mut ::std::ffi::c_void,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetAce(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(dwaceindex),
            ::std::mem::transmute(pace),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetAclInformation(
    pacl: *const ACL,
    paclinformation: *mut ::std::ffi::c_void,
    naclinformationlength: u32,
    dwaclinformationclass: ACL_INFORMATION_CLASS,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAclInformation(
                pacl: *const ACL,
                paclinformation: *mut ::std::ffi::c_void,
                naclinformationlength: u32,
                dwaclinformationclass: ACL_INFORMATION_CLASS,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetAclInformation(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(paclinformation),
            ::std::mem::transmute(naclinformationlength),
            ::std::mem::transmute(dwaclinformationclass),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetAppContainerAce(
    acl: *const ACL,
    startingaceindex: u32,
    appcontainerace: *mut *mut ::std::ffi::c_void,
    appcontaineraceindex: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerAce(
                acl: *const ACL,
                startingaceindex: u32,
                appcontainerace: *mut *mut ::std::ffi::c_void,
                appcontaineraceindex: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetAppContainerAce(
            ::std::mem::transmute(acl),
            ::std::mem::transmute(startingaceindex),
            ::std::mem::transmute(appcontainerace),
            ::std::mem::transmute(appcontaineraceindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetCachedSigningLevel<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    file: Param0,
    flags: *mut u32,
    signinglevel: *mut u32,
    thumbprint: *mut u8,
    thumbprintsize: *mut u32,
    thumbprintalgorithm: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCachedSigningLevel(
                file: super::Foundation::HANDLE,
                flags: *mut u32,
                signinglevel: *mut u32,
                thumbprint: *mut u8,
                thumbprintsize: *mut u32,
                thumbprintalgorithm: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCachedSigningLevel(
            file.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(signinglevel),
            ::std::mem::transmute(thumbprint),
            ::std::mem::transmute(thumbprintsize),
            ::std::mem::transmute(thumbprintalgorithm),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetFileSecurityA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    requestedinformation: u32,
    psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
    nlength: u32,
    lpnlengthneeded: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileSecurityA(
                lpfilename: super::Foundation::PSTR,
                requestedinformation: u32,
                psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
                nlength: u32,
                lpnlengthneeded: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileSecurityA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(requestedinformation),
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnlengthneeded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetFileSecurityW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    requestedinformation: u32,
    psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
    nlength: u32,
    lpnlengthneeded: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileSecurityW(
                lpfilename: super::Foundation::PWSTR,
                requestedinformation: u32,
                psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
                nlength: u32,
                lpnlengthneeded: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFileSecurityW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(requestedinformation),
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnlengthneeded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetKernelObjectSecurity<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    handle: Param0,
    requestedinformation: u32,
    psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
    nlength: u32,
    lpnlengthneeded: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKernelObjectSecurity(
                handle: super::Foundation::HANDLE,
                requestedinformation: u32,
                psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
                nlength: u32,
                lpnlengthneeded: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetKernelObjectSecurity(
            handle.into_param().abi(),
            ::std::mem::transmute(requestedinformation),
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnlengthneeded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetLengthSid<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    psid: Param0,
) -> u32 {
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetPrivateObjectSecurity(
    objectdescriptor: *const SECURITY_DESCRIPTOR,
    securityinformation: u32,
    resultantdescriptor: *mut SECURITY_DESCRIPTOR,
    descriptorlength: u32,
    returnlength: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateObjectSecurity(
                objectdescriptor: *const SECURITY_DESCRIPTOR,
                securityinformation: u32,
                resultantdescriptor: *mut SECURITY_DESCRIPTOR,
                descriptorlength: u32,
                returnlength: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPrivateObjectSecurity(
            ::std::mem::transmute(objectdescriptor),
            ::std::mem::transmute(securityinformation),
            ::std::mem::transmute(resultantdescriptor),
            ::std::mem::transmute(descriptorlength),
            ::std::mem::transmute(returnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSecurityDescriptorControl(
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
    pcontrol: *mut u16,
    lpdwrevision: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorControl(
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
                pcontrol: *mut u16,
                lpdwrevision: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSecurityDescriptorControl(
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(pcontrol),
            ::std::mem::transmute(lpdwrevision),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSecurityDescriptorDacl(
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
    lpbdaclpresent: *mut i32,
    pdacl: *mut *mut ACL,
    lpbdacldefaulted: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorDacl(
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
                lpbdaclpresent: *mut i32,
                pdacl: *mut *mut ACL,
                lpbdacldefaulted: *mut i32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSecurityDescriptorDacl(
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(lpbdaclpresent),
            ::std::mem::transmute(pdacl),
            ::std::mem::transmute(lpbdacldefaulted),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSecurityDescriptorGroup(
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
    pgroup: *mut super::Foundation::PSID,
    lpbgroupdefaulted: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorGroup(
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
                pgroup: *mut super::Foundation::PSID,
                lpbgroupdefaulted: *mut i32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSecurityDescriptorGroup(
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(pgroup),
            ::std::mem::transmute(lpbgroupdefaulted),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSecurityDescriptorLength(psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorLength(psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> u32;
        }
        ::std::mem::transmute(GetSecurityDescriptorLength(::std::mem::transmute(
            psecuritydescriptor,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSecurityDescriptorOwner(
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
    powner: *mut super::Foundation::PSID,
    lpbownerdefaulted: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorOwner(
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
                powner: *mut super::Foundation::PSID,
                lpbownerdefaulted: *mut i32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSecurityDescriptorOwner(
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(powner),
            ::std::mem::transmute(lpbownerdefaulted),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSecurityDescriptorRMControl(
    securitydescriptor: *const SECURITY_DESCRIPTOR,
    rmcontrol: *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorRMControl(
                securitydescriptor: *const SECURITY_DESCRIPTOR,
                rmcontrol: *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(GetSecurityDescriptorRMControl(
            ::std::mem::transmute(securitydescriptor),
            ::std::mem::transmute(rmcontrol),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSecurityDescriptorSacl(
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
    lpbsaclpresent: *mut i32,
    psacl: *mut *mut ACL,
    lpbsacldefaulted: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityDescriptorSacl(
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
                lpbsaclpresent: *mut i32,
                psacl: *mut *mut ACL,
                lpbsacldefaulted: *mut i32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSecurityDescriptorSacl(
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(lpbsaclpresent),
            ::std::mem::transmute(psacl),
            ::std::mem::transmute(lpbsacldefaulted),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSidIdentifierAuthority<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    psid: Param0,
) -> *mut SID_IDENTIFIER_AUTHORITY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSidIdentifierAuthority(
                psid: super::Foundation::PSID,
            ) -> *mut SID_IDENTIFIER_AUTHORITY;
        }
        ::std::mem::transmute(GetSidIdentifierAuthority(psid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetSidLengthRequired(nsubauthoritycount: u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSidLengthRequired(nsubauthoritycount: u8) -> u32;
        }
        ::std::mem::transmute(GetSidLengthRequired(::std::mem::transmute(
            nsubauthoritycount,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSidSubAuthority<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    psid: Param0,
    nsubauthority: u32,
) -> *mut u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSidSubAuthority(psid: super::Foundation::PSID, nsubauthority: u32) -> *mut u32;
        }
        ::std::mem::transmute(GetSidSubAuthority(
            psid.into_param().abi(),
            ::std::mem::transmute(nsubauthority),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSidSubAuthorityCount<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    psid: Param0,
) -> *mut u8 {
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetTokenInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    tokenhandle: Param0,
    tokeninformationclass: TOKEN_INFORMATION_CLASS,
    tokeninformation: *mut ::std::ffi::c_void,
    tokeninformationlength: u32,
    returnlength: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTokenInformation(
                tokenhandle: super::Foundation::HANDLE,
                tokeninformationclass: TOKEN_INFORMATION_CLASS,
                tokeninformation: *mut ::std::ffi::c_void,
                tokeninformationlength: u32,
                returnlength: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetTokenInformation(
            tokenhandle.into_param().abi(),
            ::std::mem::transmute(tokeninformationclass),
            ::std::mem::transmute(tokeninformation),
            ::std::mem::transmute(tokeninformationlength),
            ::std::mem::transmute(returnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetUserObjectSecurity<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hobj: Param0,
    psirequested: *const u32,
    psid: *mut SECURITY_DESCRIPTOR,
    nlength: u32,
    lpnlengthneeded: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUserObjectSecurity(
                hobj: super::Foundation::HANDLE,
                psirequested: *const u32,
                psid: *mut SECURITY_DESCRIPTOR,
                nlength: u32,
                lpnlengthneeded: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetUserObjectSecurity(
            hobj.into_param().abi(),
            ::std::mem::transmute(psirequested),
            ::std::mem::transmute(psid),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnlengthneeded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetWindowsAccountDomainSid<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    psid: Param0,
    pdomainsid: super::Foundation::PSID,
    cbdomainsid: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowsAccountDomainSid(
                psid: super::Foundation::PSID,
                pdomainsid: super::Foundation::PSID,
                cbdomainsid: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetWindowsAccountDomainSid(
            psid.into_param().abi(),
            ::std::mem::transmute(pdomainsid),
            ::std::mem::transmute(cbdomainsid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
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
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
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
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
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
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
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
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
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
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
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
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICertSrvSetup(::windows::runtime::IUnknown);
impl ICertSrvSetup {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::ITypeInfo> {
        let mut result__ : < super::System::OleAutomation:: ITypeInfo as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut super::System::OleAutomation::VARIANT,
        pexcepinfo: *mut super::System::OleAutomation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn CAErrorId(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CAErrorString(&self) -> ::windows::runtime::Result<super::Foundation::BSTR> {
        let mut result__: <super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Foundation::BSTR>(result__)
    }
    pub unsafe fn InitializeDefaults(
        &self,
        bserver: i16,
        bclient: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bserver),
            ::std::mem::transmute(bclient),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetCASetupProperty(
        &self,
        propertyid: CASetupProperty,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propertyid),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn SetCASetupProperty(
        &self,
        propertyid: CASetupProperty,
        ppropertyvalue: *const super::System::OleAutomation::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propertyid),
            ::std::mem::transmute(ppropertyvalue),
        )
        .ok()
    }
    pub unsafe fn IsPropertyEditable(
        &self,
        propertyid: CASetupProperty,
    ) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propertyid),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetSupportedCATypes(
        &self,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetProviderNameList(
        &self,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetKeyLengthList<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrprovidername: Param0,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            bstrprovidername.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetHashAlgorithmList<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrprovidername: Param0,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            bstrprovidername.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetPrivateKeyContainerList<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrprovidername: Param0,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            bstrprovidername.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    pub unsafe fn GetExistingCACertificates(
        &self,
    ) -> ::windows::runtime::Result<ICertSrvSetupKeyInformationCollection> {
        let mut result__: <ICertSrvSetupKeyInformationCollection as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<ICertSrvSetupKeyInformationCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CAImportPFX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrfilename: Param0,
        bstrpasswd: Param1,
        boverwriteexistingkey: i16,
    ) -> ::windows::runtime::Result<ICertSrvSetupKeyInformation> {
        let mut result__: <ICertSrvSetupKeyInformation as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            bstrfilename.into_param().abi(),
            bstrpasswd.into_param().abi(),
            ::std::mem::transmute(boverwriteexistingkey),
            &mut result__,
        )
        .from_abi::<ICertSrvSetupKeyInformation>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCADistinguishedName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrcadn: Param0,
        bignoreunicode: i16,
        boverwriteexistingkey: i16,
        boverwriteexistingcainds: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            bstrcadn.into_param().abi(),
            ::std::mem::transmute(bignoreunicode),
            ::std::mem::transmute(boverwriteexistingkey),
            ::std::mem::transmute(boverwriteexistingcainds),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDatabaseInformation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrdbdirectory: Param0,
        bstrlogdirectory: Param1,
        bstrsharedfolder: Param2,
        bforceoverwrite: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            bstrdbdirectory.into_param().abi(),
            bstrlogdirectory.into_param().abi(),
            bstrsharedfolder.into_param().abi(),
            ::std::mem::transmute(bforceoverwrite),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetParentCAInformation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrcaconfiguration: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            bstrcaconfiguration.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWebCAInformation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrcaconfiguration: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            bstrcaconfiguration.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Install(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn PreUnInstall(&self, bclientonly: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bclientonly),
        )
        .ok()
    }
    pub unsafe fn PostUnInstall(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICertSrvSetup {
    type Vtable = ICertSrvSetup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3076563387,
        18308,
        17600,
        [143, 18, 85, 95, 7, 128, 255, 37],
    );
}
impl ::std::convert::From<ICertSrvSetup> for ::windows::runtime::IUnknown {
    fn from(value: ICertSrvSetup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICertSrvSetup> for ::windows::runtime::IUnknown {
    fn from(value: &ICertSrvSetup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICertSrvSetup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICertSrvSetup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<ICertSrvSetup> for super::System::OleAutomation::IDispatch {
    fn from(value: ICertSrvSetup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&ICertSrvSetup> for super::System::OleAutomation::IDispatch {
    fn from(value: &ICertSrvSetup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for ICertSrvSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for &ICertSrvSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertSrvSetup_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bserver: i16,
        bclient: i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyid: CASetupProperty,
        ppropertyvalue: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyid: CASetupProperty,
        ppropertyvalue: *const ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyid: CASetupProperty,
        pbeditable: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcatypes: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrprovidername: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        pval: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrprovidername: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        pval: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrprovidername: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        pval: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrfilename: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        bstrpasswd: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        boverwriteexistingkey: i16,
        ppval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrcadn: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        bignoreunicode: i16,
        boverwriteexistingkey: i16,
        boverwriteexistingcainds: i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrdbdirectory: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        bstrlogdirectory: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        bstrsharedfolder: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        bforceoverwrite: i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrcaconfiguration: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrcaconfiguration: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bclientonly: i16,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICertSrvSetupKeyInformation(::windows::runtime::IUnknown);
impl ICertSrvSetupKeyInformation {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::ITypeInfo> {
        let mut result__ : < super::System::OleAutomation:: ITypeInfo as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut super::System::OleAutomation::VARIANT,
        pexcepinfo: *mut super::System::OleAutomation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProviderName(&self) -> ::windows::runtime::Result<super::Foundation::BSTR> {
        let mut result__: <super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProviderName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrval: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            bstrval.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Length(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn SetLength(&self, lval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lval),
        )
        .ok()
    }
    pub unsafe fn Existing(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn SetExisting(&self, bval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bval),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContainerName(&self) -> ::windows::runtime::Result<super::Foundation::BSTR> {
        let mut result__: <super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContainerName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrval: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            bstrval.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HashAlgorithm(&self) -> ::windows::runtime::Result<super::Foundation::BSTR> {
        let mut result__: <super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHashAlgorithm<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrval: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            bstrval.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn ExistingCACertificate(
        &self,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn SetExistingCACertificate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::System::OleAutomation::VARIANT>,
    >(
        &self,
        varval: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            varval.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICertSrvSetupKeyInformation {
    type Vtable = ICertSrvSetupKeyInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1806120824,
        14042,
        19513,
        [138, 133, 188, 250, 125, 0, 7, 147],
    );
}
impl ::std::convert::From<ICertSrvSetupKeyInformation> for ::windows::runtime::IUnknown {
    fn from(value: ICertSrvSetupKeyInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICertSrvSetupKeyInformation> for ::windows::runtime::IUnknown {
    fn from(value: &ICertSrvSetupKeyInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ICertSrvSetupKeyInformation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ICertSrvSetupKeyInformation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<ICertSrvSetupKeyInformation> for super::System::OleAutomation::IDispatch {
    fn from(value: ICertSrvSetupKeyInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&ICertSrvSetupKeyInformation>
    for super::System::OleAutomation::IDispatch
{
    fn from(value: &ICertSrvSetupKeyInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for ICertSrvSetupKeyInformation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for &ICertSrvSetupKeyInformation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertSrvSetupKeyInformation_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrval: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lval: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bval: i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrval: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrval: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        varval: ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICertSrvSetupKeyInformationCollection(::windows::runtime::IUnknown);
impl ICertSrvSetupKeyInformationCollection {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::ITypeInfo> {
        let mut result__ : < super::System::OleAutomation:: ITypeInfo as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut super::System::OleAutomation::VARIANT,
        pexcepinfo: *mut super::System::OleAutomation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Item(
        &self,
        index: i32,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn Add<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ICertSrvSetupKeyInformation>,
    >(
        &self,
        pikeyinformation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            pikeyinformation.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICertSrvSetupKeyInformationCollection {
    type Vtable = ICertSrvSetupKeyInformationCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3864824576,
        58767,
        16889,
        [169, 236, 162, 141, 116, 39, 200, 68],
    );
}
impl ::std::convert::From<ICertSrvSetupKeyInformationCollection> for ::windows::runtime::IUnknown {
    fn from(value: ICertSrvSetupKeyInformationCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICertSrvSetupKeyInformationCollection> for ::windows::runtime::IUnknown {
    fn from(value: &ICertSrvSetupKeyInformationCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ICertSrvSetupKeyInformationCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ICertSrvSetupKeyInformationCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<ICertSrvSetupKeyInformationCollection>
    for super::System::OleAutomation::IDispatch
{
    fn from(value: ICertSrvSetupKeyInformationCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&ICertSrvSetupKeyInformationCollection>
    for super::System::OleAutomation::IDispatch
{
    fn from(value: &ICertSrvSetupKeyInformationCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for ICertSrvSetupKeyInformationCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for &ICertSrvSetupKeyInformationCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertSrvSetupKeyInformationCollection_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppval: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: i32,
        pval: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pikeyinformation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICertificateEnrollmentPolicyServerSetup(::windows::runtime::IUnknown);
impl ICertificateEnrollmentPolicyServerSetup {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::ITypeInfo> {
        let mut result__ : < super::System::OleAutomation:: ITypeInfo as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut super::System::OleAutomation::VARIANT,
        pexcepinfo: *mut super::System::OleAutomation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ErrorString(&self) -> ::windows::runtime::Result<super::Foundation::BSTR> {
        let mut result__: <super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Foundation::BSTR>(result__)
    }
    pub unsafe fn InitializeInstallDefaults(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetProperty(
        &self,
        propertyid: CEPSetupProperty,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propertyid),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn SetProperty(
        &self,
        propertyid: CEPSetupProperty,
        ppropertyvalue: *const super::System::OleAutomation::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propertyid),
            ::std::mem::transmute(ppropertyvalue),
        )
        .ok()
    }
    pub unsafe fn Install(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn UnInstall(
        &self,
        pauthkeybasedrenewal: *const super::System::OleAutomation::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pauthkeybasedrenewal),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICertificateEnrollmentPolicyServerSetup {
    type Vtable = ICertificateEnrollmentPolicyServerSetup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2240959180,
        9100,
        19080,
        [184, 253, 163, 126, 125, 4, 230, 139],
    );
}
impl ::std::convert::From<ICertificateEnrollmentPolicyServerSetup>
    for ::windows::runtime::IUnknown
{
    fn from(value: ICertificateEnrollmentPolicyServerSetup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICertificateEnrollmentPolicyServerSetup>
    for ::windows::runtime::IUnknown
{
    fn from(value: &ICertificateEnrollmentPolicyServerSetup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ICertificateEnrollmentPolicyServerSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ICertificateEnrollmentPolicyServerSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<ICertificateEnrollmentPolicyServerSetup>
    for super::System::OleAutomation::IDispatch
{
    fn from(value: ICertificateEnrollmentPolicyServerSetup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&ICertificateEnrollmentPolicyServerSetup>
    for super::System::OleAutomation::IDispatch
{
    fn from(value: &ICertificateEnrollmentPolicyServerSetup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for ICertificateEnrollmentPolicyServerSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for &ICertificateEnrollmentPolicyServerSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateEnrollmentPolicyServerSetup_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyid: CEPSetupProperty,
        ppropertyvalue: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyid: CEPSetupProperty,
        ppropertyvalue: *const ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pauthkeybasedrenewal: *const ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICertificateEnrollmentServerSetup(::windows::runtime::IUnknown);
impl ICertificateEnrollmentServerSetup {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::ITypeInfo> {
        let mut result__ : < super::System::OleAutomation:: ITypeInfo as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut super::System::OleAutomation::VARIANT,
        pexcepinfo: *mut super::System::OleAutomation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ErrorString(&self) -> ::windows::runtime::Result<super::Foundation::BSTR> {
        let mut result__: <super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Foundation::BSTR>(result__)
    }
    pub unsafe fn InitializeInstallDefaults(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetProperty(
        &self,
        propertyid: CESSetupProperty,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propertyid),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn SetProperty(
        &self,
        propertyid: CESSetupProperty,
        ppropertyvalue: *const super::System::OleAutomation::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propertyid),
            ::std::mem::transmute(ppropertyvalue),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplicationPoolCredentials<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrusername: Param0,
        bstrpassword: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            bstrusername.into_param().abi(),
            bstrpassword.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Install(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn UnInstall(
        &self,
        pcaconfig: *const super::System::OleAutomation::VARIANT,
        pauthentication: *const super::System::OleAutomation::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcaconfig),
            ::std::mem::transmute(pauthentication),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICertificateEnrollmentServerSetup {
    type Vtable = ICertificateEnrollmentServerSetup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1879211995,
        40409,
        18721,
        [137, 68, 179, 92, 179, 27, 210, 236],
    );
}
impl ::std::convert::From<ICertificateEnrollmentServerSetup> for ::windows::runtime::IUnknown {
    fn from(value: ICertificateEnrollmentServerSetup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICertificateEnrollmentServerSetup> for ::windows::runtime::IUnknown {
    fn from(value: &ICertificateEnrollmentServerSetup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ICertificateEnrollmentServerSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ICertificateEnrollmentServerSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<ICertificateEnrollmentServerSetup>
    for super::System::OleAutomation::IDispatch
{
    fn from(value: ICertificateEnrollmentServerSetup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&ICertificateEnrollmentServerSetup>
    for super::System::OleAutomation::IDispatch
{
    fn from(value: &ICertificateEnrollmentServerSetup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for ICertificateEnrollmentServerSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for &ICertificateEnrollmentServerSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateEnrollmentServerSetup_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyid: CESSetupProperty,
        ppropertyvalue: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyid: CESSetupProperty,
        ppropertyvalue: *const ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrusername: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        bstrpassword: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcaconfig: *const ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
        pauthentication: *const ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMSCEPSetup(::windows::runtime::IUnknown);
impl IMSCEPSetup {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub unsafe fn GetTypeInfo(
        &self,
        itinfo: u32,
        lcid: u32,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::ITypeInfo> {
        let mut result__ : < super::System::OleAutomation:: ITypeInfo as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(itinfo),
            ::std::mem::transmute(lcid),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(rgsznames),
            ::std::mem::transmute(cnames),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(rgdispid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn Invoke(
        &self,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut super::System::OleAutomation::VARIANT,
        pexcepinfo: *mut super::System::OleAutomation::EXCEPINFO,
        puargerr: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dispidmember),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(lcid),
            ::std::mem::transmute(wflags),
            ::std::mem::transmute(pdispparams),
            ::std::mem::transmute(pvarresult),
            ::std::mem::transmute(pexcepinfo),
            ::std::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn MSCEPErrorId(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MSCEPErrorString(&self) -> ::windows::runtime::Result<super::Foundation::BSTR> {
        let mut result__: <super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::Foundation::BSTR>(result__)
    }
    pub unsafe fn InitializeDefaults(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetMSCEPSetupProperty(
        &self,
        propertyid: MSCEPSetupProperty,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propertyid),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn SetMSCEPSetupProperty(
        &self,
        propertyid: MSCEPSetupProperty,
        ppropertyvalue: *const super::System::OleAutomation::VARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propertyid),
            ::std::mem::transmute(ppropertyvalue),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAccountInformation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bstrusername: Param0,
        bstrpassword: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            bstrusername.into_param().abi(),
            bstrpassword.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn IsMSCEPStoreEmpty(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetProviderNameList(
        &self,
        bexchange: i16,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bexchange),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub unsafe fn GetKeyLengthList<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BSTR>,
    >(
        &self,
        bexchange: i16,
        bstrprovidername: Param1,
    ) -> ::windows::runtime::Result<super::System::OleAutomation::VARIANT> {
        let mut result__: <super::System::OleAutomation::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bexchange),
            bstrprovidername.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::System::OleAutomation::VARIANT>(result__)
    }
    pub unsafe fn Install(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn PreUnInstall(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn PostUnInstall(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMSCEPSetup {
    type Vtable = IMSCEPSetup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1333223867,
        40763,
        17810,
        [158, 224, 154, 115, 37, 156, 49, 62],
    );
}
impl ::std::convert::From<IMSCEPSetup> for ::windows::runtime::IUnknown {
    fn from(value: IMSCEPSetup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMSCEPSetup> for ::windows::runtime::IUnknown {
    fn from(value: &IMSCEPSetup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMSCEPSetup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMSCEPSetup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<IMSCEPSetup> for super::System::OleAutomation::IDispatch {
    fn from(value: IMSCEPSetup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl ::std::convert::From<&IMSCEPSetup> for super::System::OleAutomation::IDispatch {
    fn from(value: &IMSCEPSetup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for IMSCEPSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_OleAutomation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::System::OleAutomation::IDispatch>
    for &IMSCEPSetup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::System::OleAutomation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::System::OleAutomation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSCEPSetup_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_OleAutomation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_OleAutomation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::System::OleAutomation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pval: *mut ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyid: MSCEPSetupProperty,
        pval: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyid: MSCEPSetupProperty,
        ppropertyvalue: *const ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrusername: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        bstrpassword: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbempty: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bexchange: i16,
        pval: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bexchange: i16,
        bstrprovidername: ::std::mem::ManuallyDrop<super::Foundation::BSTR>,
        pval: *mut ::std::mem::ManuallyDrop<super::System::OleAutomation::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_OleAutomation",
        feature = "Win32_System_SystemServices"
    )))]
    usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTENT_TO_SEAL_ATTRIBUTE {
    pub version: u32,
    pub seal: super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl INTENT_TO_SEAL_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for INTENT_TO_SEAL_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for INTENT_TO_SEAL_ATTRIBUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INTENT_TO_SEAL_ATTRIBUTE")
            .field("version", &self.version)
            .field("seal", &self.seal)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for INTENT_TO_SEAL_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.seal == other.seal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for INTENT_TO_SEAL_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for INTENT_TO_SEAL_ATTRIBUTE {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISceSvcAttachmentData(::windows::runtime::IUnknown);
impl ISceSvcAttachmentData {
    pub unsafe fn GetData(
        &self,
        scesvchandle: *mut ::std::ffi::c_void,
        scetype: SCESVC_INFO_TYPE,
        ppvdata: *mut *mut ::std::ffi::c_void,
        psceenumhandle: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(scesvchandle),
            ::std::mem::transmute(scetype),
            ::std::mem::transmute(ppvdata),
            ::std::mem::transmute(psceenumhandle),
        )
        .ok()
    }
    pub unsafe fn Initialize<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ISceSvcAttachmentPersistInfo>,
    >(
        &self,
        lpservicename: *mut i8,
        lptemplatename: *mut i8,
        lpscesvcpersistinfo: Param2,
        pscesvchandle: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpservicename),
            ::std::mem::transmute(lptemplatename),
            lpscesvcpersistinfo.into_param().abi(),
            ::std::mem::transmute(pscesvchandle),
        )
        .ok()
    }
    pub unsafe fn FreeBuffer(
        &self,
        pvdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvdata),
        )
        .ok()
    }
    pub unsafe fn CloseHandle(
        &self,
        scesvchandle: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(scesvchandle),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISceSvcAttachmentData {
    type Vtable = ISceSvcAttachmentData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        398680030,
        8205,
        4561,
        [175, 251, 0, 192, 79, 185, 132, 249],
    );
}
impl ::std::convert::From<ISceSvcAttachmentData> for ::windows::runtime::IUnknown {
    fn from(value: ISceSvcAttachmentData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISceSvcAttachmentData> for ::windows::runtime::IUnknown {
    fn from(value: &ISceSvcAttachmentData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISceSvcAttachmentData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISceSvcAttachmentData
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentData_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scesvchandle: *mut ::std::ffi::c_void,
        scetype: SCESVC_INFO_TYPE,
        ppvdata: *mut *mut ::std::ffi::c_void,
        psceenumhandle: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpservicename: *mut i8,
        lptemplatename: *mut i8,
        lpscesvcpersistinfo: ::windows::runtime::RawPtr,
        pscesvchandle: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scesvchandle: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ISceSvcAttachmentPersistInfo(::windows::runtime::IUnknown);
impl ISceSvcAttachmentPersistInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save(
        &self,
        lptemplatename: *mut i8,
        scesvchandle: *mut *mut ::std::ffi::c_void,
        ppvdata: *mut *mut ::std::ffi::c_void,
        pboverwriteall: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lptemplatename),
            ::std::mem::transmute(scesvchandle),
            ::std::mem::transmute(ppvdata),
            ::std::mem::transmute(pboverwriteall),
        )
        .ok()
    }
    pub unsafe fn IsDirty(&self, lptemplatename: *mut i8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lptemplatename),
        )
        .ok()
    }
    pub unsafe fn FreeBuffer(
        &self,
        pvdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pvdata),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISceSvcAttachmentPersistInfo {
    type Vtable = ISceSvcAttachmentPersistInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1838211280,
        8205,
        4561,
        [175, 251, 0, 192, 79, 185, 132, 249],
    );
}
impl ::std::convert::From<ISceSvcAttachmentPersistInfo> for ::windows::runtime::IUnknown {
    fn from(value: ISceSvcAttachmentPersistInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISceSvcAttachmentPersistInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ISceSvcAttachmentPersistInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ISceSvcAttachmentPersistInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ISceSvcAttachmentPersistInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentPersistInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lptemplatename: *mut i8,
        scesvchandle: *mut *mut ::std::ffi::c_void,
        ppvdata: *mut *mut ::std::ffi::c_void,
        pboverwriteall: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lptemplatename: *mut i8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pvdata: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITpmVirtualSmartCardManager(::windows::runtime::IUnknown);
impl ITpmVirtualSmartCardManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param10: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
        Param11: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>,
    >(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        fgenerate: Param10,
        pstatuscallback: Param11,
        ppszinstanceid: *mut super::Foundation::PWSTR,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::std::mem::transmute(ppszinstanceid),
            ::std::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>,
    >(
        &self,
        pszinstanceid: Param0,
        pstatuscallback: Param1,
    ) -> ::windows::runtime::Result<super::Foundation::BOOL> {
        let mut result__: <super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pszinstanceid.into_param().abi(),
            pstatuscallback.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITpmVirtualSmartCardManager {
    type Vtable = ITpmVirtualSmartCardManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        288038399,
        55772,
        16887,
        [134, 159, 214, 127, 238, 124, 181, 145],
    );
}
impl ::std::convert::From<ITpmVirtualSmartCardManager> for ::windows::runtime::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager> for ::windows::runtime::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ITpmVirtualSmartCardManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ITpmVirtualSmartCardManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszfriendlyname: super::Foundation::PWSTR,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        fgenerate: super::Foundation::BOOL,
        pstatuscallback: ::windows::runtime::RawPtr,
        ppszinstanceid: *mut super::Foundation::PWSTR,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszinstanceid: super::Foundation::PWSTR,
        pstatuscallback: ::windows::runtime::RawPtr,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITpmVirtualSmartCardManager2(::windows::runtime::IUnknown);
impl ITpmVirtualSmartCardManager2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param10: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
        Param11: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>,
    >(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        fgenerate: Param10,
        pstatuscallback: Param11,
        ppszinstanceid: *mut super::Foundation::PWSTR,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::std::mem::transmute(ppszinstanceid),
            ::std::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>,
    >(
        &self,
        pszinstanceid: Param0,
        pstatuscallback: Param1,
    ) -> ::windows::runtime::Result<super::Foundation::BOOL> {
        let mut result__: <super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pszinstanceid.into_param().abi(),
            pstatuscallback.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param12: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
        Param13: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>,
    >(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        fgenerate: Param12,
        pstatuscallback: Param13,
        ppszinstanceid: *mut super::Foundation::PWSTR,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            ::std::mem::transmute(pbpinpolicy),
            ::std::mem::transmute(cbpinpolicy),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::std::mem::transmute(ppszinstanceid),
            ::std::mem::transmute(pfneedreboot),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITpmVirtualSmartCardManager2 {
    type Vtable = ITpmVirtualSmartCardManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4260930233,
        734,
        18420,
        [188, 38, 170, 133, 171, 94, 82, 103],
    );
}
impl ::std::convert::From<ITpmVirtualSmartCardManager2> for ::windows::runtime::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager2> for ::windows::runtime::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ITpmVirtualSmartCardManager2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ITpmVirtualSmartCardManager2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITpmVirtualSmartCardManager2> for ITpmVirtualSmartCardManager {
    fn from(value: ITpmVirtualSmartCardManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager2> for ITpmVirtualSmartCardManager {
    fn from(value: &ITpmVirtualSmartCardManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager>
    for ITpmVirtualSmartCardManager2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITpmVirtualSmartCardManager>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager>
    for &ITpmVirtualSmartCardManager2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITpmVirtualSmartCardManager>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszfriendlyname: super::Foundation::PWSTR,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        fgenerate: super::Foundation::BOOL,
        pstatuscallback: ::windows::runtime::RawPtr,
        ppszinstanceid: *mut super::Foundation::PWSTR,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszinstanceid: super::Foundation::PWSTR,
        pstatuscallback: ::windows::runtime::RawPtr,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszfriendlyname: super::Foundation::PWSTR,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        fgenerate: super::Foundation::BOOL,
        pstatuscallback: ::windows::runtime::RawPtr,
        ppszinstanceid: *mut super::Foundation::PWSTR,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITpmVirtualSmartCardManager3(::windows::runtime::IUnknown);
impl ITpmVirtualSmartCardManager3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param10: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
        Param11: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>,
    >(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        fgenerate: Param10,
        pstatuscallback: Param11,
        ppszinstanceid: *mut super::Foundation::PWSTR,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::std::mem::transmute(ppszinstanceid),
            ::std::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>,
    >(
        &self,
        pszinstanceid: Param0,
        pstatuscallback: Param1,
    ) -> ::windows::runtime::Result<super::Foundation::BOOL> {
        let mut result__: <super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pszinstanceid.into_param().abi(),
            pstatuscallback.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param12: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
        Param13: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>,
    >(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        fgenerate: Param12,
        pstatuscallback: Param13,
        ppszinstanceid: *mut super::Foundation::PWSTR,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            ::std::mem::transmute(pbpinpolicy),
            ::std::mem::transmute(cbpinpolicy),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::std::mem::transmute(ppszinstanceid),
            ::std::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithAttestation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
        Param13: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
        Param14: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>,
    >(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        attestationtype: TPMVSC_ATTESTATION_TYPE,
        fgenerate: Param13,
        pstatuscallback: Param14,
    ) -> ::windows::runtime::Result<super::Foundation::PWSTR> {
        let mut result__: <super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            ::std::mem::transmute(pbpinpolicy),
            ::std::mem::transmute(cbpinpolicy),
            ::std::mem::transmute(attestationtype),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITpmVirtualSmartCardManager3 {
    type Vtable = ITpmVirtualSmartCardManager3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1014258327,
        62325,
        16720,
        [190, 23, 89, 80, 246, 148, 198, 153],
    );
}
impl ::std::convert::From<ITpmVirtualSmartCardManager3> for ::windows::runtime::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager3> for ::windows::runtime::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ITpmVirtualSmartCardManager3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ITpmVirtualSmartCardManager3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager2 {
    fn from(value: ITpmVirtualSmartCardManager3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager2 {
    fn from(value: &ITpmVirtualSmartCardManager3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager2>
    for ITpmVirtualSmartCardManager3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager2> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<ITpmVirtualSmartCardManager2>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager2>
    for &ITpmVirtualSmartCardManager3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager2> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<ITpmVirtualSmartCardManager2>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager {
    fn from(value: ITpmVirtualSmartCardManager3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager {
    fn from(value: &ITpmVirtualSmartCardManager3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager>
    for ITpmVirtualSmartCardManager3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITpmVirtualSmartCardManager>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager>
    for &ITpmVirtualSmartCardManager3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITpmVirtualSmartCardManager>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszfriendlyname: super::Foundation::PWSTR,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        fgenerate: super::Foundation::BOOL,
        pstatuscallback: ::windows::runtime::RawPtr,
        ppszinstanceid: *mut super::Foundation::PWSTR,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszinstanceid: super::Foundation::PWSTR,
        pstatuscallback: ::windows::runtime::RawPtr,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszfriendlyname: super::Foundation::PWSTR,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        fgenerate: super::Foundation::BOOL,
        pstatuscallback: ::windows::runtime::RawPtr,
        ppszinstanceid: *mut super::Foundation::PWSTR,
        pfneedreboot: *mut super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszfriendlyname: super::Foundation::PWSTR,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        attestationtype: TPMVSC_ATTESTATION_TYPE,
        fgenerate: super::Foundation::BOOL,
        pstatuscallback: ::windows::runtime::RawPtr,
        ppszinstanceid: *mut super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITpmVirtualSmartCardManagerStatusCallback(::windows::runtime::IUnknown);
impl ITpmVirtualSmartCardManagerStatusCallback {
    pub unsafe fn ReportProgress(
        &self,
        status: TPMVSCMGR_STATUS,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(status),
        )
        .ok()
    }
    pub unsafe fn ReportError(&self, error: TPMVSCMGR_ERROR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(error),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITpmVirtualSmartCardManagerStatusCallback {
    type Vtable = ITpmVirtualSmartCardManagerStatusCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        438023007,
        43960,
        17692,
        [161, 174, 51, 217, 143, 27, 239, 74],
    );
}
impl ::std::convert::From<ITpmVirtualSmartCardManagerStatusCallback>
    for ::windows::runtime::IUnknown
{
    fn from(value: ITpmVirtualSmartCardManagerStatusCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManagerStatusCallback>
    for ::windows::runtime::IUnknown
{
    fn from(value: &ITpmVirtualSmartCardManagerStatusCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ITpmVirtualSmartCardManagerStatusCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ITpmVirtualSmartCardManagerStatusCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManagerStatusCallback_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        status: TPMVSCMGR_STATUS,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        error: TPMVSCMGR_ERROR,
    ) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ImpersonateAnonymousToken<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    threadhandle: Param0,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImpersonateAnonymousToken(
                threadhandle: super::Foundation::HANDLE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImpersonateAnonymousToken(threadhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ImpersonateLoggedOnUser<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    htoken: Param0,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImpersonateLoggedOnUser(
                htoken: super::Foundation::HANDLE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImpersonateLoggedOnUser(htoken.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ImpersonateSelf(
    impersonationlevel: SECURITY_IMPERSONATION_LEVEL,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImpersonateSelf(
                impersonationlevel: SECURITY_IMPERSONATION_LEVEL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImpersonateSelf(::std::mem::transmute(impersonationlevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn InitializeAcl(
    pacl: *mut ACL,
    nacllength: u32,
    dwaclrevision: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeAcl(
                pacl: *mut ACL,
                nacllength: u32,
                dwaclrevision: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeAcl(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(nacllength),
            ::std::mem::transmute(dwaclrevision),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn InitializeSecurityDescriptor(
    psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
    dwrevision: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeSecurityDescriptor(
                psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
                dwrevision: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeSecurityDescriptor(
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(dwrevision),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn InitializeSid(
    sid: super::Foundation::PSID,
    pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY,
    nsubauthoritycount: u8,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeSid(
                sid: super::Foundation::PSID,
                pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY,
                nsubauthoritycount: u8,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeSid(
            ::std::mem::transmute(sid),
            ::std::mem::transmute(pidentifierauthority),
            ::std::mem::transmute(nsubauthoritycount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsCatalogFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hfile: Param0,
    pwszfilename: Param1,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsCatalogFile(
                hfile: super::Foundation::HANDLE,
                pwszfilename: super::Foundation::PWSTR,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsCatalogFile(
            hfile.into_param().abi(),
            pwszfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsTokenRestricted<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    tokenhandle: Param0,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsTokenRestricted(tokenhandle: super::Foundation::HANDLE)
                -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsTokenRestricted(tokenhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsValidSecurityDescriptor(
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsValidSecurityDescriptor(
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsValidSecurityDescriptor(::std::mem::transmute(
            psecuritydescriptor,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsValidSid<'a, Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>>(
    psid: Param0,
) -> super::Foundation::BOOL {
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsWellKnownSid<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    psid: Param0,
    wellknownsidtype: WELL_KNOWN_SID_TYPE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsWellKnownSid(
                psid: super::Foundation::PSID,
                wellknownsidtype: WELL_KNOWN_SID_TYPE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsWellKnownSid(
            psid.into_param().abi(),
            ::std::mem::transmute(wellknownsidtype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    type DefaultType = Self;
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
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
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
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
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
#[cfg(feature = "Win32_System_SystemServices")]
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: super::System::SystemServices::LUID,
    pub Attributes: TOKEN_PRIVILEGES_ATTRIBUTES,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl LUID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for LUID_AND_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for LUID_AND_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LUID_AND_ATTRIBUTES")
            .field("Luid", &self.Luid)
            .field("Attributes", &self.Attributes)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for LUID_AND_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Luid == other.Luid && self.Attributes == other.Attributes
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for LUID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for LUID_AND_ATTRIBUTES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LicenseProtectionStatus(pub i32);
pub const Success: LicenseProtectionStatus = LicenseProtectionStatus(0i32);
pub const LicenseKeyNotFound: LicenseProtectionStatus = LicenseProtectionStatus(1i32);
pub const LicenseKeyUnprotected: LicenseProtectionStatus = LicenseProtectionStatus(2i32);
pub const LicenseKeyCorrupted: LicenseProtectionStatus = LicenseProtectionStatus(3i32);
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = LicenseProtectionStatus(4i32);
impl ::std::convert::From<i32> for LicenseProtectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LicenseProtectionStatus {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LogonUserA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
>(
    lpszusername: Param0,
    lpszdomain: Param1,
    lpszpassword: Param2,
    dwlogontype: LOGON32_LOGON,
    dwlogonprovider: LOGON32_PROVIDER,
    phtoken: *mut super::Foundation::HANDLE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogonUserA(
                lpszusername: super::Foundation::PSTR,
                lpszdomain: super::Foundation::PSTR,
                lpszpassword: super::Foundation::PSTR,
                dwlogontype: LOGON32_LOGON,
                dwlogonprovider: LOGON32_PROVIDER,
                phtoken: *mut super::Foundation::HANDLE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LogonUserA(
            lpszusername.into_param().abi(),
            lpszdomain.into_param().abi(),
            lpszpassword.into_param().abi(),
            ::std::mem::transmute(dwlogontype),
            ::std::mem::transmute(dwlogonprovider),
            ::std::mem::transmute(phtoken),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LogonUserExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
>(
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
            fn LogonUserExA(
                lpszusername: super::Foundation::PSTR,
                lpszdomain: super::Foundation::PSTR,
                lpszpassword: super::Foundation::PSTR,
                dwlogontype: LOGON32_LOGON,
                dwlogonprovider: LOGON32_PROVIDER,
                phtoken: *mut super::Foundation::HANDLE,
                pplogonsid: *mut super::Foundation::PSID,
                ppprofilebuffer: *mut *mut ::std::ffi::c_void,
                pdwprofilelength: *mut u32,
                pquotalimits: *mut QUOTA_LIMITS,
            ) -> super::Foundation::BOOL;
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LogonUserExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
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
            fn LogonUserExW(
                lpszusername: super::Foundation::PWSTR,
                lpszdomain: super::Foundation::PWSTR,
                lpszpassword: super::Foundation::PWSTR,
                dwlogontype: LOGON32_LOGON,
                dwlogonprovider: LOGON32_PROVIDER,
                phtoken: *mut super::Foundation::HANDLE,
                pplogonsid: *mut super::Foundation::PSID,
                ppprofilebuffer: *mut *mut ::std::ffi::c_void,
                pdwprofilelength: *mut u32,
                pquotalimits: *mut QUOTA_LIMITS,
            ) -> super::Foundation::BOOL;
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LogonUserW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    lpszusername: Param0,
    lpszdomain: Param1,
    lpszpassword: Param2,
    dwlogontype: LOGON32_LOGON,
    dwlogonprovider: LOGON32_PROVIDER,
    phtoken: *mut super::Foundation::HANDLE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogonUserW(
                lpszusername: super::Foundation::PWSTR,
                lpszdomain: super::Foundation::PWSTR,
                lpszpassword: super::Foundation::PWSTR,
                dwlogontype: LOGON32_LOGON,
                dwlogonprovider: LOGON32_PROVIDER,
                phtoken: *mut super::Foundation::HANDLE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LogonUserW(
            lpszusername.into_param().abi(),
            lpszdomain.into_param().abi(),
            lpszpassword.into_param().abi(),
            ::std::mem::transmute(dwlogontype),
            ::std::mem::transmute(dwlogonprovider),
            ::std::mem::transmute(phtoken),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LookupAccountNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
>(
    lpsystemname: Param0,
    lpaccountname: Param1,
    sid: super::Foundation::PSID,
    cbsid: *mut u32,
    referenceddomainname: super::Foundation::PSTR,
    cchreferenceddomainname: *mut u32,
    peuse: *mut SID_NAME_USE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupAccountNameA(
                lpsystemname: super::Foundation::PSTR,
                lpaccountname: super::Foundation::PSTR,
                sid: super::Foundation::PSID,
                cbsid: *mut u32,
                referenceddomainname: super::Foundation::PSTR,
                cchreferenceddomainname: *mut u32,
                peuse: *mut SID_NAME_USE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupAccountNameA(
            lpsystemname.into_param().abi(),
            lpaccountname.into_param().abi(),
            ::std::mem::transmute(sid),
            ::std::mem::transmute(cbsid),
            ::std::mem::transmute(referenceddomainname),
            ::std::mem::transmute(cchreferenceddomainname),
            ::std::mem::transmute(peuse),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LookupAccountNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    lpsystemname: Param0,
    lpaccountname: Param1,
    sid: super::Foundation::PSID,
    cbsid: *mut u32,
    referenceddomainname: super::Foundation::PWSTR,
    cchreferenceddomainname: *mut u32,
    peuse: *mut SID_NAME_USE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupAccountNameW(
                lpsystemname: super::Foundation::PWSTR,
                lpaccountname: super::Foundation::PWSTR,
                sid: super::Foundation::PSID,
                cbsid: *mut u32,
                referenceddomainname: super::Foundation::PWSTR,
                cchreferenceddomainname: *mut u32,
                peuse: *mut SID_NAME_USE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupAccountNameW(
            lpsystemname.into_param().abi(),
            lpaccountname.into_param().abi(),
            ::std::mem::transmute(sid),
            ::std::mem::transmute(cbsid),
            ::std::mem::transmute(referenceddomainname),
            ::std::mem::transmute(cchreferenceddomainname),
            ::std::mem::transmute(peuse),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LookupAccountSidA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    lpsystemname: Param0,
    sid: Param1,
    name: super::Foundation::PSTR,
    cchname: *mut u32,
    referenceddomainname: super::Foundation::PSTR,
    cchreferenceddomainname: *mut u32,
    peuse: *mut SID_NAME_USE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupAccountSidA(
                lpsystemname: super::Foundation::PSTR,
                sid: super::Foundation::PSID,
                name: super::Foundation::PSTR,
                cchname: *mut u32,
                referenceddomainname: super::Foundation::PSTR,
                cchreferenceddomainname: *mut u32,
                peuse: *mut SID_NAME_USE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupAccountSidA(
            lpsystemname.into_param().abi(),
            sid.into_param().abi(),
            ::std::mem::transmute(name),
            ::std::mem::transmute(cchname),
            ::std::mem::transmute(referenceddomainname),
            ::std::mem::transmute(cchreferenceddomainname),
            ::std::mem::transmute(peuse),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LookupAccountSidW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
>(
    lpsystemname: Param0,
    sid: Param1,
    name: super::Foundation::PWSTR,
    cchname: *mut u32,
    referenceddomainname: super::Foundation::PWSTR,
    cchreferenceddomainname: *mut u32,
    peuse: *mut SID_NAME_USE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupAccountSidW(
                lpsystemname: super::Foundation::PWSTR,
                sid: super::Foundation::PSID,
                name: super::Foundation::PWSTR,
                cchname: *mut u32,
                referenceddomainname: super::Foundation::PWSTR,
                cchreferenceddomainname: *mut u32,
                peuse: *mut SID_NAME_USE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupAccountSidW(
            lpsystemname.into_param().abi(),
            sid.into_param().abi(),
            ::std::mem::transmute(name),
            ::std::mem::transmute(cchname),
            ::std::mem::transmute(referenceddomainname),
            ::std::mem::transmute(cchreferenceddomainname),
            ::std::mem::transmute(peuse),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LookupPrivilegeDisplayNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
>(
    lpsystemname: Param0,
    lpname: Param1,
    lpdisplayname: super::Foundation::PSTR,
    cchdisplayname: *mut u32,
    lplanguageid: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeDisplayNameA(
                lpsystemname: super::Foundation::PSTR,
                lpname: super::Foundation::PSTR,
                lpdisplayname: super::Foundation::PSTR,
                cchdisplayname: *mut u32,
                lplanguageid: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeDisplayNameA(
            lpsystemname.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(lpdisplayname),
            ::std::mem::transmute(cchdisplayname),
            ::std::mem::transmute(lplanguageid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LookupPrivilegeDisplayNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    lpsystemname: Param0,
    lpname: Param1,
    lpdisplayname: super::Foundation::PWSTR,
    cchdisplayname: *mut u32,
    lplanguageid: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeDisplayNameW(
                lpsystemname: super::Foundation::PWSTR,
                lpname: super::Foundation::PWSTR,
                lpdisplayname: super::Foundation::PWSTR,
                cchdisplayname: *mut u32,
                lplanguageid: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeDisplayNameW(
            lpsystemname.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(lpdisplayname),
            ::std::mem::transmute(cchdisplayname),
            ::std::mem::transmute(lplanguageid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn LookupPrivilegeNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
>(
    lpsystemname: Param0,
    lpluid: *const super::System::SystemServices::LUID,
    lpname: super::Foundation::PSTR,
    cchname: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeNameA(
                lpsystemname: super::Foundation::PSTR,
                lpluid: *const super::System::SystemServices::LUID,
                lpname: super::Foundation::PSTR,
                cchname: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeNameA(
            lpsystemname.into_param().abi(),
            ::std::mem::transmute(lpluid),
            ::std::mem::transmute(lpname),
            ::std::mem::transmute(cchname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn LookupPrivilegeNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    lpsystemname: Param0,
    lpluid: *const super::System::SystemServices::LUID,
    lpname: super::Foundation::PWSTR,
    cchname: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeNameW(
                lpsystemname: super::Foundation::PWSTR,
                lpluid: *const super::System::SystemServices::LUID,
                lpname: super::Foundation::PWSTR,
                cchname: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeNameW(
            lpsystemname.into_param().abi(),
            ::std::mem::transmute(lpluid),
            ::std::mem::transmute(lpname),
            ::std::mem::transmute(cchname),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn LookupPrivilegeValueA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
>(
    lpsystemname: Param0,
    lpname: Param1,
    lpluid: *mut super::System::SystemServices::LUID,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeValueA(
                lpsystemname: super::Foundation::PSTR,
                lpname: super::Foundation::PSTR,
                lpluid: *mut super::System::SystemServices::LUID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeValueA(
            lpsystemname.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(lpluid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn LookupPrivilegeValueW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    lpsystemname: Param0,
    lpname: Param1,
    lpluid: *mut super::System::SystemServices::LUID,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupPrivilegeValueW(
                lpsystemname: super::Foundation::PWSTR,
                lpname: super::Foundation::PWSTR,
                lpluid: *mut super::System::SystemServices::LUID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(LookupPrivilegeValueW(
            lpsystemname.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(lpluid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MSCEPSetupProperty(pub i32);
pub const ENUM_CEPSETUPPROP_USELOCALSYSTEM: MSCEPSetupProperty = MSCEPSetupProperty(0i32);
pub const ENUM_CEPSETUPPROP_USECHALLENGE: MSCEPSetupProperty = MSCEPSetupProperty(1i32);
pub const ENUM_CEPSETUPPROP_RANAME_CN: MSCEPSetupProperty = MSCEPSetupProperty(2i32);
pub const ENUM_CEPSETUPPROP_RANAME_EMAIL: MSCEPSetupProperty = MSCEPSetupProperty(3i32);
pub const ENUM_CEPSETUPPROP_RANAME_COMPANY: MSCEPSetupProperty = MSCEPSetupProperty(4i32);
pub const ENUM_CEPSETUPPROP_RANAME_DEPT: MSCEPSetupProperty = MSCEPSetupProperty(5i32);
pub const ENUM_CEPSETUPPROP_RANAME_CITY: MSCEPSetupProperty = MSCEPSetupProperty(6i32);
pub const ENUM_CEPSETUPPROP_RANAME_STATE: MSCEPSetupProperty = MSCEPSetupProperty(7i32);
pub const ENUM_CEPSETUPPROP_RANAME_COUNTRY: MSCEPSetupProperty = MSCEPSetupProperty(8i32);
pub const ENUM_CEPSETUPPROP_SIGNINGKEYINFORMATION: MSCEPSetupProperty = MSCEPSetupProperty(9i32);
pub const ENUM_CEPSETUPPROP_EXCHANGEKEYINFORMATION: MSCEPSetupProperty = MSCEPSetupProperty(10i32);
pub const ENUM_CEPSETUPPROP_CAINFORMATION: MSCEPSetupProperty = MSCEPSetupProperty(11i32);
pub const ENUM_CEPSETUPPROP_MSCEPURL: MSCEPSetupProperty = MSCEPSetupProperty(12i32);
pub const ENUM_CEPSETUPPROP_CHALLENGEURL: MSCEPSetupProperty = MSCEPSetupProperty(13i32);
impl ::std::convert::From<i32> for MSCEPSetupProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSCEPSetupProperty {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MSSIP_FLAGS_MULTI_HASH: u32 = 262144u32;
pub const MSSIP_FLAGS_PROHIBIT_RESIZE_ON_CREATE: u32 = 65536u32;
pub const MSSIP_FLAGS_USE_CATALOG: u32 = 131072u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MS_ADDINFO_BLOB {
    pub cbStruct: u32,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
impl MS_ADDINFO_BLOB {}
impl ::std::default::Default for MS_ADDINFO_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MS_ADDINFO_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MS_ADDINFO_BLOB")
            .field("cbStruct", &self.cbStruct)
            .field("cbMemObject", &self.cbMemObject)
            .field("pbMemObject", &self.pbMemObject)
            .field("cbMemSignedMsg", &self.cbMemSignedMsg)
            .field("pbMemSignedMsg", &self.pbMemSignedMsg)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MS_ADDINFO_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.cbMemObject == other.cbMemObject
            && self.pbMemObject == other.pbMemObject
            && self.cbMemSignedMsg == other.cbMemSignedMsg
            && self.pbMemSignedMsg == other.pbMemSignedMsg
    }
}
impl ::std::cmp::Eq for MS_ADDINFO_BLOB {}
unsafe impl ::windows::runtime::Abi for MS_ADDINFO_BLOB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct MS_ADDINFO_CATALOGMEMBER {
    pub cbStruct: u32,
    pub pStore: *mut CRYPTCATSTORE,
    pub pMember: *mut CRYPTCATMEMBER,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl MS_ADDINFO_CATALOGMEMBER {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for MS_ADDINFO_CATALOGMEMBER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for MS_ADDINFO_CATALOGMEMBER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MS_ADDINFO_CATALOGMEMBER")
            .field("cbStruct", &self.cbStruct)
            .field("pStore", &self.pStore)
            .field("pMember", &self.pMember)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for MS_ADDINFO_CATALOGMEMBER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pStore == other.pStore
            && self.pMember == other.pMember
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for MS_ADDINFO_CATALOGMEMBER {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for MS_ADDINFO_CATALOGMEMBER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct MS_ADDINFO_FLAT {
    pub cbStruct: u32,
    pub pIndirectData: *mut SIP_INDIRECT_DATA,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl MS_ADDINFO_FLAT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for MS_ADDINFO_FLAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for MS_ADDINFO_FLAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MS_ADDINFO_FLAT")
            .field("cbStruct", &self.cbStruct)
            .field("pIndirectData", &self.pIndirectData)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for MS_ADDINFO_FLAT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pIndirectData == other.pIndirectData
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for MS_ADDINFO_FLAT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for MS_ADDINFO_FLAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn MakeAbsoluteSD(
    pselfrelativesecuritydescriptor: *const SECURITY_DESCRIPTOR,
    pabsolutesecuritydescriptor: *mut SECURITY_DESCRIPTOR,
    lpdwabsolutesecuritydescriptorsize: *mut u32,
    pdacl: *mut ACL,
    lpdwdaclsize: *mut u32,
    psacl: *mut ACL,
    lpdwsaclsize: *mut u32,
    powner: super::Foundation::PSID,
    lpdwownersize: *mut u32,
    pprimarygroup: super::Foundation::PSID,
    lpdwprimarygroupsize: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MakeAbsoluteSD(
                pselfrelativesecuritydescriptor: *const SECURITY_DESCRIPTOR,
                pabsolutesecuritydescriptor: *mut SECURITY_DESCRIPTOR,
                lpdwabsolutesecuritydescriptorsize: *mut u32,
                pdacl: *mut ACL,
                lpdwdaclsize: *mut u32,
                psacl: *mut ACL,
                lpdwsaclsize: *mut u32,
                powner: super::Foundation::PSID,
                lpdwownersize: *mut u32,
                pprimarygroup: super::Foundation::PSID,
                lpdwprimarygroupsize: *mut u32,
            ) -> super::Foundation::BOOL;
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
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn MakeSelfRelativeSD(
    pabsolutesecuritydescriptor: *const SECURITY_DESCRIPTOR,
    pselfrelativesecuritydescriptor: *mut SECURITY_DESCRIPTOR,
    lpdwbufferlength: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MakeSelfRelativeSD(
                pabsolutesecuritydescriptor: *const SECURITY_DESCRIPTOR,
                pselfrelativesecuritydescriptor: *mut SECURITY_DESCRIPTOR,
                lpdwbufferlength: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(MakeSelfRelativeSD(
            ::std::mem::transmute(pabsolutesecuritydescriptor),
            ::std::mem::transmute(pselfrelativesecuritydescriptor),
            ::std::mem::transmute(lpdwbufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn MapGenericMask(accessmask: *mut u32, genericmapping: *const GENERIC_MAPPING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapGenericMask(accessmask: *mut u32, genericmapping: *const GENERIC_MAPPING);
        }
        ::std::mem::transmute(MapGenericMask(
            ::std::mem::transmute(accessmask),
            ::std::mem::transmute(genericmapping),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
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
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("OBJECT_TYPE_LIST")
            .field("Level", &self.Level)
            .field("Sbz", &self.Sbz)
            .field("ObjectType", &self.ObjectType)
            .finish()
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
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ObjectCloseAuditAlarmA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    generateonclose: Param2,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectCloseAuditAlarmA(
                subsystemname: super::Foundation::PSTR,
                handleid: *const ::std::ffi::c_void,
                generateonclose: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectCloseAuditAlarmA(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            generateonclose.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ObjectCloseAuditAlarmW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    generateonclose: Param2,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectCloseAuditAlarmW(
                subsystemname: super::Foundation::PWSTR,
                handleid: *const ::std::ffi::c_void,
                generateonclose: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectCloseAuditAlarmW(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            generateonclose.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ObjectDeleteAuditAlarmA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    generateonclose: Param2,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectDeleteAuditAlarmA(
                subsystemname: super::Foundation::PSTR,
                handleid: *const ::std::ffi::c_void,
                generateonclose: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectDeleteAuditAlarmA(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            generateonclose.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ObjectDeleteAuditAlarmW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    generateonclose: Param2,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectDeleteAuditAlarmW(
                subsystemname: super::Foundation::PWSTR,
                handleid: *const ::std::ffi::c_void,
                generateonclose: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectDeleteAuditAlarmW(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            generateonclose.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ObjectOpenAuditAlarmA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param9: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
    Param10: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
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
            fn ObjectOpenAuditAlarmA(
                subsystemname: super::Foundation::PSTR,
                handleid: *const ::std::ffi::c_void,
                objecttypename: super::Foundation::PSTR,
                objectname: super::Foundation::PSTR,
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
                clienttoken: super::Foundation::HANDLE,
                desiredaccess: u32,
                grantedaccess: u32,
                privileges: *const PRIVILEGE_SET,
                objectcreation: super::Foundation::BOOL,
                accessgranted: super::Foundation::BOOL,
                generateonclose: *mut i32,
            ) -> super::Foundation::BOOL;
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ObjectOpenAuditAlarmW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param9: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
    Param10: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
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
            fn ObjectOpenAuditAlarmW(
                subsystemname: super::Foundation::PWSTR,
                handleid: *const ::std::ffi::c_void,
                objecttypename: super::Foundation::PWSTR,
                objectname: super::Foundation::PWSTR,
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
                clienttoken: super::Foundation::HANDLE,
                desiredaccess: u32,
                grantedaccess: u32,
                privileges: *const PRIVILEGE_SET,
                objectcreation: super::Foundation::BOOL,
                accessgranted: super::Foundation::BOOL,
                generateonclose: *mut i32,
            ) -> super::Foundation::BOOL;
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ObjectPrivilegeAuditAlarmA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    clienttoken: Param2,
    desiredaccess: u32,
    privileges: *const PRIVILEGE_SET,
    accessgranted: Param5,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectPrivilegeAuditAlarmA(
                subsystemname: super::Foundation::PSTR,
                handleid: *const ::std::ffi::c_void,
                clienttoken: super::Foundation::HANDLE,
                desiredaccess: u32,
                privileges: *const PRIVILEGE_SET,
                accessgranted: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectPrivilegeAuditAlarmA(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            clienttoken.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(privileges),
            accessgranted.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ObjectPrivilegeAuditAlarmW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    subsystemname: Param0,
    handleid: *const ::std::ffi::c_void,
    clienttoken: Param2,
    desiredaccess: u32,
    privileges: *const PRIVILEGE_SET,
    accessgranted: Param5,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObjectPrivilegeAuditAlarmW(
                subsystemname: super::Foundation::PWSTR,
                handleid: *const ::std::ffi::c_void,
                clienttoken: super::Foundation::HANDLE,
                desiredaccess: u32,
                privileges: *const PRIVILEGE_SET,
                accessgranted: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(ObjectPrivilegeAuditAlarmW(
            subsystemname.into_param().abi(),
            ::std::mem::transmute(handleid),
            clienttoken.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(privileges),
            accessgranted.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenPersonalTrustDBDialog<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HWND>,
>(
    hwndparent: Param0,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPersonalTrustDBDialog(
                hwndparent: super::Foundation::HWND,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(OpenPersonalTrustDBDialog(hwndparent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenPersonalTrustDBDialogEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HWND>,
>(
    hwndparent: Param0,
    dwflags: u32,
    pvreserved: *mut *mut ::std::ffi::c_void,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPersonalTrustDBDialogEx(
                hwndparent: super::Foundation::HWND,
                dwflags: u32,
                pvreserved: *mut *mut ::std::ffi::c_void,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(OpenPersonalTrustDBDialogEx(
            hwndparent.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pvreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
pub type PFNDSCREATEISECINFO = unsafe extern "system" fn(
    param0: super::Foundation::PWSTR,
    param1: super::Foundation::PWSTR,
    param2: u32,
    param3: *mut ::windows::runtime::RawPtr,
    param4: ::windows::runtime::RawPtr,
    param5: ::windows::runtime::RawPtr,
    param6: super::Foundation::LPARAM,
) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
pub type PFNDSCREATEISECINFOEX = unsafe extern "system" fn(
    param0: super::Foundation::PWSTR,
    param1: super::Foundation::PWSTR,
    param2: super::Foundation::PWSTR,
    param3: super::Foundation::PWSTR,
    param4: super::Foundation::PWSTR,
    param5: u32,
    param6: *mut ::windows::runtime::RawPtr,
    param7: ::windows::runtime::RawPtr,
    param8: ::windows::runtime::RawPtr,
    param9: super::Foundation::LPARAM,
) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type PFNDSCREATESECPAGE = unsafe extern "system" fn(
    param0: super::Foundation::PWSTR,
    param1: super::Foundation::PWSTR,
    param2: u32,
    param3: *mut super::UI::Controls::HPROPSHEETPAGE,
    param4: ::windows::runtime::RawPtr,
    param5: ::windows::runtime::RawPtr,
    param6: super::Foundation::LPARAM,
) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFNDSEDITSECURITY = unsafe extern "system" fn(
    param0: super::Foundation::HWND,
    param1: super::Foundation::PWSTR,
    param2: super::Foundation::PWSTR,
    param3: u32,
    param4: super::Foundation::PWSTR,
    param5: ::windows::runtime::RawPtr,
    param6: ::windows::runtime::RawPtr,
    param7: super::Foundation::LPARAM,
) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFNMSGECALLBACK = unsafe extern "system" fn(
    bverbose: super::Foundation::BOOL,
    lpmessage: super::Foundation::PWSTR,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNREADOBJECTSECURITY = unsafe extern "system" fn(
    param0: super::Foundation::PWSTR,
    param1: u32,
    param2: *mut *mut SECURITY_DESCRIPTOR,
    param3: super::Foundation::LPARAM,
) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFNWRITEOBJECTSECURITY = unsafe extern "system" fn(
    param0: super::Foundation::PWSTR,
    param1: u32,
    param2: *mut SECURITY_DESCRIPTOR,
    param3: super::Foundation::LPARAM,
) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_ALLOCANDFILLDEFUSAGE = unsafe extern "system" fn(
    pszusageoid: super::Foundation::PSTR,
    psdefusage: *const CRYPT_PROVIDER_DEFUSAGE,
) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_CDF_PARSE_ERROR_CALLBACK = unsafe extern "system" fn(
    dwerrorarea: u32,
    dwlocalerror: u32,
    pwszline: super::Foundation::PWSTR,
);
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_CPD_ADD_CERT = unsafe extern "system" fn(
    pprovdata: *const CRYPT_PROVIDER_DATA,
    idxsigner: u32,
    fcountersigner: super::Foundation::BOOL,
    idxcountersigner: u32,
    pcert2add: *const Cryptography::Core::CERT_CONTEXT,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_CPD_ADD_PRIVDATA = unsafe extern "system" fn(
    pprovdata: *const CRYPT_PROVIDER_DATA,
    pprivdata2add: *const CRYPT_PROVIDER_PRIVDATA,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_CPD_ADD_SGNR = unsafe extern "system" fn(
    pprovdata: *const CRYPT_PROVIDER_DATA,
    fcountersigner: super::Foundation::BOOL,
    idxsigner: u32,
    psgnr2add: *const CRYPT_PROVIDER_SGNR,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_CPD_ADD_STORE = unsafe extern "system" fn(
    pprovdata: *const CRYPT_PROVIDER_DATA,
    hstore2add: *const ::std::ffi::c_void,
) -> super::Foundation::BOOL;
pub type PFN_CPD_MEM_ALLOC = unsafe extern "system" fn(cbsize: u32) -> *mut ::std::ffi::c_void;
pub type PFN_CPD_MEM_FREE = unsafe extern "system" fn(pvmem2free: *const ::std::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FREEDEFUSAGE = unsafe extern "system" fn(
    pszusageoid: super::Foundation::PSTR,
    psdefusage: *const CRYPT_PROVIDER_DEFUSAGE,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_PROVIDER_CERTCHKPOLICY_CALL = unsafe extern "system" fn(
    pprovdata: *const CRYPT_PROVIDER_DATA,
    idxsigner: u32,
    fcountersignerchain: super::Foundation::BOOL,
    idxcountersigner: u32,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_PROVIDER_CERTTRUST_CALL =
    unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_PROVIDER_CLEANUP_CALL =
    unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_PROVIDER_FINALPOLICY_CALL =
    unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_PROVIDER_INIT_CALL =
    unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_PROVIDER_OBJTRUST_CALL =
    unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_PROVIDER_SIGTRUST_CALL =
    unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_PROVIDER_TESTFINALPOLICY_CALL =
    unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_PROVUI_CALL = unsafe extern "system" fn(
    hwndsecuritydialog: super::Foundation::HWND,
    pprovdata: *const CRYPT_PROVIDER_DATA,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type PFN_WTD_GENERIC_CHAIN_POLICY_CALLBACK =
    unsafe extern "system" fn(
        pprovdata: *mut CRYPT_PROVIDER_DATA,
        dwsteperror: u32,
        dwregpolicysettings: u32,
        csigner: u32,
        rgpsigner: *mut *mut WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO,
        pvpolicyarg: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT;
pub type PFSCE_FREE_INFO = unsafe extern "system" fn(pvserviceinfo: *mut ::std::ffi::c_void) -> u32;
pub type PFSCE_LOG_INFO =
    unsafe extern "system" fn(errlevel: SCE_LOG_ERR_LEVEL, win32rc: u32, perrfmt: *mut i8) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_QUERY_INFO = unsafe extern "system" fn(
    scehandle: *mut ::std::ffi::c_void,
    scetype: SCESVC_INFO_TYPE,
    lpprefix: *mut i8,
    bexact: super::Foundation::BOOL,
    ppvinfo: *mut *mut ::std::ffi::c_void,
    psceenumhandle: *mut u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_SET_INFO = unsafe extern "system" fn(
    scehandle: *mut ::std::ffi::c_void,
    scetype: SCESVC_INFO_TYPE,
    lpprefix: *mut i8,
    bexact: super::Foundation::BOOL,
    pvinfo: *mut ::std::ffi::c_void,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_ConfigAnalyzeService = unsafe extern "system" fn(
    pscecbinfo: *mut ::std::mem::ManuallyDrop<SCESVC_CALLBACK_INFO>,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_UpdateService = unsafe extern "system" fn(
    pscecbinfo: *mut ::std::mem::ManuallyDrop<SCESVC_CALLBACK_INFO>,
    serviceinfo: *mut SCESVC_CONFIGURATION_INFO,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_CALL_PACKAGE_UNTRUSTED = unsafe extern "system" fn(
    clientrequest: *const *const ::std::ffi::c_void,
    protocolsubmitbuffer: *const ::std::ffi::c_void,
    clientbufferbase: *const ::std::ffi::c_void,
    submitbufferlength: u32,
    protocolreturnbuffer: *mut *mut ::std::ffi::c_void,
    returnbufferlength: *mut u32,
    protocolstatus: *mut i32,
) -> super::Foundation::NTSTATUS;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct PRIVILEGE_SET {
    pub PrivilegeCount: u32,
    pub Control: u32,
    pub Privilege: [LUID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_System_SystemServices")]
impl PRIVILEGE_SET {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for PRIVILEGE_SET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for PRIVILEGE_SET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PRIVILEGE_SET")
            .field("PrivilegeCount", &self.PrivilegeCount)
            .field("Control", &self.Control)
            .field("Privilege", &self.Privilege)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for PRIVILEGE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.PrivilegeCount == other.PrivilegeCount
            && self.Control == other.Control
            && self.Privilege == other.Privilege
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for PRIVILEGE_SET {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for PRIVILEGE_SET {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct PROVDATA_SIP {
    pub cbStruct: u32,
    pub gSubject: ::windows::runtime::GUID,
    pub pSip: *mut SIP_DISPATCH_INFO,
    pub pCATSip: *mut SIP_DISPATCH_INFO,
    pub psSipSubjectInfo: *mut SIP_SUBJECTINFO,
    pub psSipCATSubjectInfo: *mut SIP_SUBJECTINFO,
    pub psIndirectData: *mut SIP_INDIRECT_DATA,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl PROVDATA_SIP {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for PROVDATA_SIP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for PROVDATA_SIP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROVDATA_SIP")
            .field("cbStruct", &self.cbStruct)
            .field("gSubject", &self.gSubject)
            .field("pSip", &self.pSip)
            .field("pCATSip", &self.pCATSip)
            .field("psSipSubjectInfo", &self.psSipSubjectInfo)
            .field("psSipCATSubjectInfo", &self.psSipCATSubjectInfo)
            .field("psIndirectData", &self.psIndirectData)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for PROVDATA_SIP {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.gSubject == other.gSubject
            && self.pSip == other.pSip
            && self.pCATSip == other.pCATSip
            && self.psSipSubjectInfo == other.psSipSubjectInfo
            && self.psSipCATSubjectInfo == other.psSipCATSubjectInfo
            && self.psIndirectData == other.psIndirectData
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for PROVDATA_SIP {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for PROVDATA_SIP {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_ASSIGN_SHELL_PROTECTION = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    htoken: super::Foundation::HANDLE,
    hprocess: super::Foundation::HANDLE,
    hthread: super::Foundation::HANDLE,
) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_CHANGE_PASSWORD_NOTIFY = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    pmprinfo: *mut WLX_MPR_NOTIFY_INFO,
    dwchangeinfo: u32,
) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_CHANGE_PASSWORD_NOTIFY_EX = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    pmprinfo: *mut WLX_MPR_NOTIFY_INFO,
    dwchangeinfo: u32,
    providername: super::Foundation::PWSTR,
    reserved: *mut ::std::ffi::c_void,
) -> i32;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
pub type PWLX_CLOSE_USER_DESKTOP = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    pdesktop: *mut WLX_DESKTOP,
    htoken: super::Foundation::HANDLE,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
pub type PWLX_CREATE_USER_DESKTOP = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    htoken: super::Foundation::HANDLE,
    flags: u32,
    pszdesktopname: super::Foundation::PWSTR,
    ppdesktop: *mut *mut WLX_DESKTOP,
) -> super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PWLX_DIALOG_BOX = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    hinst: super::Foundation::HANDLE,
    lpsztemplate: super::Foundation::PWSTR,
    hwndowner: super::Foundation::HWND,
    dlgprc: ::windows::runtime::RawPtr,
) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PWLX_DIALOG_BOX_INDIRECT = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    hinst: super::Foundation::HANDLE,
    hdialogtemplate: *mut super::UI::WindowsAndMessaging::DLGTEMPLATE,
    hwndowner: super::Foundation::HWND,
    dlgprc: ::windows::runtime::RawPtr,
) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PWLX_DIALOG_BOX_INDIRECT_PARAM = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    hinst: super::Foundation::HANDLE,
    hdialogtemplate: *mut super::UI::WindowsAndMessaging::DLGTEMPLATE,
    hwndowner: super::Foundation::HWND,
    dlgprc: ::windows::runtime::RawPtr,
    dwinitparam: super::Foundation::LPARAM,
) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PWLX_DIALOG_BOX_PARAM = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    hinst: super::Foundation::HANDLE,
    lpsztemplate: super::Foundation::PWSTR,
    hwndowner: super::Foundation::HWND,
    dlgprc: ::windows::runtime::RawPtr,
    dwinitparam: super::Foundation::LPARAM,
) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_DISCONNECT = unsafe extern "system" fn() -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_GET_OPTION = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    option: u32,
    value: *mut usize,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
pub type PWLX_GET_SOURCE_DESKTOP = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    ppdesktop: *mut *mut WLX_DESKTOP,
) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_MESSAGE_BOX = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    hwndowner: super::Foundation::HWND,
    lpsztext: super::Foundation::PWSTR,
    lpsztitle: super::Foundation::PWSTR,
    fustyle: u32,
) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_QUERY_CLIENT_CREDENTIALS = unsafe extern "system" fn(
    pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V1_0,
) -> super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub type PWLX_QUERY_CONSOLESWITCH_CREDENTIALS =
    unsafe extern "system" fn(pcred: *mut WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_QUERY_IC_CREDENTIALS = unsafe extern "system" fn(
    pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V1_0,
) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_QUERY_TERMINAL_SERVICES_DATA = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    ptsdata: *mut WLX_TERMINAL_SERVICES_DATA,
    username: super::Foundation::PWSTR,
    domain: super::Foundation::PWSTR,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_QUERY_TS_LOGON_CREDENTIALS = unsafe extern "system" fn(
    pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V2_0,
) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SAS_NOTIFY =
    unsafe extern "system" fn(hwlx: super::Foundation::HANDLE, dwsastype: u32);
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SET_CONTEXT_POINTER = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    pwlxcontext: *mut ::std::ffi::c_void,
);
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SET_OPTION = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    option: u32,
    value: usize,
    oldvalue: *mut usize,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
pub type PWLX_SET_RETURN_DESKTOP = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    pdesktop: *mut WLX_DESKTOP,
) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SET_TIMEOUT = unsafe extern "system" fn(
    hwlx: super::Foundation::HANDLE,
    timeout: u32,
) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SWITCH_DESKTOP_TO_USER =
    unsafe extern "system" fn(hwlx: super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SWITCH_DESKTOP_TO_WINLOGON =
    unsafe extern "system" fn(hwlx: super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_USE_CTRL_ALT_DEL = unsafe extern "system" fn(hwlx: super::Foundation::HANDLE);
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_WIN31_MIGRATE = unsafe extern "system" fn(hwlx: super::Foundation::HANDLE);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn PrivilegeCheck<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    clienttoken: Param0,
    requiredprivileges: *mut PRIVILEGE_SET,
    pfresult: *mut i32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrivilegeCheck(
                clienttoken: super::Foundation::HANDLE,
                requiredprivileges: *mut PRIVILEGE_SET,
                pfresult: *mut i32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrivilegeCheck(
            clienttoken.into_param().abi(),
            ::std::mem::transmute(requiredprivileges),
            ::std::mem::transmute(pfresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn PrivilegedServiceAuditAlarmA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    subsystemname: Param0,
    servicename: Param1,
    clienttoken: Param2,
    privileges: *const PRIVILEGE_SET,
    accessgranted: Param4,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrivilegedServiceAuditAlarmA(
                subsystemname: super::Foundation::PSTR,
                servicename: super::Foundation::PSTR,
                clienttoken: super::Foundation::HANDLE,
                privileges: *const PRIVILEGE_SET,
                accessgranted: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrivilegedServiceAuditAlarmA(
            subsystemname.into_param().abi(),
            servicename.into_param().abi(),
            clienttoken.into_param().abi(),
            ::std::mem::transmute(privileges),
            accessgranted.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn PrivilegedServiceAuditAlarmW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    subsystemname: Param0,
    servicename: Param1,
    clienttoken: Param2,
    privileges: *const PRIVILEGE_SET,
    accessgranted: Param4,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrivilegedServiceAuditAlarmW(
                subsystemname: super::Foundation::PWSTR,
                servicename: super::Foundation::PWSTR,
                clienttoken: super::Foundation::HANDLE,
                privileges: *const PRIVILEGE_SET,
                accessgranted: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrivilegedServiceAuditAlarmW(
            subsystemname.into_param().abi(),
            servicename.into_param().abi(),
            clienttoken.into_param().abi(),
            ::std::mem::transmute(privileges),
            accessgranted.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        self.PagedPoolLimit == other.PagedPoolLimit
            && self.NonPagedPoolLimit == other.NonPagedPoolLimit
            && self.MinimumWorkingSetSize == other.MinimumWorkingSetSize
            && self.MaximumWorkingSetSize == other.MaximumWorkingSetSize
            && self.PagefileLimit == other.PagefileLimit
            && self.TimeLimit == other.TimeLimit
    }
}
impl ::std::cmp::Eq for QUOTA_LIMITS {}
unsafe impl ::windows::runtime::Abi for QUOTA_LIMITS {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn QuerySecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QuerySecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32);
        }
        ::std::mem::transmute(QuerySecurityAccessMask(
            ::std::mem::transmute(securityinformation),
            ::std::mem::transmute(desiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RegisterLicenseKeyWithExpiration<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    licensekey: Param0,
    validityindays: u32,
) -> ::windows::runtime::Result<LicenseProtectionStatus> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterLicenseKeyWithExpiration(
                licensekey: super::Foundation::PWSTR,
                validityindays: u32,
                status: *mut LicenseProtectionStatus,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <LicenseProtectionStatus as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        RegisterLicenseKeyWithExpiration(
            licensekey.into_param().abi(),
            ::std::mem::transmute(validityindays),
            &mut result__,
        )
        .from_abi::<LicenseProtectionStatus>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const RemoteTpmVirtualSmartCardManager: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        355377832,
        28892,
        19545,
        [139, 42, 50, 170, 60, 160, 220, 172],
    );
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub unsafe fn RtlConvertSidToUnicodeString<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOLEAN>,
>(
    unicodestring: *mut super::System::Kernel::UNICODE_STRING,
    sid: Param1,
    allocatedestinationstring: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlConvertSidToUnicodeString(
                unicodestring: *mut super::System::Kernel::UNICODE_STRING,
                sid: super::Foundation::PSID,
                allocatedestinationstring: super::Foundation::BOOLEAN,
            ) -> super::Foundation::NTSTATUS;
        }
        RtlConvertSidToUnicodeString(
            ::std::mem::transmute(unicodestring),
            sid.into_param().abi(),
            allocatedestinationstring.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_CODE_PROPERTIES_V1 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: super::Foundation::PWSTR,
    pub hImageFileHandle: super::Foundation::HANDLE,
    pub UrlZoneId: u32,
    pub ImageHash: [u8; 64],
    pub dwImageHashSize: u32,
    pub ImageSize: i64,
    pub HashAlgorithm: u32,
    pub pByteBlock: *mut u8,
    pub hWndParent: super::Foundation::HWND,
    pub dwWVTUIChoice: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_CODE_PROPERTIES_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SAFER_CODE_PROPERTIES_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SAFER_CODE_PROPERTIES_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAFER_CODE_PROPERTIES_V1")
            .field("cbSize", &self.cbSize)
            .field("dwCheckFlags", &self.dwCheckFlags)
            .field("ImagePath", &self.ImagePath)
            .field("hImageFileHandle", &self.hImageFileHandle)
            .field("UrlZoneId", &self.UrlZoneId)
            .field("ImageHash", &self.ImageHash)
            .field("dwImageHashSize", &self.dwImageHashSize)
            .field("ImageSize", &self.ImageSize)
            .field("HashAlgorithm", &self.HashAlgorithm)
            .field("pByteBlock", &self.pByteBlock)
            .field("hWndParent", &self.hWndParent)
            .field("dwWVTUIChoice", &self.dwWVTUIChoice)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SAFER_CODE_PROPERTIES_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwCheckFlags == other.dwCheckFlags
            && self.ImagePath == other.ImagePath
            && self.hImageFileHandle == other.hImageFileHandle
            && self.UrlZoneId == other.UrlZoneId
            && self.ImageHash == other.ImageHash
            && self.dwImageHashSize == other.dwImageHashSize
            && self.ImageSize == other.ImageSize
            && self.HashAlgorithm == other.HashAlgorithm
            && self.pByteBlock == other.pByteBlock
            && self.hWndParent == other.hWndParent
            && self.dwWVTUIChoice == other.dwWVTUIChoice
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SAFER_CODE_PROPERTIES_V1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SAFER_CODE_PROPERTIES_V1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_CODE_PROPERTIES_V2 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: super::Foundation::PWSTR,
    pub hImageFileHandle: super::Foundation::HANDLE,
    pub UrlZoneId: u32,
    pub ImageHash: [u8; 64],
    pub dwImageHashSize: u32,
    pub ImageSize: i64,
    pub HashAlgorithm: u32,
    pub pByteBlock: *mut u8,
    pub hWndParent: super::Foundation::HWND,
    pub dwWVTUIChoice: u32,
    pub PackageMoniker: super::Foundation::PWSTR,
    pub PackagePublisher: super::Foundation::PWSTR,
    pub PackageName: super::Foundation::PWSTR,
    pub PackageVersion: u64,
    pub PackageIsFramework: super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_CODE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SAFER_CODE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SAFER_CODE_PROPERTIES_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAFER_CODE_PROPERTIES_V2")
            .field("cbSize", &self.cbSize)
            .field("dwCheckFlags", &self.dwCheckFlags)
            .field("ImagePath", &self.ImagePath)
            .field("hImageFileHandle", &self.hImageFileHandle)
            .field("UrlZoneId", &self.UrlZoneId)
            .field("ImageHash", &self.ImageHash)
            .field("dwImageHashSize", &self.dwImageHashSize)
            .field("ImageSize", &self.ImageSize)
            .field("HashAlgorithm", &self.HashAlgorithm)
            .field("pByteBlock", &self.pByteBlock)
            .field("hWndParent", &self.hWndParent)
            .field("dwWVTUIChoice", &self.dwWVTUIChoice)
            .field("PackageMoniker", &self.PackageMoniker)
            .field("PackagePublisher", &self.PackagePublisher)
            .field("PackageName", &self.PackageName)
            .field("PackageVersion", &self.PackageVersion)
            .field("PackageIsFramework", &self.PackageIsFramework)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SAFER_CODE_PROPERTIES_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwCheckFlags == other.dwCheckFlags
            && self.ImagePath == other.ImagePath
            && self.hImageFileHandle == other.hImageFileHandle
            && self.UrlZoneId == other.UrlZoneId
            && self.ImageHash == other.ImageHash
            && self.dwImageHashSize == other.dwImageHashSize
            && self.ImageSize == other.ImageSize
            && self.HashAlgorithm == other.HashAlgorithm
            && self.pByteBlock == other.pByteBlock
            && self.hWndParent == other.hWndParent
            && self.dwWVTUIChoice == other.dwWVTUIChoice
            && self.PackageMoniker == other.PackageMoniker
            && self.PackagePublisher == other.PackagePublisher
            && self.PackageName == other.PackageName
            && self.PackageVersion == other.PackageVersion
            && self.PackageIsFramework == other.PackageIsFramework
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SAFER_CODE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SAFER_CODE_PROPERTIES_V2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(pub u32);
pub const SAFER_TOKEN_NULL_IF_EQUAL: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS =
    SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(1u32);
pub const SAFER_TOKEN_COMPARE_ONLY: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS =
    SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(2u32);
pub const SAFER_TOKEN_MAKE_INERT: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS =
    SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(4u32);
pub const SAFER_TOKEN_WANT_FLAGS: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS =
    SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(8u32);
impl ::std::convert::From<u32> for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SAFER_CRITERIA_APPX_PACKAGE: u32 = 32u32;
pub const SAFER_CRITERIA_AUTHENTICODE: u32 = 8u32;
pub const SAFER_CRITERIA_IMAGEHASH: u32 = 4u32;
pub const SAFER_CRITERIA_IMAGEPATH: u32 = 1u32;
pub const SAFER_CRITERIA_IMAGEPATH_NT: u32 = 4096u32;
pub const SAFER_CRITERIA_NOSIGNEDHASH: u32 = 2u32;
pub const SAFER_CRITERIA_URLZONE: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_HASH_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub Description: [u16; 256],
    pub FriendlyName: [u16; 256],
    pub HashSize: u32,
    pub ImageHash: [u8; 64],
    pub HashAlgorithm: u32,
    pub ImageSize: i64,
    pub dwSaferFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_HASH_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SAFER_HASH_IDENTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SAFER_HASH_IDENTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAFER_HASH_IDENTIFICATION")
            .field("header", &self.header)
            .field("Description", &self.Description)
            .field("FriendlyName", &self.FriendlyName)
            .field("HashSize", &self.HashSize)
            .field("ImageHash", &self.ImageHash)
            .field("HashAlgorithm", &self.HashAlgorithm)
            .field("ImageSize", &self.ImageSize)
            .field("dwSaferFlags", &self.dwSaferFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SAFER_HASH_IDENTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.Description == other.Description
            && self.FriendlyName == other.FriendlyName
            && self.HashSize == other.HashSize
            && self.ImageHash == other.ImageHash
            && self.HashAlgorithm == other.HashAlgorithm
            && self.ImageSize == other.ImageSize
            && self.dwSaferFlags == other.dwSaferFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SAFER_HASH_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SAFER_HASH_IDENTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_HASH_IDENTIFICATION2 {
    pub hashIdentification: SAFER_HASH_IDENTIFICATION,
    pub HashSize: u32,
    pub ImageHash: [u8; 64],
    pub HashAlgorithm: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_HASH_IDENTIFICATION2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SAFER_HASH_IDENTIFICATION2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SAFER_HASH_IDENTIFICATION2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAFER_HASH_IDENTIFICATION2")
            .field("hashIdentification", &self.hashIdentification)
            .field("HashSize", &self.HashSize)
            .field("ImageHash", &self.ImageHash)
            .field("HashAlgorithm", &self.HashAlgorithm)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SAFER_HASH_IDENTIFICATION2 {
    fn eq(&self, other: &Self) -> bool {
        self.hashIdentification == other.hashIdentification
            && self.HashSize == other.HashSize
            && self.ImageHash == other.ImageHash
            && self.HashAlgorithm == other.HashAlgorithm
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SAFER_HASH_IDENTIFICATION2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SAFER_HASH_IDENTIFICATION2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_IDENTIFICATION_HEADER {
    pub dwIdentificationType: SAFER_IDENTIFICATION_TYPES,
    pub cbStructSize: u32,
    pub IdentificationGuid: ::windows::runtime::GUID,
    pub lastModified: super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_IDENTIFICATION_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SAFER_IDENTIFICATION_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SAFER_IDENTIFICATION_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAFER_IDENTIFICATION_HEADER")
            .field("dwIdentificationType", &self.dwIdentificationType)
            .field("cbStructSize", &self.cbStructSize)
            .field("IdentificationGuid", &self.IdentificationGuid)
            .field("lastModified", &self.lastModified)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SAFER_IDENTIFICATION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdentificationType == other.dwIdentificationType
            && self.cbStructSize == other.cbStructSize
            && self.IdentificationGuid == other.IdentificationGuid
            && self.lastModified == other.lastModified
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SAFER_IDENTIFICATION_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SAFER_IDENTIFICATION_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SAFER_IDENTIFICATION_TYPES(pub i32);
pub const SaferIdentityDefault: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(0i32);
pub const SaferIdentityTypeImageName: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(1i32);
pub const SaferIdentityTypeImageHash: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(2i32);
pub const SaferIdentityTypeUrlZone: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(3i32);
pub const SaferIdentityTypeCertificate: SAFER_IDENTIFICATION_TYPES =
    SAFER_IDENTIFICATION_TYPES(4i32);
impl ::std::convert::From<i32> for SAFER_IDENTIFICATION_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SAFER_IDENTIFICATION_TYPES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SAFER_LEVELID_CONSTRAINED: u32 = 65536u32;
pub const SAFER_LEVELID_DISALLOWED: u32 = 0u32;
pub const SAFER_LEVELID_FULLYTRUSTED: u32 = 262144u32;
pub const SAFER_LEVELID_NORMALUSER: u32 = 131072u32;
pub const SAFER_LEVELID_UNTRUSTED: u32 = 4096u32;
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
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
    type DefaultType = Self;
}
pub const SAFER_LEVEL_OPEN: u32 = 1u32;
pub const SAFER_MAX_DESCRIPTION_SIZE: u32 = 256u32;
pub const SAFER_MAX_FRIENDLYNAME_SIZE: u32 = 256u32;
pub const SAFER_MAX_HASH_SIZE: u32 = 64u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SAFER_OBJECT_INFO_CLASS(pub i32);
pub const SaferObjectLevelId: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(1i32);
pub const SaferObjectScopeId: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(2i32);
pub const SaferObjectFriendlyName: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(3i32);
pub const SaferObjectDescription: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(4i32);
pub const SaferObjectBuiltin: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(5i32);
pub const SaferObjectDisallowed: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(6i32);
pub const SaferObjectDisableMaxPrivilege: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(7i32);
pub const SaferObjectInvertDeletedPrivileges: SAFER_OBJECT_INFO_CLASS =
    SAFER_OBJECT_INFO_CLASS(8i32);
pub const SaferObjectDeletedPrivileges: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(9i32);
pub const SaferObjectDefaultOwner: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(10i32);
pub const SaferObjectSidsToDisable: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(11i32);
pub const SaferObjectRestrictedSidsInverted: SAFER_OBJECT_INFO_CLASS =
    SAFER_OBJECT_INFO_CLASS(12i32);
pub const SaferObjectRestrictedSidsAdded: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(13i32);
pub const SaferObjectAllIdentificationGuids: SAFER_OBJECT_INFO_CLASS =
    SAFER_OBJECT_INFO_CLASS(14i32);
pub const SaferObjectSingleIdentification: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(15i32);
pub const SaferObjectExtendedError: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(16i32);
impl ::std::convert::From<i32> for SAFER_OBJECT_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SAFER_OBJECT_INFO_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_PATHNAME_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub Description: [u16; 256],
    pub ImageName: super::Foundation::PWSTR,
    pub dwSaferFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_PATHNAME_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SAFER_PATHNAME_IDENTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SAFER_PATHNAME_IDENTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAFER_PATHNAME_IDENTIFICATION")
            .field("header", &self.header)
            .field("Description", &self.Description)
            .field("ImageName", &self.ImageName)
            .field("dwSaferFlags", &self.dwSaferFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SAFER_PATHNAME_IDENTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.Description == other.Description
            && self.ImageName == other.ImageName
            && self.dwSaferFlags == other.dwSaferFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SAFER_PATHNAME_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SAFER_PATHNAME_IDENTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SAFER_POLICY_BLOCK_CLIENT_UI: u32 = 8192u32;
pub const SAFER_POLICY_HASH_DUPLICATE: u32 = 262144u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SAFER_POLICY_INFO_CLASS(pub i32);
pub const SaferPolicyLevelList: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(1i32);
pub const SaferPolicyEnableTransparentEnforcement: SAFER_POLICY_INFO_CLASS =
    SAFER_POLICY_INFO_CLASS(2i32);
pub const SaferPolicyDefaultLevel: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(3i32);
pub const SaferPolicyEvaluateUserScope: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(4i32);
pub const SaferPolicyScopeFlags: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(5i32);
pub const SaferPolicyDefaultLevelFlags: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(6i32);
pub const SaferPolicyAuthenticodeEnabled: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(7i32);
impl ::std::convert::From<i32> for SAFER_POLICY_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SAFER_POLICY_INFO_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SAFER_POLICY_JOBID_CONSTRAINED: u32 = 67108864u32;
pub const SAFER_POLICY_JOBID_MASK: u32 = 4278190080u32;
pub const SAFER_POLICY_JOBID_UNTRUSTED: u32 = 50331648u32;
pub const SAFER_POLICY_ONLY_AUDIT: u32 = 4096u32;
pub const SAFER_POLICY_ONLY_EXES: u32 = 65536u32;
pub const SAFER_POLICY_SANDBOX_INERT: u32 = 131072u32;
pub const SAFER_POLICY_UIFLAGS_HIDDEN: u32 = 4u32;
pub const SAFER_POLICY_UIFLAGS_INFORMATION_PROMPT: u32 = 1u32;
pub const SAFER_POLICY_UIFLAGS_MASK: u32 = 255u32;
pub const SAFER_POLICY_UIFLAGS_OPTION_PROMPT: u32 = 2u32;
pub const SAFER_SCOPEID_MACHINE: u32 = 1u32;
pub const SAFER_SCOPEID_USER: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_URLZONE_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub UrlZoneId: u32,
    pub dwSaferFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_URLZONE_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SAFER_URLZONE_IDENTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SAFER_URLZONE_IDENTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAFER_URLZONE_IDENTIFICATION")
            .field("header", &self.header)
            .field("UrlZoneId", &self.UrlZoneId)
            .field("dwSaferFlags", &self.dwSaferFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SAFER_URLZONE_IDENTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.UrlZoneId == other.UrlZoneId
            && self.dwSaferFlags == other.dwSaferFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SAFER_URLZONE_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SAFER_URLZONE_IDENTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCESTATUS_ACCESS_DENIED: i32 = 9i32;
pub const SCESTATUS_ALREADY_RUNNING: i32 = 13i32;
pub const SCESTATUS_BAD_FORMAT: i32 = 7i32;
pub const SCESTATUS_BUFFER_TOO_SMALL: i32 = 5i32;
pub const SCESTATUS_CANT_DELETE: i32 = 10i32;
pub const SCESTATUS_EXCEPTION_IN_SERVER: i32 = 16i32;
pub const SCESTATUS_INVALID_DATA: i32 = 3i32;
pub const SCESTATUS_INVALID_PARAMETER: i32 = 1i32;
pub const SCESTATUS_MOD_NOT_FOUND: i32 = 15i32;
pub const SCESTATUS_NOT_ENOUGH_RESOURCE: i32 = 8i32;
pub const SCESTATUS_NO_MAPPING: i32 = 18i32;
pub const SCESTATUS_NO_TEMPLATE_GIVEN: i32 = 17i32;
pub const SCESTATUS_OBJECT_EXIST: i32 = 4i32;
pub const SCESTATUS_OTHER_ERROR: i32 = 12i32;
pub const SCESTATUS_PREFIX_OVERFLOW: i32 = 11i32;
pub const SCESTATUS_PROFILE_NOT_FOUND: i32 = 6i32;
pub const SCESTATUS_RECORD_NOT_FOUND: i32 = 2i32;
pub const SCESTATUS_SERVICE_NOT_SUPPORT: i32 = 14i32;
pub const SCESTATUS_SUCCESS: i32 = 0i32;
pub const SCESTATUS_TRUST_FAIL: i32 = 19i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCESVC_ANALYSIS_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_ANALYSIS_LINE,
}
impl SCESVC_ANALYSIS_INFO {}
impl ::std::default::Default for SCESVC_ANALYSIS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCESVC_ANALYSIS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCESVC_ANALYSIS_INFO")
            .field("Count", &self.Count)
            .field("Lines", &self.Lines)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCESVC_ANALYSIS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Lines == other.Lines
    }
}
impl ::std::cmp::Eq for SCESVC_ANALYSIS_INFO {}
unsafe impl ::windows::runtime::Abi for SCESVC_ANALYSIS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCESVC_ANALYSIS_LINE {
    pub Key: *mut i8,
    pub Value: *mut u8,
    pub ValueLen: u32,
}
impl SCESVC_ANALYSIS_LINE {}
impl ::std::default::Default for SCESVC_ANALYSIS_LINE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCESVC_ANALYSIS_LINE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCESVC_ANALYSIS_LINE")
            .field("Key", &self.Key)
            .field("Value", &self.Value)
            .field("ValueLen", &self.ValueLen)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCESVC_ANALYSIS_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Value == other.Value && self.ValueLen == other.ValueLen
    }
}
impl ::std::cmp::Eq for SCESVC_ANALYSIS_LINE {}
unsafe impl ::windows::runtime::Abi for SCESVC_ANALYSIS_LINE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SCESVC_CALLBACK_INFO {
    pub sceHandle: *mut ::std::ffi::c_void,
    pub pfQueryInfo: ::std::option::Option<PFSCE_QUERY_INFO>,
    pub pfSetInfo: ::std::option::Option<PFSCE_SET_INFO>,
    pub pfFreeInfo: ::std::option::Option<PFSCE_FREE_INFO>,
    pub pfLogInfo: ::std::option::Option<PFSCE_LOG_INFO>,
}
#[cfg(feature = "Win32_Foundation")]
impl SCESVC_CALLBACK_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SCESVC_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SCESVC_CALLBACK_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCESVC_CALLBACK_INFO")
            .field("sceHandle", &self.sceHandle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SCESVC_CALLBACK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.sceHandle == other.sceHandle
            && self.pfQueryInfo.map(|f| f as usize) == other.pfQueryInfo.map(|f| f as usize)
            && self.pfSetInfo.map(|f| f as usize) == other.pfSetInfo.map(|f| f as usize)
            && self.pfFreeInfo.map(|f| f as usize) == other.pfFreeInfo.map(|f| f as usize)
            && self.pfLogInfo.map(|f| f as usize) == other.pfLogInfo.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SCESVC_CALLBACK_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SCESVC_CALLBACK_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCESVC_CONFIGURATION_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_CONFIGURATION_LINE,
}
impl SCESVC_CONFIGURATION_INFO {}
impl ::std::default::Default for SCESVC_CONFIGURATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCESVC_CONFIGURATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCESVC_CONFIGURATION_INFO")
            .field("Count", &self.Count)
            .field("Lines", &self.Lines)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCESVC_CONFIGURATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Lines == other.Lines
    }
}
impl ::std::cmp::Eq for SCESVC_CONFIGURATION_INFO {}
unsafe impl ::windows::runtime::Abi for SCESVC_CONFIGURATION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCESVC_CONFIGURATION_LINE {
    pub Key: *mut i8,
    pub Value: *mut i8,
    pub ValueLen: u32,
}
impl SCESVC_CONFIGURATION_LINE {}
impl ::std::default::Default for SCESVC_CONFIGURATION_LINE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCESVC_CONFIGURATION_LINE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCESVC_CONFIGURATION_LINE")
            .field("Key", &self.Key)
            .field("Value", &self.Value)
            .field("ValueLen", &self.ValueLen)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCESVC_CONFIGURATION_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Value == other.Value && self.ValueLen == other.ValueLen
    }
}
impl ::std::cmp::Eq for SCESVC_CONFIGURATION_LINE {}
unsafe impl ::windows::runtime::Abi for SCESVC_CONFIGURATION_LINE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCESVC_ENUMERATION_MAX: i32 = 100i32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SCESVC_INFO_TYPE(pub i32);
pub const SceSvcConfigurationInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(0i32);
pub const SceSvcMergedPolicyInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(1i32);
pub const SceSvcAnalysisInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(2i32);
pub const SceSvcInternalUse: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(3i32);
impl ::std::convert::From<i32> for SCESVC_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SCESVC_INFO_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SCE_LOG_ERR_LEVEL(pub u32);
pub const SCE_LOG_LEVEL_ALWAYS: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(0u32);
pub const SCE_LOG_LEVEL_ERROR: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(1u32);
pub const SCE_LOG_LEVEL_DETAIL: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(2u32);
pub const SCE_LOG_LEVEL_DEBUG: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(3u32);
impl ::std::convert::From<u32> for SCE_LOG_ERR_LEVEL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SCE_LOG_ERR_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SCE_LOG_ERR_LEVEL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SCE_LOG_ERR_LEVEL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SCE_LOG_ERR_LEVEL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SCE_LOG_ERR_LEVEL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SCE_LOG_ERR_LEVEL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct SEALING_SIGNATURE_ATTRIBUTE {
    pub version: u32,
    pub signerIndex: u32,
    pub signatureAlgorithm: Cryptography::Core::CRYPT_ALGORITHM_IDENTIFIER,
    pub encryptedDigest: Cryptography::Core::CRYPTOAPI_BLOB,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SEALING_SIGNATURE_ATTRIBUTE {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SEALING_SIGNATURE_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for SEALING_SIGNATURE_ATTRIBUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SEALING_SIGNATURE_ATTRIBUTE")
            .field("version", &self.version)
            .field("signerIndex", &self.signerIndex)
            .field("signatureAlgorithm", &self.signatureAlgorithm)
            .field("encryptedDigest", &self.encryptedDigest)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SEALING_SIGNATURE_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.signerIndex == other.signerIndex
            && self.signatureAlgorithm == other.signatureAlgorithm
            && self.encryptedDigest == other.encryptedDigest
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SEALING_SIGNATURE_ATTRIBUTE {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SEALING_SIGNATURE_ATTRIBUTE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography_Core")]
pub struct SEALING_TIMESTAMP_ATTRIBUTE {
    pub version: u32,
    pub signerIndex: u32,
    pub sealTimeStampToken: Cryptography::Core::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
impl SEALING_TIMESTAMP_ATTRIBUTE {}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
impl ::std::default::Default for SEALING_TIMESTAMP_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
impl ::std::fmt::Debug for SEALING_TIMESTAMP_ATTRIBUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SEALING_TIMESTAMP_ATTRIBUTE")
            .field("version", &self.version)
            .field("signerIndex", &self.signerIndex)
            .field("sealTimeStampToken", &self.sealTimeStampToken)
            .finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
impl ::std::cmp::PartialEq for SEALING_TIMESTAMP_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.signerIndex == other.signerIndex
            && self.sealTimeStampToken == other.sealTimeStampToken
    }
}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
impl ::std::cmp::Eq for SEALING_TIMESTAMP_ATTRIBUTE {}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
unsafe impl ::windows::runtime::Abi for SEALING_TIMESTAMP_ATTRIBUTE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
        fmt.debug_struct("SECURITY_ATTRIBUTES")
            .field("nLength", &self.nLength)
            .field("lpSecurityDescriptor", &self.lpSecurityDescriptor)
            .field("bInheritHandle", &self.bInheritHandle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.nLength == other.nLength
            && self.lpSecurityDescriptor == other.lpSecurityDescriptor
            && self.bInheritHandle == other.bInheritHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SECURITY_ATTRIBUTES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SECURITY_AUTO_INHERIT_FLAGS(pub u32);
pub const SEF_AVOID_OWNER_CHECK: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(16u32);
pub const SEF_AVOID_OWNER_RESTRICTION: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(4096u32);
pub const SEF_AVOID_PRIVILEGE_CHECK: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(8u32);
pub const SEF_DACL_AUTO_INHERIT: SECURITY_AUTO_INHERIT_FLAGS = SECURITY_AUTO_INHERIT_FLAGS(1u32);
pub const SEF_DEFAULT_DESCRIPTOR_FOR_OBJECT: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(4u32);
pub const SEF_DEFAULT_GROUP_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(64u32);
pub const SEF_DEFAULT_OWNER_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(32u32);
pub const SEF_MACL_NO_EXECUTE_UP: SECURITY_AUTO_INHERIT_FLAGS =
    SECURITY_AUTO_INHERIT_FLAGS(1024u32);
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
    type DefaultType = Self;
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
        fmt.debug_struct("SECURITY_CAPABILITIES")
            .field("AppContainerSid", &self.AppContainerSid)
            .field("Capabilities", &self.Capabilities)
            .field("CapabilityCount", &self.CapabilityCount)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SECURITY_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.AppContainerSid == other.AppContainerSid
            && self.Capabilities == other.Capabilities
            && self.CapabilityCount == other.CapabilityCount
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SECURITY_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SECURITY_CAPABILITIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
        fmt.debug_struct("SECURITY_DESCRIPTOR")
            .field("Revision", &self.Revision)
            .field("Sbz1", &self.Sbz1)
            .field("Control", &self.Control)
            .field("Owner", &self.Owner)
            .field("Group", &self.Group)
            .field("Sacl", &self.Sacl)
            .field("Dacl", &self.Dacl)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SECURITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision
            && self.Sbz1 == other.Sbz1
            && self.Control == other.Control
            && self.Owner == other.Owner
            && self.Group == other.Group
            && self.Sacl == other.Sacl
            && self.Dacl == other.Dacl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SECURITY_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SECURITY_DESCRIPTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
        fmt.debug_struct("SECURITY_QUALITY_OF_SERVICE")
            .field("Length", &self.Length)
            .field("ImpersonationLevel", &self.ImpersonationLevel)
            .field("ContextTrackingMode", &self.ContextTrackingMode)
            .field("EffectiveOnly", &self.EffectiveOnly)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SECURITY_QUALITY_OF_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.ImpersonationLevel == other.ImpersonationLevel
            && self.ContextTrackingMode == other.ContextTrackingMode
            && self.EffectiveOnly == other.EffectiveOnly
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SECURITY_QUALITY_OF_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SECURITY_QUALITY_OF_SERVICE {
    type Abi = Self;
    type DefaultType = Self;
}
pub type SEC_THREAD_START =
    unsafe extern "system" fn(lpthreadparameter: *mut ::std::ffi::c_void) -> u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SID")
            .field("Revision", &self.Revision)
            .field("SubAuthorityCount", &self.SubAuthorityCount)
            .field("IdentifierAuthority", &self.IdentifierAuthority)
            .field("SubAuthority", &self.SubAuthority)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SID {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision
            && self.SubAuthorityCount == other.SubAuthorityCount
            && self.IdentifierAuthority == other.IdentifierAuthority
            && self.SubAuthority == other.SubAuthority
    }
}
impl ::std::cmp::Eq for SID {}
unsafe impl ::windows::runtime::Abi for SID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
        fmt.debug_struct("SID_AND_ATTRIBUTES")
            .field("Sid", &self.Sid)
            .field("Attributes", &self.Attributes)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
        fmt.debug_struct("SID_AND_ATTRIBUTES_HASH")
            .field("SidCount", &self.SidCount)
            .field("SidAttr", &self.SidAttr)
            .field("Hash", &self.Hash)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SID_IDENTIFIER_AUTHORITY")
            .field("Value", &self.Value)
            .finish()
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
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SIP_ADD_NEWPROVIDER {
    pub cbStruct: u32,
    pub pgSubject: *mut ::windows::runtime::GUID,
    pub pwszDLLFileName: super::Foundation::PWSTR,
    pub pwszMagicNumber: super::Foundation::PWSTR,
    pub pwszIsFunctionName: super::Foundation::PWSTR,
    pub pwszGetFuncName: super::Foundation::PWSTR,
    pub pwszPutFuncName: super::Foundation::PWSTR,
    pub pwszCreateFuncName: super::Foundation::PWSTR,
    pub pwszVerifyFuncName: super::Foundation::PWSTR,
    pub pwszRemoveFuncName: super::Foundation::PWSTR,
    pub pwszIsFunctionNameFmt2: super::Foundation::PWSTR,
    pub pwszGetCapFuncName: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SIP_ADD_NEWPROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SIP_ADD_NEWPROVIDER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SIP_ADD_NEWPROVIDER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SIP_ADD_NEWPROVIDER")
            .field("cbStruct", &self.cbStruct)
            .field("pgSubject", &self.pgSubject)
            .field("pwszDLLFileName", &self.pwszDLLFileName)
            .field("pwszMagicNumber", &self.pwszMagicNumber)
            .field("pwszIsFunctionName", &self.pwszIsFunctionName)
            .field("pwszGetFuncName", &self.pwszGetFuncName)
            .field("pwszPutFuncName", &self.pwszPutFuncName)
            .field("pwszCreateFuncName", &self.pwszCreateFuncName)
            .field("pwszVerifyFuncName", &self.pwszVerifyFuncName)
            .field("pwszRemoveFuncName", &self.pwszRemoveFuncName)
            .field("pwszIsFunctionNameFmt2", &self.pwszIsFunctionNameFmt2)
            .field("pwszGetCapFuncName", &self.pwszGetCapFuncName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SIP_ADD_NEWPROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pgSubject == other.pgSubject
            && self.pwszDLLFileName == other.pwszDLLFileName
            && self.pwszMagicNumber == other.pwszMagicNumber
            && self.pwszIsFunctionName == other.pwszIsFunctionName
            && self.pwszGetFuncName == other.pwszGetFuncName
            && self.pwszPutFuncName == other.pwszPutFuncName
            && self.pwszCreateFuncName == other.pwszCreateFuncName
            && self.pwszVerifyFuncName == other.pwszVerifyFuncName
            && self.pwszRemoveFuncName == other.pwszRemoveFuncName
            && self.pwszIsFunctionNameFmt2 == other.pwszIsFunctionNameFmt2
            && self.pwszGetCapFuncName == other.pwszGetCapFuncName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SIP_ADD_NEWPROVIDER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SIP_ADD_NEWPROVIDER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SIP_CAP_FLAG_SEALING: u32 = 1u32;
pub const SIP_CAP_SET_CUR_VER: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SIP_CAP_SET_V2 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: super::Foundation::BOOL,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SIP_CAP_SET_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SIP_CAP_SET_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SIP_CAP_SET_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SIP_CAP_SET_V2")
            .field("cbSize", &self.cbSize)
            .field("dwVersion", &self.dwVersion)
            .field("isMultiSign", &self.isMultiSign)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SIP_CAP_SET_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwVersion == other.dwVersion
            && self.isMultiSign == other.isMultiSign
            && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SIP_CAP_SET_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SIP_CAP_SET_V2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SIP_CAP_SET_V3 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: super::Foundation::BOOL,
    pub Anonymous: SIP_CAP_SET_V3_0,
}
#[cfg(feature = "Win32_Foundation")]
impl SIP_CAP_SET_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SIP_CAP_SET_V3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SIP_CAP_SET_V3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SIP_CAP_SET_V3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SIP_CAP_SET_V3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union SIP_CAP_SET_V3_0 {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl SIP_CAP_SET_V3_0 {}
impl ::std::default::Default for SIP_CAP_SET_V3_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SIP_CAP_SET_V3_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SIP_CAP_SET_V3_0 {}
unsafe impl ::windows::runtime::Abi for SIP_CAP_SET_V3_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SIP_CAP_SET_VERSION_2: u32 = 2u32;
pub const SIP_CAP_SET_VERSION_3: u32 = 3u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct SIP_DISPATCH_INFO {
    pub cbSize: u32,
    pub hSIP: super::Foundation::HANDLE,
    pub pfGet: ::std::option::Option<pCryptSIPGetSignedDataMsg>,
    pub pfPut: ::std::option::Option<pCryptSIPPutSignedDataMsg>,
    pub pfCreate: ::std::option::Option<pCryptSIPCreateIndirectData>,
    pub pfVerify: ::std::option::Option<pCryptSIPVerifyIndirectData>,
    pub pfRemove: ::std::option::Option<pCryptSIPRemoveSignedDataMsg>,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SIP_DISPATCH_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SIP_DISPATCH_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for SIP_DISPATCH_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SIP_DISPATCH_INFO")
            .field("cbSize", &self.cbSize)
            .field("hSIP", &self.hSIP)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SIP_DISPATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.hSIP == other.hSIP
            && self.pfGet.map(|f| f as usize) == other.pfGet.map(|f| f as usize)
            && self.pfPut.map(|f| f as usize) == other.pfPut.map(|f| f as usize)
            && self.pfCreate.map(|f| f as usize) == other.pfCreate.map(|f| f as usize)
            && self.pfVerify.map(|f| f as usize) == other.pfVerify.map(|f| f as usize)
            && self.pfRemove.map(|f| f as usize) == other.pfRemove.map(|f| f as usize)
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SIP_DISPATCH_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SIP_DISPATCH_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct SIP_INDIRECT_DATA {
    pub Data: Cryptography::Core::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: Cryptography::Core::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: Cryptography::Core::CRYPTOAPI_BLOB,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SIP_INDIRECT_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SIP_INDIRECT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for SIP_INDIRECT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SIP_INDIRECT_DATA")
            .field("Data", &self.Data)
            .field("DigestAlgorithm", &self.DigestAlgorithm)
            .field("Digest", &self.Digest)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SIP_INDIRECT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
            && self.DigestAlgorithm == other.DigestAlgorithm
            && self.Digest == other.Digest
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SIP_INDIRECT_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SIP_INDIRECT_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SIP_MAX_MAGIC_NUMBER: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct SIP_SUBJECTINFO {
    pub cbSize: u32,
    pub pgSubjectType: *mut ::windows::runtime::GUID,
    pub hFile: super::Foundation::HANDLE,
    pub pwsFileName: super::Foundation::PWSTR,
    pub pwsDisplayName: super::Foundation::PWSTR,
    pub dwReserved1: u32,
    pub dwIntVersion: u32,
    pub hProv: usize,
    pub DigestAlgorithm: Cryptography::Core::CRYPT_ALGORITHM_IDENTIFIER,
    pub dwFlags: u32,
    pub dwEncodingType: u32,
    pub dwReserved2: u32,
    pub fdwCAPISettings: u32,
    pub fdwSecuritySettings: u32,
    pub dwIndex: u32,
    pub dwUnionChoice: u32,
    pub Anonymous: SIP_SUBJECTINFO_0,
    pub pClientData: *mut ::std::ffi::c_void,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SIP_SUBJECTINFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SIP_SUBJECTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SIP_SUBJECTINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SIP_SUBJECTINFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SIP_SUBJECTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub union SIP_SUBJECTINFO_0 {
    pub psFlat: *mut MS_ADDINFO_FLAT,
    pub psCatMember: *mut MS_ADDINFO_CATALOGMEMBER,
    pub psBlob: *mut MS_ADDINFO_BLOB,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SIP_SUBJECTINFO_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SIP_SUBJECTINFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SIP_SUBJECTINFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SIP_SUBJECTINFO_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SIP_SUBJECTINFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SPC_DIGEST_GENERATE_FLAG: u32 = 512u32;
pub const SPC_DIGEST_SIGN_EX_FLAG: u32 = 16384u32;
pub const SPC_DIGEST_SIGN_FLAG: u32 = 1024u32;
pub const SPC_EXC_PE_PAGE_HASHES_FLAG: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPC_FINANCIAL_CRITERIA {
    pub fFinancialInfoAvailable: super::Foundation::BOOL,
    pub fMeetsCriteria: super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl SPC_FINANCIAL_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SPC_FINANCIAL_CRITERIA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SPC_FINANCIAL_CRITERIA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_FINANCIAL_CRITERIA")
            .field("fFinancialInfoAvailable", &self.fFinancialInfoAvailable)
            .field("fMeetsCriteria", &self.fMeetsCriteria)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SPC_FINANCIAL_CRITERIA {
    fn eq(&self, other: &Self) -> bool {
        self.fFinancialInfoAvailable == other.fFinancialInfoAvailable
            && self.fMeetsCriteria == other.fMeetsCriteria
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SPC_FINANCIAL_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SPC_FINANCIAL_CRITERIA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct SPC_IMAGE {
    pub pImageLink: *mut SPC_LINK,
    pub Bitmap: Cryptography::Core::CRYPTOAPI_BLOB,
    pub Metafile: Cryptography::Core::CRYPTOAPI_BLOB,
    pub EnhancedMetafile: Cryptography::Core::CRYPTOAPI_BLOB,
    pub GifFile: Cryptography::Core::CRYPTOAPI_BLOB,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SPC_IMAGE {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SPC_IMAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for SPC_IMAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_IMAGE")
            .field("pImageLink", &self.pImageLink)
            .field("Bitmap", &self.Bitmap)
            .field("Metafile", &self.Metafile)
            .field("EnhancedMetafile", &self.EnhancedMetafile)
            .field("GifFile", &self.GifFile)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SPC_IMAGE {
    fn eq(&self, other: &Self) -> bool {
        self.pImageLink == other.pImageLink
            && self.Bitmap == other.Bitmap
            && self.Metafile == other.Metafile
            && self.EnhancedMetafile == other.EnhancedMetafile
            && self.GifFile == other.GifFile
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SPC_IMAGE {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SPC_IMAGE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SPC_INC_PE_DEBUG_INFO_FLAG: u32 = 64u32;
pub const SPC_INC_PE_IMPORT_ADDR_TABLE_FLAG: u32 = 32u32;
pub const SPC_INC_PE_PAGE_HASHES_FLAG: u32 = 256u32;
pub const SPC_INC_PE_RESOURCES_FLAG: u32 = 128u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct SPC_INDIRECT_DATA_CONTENT {
    pub Data: Cryptography::Core::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: Cryptography::Core::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: Cryptography::Core::CRYPTOAPI_BLOB,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SPC_INDIRECT_DATA_CONTENT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SPC_INDIRECT_DATA_CONTENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for SPC_INDIRECT_DATA_CONTENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_INDIRECT_DATA_CONTENT")
            .field("Data", &self.Data)
            .field("DigestAlgorithm", &self.DigestAlgorithm)
            .field("Digest", &self.Digest)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SPC_INDIRECT_DATA_CONTENT {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
            && self.DigestAlgorithm == other.DigestAlgorithm
            && self.Digest == other.Digest
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SPC_INDIRECT_DATA_CONTENT {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SPC_INDIRECT_DATA_CONTENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct SPC_LINK {
    pub dwLinkChoice: u32,
    pub Anonymous: SPC_LINK_0,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SPC_LINK {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SPC_LINK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SPC_LINK {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SPC_LINK {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SPC_LINK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub union SPC_LINK_0 {
    pub pwszUrl: super::Foundation::PWSTR,
    pub Moniker: SPC_SERIALIZED_OBJECT,
    pub pwszFile: super::Foundation::PWSTR,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SPC_LINK_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SPC_LINK_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SPC_LINK_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SPC_LINK_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SPC_LINK_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SPC_MARKER_CHECK_CURRENTLY_SUPPORTED_FLAGS: u32 = 1u32;
pub const SPC_MARKER_CHECK_SKIP_SIP_INDIRECT_DATA_FLAG: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct SPC_PE_IMAGE_DATA {
    pub Flags: Cryptography::Core::CRYPT_BIT_BLOB,
    pub pFile: *mut SPC_LINK,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SPC_PE_IMAGE_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SPC_PE_IMAGE_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for SPC_PE_IMAGE_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_PE_IMAGE_DATA")
            .field("Flags", &self.Flags)
            .field("pFile", &self.pFile)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SPC_PE_IMAGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pFile == other.pFile
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SPC_PE_IMAGE_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SPC_PE_IMAGE_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SPC_RELAXED_PE_MARKER_CHECK: u32 = 2048u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography_Core")]
pub struct SPC_SERIALIZED_OBJECT {
    pub ClassId: [u8; 16],
    pub SerializedData: Cryptography::Core::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
impl SPC_SERIALIZED_OBJECT {}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
impl ::std::default::Default for SPC_SERIALIZED_OBJECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
impl ::std::fmt::Debug for SPC_SERIALIZED_OBJECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_SERIALIZED_OBJECT")
            .field("ClassId", &self.ClassId)
            .field("SerializedData", &self.SerializedData)
            .finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
impl ::std::cmp::PartialEq for SPC_SERIALIZED_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.ClassId == other.ClassId && self.SerializedData == other.SerializedData
    }
}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
impl ::std::cmp::Eq for SPC_SERIALIZED_OBJECT {}
#[cfg(feature = "Win32_Security_Cryptography_Core")]
unsafe impl ::windows::runtime::Abi for SPC_SERIALIZED_OBJECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SPC_SIGINFO {
    pub dwSipVersion: u32,
    pub gSIPGuid: ::windows::runtime::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwReserved4: u32,
    pub dwReserved5: u32,
}
impl SPC_SIGINFO {}
impl ::std::default::Default for SPC_SIGINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SPC_SIGINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_SIGINFO")
            .field("dwSipVersion", &self.dwSipVersion)
            .field("gSIPGuid", &self.gSIPGuid)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("dwReserved4", &self.dwReserved4)
            .field("dwReserved5", &self.dwReserved5)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SPC_SIGINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSipVersion == other.dwSipVersion
            && self.gSIPGuid == other.gSIPGuid
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.dwReserved4 == other.dwReserved4
            && self.dwReserved5 == other.dwReserved5
    }
}
impl ::std::cmp::Eq for SPC_SIGINFO {}
unsafe impl ::windows::runtime::Abi for SPC_SIGINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct SPC_SP_AGENCY_INFO {
    pub pPolicyInformation: *mut SPC_LINK,
    pub pwszPolicyDisplayText: super::Foundation::PWSTR,
    pub pLogoImage: *mut SPC_IMAGE,
    pub pLogoLink: *mut SPC_LINK,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SPC_SP_AGENCY_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SPC_SP_AGENCY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for SPC_SP_AGENCY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_SP_AGENCY_INFO")
            .field("pPolicyInformation", &self.pPolicyInformation)
            .field("pwszPolicyDisplayText", &self.pwszPolicyDisplayText)
            .field("pLogoImage", &self.pLogoImage)
            .field("pLogoLink", &self.pLogoLink)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SPC_SP_AGENCY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pPolicyInformation == other.pPolicyInformation
            && self.pwszPolicyDisplayText == other.pwszPolicyDisplayText
            && self.pLogoImage == other.pLogoImage
            && self.pLogoLink == other.pLogoLink
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SPC_SP_AGENCY_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SPC_SP_AGENCY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct SPC_SP_OPUS_INFO {
    pub pwszProgramName: super::Foundation::PWSTR,
    pub pMoreInfo: *mut SPC_LINK,
    pub pPublisherInfo: *mut SPC_LINK,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl SPC_SP_OPUS_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for SPC_SP_OPUS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for SPC_SP_OPUS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_SP_OPUS_INFO")
            .field("pwszProgramName", &self.pwszProgramName)
            .field("pMoreInfo", &self.pMoreInfo)
            .field("pPublisherInfo", &self.pPublisherInfo)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for SPC_SP_OPUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszProgramName == other.pwszProgramName
            && self.pMoreInfo == other.pMoreInfo
            && self.pPublisherInfo == other.pPublisherInfo
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for SPC_SP_OPUS_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for SPC_SP_OPUS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPC_STATEMENT_TYPE {
    pub cKeyPurposeId: u32,
    pub rgpszKeyPurposeId: *mut super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SPC_STATEMENT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SPC_STATEMENT_TYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SPC_STATEMENT_TYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SPC_STATEMENT_TYPE")
            .field("cKeyPurposeId", &self.cKeyPurposeId)
            .field("rgpszKeyPurposeId", &self.rgpszKeyPurposeId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SPC_STATEMENT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.cKeyPurposeId == other.cKeyPurposeId
            && self.rgpszKeyPurposeId == other.rgpszKeyPurposeId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SPC_STATEMENT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SPC_STATEMENT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SPC_UUID_LENGTH: u32 = 16u32;
pub const STATUSMSG_OPTION_NOANIMATION: u32 = 1u32;
pub const STATUSMSG_OPTION_SETFOREGROUND: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SYSTEM_ALARM_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SYSTEM_ALARM_CALLBACK_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SYSTEM_ALARM_CALLBACK_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SYSTEM_ALARM_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_ALARM_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_ALARM_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_ALARM_OBJECT_ACE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SYSTEM_AUDIT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SYSTEM_AUDIT_CALLBACK_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SYSTEM_AUDIT_CALLBACK_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SYSTEM_AUDIT_OBJECT_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("Flags", &self.Flags)
            .field("ObjectType", &self.ObjectType)
            .field("InheritedObjectType", &self.InheritedObjectType)
            .field("SidStart", &self.SidStart)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_AUDIT_OBJECT_ACE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.Mask == other.Mask
            && self.Flags == other.Flags
            && self.ObjectType == other.ObjectType
            && self.InheritedObjectType == other.InheritedObjectType
            && self.SidStart == other.SidStart
    }
}
impl ::std::cmp::Eq for SYSTEM_AUDIT_OBJECT_ACE {}
unsafe impl ::windows::runtime::Abi for SYSTEM_AUDIT_OBJECT_ACE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SYSTEM_AUDIT_OBJECT_ACE_FLAGS(pub u32);
pub const ACE_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS =
    SYSTEM_AUDIT_OBJECT_ACE_FLAGS(1u32);
pub const ACE_INHERITED_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS =
    SYSTEM_AUDIT_OBJECT_ACE_FLAGS(2u32);
impl ::std::convert::From<u32> for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYSTEM_AUDIT_OBJECT_ACE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
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
        fmt.debug_struct("SYSTEM_MANDATORY_LABEL_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SYSTEM_RESOURCE_ATTRIBUTE_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("SYSTEM_SCOPED_POLICY_ID_ACE")
            .field("Header", &self.Header)
            .field("Mask", &self.Mask)
            .field("SidStart", &self.SidStart)
            .finish()
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
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SaferCloseLevel<'a, Param0: ::windows::runtime::IntoParam<'a, SAFER_LEVEL_HANDLE>>(
    hlevelhandle: Param0,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferCloseLevel(hlevelhandle: SAFER_LEVEL_HANDLE) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SaferCloseLevel(hlevelhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SaferComputeTokenFromLevel<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, SAFER_LEVEL_HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    levelhandle: Param0,
    inaccesstoken: Param1,
    outaccesstoken: *mut super::Foundation::HANDLE,
    dwflags: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS,
    lpreserved: *mut ::std::ffi::c_void,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferComputeTokenFromLevel(
                levelhandle: SAFER_LEVEL_HANDLE,
                inaccesstoken: super::Foundation::HANDLE,
                outaccesstoken: *mut super::Foundation::HANDLE,
                dwflags: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS,
                lpreserved: *mut ::std::ffi::c_void,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SaferComputeTokenFromLevel(
            levelhandle.into_param().abi(),
            inaccesstoken.into_param().abi(),
            ::std::mem::transmute(outaccesstoken),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SaferCreateLevel(
    dwscopeid: u32,
    dwlevelid: u32,
    openflags: u32,
    plevelhandle: *mut SAFER_LEVEL_HANDLE,
    lpreserved: *mut ::std::ffi::c_void,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferCreateLevel(
                dwscopeid: u32,
                dwlevelid: u32,
                openflags: u32,
                plevelhandle: *mut SAFER_LEVEL_HANDLE,
                lpreserved: *mut ::std::ffi::c_void,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SaferCreateLevel(
            ::std::mem::transmute(dwscopeid),
            ::std::mem::transmute(dwlevelid),
            ::std::mem::transmute(openflags),
            ::std::mem::transmute(plevelhandle),
            ::std::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SaferGetLevelInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, SAFER_LEVEL_HANDLE>,
>(
    levelhandle: Param0,
    dwinfotype: SAFER_OBJECT_INFO_CLASS,
    lpquerybuffer: *mut ::std::ffi::c_void,
    dwinbuffersize: u32,
    lpdwoutbuffersize: *mut u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferGetLevelInformation(
                levelhandle: SAFER_LEVEL_HANDLE,
                dwinfotype: SAFER_OBJECT_INFO_CLASS,
                lpquerybuffer: *mut ::std::ffi::c_void,
                dwinbuffersize: u32,
                lpdwoutbuffersize: *mut u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SaferGetLevelInformation(
            levelhandle.into_param().abi(),
            ::std::mem::transmute(dwinfotype),
            ::std::mem::transmute(lpquerybuffer),
            ::std::mem::transmute(dwinbuffersize),
            ::std::mem::transmute(lpdwoutbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SaferGetPolicyInformation(
    dwscopeid: u32,
    saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS,
    infobuffersize: u32,
    infobuffer: *mut ::std::ffi::c_void,
    infobufferretsize: *mut u32,
    lpreserved: *mut ::std::ffi::c_void,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferGetPolicyInformation(
                dwscopeid: u32,
                saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS,
                infobuffersize: u32,
                infobuffer: *mut ::std::ffi::c_void,
                infobufferretsize: *mut u32,
                lpreserved: *mut ::std::ffi::c_void,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SaferGetPolicyInformation(
            ::std::mem::transmute(dwscopeid),
            ::std::mem::transmute(saferpolicyinfoclass),
            ::std::mem::transmute(infobuffersize),
            ::std::mem::transmute(infobuffer),
            ::std::mem::transmute(infobufferretsize),
            ::std::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SaferIdentifyLevel(
    dwnumproperties: u32,
    pcodeproperties: *const SAFER_CODE_PROPERTIES_V2,
    plevelhandle: *mut SAFER_LEVEL_HANDLE,
    lpreserved: *const ::std::ffi::c_void,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferIdentifyLevel(
                dwnumproperties: u32,
                pcodeproperties: *const SAFER_CODE_PROPERTIES_V2,
                plevelhandle: *mut SAFER_LEVEL_HANDLE,
                lpreserved: *const ::std::ffi::c_void,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SaferIdentifyLevel(
            ::std::mem::transmute(dwnumproperties),
            ::std::mem::transmute(pcodeproperties),
            ::std::mem::transmute(plevelhandle),
            ::std::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SaferRecordEventLogEntry<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, SAFER_LEVEL_HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    hlevel: Param0,
    sztargetpath: Param1,
    lpreserved: *mut ::std::ffi::c_void,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferRecordEventLogEntry(
                hlevel: SAFER_LEVEL_HANDLE,
                sztargetpath: super::Foundation::PWSTR,
                lpreserved: *mut ::std::ffi::c_void,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SaferRecordEventLogEntry(
            hlevel.into_param().abi(),
            sztargetpath.into_param().abi(),
            ::std::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SaferSetLevelInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, SAFER_LEVEL_HANDLE>,
>(
    levelhandle: Param0,
    dwinfotype: SAFER_OBJECT_INFO_CLASS,
    lpquerybuffer: *const ::std::ffi::c_void,
    dwinbuffersize: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferSetLevelInformation(
                levelhandle: SAFER_LEVEL_HANDLE,
                dwinfotype: SAFER_OBJECT_INFO_CLASS,
                lpquerybuffer: *const ::std::ffi::c_void,
                dwinbuffersize: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SaferSetLevelInformation(
            levelhandle.into_param().abi(),
            ::std::mem::transmute(dwinfotype),
            ::std::mem::transmute(lpquerybuffer),
            ::std::mem::transmute(dwinbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SaferSetPolicyInformation(
    dwscopeid: u32,
    saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS,
    infobuffersize: u32,
    infobuffer: *const ::std::ffi::c_void,
    lpreserved: *mut ::std::ffi::c_void,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferSetPolicyInformation(
                dwscopeid: u32,
                saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS,
                infobuffersize: u32,
                infobuffer: *const ::std::ffi::c_void,
                lpreserved: *mut ::std::ffi::c_void,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SaferSetPolicyInformation(
            ::std::mem::transmute(dwscopeid),
            ::std::mem::transmute(saferpolicyinfoclass),
            ::std::mem::transmute(infobuffersize),
            ::std::mem::transmute(infobuffer),
            ::std::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SaferiIsExecutableFileType<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BOOLEAN>,
>(
    szfullpathname: Param0,
    bfromshellexecute: Param1,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferiIsExecutableFileType(
                szfullpathname: super::Foundation::PWSTR,
                bfromshellexecute: super::Foundation::BOOLEAN,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SaferiIsExecutableFileType(
            szfullpathname.into_param().abi(),
            bfromshellexecute.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetAclInformation(
    pacl: *mut ACL,
    paclinformation: *const ::std::ffi::c_void,
    naclinformationlength: u32,
    dwaclinformationclass: ACL_INFORMATION_CLASS,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetAclInformation(
                pacl: *mut ACL,
                paclinformation: *const ::std::ffi::c_void,
                naclinformationlength: u32,
                dwaclinformationclass: ACL_INFORMATION_CLASS,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetAclInformation(
            ::std::mem::transmute(pacl),
            ::std::mem::transmute(paclinformation),
            ::std::mem::transmute(naclinformationlength),
            ::std::mem::transmute(dwaclinformationclass),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetCachedSigningLevel<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    sourcefiles: *const super::Foundation::HANDLE,
    sourcefilecount: u32,
    flags: u32,
    targetfile: Param3,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCachedSigningLevel(
                sourcefiles: *const super::Foundation::HANDLE,
                sourcefilecount: u32,
                flags: u32,
                targetfile: super::Foundation::HANDLE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetCachedSigningLevel(
            ::std::mem::transmute(sourcefiles),
            ::std::mem::transmute(sourcefilecount),
            ::std::mem::transmute(flags),
            targetfile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetFileSecurityA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
>(
    lpfilename: Param0,
    securityinformation: u32,
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileSecurityA(
                lpfilename: super::Foundation::PSTR,
                securityinformation: u32,
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileSecurityA(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(securityinformation),
            ::std::mem::transmute(psecuritydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetFileSecurityW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    lpfilename: Param0,
    securityinformation: u32,
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFileSecurityW(
                lpfilename: super::Foundation::PWSTR,
                securityinformation: u32,
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFileSecurityW(
            lpfilename.into_param().abi(),
            ::std::mem::transmute(securityinformation),
            ::std::mem::transmute(psecuritydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetKernelObjectSecurity<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    handle: Param0,
    securityinformation: u32,
    securitydescriptor: *const SECURITY_DESCRIPTOR,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetKernelObjectSecurity(
                handle: super::Foundation::HANDLE,
                securityinformation: u32,
                securitydescriptor: *const SECURITY_DESCRIPTOR,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetKernelObjectSecurity(
            handle.into_param().abi(),
            ::std::mem::transmute(securityinformation),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetPrivateObjectSecurity<
    'a,
    Param4: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    securityinformation: u32,
    modificationdescriptor: *const SECURITY_DESCRIPTOR,
    objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR,
    genericmapping: *const GENERIC_MAPPING,
    token: Param4,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPrivateObjectSecurity(
                securityinformation: u32,
                modificationdescriptor: *const SECURITY_DESCRIPTOR,
                objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR,
                genericmapping: *const GENERIC_MAPPING,
                token: super::Foundation::HANDLE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetPrivateObjectSecurity(
            ::std::mem::transmute(securityinformation),
            ::std::mem::transmute(modificationdescriptor),
            ::std::mem::transmute(objectssecuritydescriptor),
            ::std::mem::transmute(genericmapping),
            token.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetPrivateObjectSecurityEx<
    'a,
    Param5: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    securityinformation: u32,
    modificationdescriptor: *const SECURITY_DESCRIPTOR,
    objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR,
    autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS,
    genericmapping: *const GENERIC_MAPPING,
    token: Param5,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPrivateObjectSecurityEx(
                securityinformation: u32,
                modificationdescriptor: *const SECURITY_DESCRIPTOR,
                objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR,
                autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS,
                genericmapping: *const GENERIC_MAPPING,
                token: super::Foundation::HANDLE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetPrivateObjectSecurityEx(
            ::std::mem::transmute(securityinformation),
            ::std::mem::transmute(modificationdescriptor),
            ::std::mem::transmute(objectssecuritydescriptor),
            ::std::mem::transmute(autoinheritflags),
            ::std::mem::transmute(genericmapping),
            token.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SetSecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32);
        }
        ::std::mem::transmute(SetSecurityAccessMask(
            ::std::mem::transmute(securityinformation),
            ::std::mem::transmute(desiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetSecurityDescriptorControl(
    psecuritydescriptor: *const SECURITY_DESCRIPTOR,
    controlbitsofinterest: u16,
    controlbitstoset: u16,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorControl(
                psecuritydescriptor: *const SECURITY_DESCRIPTOR,
                controlbitsofinterest: u16,
                controlbitstoset: u16,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSecurityDescriptorControl(
            ::std::mem::transmute(psecuritydescriptor),
            ::std::mem::transmute(controlbitsofinterest),
            ::std::mem::transmute(controlbitstoset),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetSecurityDescriptorDacl<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
    bdaclpresent: Param1,
    pdacl: *const ACL,
    bdacldefaulted: Param3,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorDacl(
                psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
                bdaclpresent: super::Foundation::BOOL,
                pdacl: *const ACL,
                bdacldefaulted: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSecurityDescriptorDacl(
            ::std::mem::transmute(psecuritydescriptor),
            bdaclpresent.into_param().abi(),
            ::std::mem::transmute(pdacl),
            bdacldefaulted.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetSecurityDescriptorGroup<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
    pgroup: Param1,
    bgroupdefaulted: Param2,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorGroup(
                psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
                pgroup: super::Foundation::PSID,
                bgroupdefaulted: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSecurityDescriptorGroup(
            ::std::mem::transmute(psecuritydescriptor),
            pgroup.into_param().abi(),
            bgroupdefaulted.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetSecurityDescriptorOwner<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSID>,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
    powner: Param1,
    bownerdefaulted: Param2,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorOwner(
                psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
                powner: super::Foundation::PSID,
                bownerdefaulted: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSecurityDescriptorOwner(
            ::std::mem::transmute(psecuritydescriptor),
            powner.into_param().abi(),
            bownerdefaulted.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetSecurityDescriptorRMControl(
    securitydescriptor: *mut SECURITY_DESCRIPTOR,
    rmcontrol: *const u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorRMControl(
                securitydescriptor: *mut SECURITY_DESCRIPTOR,
                rmcontrol: *const u8,
            ) -> u32;
        }
        ::std::mem::transmute(SetSecurityDescriptorRMControl(
            ::std::mem::transmute(securitydescriptor),
            ::std::mem::transmute(rmcontrol),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetSecurityDescriptorSacl<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
    Param3: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
    bsaclpresent: Param1,
    psacl: *const ACL,
    bsacldefaulted: Param3,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityDescriptorSacl(
                psecuritydescriptor: *mut SECURITY_DESCRIPTOR,
                bsaclpresent: super::Foundation::BOOL,
                psacl: *const ACL,
                bsacldefaulted: super::Foundation::BOOL,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSecurityDescriptorSacl(
            ::std::mem::transmute(psecuritydescriptor),
            bsaclpresent.into_param().abi(),
            ::std::mem::transmute(psacl),
            bsacldefaulted.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetTokenInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    tokenhandle: Param0,
    tokeninformationclass: TOKEN_INFORMATION_CLASS,
    tokeninformation: *const ::std::ffi::c_void,
    tokeninformationlength: u32,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTokenInformation(
                tokenhandle: super::Foundation::HANDLE,
                tokeninformationclass: TOKEN_INFORMATION_CLASS,
                tokeninformation: *const ::std::ffi::c_void,
                tokeninformationlength: u32,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetTokenInformation(
            tokenhandle.into_param().abi(),
            ::std::mem::transmute(tokeninformationclass),
            ::std::mem::transmute(tokeninformation),
            ::std::mem::transmute(tokeninformationlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
pub unsafe fn SetUserObjectSecurity<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hobj: Param0,
    psirequested: *const Authorization::OBJECT_SECURITY_INFORMATION,
    psid: *const SECURITY_DESCRIPTOR,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUserObjectSecurity(
                hobj: super::Foundation::HANDLE,
                psirequested: *const Authorization::OBJECT_SECURITY_INFORMATION,
                psid: *const SECURITY_DESCRIPTOR,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetUserObjectSecurity(
            hobj.into_param().abi(),
            ::std::mem::transmute(psirequested),
            ::std::mem::transmute(psid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct TOKEN_ACCESS_INFORMATION {
    pub SidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub RestrictedSidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub Privileges: *mut TOKEN_PRIVILEGES,
    pub AuthenticationId: super::System::SystemServices::LUID,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl TOKEN_ACCESS_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for TOKEN_ACCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for TOKEN_ACCESS_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for TOKEN_ACCESS_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
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
        fmt.debug_struct("TOKEN_APPCONTAINER_INFORMATION")
            .field("TokenAppContainer", &self.TokenAppContainer)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("TOKEN_AUDIT_POLICY")
            .field("PerUserPolicy", &self.PerUserPolicy)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct TOKEN_CONTROL {
    pub TokenId: super::System::SystemServices::LUID,
    pub AuthenticationId: super::System::SystemServices::LUID,
    pub ModifiedId: super::System::SystemServices::LUID,
    pub TokenSource: TOKEN_SOURCE,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl TOKEN_CONTROL {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for TOKEN_CONTROL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for TOKEN_CONTROL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_CONTROL")
            .field("TokenId", &self.TokenId)
            .field("AuthenticationId", &self.AuthenticationId)
            .field("ModifiedId", &self.ModifiedId)
            .field("TokenSource", &self.TokenSource)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for TOKEN_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.TokenId == other.TokenId
            && self.AuthenticationId == other.AuthenticationId
            && self.ModifiedId == other.ModifiedId
            && self.TokenSource == other.TokenSource
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for TOKEN_CONTROL {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for TOKEN_CONTROL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("TOKEN_DEFAULT_DACL")
            .field("DefaultDacl", &self.DefaultDacl)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("TOKEN_DEVICE_CLAIMS")
            .field("DeviceClaims", &self.DeviceClaims)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("TOKEN_ELEVATION")
            .field("TokenIsElevated", &self.TokenIsElevated)
            .finish()
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
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
        fmt.debug_struct("TOKEN_GROUPS")
            .field("GroupCount", &self.GroupCount)
            .field("Groups", &self.Groups)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
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
    pub AuthenticationId: super::System::SystemServices::LUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl TOKEN_GROUPS_AND_PRIVILEGES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for TOKEN_GROUPS_AND_PRIVILEGES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for TOKEN_GROUPS_AND_PRIVILEGES {
    fn eq(&self, other: &Self) -> bool {
        self.SidCount == other.SidCount
            && self.SidLength == other.SidLength
            && self.Sids == other.Sids
            && self.RestrictedSidCount == other.RestrictedSidCount
            && self.RestrictedSidLength == other.RestrictedSidLength
            && self.RestrictedSids == other.RestrictedSids
            && self.PrivilegeCount == other.PrivilegeCount
            && self.PrivilegeLength == other.PrivilegeLength
            && self.Privileges == other.Privileges
            && self.AuthenticationId == other.AuthenticationId
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for TOKEN_GROUPS_AND_PRIVILEGES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for TOKEN_GROUPS_AND_PRIVILEGES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
pub const TokenRestrictedUserClaimAttributes: TOKEN_INFORMATION_CLASS =
    TOKEN_INFORMATION_CLASS(35i32);
pub const TokenRestrictedDeviceClaimAttributes: TOKEN_INFORMATION_CLASS =
    TOKEN_INFORMATION_CLASS(36i32);
pub const TokenDeviceGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(37i32);
pub const TokenRestrictedDeviceGroups: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(38i32);
pub const TokenSecurityAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(39i32);
pub const TokenIsRestricted: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(40i32);
pub const TokenProcessTrustLevel: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(41i32);
pub const TokenPrivateNameSpace: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(42i32);
pub const TokenSingletonAttributes: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(43i32);
pub const TokenBnoIsolation: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(44i32);
pub const TokenChildProcessFlags: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(45i32);
pub const TokenIsLessPrivilegedAppContainer: TOKEN_INFORMATION_CLASS =
    TOKEN_INFORMATION_CLASS(46i32);
pub const TokenIsSandboxed: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(47i32);
pub const MaxTokenInfoClass: TOKEN_INFORMATION_CLASS = TOKEN_INFORMATION_CLASS(48i32);
impl ::std::convert::From<i32> for TOKEN_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOKEN_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
        fmt.debug_struct("TOKEN_LINKED_TOKEN")
            .field("LinkedToken", &self.LinkedToken)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
        fmt.debug_struct("TOKEN_MANDATORY_LABEL")
            .field("Label", &self.Label)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("TOKEN_MANDATORY_POLICY")
            .field("Policy", &self.Policy)
            .finish()
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
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TOKEN_MANDATORY_POLICY_ID(pub u32);
pub const TOKEN_MANDATORY_POLICY_OFF: TOKEN_MANDATORY_POLICY_ID = TOKEN_MANDATORY_POLICY_ID(0u32);
pub const TOKEN_MANDATORY_POLICY_NO_WRITE_UP: TOKEN_MANDATORY_POLICY_ID =
    TOKEN_MANDATORY_POLICY_ID(1u32);
pub const TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN: TOKEN_MANDATORY_POLICY_ID =
    TOKEN_MANDATORY_POLICY_ID(2u32);
pub const TOKEN_MANDATORY_POLICY_VALID_MASK: TOKEN_MANDATORY_POLICY_ID =
    TOKEN_MANDATORY_POLICY_ID(3u32);
impl ::std::convert::From<u32> for TOKEN_MANDATORY_POLICY_ID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOKEN_MANDATORY_POLICY_ID {
    type Abi = Self;
    type DefaultType = Self;
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
#[cfg(feature = "Win32_System_SystemServices")]
pub struct TOKEN_ORIGIN {
    pub OriginatingLogonSession: super::System::SystemServices::LUID,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl TOKEN_ORIGIN {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for TOKEN_ORIGIN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for TOKEN_ORIGIN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_ORIGIN")
            .field("OriginatingLogonSession", &self.OriginatingLogonSession)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for TOKEN_ORIGIN {
    fn eq(&self, other: &Self) -> bool {
        self.OriginatingLogonSession == other.OriginatingLogonSession
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for TOKEN_ORIGIN {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for TOKEN_ORIGIN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
        fmt.debug_struct("TOKEN_OWNER")
            .field("Owner", &self.Owner)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
        fmt.debug_struct("TOKEN_PRIMARY_GROUP")
            .field("PrimaryGroup", &self.PrimaryGroup)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: u32,
    pub Privileges: [LUID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_System_SystemServices")]
impl TOKEN_PRIVILEGES {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for TOKEN_PRIVILEGES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for TOKEN_PRIVILEGES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_PRIVILEGES")
            .field("PrivilegeCount", &self.PrivilegeCount)
            .field("Privileges", &self.Privileges)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for TOKEN_PRIVILEGES {
    fn eq(&self, other: &Self) -> bool {
        self.PrivilegeCount == other.PrivilegeCount && self.Privileges == other.Privileges
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for TOKEN_PRIVILEGES {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for TOKEN_PRIVILEGES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TOKEN_PRIVILEGES_ATTRIBUTES(pub u32);
pub const SE_PRIVILEGE_ENABLED: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(2u32);
pub const SE_PRIVILEGE_ENABLED_BY_DEFAULT: TOKEN_PRIVILEGES_ATTRIBUTES =
    TOKEN_PRIVILEGES_ATTRIBUTES(1u32);
pub const SE_PRIVILEGE_REMOVED: TOKEN_PRIVILEGES_ATTRIBUTES = TOKEN_PRIVILEGES_ATTRIBUTES(4u32);
pub const SE_PRIVILEGE_USED_FOR_ACCESS: TOKEN_PRIVILEGES_ATTRIBUTES =
    TOKEN_PRIVILEGES_ATTRIBUTES(2147483648u32);
impl ::std::convert::From<u32> for TOKEN_PRIVILEGES_ATTRIBUTES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TOKEN_PRIVILEGES_ATTRIBUTES {
    type Abi = Self;
    type DefaultType = Self;
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
#[cfg(feature = "Win32_System_SystemServices")]
pub struct TOKEN_SOURCE {
    pub SourceName: [super::System::SystemServices::CHAR; 8],
    pub SourceIdentifier: super::System::SystemServices::LUID,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl TOKEN_SOURCE {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for TOKEN_SOURCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for TOKEN_SOURCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOKEN_SOURCE")
            .field("SourceName", &self.SourceName)
            .field("SourceIdentifier", &self.SourceIdentifier)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for TOKEN_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.SourceName == other.SourceName && self.SourceIdentifier == other.SourceIdentifier
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for TOKEN_SOURCE {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for TOKEN_SOURCE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct TOKEN_STATISTICS {
    pub TokenId: super::System::SystemServices::LUID,
    pub AuthenticationId: super::System::SystemServices::LUID,
    pub ExpirationTime: i64,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub DynamicCharged: u32,
    pub DynamicAvailable: u32,
    pub GroupCount: u32,
    pub PrivilegeCount: u32,
    pub ModifiedId: super::System::SystemServices::LUID,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl TOKEN_STATISTICS {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for TOKEN_STATISTICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
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
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for TOKEN_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.TokenId == other.TokenId
            && self.AuthenticationId == other.AuthenticationId
            && self.ExpirationTime == other.ExpirationTime
            && self.TokenType == other.TokenType
            && self.ImpersonationLevel == other.ImpersonationLevel
            && self.DynamicCharged == other.DynamicCharged
            && self.DynamicAvailable == other.DynamicAvailable
            && self.GroupCount == other.GroupCount
            && self.PrivilegeCount == other.PrivilegeCount
            && self.ModifiedId == other.ModifiedId
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for TOKEN_STATISTICS {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for TOKEN_STATISTICS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
        fmt.debug_struct("TOKEN_USER")
            .field("User", &self.User)
            .finish()
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
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
        fmt.debug_struct("TOKEN_USER_CLAIMS")
            .field("UserClaims", &self.UserClaims)
            .finish()
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
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TPMVSCMGR_ERROR(pub i32);
pub const TPMVSCMGR_ERROR_IMPERSONATION: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(0i32);
pub const TPMVSCMGR_ERROR_PIN_COMPLEXITY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(1i32);
pub const TPMVSCMGR_ERROR_READER_COUNT_LIMIT: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(2i32);
pub const TPMVSCMGR_ERROR_TERMINAL_SERVICES_SESSION: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(3i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(4i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(5i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(6i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(7i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(8i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(9i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_WRITE_PROPERTY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(10i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_READ_PROPERTY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(11i32);
pub const TPMVSCMGR_ERROR_VREADER_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(12i32);
pub const TPMVSCMGR_ERROR_VREADER_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(13i32);
pub const TPMVSCMGR_ERROR_VREADER_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(14i32);
pub const TPMVSCMGR_ERROR_GENERATE_LOCATE_READER: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(15i32);
pub const TPMVSCMGR_ERROR_GENERATE_FILESYSTEM: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(16i32);
pub const TPMVSCMGR_ERROR_CARD_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(17i32);
pub const TPMVSCMGR_ERROR_CARD_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(18i32);
impl ::std::convert::From<i32> for TPMVSCMGR_ERROR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TPMVSCMGR_ERROR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TPMVSCMGR_STATUS(pub i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(0i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(1i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(2i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(3i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(4i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(5i32);
pub const TPMVSCMGR_STATUS_VREADER_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(6i32);
pub const TPMVSCMGR_STATUS_VREADER_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(7i32);
pub const TPMVSCMGR_STATUS_VREADER_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(8i32);
pub const TPMVSCMGR_STATUS_GENERATE_WAITING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(9i32);
pub const TPMVSCMGR_STATUS_GENERATE_AUTHENTICATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(10i32);
pub const TPMVSCMGR_STATUS_GENERATE_RUNNING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(11i32);
pub const TPMVSCMGR_STATUS_CARD_CREATED: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(12i32);
pub const TPMVSCMGR_STATUS_CARD_DESTROYED: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(13i32);
impl ::std::convert::From<i32> for TPMVSCMGR_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TPMVSCMGR_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TPMVSC_ATTESTATION_TYPE(pub i32);
pub const TPMVSC_ATTESTATION_NONE: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(0i32);
pub const TPMVSC_ATTESTATION_AIK_ONLY: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(1i32);
pub const TPMVSC_ATTESTATION_AIK_AND_CERTIFICATE: TPMVSC_ATTESTATION_TYPE =
    TPMVSC_ATTESTATION_TYPE(2i32);
impl ::std::convert::From<i32> for TPMVSC_ATTESTATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TPMVSC_ATTESTATION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
pub const TRUSTERROR_MAX_STEPS: u32 = 38u32;
pub const TRUSTERROR_STEP_CATALOGFILE: u32 = 6u32;
pub const TRUSTERROR_STEP_CERTSTORE: u32 = 7u32;
pub const TRUSTERROR_STEP_FILEIO: u32 = 2u32;
pub const TRUSTERROR_STEP_FINAL_CERTCHKPROV: u32 = 35u32;
pub const TRUSTERROR_STEP_FINAL_CERTPROV: u32 = 34u32;
pub const TRUSTERROR_STEP_FINAL_INITPROV: u32 = 31u32;
pub const TRUSTERROR_STEP_FINAL_OBJPROV: u32 = 32u32;
pub const TRUSTERROR_STEP_FINAL_POLICYPROV: u32 = 36u32;
pub const TRUSTERROR_STEP_FINAL_SIGPROV: u32 = 33u32;
pub const TRUSTERROR_STEP_FINAL_UIPROV: u32 = 37u32;
pub const TRUSTERROR_STEP_FINAL_WVTINIT: u32 = 30u32;
pub const TRUSTERROR_STEP_MESSAGE: u32 = 8u32;
pub const TRUSTERROR_STEP_MSG_CERTCHAIN: u32 = 15u32;
pub const TRUSTERROR_STEP_MSG_COUNTERSIGCERT: u32 = 17u32;
pub const TRUSTERROR_STEP_MSG_COUNTERSIGINFO: u32 = 16u32;
pub const TRUSTERROR_STEP_MSG_INNERCNT: u32 = 11u32;
pub const TRUSTERROR_STEP_MSG_INNERCNTTYPE: u32 = 10u32;
pub const TRUSTERROR_STEP_MSG_SIGNERCERT: u32 = 14u32;
pub const TRUSTERROR_STEP_MSG_SIGNERCOUNT: u32 = 9u32;
pub const TRUSTERROR_STEP_MSG_SIGNERINFO: u32 = 13u32;
pub const TRUSTERROR_STEP_MSG_STORE: u32 = 12u32;
pub const TRUSTERROR_STEP_SIP: u32 = 3u32;
pub const TRUSTERROR_STEP_SIPSUBJINFO: u32 = 5u32;
pub const TRUSTERROR_STEP_VERIFY_MSGHASH: u32 = 18u32;
pub const TRUSTERROR_STEP_VERIFY_MSGINDIRECTDATA: u32 = 19u32;
pub const TRUSTERROR_STEP_WVTPARAMS: u32 = 0u32;
pub const TpmVirtualSmartCardManager: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        379686534,
        32622,
        19488,
        [173, 137, 79, 252, 13, 183, 169, 106],
    );
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ValidateLicenseKeyProtection<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PWSTR>,
>(
    licensekey: Param0,
    notvalidbefore: *mut super::Foundation::FILETIME,
    notvalidafter: *mut super::Foundation::FILETIME,
    status: *mut LicenseProtectionStatus,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ValidateLicenseKeyProtection(
                licensekey: super::Foundation::PWSTR,
                notvalidbefore: *mut super::Foundation::FILETIME,
                notvalidafter: *mut super::Foundation::FILETIME,
                status: *mut LicenseProtectionStatus,
            ) -> ::windows::runtime::HRESULT;
        }
        ValidateLicenseKeyProtection(
            licensekey.into_param().abi(),
            ::std::mem::transmute(notvalidbefore),
            ::std::mem::transmute(notvalidafter),
            ::std::mem::transmute(status),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
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
pub const WinBuiltinPreWindows2000CompatibleAccessSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(35i32);
pub const WinBuiltinRemoteDesktopUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(36i32);
pub const WinBuiltinNetworkConfigurationOperatorsSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(37i32);
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
pub const WinBuiltinIncomingForestTrustBuildersSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(56i32);
pub const WinBuiltinPerfMonitoringUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(57i32);
pub const WinBuiltinPerfLoggingUsersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(58i32);
pub const WinBuiltinAuthorizationAccessSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(59i32);
pub const WinBuiltinTerminalServerLicenseServersSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(60i32);
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
pub const WinCapabilityPrivateNetworkClientServerSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(87i32);
pub const WinCapabilityPicturesLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(88i32);
pub const WinCapabilityVideosLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(89i32);
pub const WinCapabilityMusicLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(90i32);
pub const WinCapabilityDocumentsLibrarySid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(91i32);
pub const WinCapabilitySharedUserCertificatesSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(92i32);
pub const WinCapabilityEnterpriseAuthenticationSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(93i32);
pub const WinCapabilityRemovableStorageSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(94i32);
pub const WinBuiltinRDSRemoteAccessServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(95i32);
pub const WinBuiltinRDSEndpointServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(96i32);
pub const WinBuiltinRDSManagementServersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(97i32);
pub const WinUserModeDriversSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(98i32);
pub const WinBuiltinHyperVAdminsSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(99i32);
pub const WinAccountCloneableControllersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(100i32);
pub const WinBuiltinAccessControlAssistanceOperatorsSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(101i32);
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
pub const WinAuthenticationKeyPropertyAttestationSid: WELL_KNOWN_SID_TYPE =
    WELL_KNOWN_SID_TYPE(117i32);
pub const WinAuthenticationFreshKeyAuthSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(118i32);
pub const WinBuiltinDeviceOwnersSid: WELL_KNOWN_SID_TYPE = WELL_KNOWN_SID_TYPE(119i32);
impl ::std::convert::From<i32> for WELL_KNOWN_SID_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WELL_KNOWN_SID_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WINTRUST_BLOB_INFO {
    pub cbStruct: u32,
    pub gSubject: ::windows::runtime::GUID,
    pub pcwszDisplayName: super::Foundation::PWSTR,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl WINTRUST_BLOB_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINTRUST_BLOB_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINTRUST_BLOB_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_BLOB_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("gSubject", &self.gSubject)
            .field("pcwszDisplayName", &self.pcwszDisplayName)
            .field("cbMemObject", &self.cbMemObject)
            .field("pbMemObject", &self.pbMemObject)
            .field("cbMemSignedMsg", &self.cbMemSignedMsg)
            .field("pbMemSignedMsg", &self.pbMemSignedMsg)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINTRUST_BLOB_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.gSubject == other.gSubject
            && self.pcwszDisplayName == other.pcwszDisplayName
            && self.cbMemObject == other.cbMemObject
            && self.pbMemObject == other.pbMemObject
            && self.cbMemSignedMsg == other.cbMemSignedMsg
            && self.pbMemSignedMsg == other.pbMemSignedMsg
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINTRUST_BLOB_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINTRUST_BLOB_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct WINTRUST_CATALOG_INFO {
    pub cbStruct: u32,
    pub dwCatalogVersion: u32,
    pub pcwszCatalogFilePath: super::Foundation::PWSTR,
    pub pcwszMemberTag: super::Foundation::PWSTR,
    pub pcwszMemberFilePath: super::Foundation::PWSTR,
    pub hMemberFile: super::Foundation::HANDLE,
    pub pbCalculatedFileHash: *mut u8,
    pub cbCalculatedFileHash: u32,
    pub pcCatalogContext: *mut Cryptography::Core::CTL_CONTEXT,
    pub hCatAdmin: isize,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl WINTRUST_CATALOG_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for WINTRUST_CATALOG_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for WINTRUST_CATALOG_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_CATALOG_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("dwCatalogVersion", &self.dwCatalogVersion)
            .field("pcwszCatalogFilePath", &self.pcwszCatalogFilePath)
            .field("pcwszMemberTag", &self.pcwszMemberTag)
            .field("pcwszMemberFilePath", &self.pcwszMemberFilePath)
            .field("hMemberFile", &self.hMemberFile)
            .field("pbCalculatedFileHash", &self.pbCalculatedFileHash)
            .field("cbCalculatedFileHash", &self.cbCalculatedFileHash)
            .field("pcCatalogContext", &self.pcCatalogContext)
            .field("hCatAdmin", &self.hCatAdmin)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for WINTRUST_CATALOG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwCatalogVersion == other.dwCatalogVersion
            && self.pcwszCatalogFilePath == other.pcwszCatalogFilePath
            && self.pcwszMemberTag == other.pcwszMemberTag
            && self.pcwszMemberFilePath == other.pcwszMemberFilePath
            && self.hMemberFile == other.hMemberFile
            && self.pbCalculatedFileHash == other.pbCalculatedFileHash
            && self.cbCalculatedFileHash == other.cbCalculatedFileHash
            && self.pcCatalogContext == other.pcCatalogContext
            && self.hCatAdmin == other.hCatAdmin
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for WINTRUST_CATALOG_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for WINTRUST_CATALOG_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct WINTRUST_CERT_INFO {
    pub cbStruct: u32,
    pub pcwszDisplayName: super::Foundation::PWSTR,
    pub psCertContext: *mut Cryptography::Core::CERT_CONTEXT,
    pub chStores: u32,
    pub pahStores: *mut *mut ::std::ffi::c_void,
    pub dwFlags: u32,
    pub psftVerifyAsOf: *mut super::Foundation::FILETIME,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl WINTRUST_CERT_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for WINTRUST_CERT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for WINTRUST_CERT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_CERT_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("pcwszDisplayName", &self.pcwszDisplayName)
            .field("psCertContext", &self.psCertContext)
            .field("chStores", &self.chStores)
            .field("pahStores", &self.pahStores)
            .field("dwFlags", &self.dwFlags)
            .field("psftVerifyAsOf", &self.psftVerifyAsOf)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for WINTRUST_CERT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pcwszDisplayName == other.pcwszDisplayName
            && self.psCertContext == other.psCertContext
            && self.chStores == other.chStores
            && self.pahStores == other.pahStores
            && self.dwFlags == other.dwFlags
            && self.psftVerifyAsOf == other.psftVerifyAsOf
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for WINTRUST_CERT_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for WINTRUST_CERT_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct WINTRUST_DATA {
    pub cbStruct: u32,
    pub pPolicyCallbackData: *mut ::std::ffi::c_void,
    pub pSIPClientData: *mut ::std::ffi::c_void,
    pub dwUIChoice: WINTRUST_DATA_UICHOICE,
    pub fdwRevocationChecks: WINTRUST_DATA_REVOCATION_CHECKS,
    pub dwUnionChoice: WINTRUST_DATA_UNION_CHOICE,
    pub Anonymous: WINTRUST_DATA_0,
    pub dwStateAction: WINTRUST_DATA_STATE_ACTION,
    pub hWVTStateData: super::Foundation::HANDLE,
    pub pwszURLReference: super::Foundation::PWSTR,
    pub dwProvFlags: u32,
    pub dwUIContext: WINTRUST_DATA_UICONTEXT,
    pub pSignatureSettings: *mut WINTRUST_SIGNATURE_SETTINGS,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl WINTRUST_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for WINTRUST_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for WINTRUST_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for WINTRUST_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub union WINTRUST_DATA_0 {
    pub pFile: *mut WINTRUST_FILE_INFO,
    pub pCatalog: *mut WINTRUST_CATALOG_INFO,
    pub pBlob: *mut WINTRUST_BLOB_INFO,
    pub pSgnr: *mut WINTRUST_SGNR_INFO,
    pub pCert: *mut WINTRUST_CERT_INFO,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl WINTRUST_DATA_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for WINTRUST_DATA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for WINTRUST_DATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for WINTRUST_DATA_0 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINTRUST_DATA_REVOCATION_CHECKS(pub u32);
pub const WTD_REVOKE_NONE: WINTRUST_DATA_REVOCATION_CHECKS = WINTRUST_DATA_REVOCATION_CHECKS(0u32);
pub const WTD_REVOKE_WHOLECHAIN: WINTRUST_DATA_REVOCATION_CHECKS =
    WINTRUST_DATA_REVOCATION_CHECKS(1u32);
impl ::std::convert::From<u32> for WINTRUST_DATA_REVOCATION_CHECKS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_REVOCATION_CHECKS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WINTRUST_DATA_REVOCATION_CHECKS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_DATA_REVOCATION_CHECKS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_DATA_REVOCATION_CHECKS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_DATA_REVOCATION_CHECKS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_DATA_REVOCATION_CHECKS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINTRUST_DATA_STATE_ACTION(pub u32);
pub const WTD_STATEACTION_IGNORE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(0u32);
pub const WTD_STATEACTION_VERIFY: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(1u32);
pub const WTD_STATEACTION_CLOSE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(2u32);
pub const WTD_STATEACTION_AUTO_CACHE: WINTRUST_DATA_STATE_ACTION = WINTRUST_DATA_STATE_ACTION(3u32);
pub const WTD_STATEACTION_AUTO_CACHE_FLUSH: WINTRUST_DATA_STATE_ACTION =
    WINTRUST_DATA_STATE_ACTION(4u32);
impl ::std::convert::From<u32> for WINTRUST_DATA_STATE_ACTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_STATE_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WINTRUST_DATA_STATE_ACTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_DATA_STATE_ACTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_DATA_STATE_ACTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_DATA_STATE_ACTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_DATA_STATE_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINTRUST_DATA_UICHOICE(pub u32);
pub const WTD_UI_ALL: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(1u32);
pub const WTD_UI_NONE: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(2u32);
pub const WTD_UI_NOBAD: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(3u32);
pub const WTD_UI_NOGOOD: WINTRUST_DATA_UICHOICE = WINTRUST_DATA_UICHOICE(4u32);
impl ::std::convert::From<u32> for WINTRUST_DATA_UICHOICE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_UICHOICE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WINTRUST_DATA_UICHOICE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_DATA_UICHOICE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_DATA_UICHOICE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_DATA_UICHOICE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_DATA_UICHOICE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINTRUST_DATA_UICONTEXT(pub u32);
pub const WTD_UICONTEXT_EXECUTE: WINTRUST_DATA_UICONTEXT = WINTRUST_DATA_UICONTEXT(0u32);
pub const WTD_UICONTEXT_INSTALL: WINTRUST_DATA_UICONTEXT = WINTRUST_DATA_UICONTEXT(1u32);
impl ::std::convert::From<u32> for WINTRUST_DATA_UICONTEXT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_UICONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WINTRUST_DATA_UICONTEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_DATA_UICONTEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_DATA_UICONTEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_DATA_UICONTEXT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_DATA_UICONTEXT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINTRUST_DATA_UNION_CHOICE(pub u32);
pub const WTD_CHOICE_FILE: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(1u32);
pub const WTD_CHOICE_CATALOG: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(2u32);
pub const WTD_CHOICE_BLOB: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(3u32);
pub const WTD_CHOICE_SIGNER: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(4u32);
pub const WTD_CHOICE_CERT: WINTRUST_DATA_UNION_CHOICE = WINTRUST_DATA_UNION_CHOICE(5u32);
impl ::std::convert::From<u32> for WINTRUST_DATA_UNION_CHOICE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_DATA_UNION_CHOICE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WINTRUST_DATA_UNION_CHOICE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_DATA_UNION_CHOICE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_DATA_UNION_CHOICE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_DATA_UNION_CHOICE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_DATA_UNION_CHOICE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WINTRUST_FILE_INFO {
    pub cbStruct: u32,
    pub pcwszFilePath: super::Foundation::PWSTR,
    pub hFile: super::Foundation::HANDLE,
    pub pgKnownSubject: *mut ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl WINTRUST_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINTRUST_FILE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINTRUST_FILE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_FILE_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("pcwszFilePath", &self.pcwszFilePath)
            .field("hFile", &self.hFile)
            .field("pgKnownSubject", &self.pgKnownSubject)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINTRUST_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pcwszFilePath == other.pcwszFilePath
            && self.hFile == other.hFile
            && self.pgKnownSubject == other.pgKnownSubject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINTRUST_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINTRUST_FILE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(pub u32);
pub const DWACTION_ALLOCANDFILL: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION =
    WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(1u32);
pub const DWACTION_FREE: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION =
    WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION(2u32);
impl ::std::convert::From<u32> for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WINTRUST_MAX_HASH_BYTES_TO_MAP_DEFAULT: u32 = 1048576u32;
pub const WINTRUST_MAX_HEADER_BYTES_TO_MAP_DEFAULT: u32 = 10485760u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINTRUST_POLICY_FLAGS(pub u32);
pub const WTPF_TRUSTTEST: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(32u32);
pub const WTPF_TESTCANBEVALID: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(128u32);
pub const WTPF_IGNOREEXPIRATION: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(256u32);
pub const WTPF_IGNOREREVOKATION: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(512u32);
pub const WTPF_OFFLINEOK_IND: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(1024u32);
pub const WTPF_OFFLINEOK_COM: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(2048u32);
pub const WTPF_OFFLINEOKNBU_IND: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(4096u32);
pub const WTPF_OFFLINEOKNBU_COM: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(8192u32);
pub const WTPF_VERIFY_V1_OFF: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(65536u32);
pub const WTPF_IGNOREREVOCATIONONTS: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(131072u32);
pub const WTPF_ALLOWONLYPERTRUST: WINTRUST_POLICY_FLAGS = WINTRUST_POLICY_FLAGS(262144u32);
impl ::std::convert::From<u32> for WINTRUST_POLICY_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_POLICY_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_POLICY_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_POLICY_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_POLICY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct WINTRUST_SGNR_INFO {
    pub cbStruct: u32,
    pub pcwszDisplayName: super::Foundation::PWSTR,
    pub psSignerInfo: *mut Cryptography::Core::CMSG_SIGNER_INFO,
    pub chStores: u32,
    pub pahStores: *mut *mut ::std::ffi::c_void,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl WINTRUST_SGNR_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for WINTRUST_SGNR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for WINTRUST_SGNR_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_SGNR_INFO")
            .field("cbStruct", &self.cbStruct)
            .field("pcwszDisplayName", &self.pcwszDisplayName)
            .field("psSignerInfo", &self.psSignerInfo)
            .field("chStores", &self.chStores)
            .field("pahStores", &self.pahStores)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for WINTRUST_SGNR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.pcwszDisplayName == other.pcwszDisplayName
            && self.psSignerInfo == other.psSignerInfo
            && self.chStores == other.chStores
            && self.pahStores == other.pahStores
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for WINTRUST_SGNR_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for WINTRUST_SGNR_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct WINTRUST_SIGNATURE_SETTINGS {
    pub cbStruct: u32,
    pub dwIndex: u32,
    pub dwFlags: WINTRUST_SIGNATURE_SETTINGS_FLAGS,
    pub cSecondarySigs: u32,
    pub dwVerifiedSigIndex: u32,
    pub pCryptoPolicy: *mut Cryptography::Core::CERT_STRONG_SIGN_PARA,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl WINTRUST_SIGNATURE_SETTINGS {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for WINTRUST_SIGNATURE_SETTINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::fmt::Debug for WINTRUST_SIGNATURE_SETTINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINTRUST_SIGNATURE_SETTINGS")
            .field("cbStruct", &self.cbStruct)
            .field("dwIndex", &self.dwIndex)
            .field("dwFlags", &self.dwFlags)
            .field("cSecondarySigs", &self.cSecondarySigs)
            .field("dwVerifiedSigIndex", &self.dwVerifiedSigIndex)
            .field("pCryptoPolicy", &self.pCryptoPolicy)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for WINTRUST_SIGNATURE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.dwIndex == other.dwIndex
            && self.dwFlags == other.dwFlags
            && self.cSecondarySigs == other.cSecondarySigs
            && self.dwVerifiedSigIndex == other.dwVerifiedSigIndex
            && self.pCryptoPolicy == other.pCryptoPolicy
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for WINTRUST_SIGNATURE_SETTINGS {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for WINTRUST_SIGNATURE_SETTINGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WINTRUST_SIGNATURE_SETTINGS_FLAGS(pub u32);
pub const WSS_VERIFY_SPECIFIC: WINTRUST_SIGNATURE_SETTINGS_FLAGS =
    WINTRUST_SIGNATURE_SETTINGS_FLAGS(1u32);
pub const WSS_GET_SECONDARY_SIG_COUNT: WINTRUST_SIGNATURE_SETTINGS_FLAGS =
    WINTRUST_SIGNATURE_SETTINGS_FLAGS(2u32);
impl ::std::convert::From<u32> for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINTRUST_SIGNATURE_SETTINGS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIN_CERTIFICATE {
    pub dwLength: u32,
    pub wRevision: u16,
    pub wCertificateType: u16,
    pub bCertificate: [u8; 1],
}
impl WIN_CERTIFICATE {}
impl ::std::default::Default for WIN_CERTIFICATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIN_CERTIFICATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_CERTIFICATE")
            .field("dwLength", &self.dwLength)
            .field("wRevision", &self.wRevision)
            .field("wCertificateType", &self.wCertificateType)
            .field("bCertificate", &self.bCertificate)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIN_CERTIFICATE {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength
            && self.wRevision == other.wRevision
            && self.wCertificateType == other.wCertificateType
            && self.bCertificate == other.bCertificate
    }
}
impl ::std::cmp::Eq for WIN_CERTIFICATE {}
unsafe impl ::windows::runtime::Abi for WIN_CERTIFICATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIN_CERT_REVISION_1_0: u32 = 256u32;
pub const WIN_CERT_REVISION_2_0: u32 = 512u32;
pub const WIN_CERT_TYPE_PKCS_SIGNED_DATA: u32 = 2u32;
pub const WIN_CERT_TYPE_RESERVED_1: u32 = 3u32;
pub const WIN_CERT_TYPE_TS_STACK_SIGNED: u32 = 4u32;
pub const WIN_CERT_TYPE_X509: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    pub hClientToken: super::Foundation::HANDLE,
    pub lpCertificate: *mut WIN_CERTIFICATE,
}
#[cfg(feature = "Win32_Foundation")]
impl WIN_SPUB_TRUSTED_PUBLISHER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_SPUB_TRUSTED_PUBLISHER_DATA")
            .field("hClientToken", &self.hClientToken)
            .field("lpCertificate", &self.lpCertificate)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.hClientToken == other.hClientToken && self.lpCertificate == other.lpCertificate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIN_SPUB_TRUSTED_PUBLISHER_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    pub hClientToken: super::Foundation::HANDLE,
    pub SubjectType: *mut ::windows::runtime::GUID,
    pub Subject: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT")
            .field("hClientToken", &self.hClientToken)
            .field("SubjectType", &self.SubjectType)
            .field("Subject", &self.Subject)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.hClientToken == other.hClientToken
            && self.SubjectType == other.SubjectType
            && self.Subject == other.Subject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    pub SubjectType: *mut ::windows::runtime::GUID,
    pub Subject: *mut ::std::ffi::c_void,
}
impl WIN_TRUST_ACTDATA_SUBJECT_ONLY {}
impl ::std::default::Default for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_TRUST_ACTDATA_SUBJECT_ONLY")
            .field("SubjectType", &self.SubjectType)
            .field("Subject", &self.Subject)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn eq(&self, other: &Self) -> bool {
        self.SubjectType == other.SubjectType && self.Subject == other.Subject
    }
}
impl ::std::cmp::Eq for WIN_TRUST_ACTDATA_SUBJECT_ONLY {}
unsafe impl ::windows::runtime::Abi for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_TRUST_SUBJECT_FILE {
    pub hFile: super::Foundation::HANDLE,
    pub lpPath: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WIN_TRUST_SUBJECT_FILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIN_TRUST_SUBJECT_FILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIN_TRUST_SUBJECT_FILE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_TRUST_SUBJECT_FILE")
            .field("hFile", &self.hFile)
            .field("lpPath", &self.lpPath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIN_TRUST_SUBJECT_FILE {
    fn eq(&self, other: &Self) -> bool {
        self.hFile == other.hFile && self.lpPath == other.lpPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIN_TRUST_SUBJECT_FILE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIN_TRUST_SUBJECT_FILE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    pub hFile: super::Foundation::HANDLE,
    pub lpPath: super::Foundation::PWSTR,
    pub lpDisplayName: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN_TRUST_SUBJECT_FILE_AND_DISPLAY")
            .field("hFile", &self.hFile)
            .field("lpPath", &self.lpPath)
            .field("lpDisplayName", &self.lpDisplayName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        self.hFile == other.hFile
            && self.lpPath == other.lpPath
            && self.lpDisplayName == other.lpDisplayName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    pub dwType: u32,
    pub pszUserName: super::Foundation::PWSTR,
    pub pszDomain: super::Foundation::PWSTR,
    pub pszPassword: super::Foundation::PWSTR,
    pub fPromptForPassword: super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl WLX_CLIENT_CREDENTIALS_INFO_V1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_CLIENT_CREDENTIALS_INFO_V1_0")
            .field("dwType", &self.dwType)
            .field("pszUserName", &self.pszUserName)
            .field("pszDomain", &self.pszDomain)
            .field("pszPassword", &self.pszPassword)
            .field("fPromptForPassword", &self.fPromptForPassword)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.pszUserName == other.pszUserName
            && self.pszDomain == other.pszDomain
            && self.pszPassword == other.pszPassword
            && self.fPromptForPassword == other.fPromptForPassword
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    pub dwType: u32,
    pub pszUserName: super::Foundation::PWSTR,
    pub pszDomain: super::Foundation::PWSTR,
    pub pszPassword: super::Foundation::PWSTR,
    pub fPromptForPassword: super::Foundation::BOOL,
    pub fDisconnectOnLogonFailure: super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl WLX_CLIENT_CREDENTIALS_INFO_V2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_CLIENT_CREDENTIALS_INFO_V2_0")
            .field("dwType", &self.dwType)
            .field("pszUserName", &self.pszUserName)
            .field("pszDomain", &self.pszDomain)
            .field("pszPassword", &self.pszPassword)
            .field("fPromptForPassword", &self.fPromptForPassword)
            .field("fDisconnectOnLogonFailure", &self.fDisconnectOnLogonFailure)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.pszUserName == other.pszUserName
            && self.pszDomain == other.pszDomain
            && self.pszPassword == other.pszPassword
            && self.fPromptForPassword == other.fPromptForPassword
            && self.fDisconnectOnLogonFailure == other.fDisconnectOnLogonFailure
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WLX_CONSOLESWITCHCREDENTIAL_TYPE_V1_0: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    pub dwType: u32,
    pub UserToken: super::Foundation::HANDLE,
    pub LogonId: super::System::SystemServices::LUID,
    pub Quotas: QUOTA_LIMITS,
    pub UserName: super::Foundation::PWSTR,
    pub Domain: super::Foundation::PWSTR,
    pub LogonTime: i64,
    pub SmartCardLogon: super::Foundation::BOOL,
    pub ProfileLength: u32,
    pub MessageType: u32,
    pub LogonCount: u16,
    pub BadPasswordCount: u16,
    pub ProfileLogonTime: i64,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub LogonScript: super::Foundation::PWSTR,
    pub HomeDirectory: super::Foundation::PWSTR,
    pub FullName: super::Foundation::PWSTR,
    pub ProfilePath: super::Foundation::PWSTR,
    pub HomeDirectoryDrive: super::Foundation::PWSTR,
    pub LogonServer: super::Foundation::PWSTR,
    pub UserFlags: u32,
    pub PrivateDataLen: u32,
    pub PrivateData: *mut u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0")
            .field("dwType", &self.dwType)
            .field("UserToken", &self.UserToken)
            .field("LogonId", &self.LogonId)
            .field("Quotas", &self.Quotas)
            .field("UserName", &self.UserName)
            .field("Domain", &self.Domain)
            .field("LogonTime", &self.LogonTime)
            .field("SmartCardLogon", &self.SmartCardLogon)
            .field("ProfileLength", &self.ProfileLength)
            .field("MessageType", &self.MessageType)
            .field("LogonCount", &self.LogonCount)
            .field("BadPasswordCount", &self.BadPasswordCount)
            .field("ProfileLogonTime", &self.ProfileLogonTime)
            .field("LogoffTime", &self.LogoffTime)
            .field("KickOffTime", &self.KickOffTime)
            .field("PasswordLastSet", &self.PasswordLastSet)
            .field("PasswordCanChange", &self.PasswordCanChange)
            .field("PasswordMustChange", &self.PasswordMustChange)
            .field("LogonScript", &self.LogonScript)
            .field("HomeDirectory", &self.HomeDirectory)
            .field("FullName", &self.FullName)
            .field("ProfilePath", &self.ProfilePath)
            .field("HomeDirectoryDrive", &self.HomeDirectoryDrive)
            .field("LogonServer", &self.LogonServer)
            .field("UserFlags", &self.UserFlags)
            .field("PrivateDataLen", &self.PrivateDataLen)
            .field("PrivateData", &self.PrivateData)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.UserToken == other.UserToken
            && self.LogonId == other.LogonId
            && self.Quotas == other.Quotas
            && self.UserName == other.UserName
            && self.Domain == other.Domain
            && self.LogonTime == other.LogonTime
            && self.SmartCardLogon == other.SmartCardLogon
            && self.ProfileLength == other.ProfileLength
            && self.MessageType == other.MessageType
            && self.LogonCount == other.LogonCount
            && self.BadPasswordCount == other.BadPasswordCount
            && self.ProfileLogonTime == other.ProfileLogonTime
            && self.LogoffTime == other.LogoffTime
            && self.KickOffTime == other.KickOffTime
            && self.PasswordLastSet == other.PasswordLastSet
            && self.PasswordCanChange == other.PasswordCanChange
            && self.PasswordMustChange == other.PasswordMustChange
            && self.LogonScript == other.LogonScript
            && self.HomeDirectory == other.HomeDirectory
            && self.FullName == other.FullName
            && self.ProfilePath == other.ProfilePath
            && self.HomeDirectoryDrive == other.HomeDirectoryDrive
            && self.LogonServer == other.LogonServer
            && self.UserFlags == other.UserFlags
            && self.PrivateDataLen == other.PrivateDataLen
            && self.PrivateData == other.PrivateData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WLX_CREATE_INSTANCE_ONLY: u32 = 1u32;
pub const WLX_CREATE_USER: u32 = 2u32;
pub const WLX_CREDENTIAL_TYPE_V1_0: u32 = 1u32;
pub const WLX_CREDENTIAL_TYPE_V2_0: u32 = 2u32;
pub const WLX_CURRENT_VERSION: u32 = 65540u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
pub struct WLX_DESKTOP {
    pub Size: u32,
    pub Flags: u32,
    pub hDesktop: super::System::StationsAndDesktops::HDESK,
    pub pszDesktopName: super::Foundation::PWSTR,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
impl WLX_DESKTOP {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
impl ::std::default::Default for WLX_DESKTOP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
impl ::std::fmt::Debug for WLX_DESKTOP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_DESKTOP")
            .field("Size", &self.Size)
            .field("Flags", &self.Flags)
            .field("hDesktop", &self.hDesktop)
            .field("pszDesktopName", &self.pszDesktopName)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
impl ::std::cmp::PartialEq for WLX_DESKTOP {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Flags == other.Flags
            && self.hDesktop == other.hDesktop
            && self.pszDesktopName == other.pszDesktopName
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
impl ::std::cmp::Eq for WLX_DESKTOP {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
unsafe impl ::windows::runtime::Abi for WLX_DESKTOP {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WLX_DESKTOP_HANDLE: u32 = 2u32;
pub const WLX_DESKTOP_NAME: u32 = 1u32;
pub const WLX_DIRECTORY_LENGTH: u32 = 256u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct WLX_DISPATCH_VERSION_1_0 {
    pub WlxUseCtrlAltDel: ::std::option::Option<PWLX_USE_CTRL_ALT_DEL>,
    pub WlxSetContextPointer: ::std::option::Option<PWLX_SET_CONTEXT_POINTER>,
    pub WlxSasNotify: ::std::option::Option<PWLX_SAS_NOTIFY>,
    pub WlxSetTimeout: ::std::option::Option<PWLX_SET_TIMEOUT>,
    pub WlxAssignShellProtection: ::std::option::Option<PWLX_ASSIGN_SHELL_PROTECTION>,
    pub WlxMessageBox: ::std::option::Option<PWLX_MESSAGE_BOX>,
    pub WlxDialogBox: ::std::option::Option<PWLX_DIALOG_BOX>,
    pub WlxDialogBoxParam: ::std::option::Option<PWLX_DIALOG_BOX_PARAM>,
    pub WlxDialogBoxIndirect: ::std::option::Option<PWLX_DIALOG_BOX_INDIRECT>,
    pub WlxDialogBoxIndirectParam: ::std::option::Option<PWLX_DIALOG_BOX_INDIRECT_PARAM>,
    pub WlxSwitchDesktopToUser: ::std::option::Option<PWLX_SWITCH_DESKTOP_TO_USER>,
    pub WlxSwitchDesktopToWinlogon: ::std::option::Option<PWLX_SWITCH_DESKTOP_TO_WINLOGON>,
    pub WlxChangePasswordNotify: ::std::option::Option<PWLX_CHANGE_PASSWORD_NOTIFY>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl WLX_DISPATCH_VERSION_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for WLX_DISPATCH_VERSION_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for WLX_DISPATCH_VERSION_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_DISPATCH_VERSION_1_0").finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for WLX_DISPATCH_VERSION_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.WlxUseCtrlAltDel.map(|f| f as usize) == other.WlxUseCtrlAltDel.map(|f| f as usize)
            && self.WlxSetContextPointer.map(|f| f as usize)
                == other.WlxSetContextPointer.map(|f| f as usize)
            && self.WlxSasNotify.map(|f| f as usize) == other.WlxSasNotify.map(|f| f as usize)
            && self.WlxSetTimeout.map(|f| f as usize) == other.WlxSetTimeout.map(|f| f as usize)
            && self.WlxAssignShellProtection.map(|f| f as usize)
                == other.WlxAssignShellProtection.map(|f| f as usize)
            && self.WlxMessageBox.map(|f| f as usize) == other.WlxMessageBox.map(|f| f as usize)
            && self.WlxDialogBox.map(|f| f as usize) == other.WlxDialogBox.map(|f| f as usize)
            && self.WlxDialogBoxParam.map(|f| f as usize)
                == other.WlxDialogBoxParam.map(|f| f as usize)
            && self.WlxDialogBoxIndirect.map(|f| f as usize)
                == other.WlxDialogBoxIndirect.map(|f| f as usize)
            && self.WlxDialogBoxIndirectParam.map(|f| f as usize)
                == other.WlxDialogBoxIndirectParam.map(|f| f as usize)
            && self.WlxSwitchDesktopToUser.map(|f| f as usize)
                == other.WlxSwitchDesktopToUser.map(|f| f as usize)
            && self.WlxSwitchDesktopToWinlogon.map(|f| f as usize)
                == other.WlxSwitchDesktopToWinlogon.map(|f| f as usize)
            && self.WlxChangePasswordNotify.map(|f| f as usize)
                == other.WlxChangePasswordNotify.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for WLX_DISPATCH_VERSION_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for WLX_DISPATCH_VERSION_1_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
pub struct WLX_DISPATCH_VERSION_1_1 {
    pub WlxUseCtrlAltDel: ::std::option::Option<PWLX_USE_CTRL_ALT_DEL>,
    pub WlxSetContextPointer: ::std::option::Option<PWLX_SET_CONTEXT_POINTER>,
    pub WlxSasNotify: ::std::option::Option<PWLX_SAS_NOTIFY>,
    pub WlxSetTimeout: ::std::option::Option<PWLX_SET_TIMEOUT>,
    pub WlxAssignShellProtection: ::std::option::Option<PWLX_ASSIGN_SHELL_PROTECTION>,
    pub WlxMessageBox: ::std::option::Option<PWLX_MESSAGE_BOX>,
    pub WlxDialogBox: ::std::option::Option<PWLX_DIALOG_BOX>,
    pub WlxDialogBoxParam: ::std::option::Option<PWLX_DIALOG_BOX_PARAM>,
    pub WlxDialogBoxIndirect: ::std::option::Option<PWLX_DIALOG_BOX_INDIRECT>,
    pub WlxDialogBoxIndirectParam: ::std::option::Option<PWLX_DIALOG_BOX_INDIRECT_PARAM>,
    pub WlxSwitchDesktopToUser: ::std::option::Option<PWLX_SWITCH_DESKTOP_TO_USER>,
    pub WlxSwitchDesktopToWinlogon: ::std::option::Option<PWLX_SWITCH_DESKTOP_TO_WINLOGON>,
    pub WlxChangePasswordNotify: ::std::option::Option<PWLX_CHANGE_PASSWORD_NOTIFY>,
    pub WlxGetSourceDesktop: ::std::option::Option<PWLX_GET_SOURCE_DESKTOP>,
    pub WlxSetReturnDesktop: ::std::option::Option<PWLX_SET_RETURN_DESKTOP>,
    pub WlxCreateUserDesktop: ::std::option::Option<PWLX_CREATE_USER_DESKTOP>,
    pub WlxChangePasswordNotifyEx: ::std::option::Option<PWLX_CHANGE_PASSWORD_NOTIFY_EX>,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl WLX_DISPATCH_VERSION_1_1 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::default::Default for WLX_DISPATCH_VERSION_1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::fmt::Debug for WLX_DISPATCH_VERSION_1_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_DISPATCH_VERSION_1_1").finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::cmp::PartialEq for WLX_DISPATCH_VERSION_1_1 {
    fn eq(&self, other: &Self) -> bool {
        self.WlxUseCtrlAltDel.map(|f| f as usize) == other.WlxUseCtrlAltDel.map(|f| f as usize)
            && self.WlxSetContextPointer.map(|f| f as usize)
                == other.WlxSetContextPointer.map(|f| f as usize)
            && self.WlxSasNotify.map(|f| f as usize) == other.WlxSasNotify.map(|f| f as usize)
            && self.WlxSetTimeout.map(|f| f as usize) == other.WlxSetTimeout.map(|f| f as usize)
            && self.WlxAssignShellProtection.map(|f| f as usize)
                == other.WlxAssignShellProtection.map(|f| f as usize)
            && self.WlxMessageBox.map(|f| f as usize) == other.WlxMessageBox.map(|f| f as usize)
            && self.WlxDialogBox.map(|f| f as usize) == other.WlxDialogBox.map(|f| f as usize)
            && self.WlxDialogBoxParam.map(|f| f as usize)
                == other.WlxDialogBoxParam.map(|f| f as usize)
            && self.WlxDialogBoxIndirect.map(|f| f as usize)
                == other.WlxDialogBoxIndirect.map(|f| f as usize)
            && self.WlxDialogBoxIndirectParam.map(|f| f as usize)
                == other.WlxDialogBoxIndirectParam.map(|f| f as usize)
            && self.WlxSwitchDesktopToUser.map(|f| f as usize)
                == other.WlxSwitchDesktopToUser.map(|f| f as usize)
            && self.WlxSwitchDesktopToWinlogon.map(|f| f as usize)
                == other.WlxSwitchDesktopToWinlogon.map(|f| f as usize)
            && self.WlxChangePasswordNotify.map(|f| f as usize)
                == other.WlxChangePasswordNotify.map(|f| f as usize)
            && self.WlxGetSourceDesktop.map(|f| f as usize)
                == other.WlxGetSourceDesktop.map(|f| f as usize)
            && self.WlxSetReturnDesktop.map(|f| f as usize)
                == other.WlxSetReturnDesktop.map(|f| f as usize)
            && self.WlxCreateUserDesktop.map(|f| f as usize)
                == other.WlxCreateUserDesktop.map(|f| f as usize)
            && self.WlxChangePasswordNotifyEx.map(|f| f as usize)
                == other.WlxChangePasswordNotifyEx.map(|f| f as usize)
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::cmp::Eq for WLX_DISPATCH_VERSION_1_1 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
unsafe impl ::windows::runtime::Abi for WLX_DISPATCH_VERSION_1_1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
pub struct WLX_DISPATCH_VERSION_1_2 {
    pub WlxUseCtrlAltDel: ::std::option::Option<PWLX_USE_CTRL_ALT_DEL>,
    pub WlxSetContextPointer: ::std::option::Option<PWLX_SET_CONTEXT_POINTER>,
    pub WlxSasNotify: ::std::option::Option<PWLX_SAS_NOTIFY>,
    pub WlxSetTimeout: ::std::option::Option<PWLX_SET_TIMEOUT>,
    pub WlxAssignShellProtection: ::std::option::Option<PWLX_ASSIGN_SHELL_PROTECTION>,
    pub WlxMessageBox: ::std::option::Option<PWLX_MESSAGE_BOX>,
    pub WlxDialogBox: ::std::option::Option<PWLX_DIALOG_BOX>,
    pub WlxDialogBoxParam: ::std::option::Option<PWLX_DIALOG_BOX_PARAM>,
    pub WlxDialogBoxIndirect: ::std::option::Option<PWLX_DIALOG_BOX_INDIRECT>,
    pub WlxDialogBoxIndirectParam: ::std::option::Option<PWLX_DIALOG_BOX_INDIRECT_PARAM>,
    pub WlxSwitchDesktopToUser: ::std::option::Option<PWLX_SWITCH_DESKTOP_TO_USER>,
    pub WlxSwitchDesktopToWinlogon: ::std::option::Option<PWLX_SWITCH_DESKTOP_TO_WINLOGON>,
    pub WlxChangePasswordNotify: ::std::option::Option<PWLX_CHANGE_PASSWORD_NOTIFY>,
    pub WlxGetSourceDesktop: ::std::option::Option<PWLX_GET_SOURCE_DESKTOP>,
    pub WlxSetReturnDesktop: ::std::option::Option<PWLX_SET_RETURN_DESKTOP>,
    pub WlxCreateUserDesktop: ::std::option::Option<PWLX_CREATE_USER_DESKTOP>,
    pub WlxChangePasswordNotifyEx: ::std::option::Option<PWLX_CHANGE_PASSWORD_NOTIFY_EX>,
    pub WlxCloseUserDesktop: ::std::option::Option<PWLX_CLOSE_USER_DESKTOP>,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl WLX_DISPATCH_VERSION_1_2 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::default::Default for WLX_DISPATCH_VERSION_1_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::fmt::Debug for WLX_DISPATCH_VERSION_1_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_DISPATCH_VERSION_1_2").finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::cmp::PartialEq for WLX_DISPATCH_VERSION_1_2 {
    fn eq(&self, other: &Self) -> bool {
        self.WlxUseCtrlAltDel.map(|f| f as usize) == other.WlxUseCtrlAltDel.map(|f| f as usize)
            && self.WlxSetContextPointer.map(|f| f as usize)
                == other.WlxSetContextPointer.map(|f| f as usize)
            && self.WlxSasNotify.map(|f| f as usize) == other.WlxSasNotify.map(|f| f as usize)
            && self.WlxSetTimeout.map(|f| f as usize) == other.WlxSetTimeout.map(|f| f as usize)
            && self.WlxAssignShellProtection.map(|f| f as usize)
                == other.WlxAssignShellProtection.map(|f| f as usize)
            && self.WlxMessageBox.map(|f| f as usize) == other.WlxMessageBox.map(|f| f as usize)
            && self.WlxDialogBox.map(|f| f as usize) == other.WlxDialogBox.map(|f| f as usize)
            && self.WlxDialogBoxParam.map(|f| f as usize)
                == other.WlxDialogBoxParam.map(|f| f as usize)
            && self.WlxDialogBoxIndirect.map(|f| f as usize)
                == other.WlxDialogBoxIndirect.map(|f| f as usize)
            && self.WlxDialogBoxIndirectParam.map(|f| f as usize)
                == other.WlxDialogBoxIndirectParam.map(|f| f as usize)
            && self.WlxSwitchDesktopToUser.map(|f| f as usize)
                == other.WlxSwitchDesktopToUser.map(|f| f as usize)
            && self.WlxSwitchDesktopToWinlogon.map(|f| f as usize)
                == other.WlxSwitchDesktopToWinlogon.map(|f| f as usize)
            && self.WlxChangePasswordNotify.map(|f| f as usize)
                == other.WlxChangePasswordNotify.map(|f| f as usize)
            && self.WlxGetSourceDesktop.map(|f| f as usize)
                == other.WlxGetSourceDesktop.map(|f| f as usize)
            && self.WlxSetReturnDesktop.map(|f| f as usize)
                == other.WlxSetReturnDesktop.map(|f| f as usize)
            && self.WlxCreateUserDesktop.map(|f| f as usize)
                == other.WlxCreateUserDesktop.map(|f| f as usize)
            && self.WlxChangePasswordNotifyEx.map(|f| f as usize)
                == other.WlxChangePasswordNotifyEx.map(|f| f as usize)
            && self.WlxCloseUserDesktop.map(|f| f as usize)
                == other.WlxCloseUserDesktop.map(|f| f as usize)
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::cmp::Eq for WLX_DISPATCH_VERSION_1_2 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
unsafe impl ::windows::runtime::Abi for WLX_DISPATCH_VERSION_1_2 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
pub struct WLX_DISPATCH_VERSION_1_3 {
    pub WlxUseCtrlAltDel: ::std::option::Option<PWLX_USE_CTRL_ALT_DEL>,
    pub WlxSetContextPointer: ::std::option::Option<PWLX_SET_CONTEXT_POINTER>,
    pub WlxSasNotify: ::std::option::Option<PWLX_SAS_NOTIFY>,
    pub WlxSetTimeout: ::std::option::Option<PWLX_SET_TIMEOUT>,
    pub WlxAssignShellProtection: ::std::option::Option<PWLX_ASSIGN_SHELL_PROTECTION>,
    pub WlxMessageBox: ::std::option::Option<PWLX_MESSAGE_BOX>,
    pub WlxDialogBox: ::std::option::Option<PWLX_DIALOG_BOX>,
    pub WlxDialogBoxParam: ::std::option::Option<PWLX_DIALOG_BOX_PARAM>,
    pub WlxDialogBoxIndirect: ::std::option::Option<PWLX_DIALOG_BOX_INDIRECT>,
    pub WlxDialogBoxIndirectParam: ::std::option::Option<PWLX_DIALOG_BOX_INDIRECT_PARAM>,
    pub WlxSwitchDesktopToUser: ::std::option::Option<PWLX_SWITCH_DESKTOP_TO_USER>,
    pub WlxSwitchDesktopToWinlogon: ::std::option::Option<PWLX_SWITCH_DESKTOP_TO_WINLOGON>,
    pub WlxChangePasswordNotify: ::std::option::Option<PWLX_CHANGE_PASSWORD_NOTIFY>,
    pub WlxGetSourceDesktop: ::std::option::Option<PWLX_GET_SOURCE_DESKTOP>,
    pub WlxSetReturnDesktop: ::std::option::Option<PWLX_SET_RETURN_DESKTOP>,
    pub WlxCreateUserDesktop: ::std::option::Option<PWLX_CREATE_USER_DESKTOP>,
    pub WlxChangePasswordNotifyEx: ::std::option::Option<PWLX_CHANGE_PASSWORD_NOTIFY_EX>,
    pub WlxCloseUserDesktop: ::std::option::Option<PWLX_CLOSE_USER_DESKTOP>,
    pub WlxSetOption: ::std::option::Option<PWLX_SET_OPTION>,
    pub WlxGetOption: ::std::option::Option<PWLX_GET_OPTION>,
    pub WlxWin31Migrate: ::std::option::Option<PWLX_WIN31_MIGRATE>,
    pub WlxQueryClientCredentials: ::std::option::Option<PWLX_QUERY_CLIENT_CREDENTIALS>,
    pub WlxQueryInetConnectorCredentials: ::std::option::Option<PWLX_QUERY_IC_CREDENTIALS>,
    pub WlxDisconnect: ::std::option::Option<PWLX_DISCONNECT>,
    pub WlxQueryTerminalServicesData: ::std::option::Option<PWLX_QUERY_TERMINAL_SERVICES_DATA>,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl WLX_DISPATCH_VERSION_1_3 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::default::Default for WLX_DISPATCH_VERSION_1_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::fmt::Debug for WLX_DISPATCH_VERSION_1_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_DISPATCH_VERSION_1_3").finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::cmp::PartialEq for WLX_DISPATCH_VERSION_1_3 {
    fn eq(&self, other: &Self) -> bool {
        self.WlxUseCtrlAltDel.map(|f| f as usize) == other.WlxUseCtrlAltDel.map(|f| f as usize)
            && self.WlxSetContextPointer.map(|f| f as usize)
                == other.WlxSetContextPointer.map(|f| f as usize)
            && self.WlxSasNotify.map(|f| f as usize) == other.WlxSasNotify.map(|f| f as usize)
            && self.WlxSetTimeout.map(|f| f as usize) == other.WlxSetTimeout.map(|f| f as usize)
            && self.WlxAssignShellProtection.map(|f| f as usize)
                == other.WlxAssignShellProtection.map(|f| f as usize)
            && self.WlxMessageBox.map(|f| f as usize) == other.WlxMessageBox.map(|f| f as usize)
            && self.WlxDialogBox.map(|f| f as usize) == other.WlxDialogBox.map(|f| f as usize)
            && self.WlxDialogBoxParam.map(|f| f as usize)
                == other.WlxDialogBoxParam.map(|f| f as usize)
            && self.WlxDialogBoxIndirect.map(|f| f as usize)
                == other.WlxDialogBoxIndirect.map(|f| f as usize)
            && self.WlxDialogBoxIndirectParam.map(|f| f as usize)
                == other.WlxDialogBoxIndirectParam.map(|f| f as usize)
            && self.WlxSwitchDesktopToUser.map(|f| f as usize)
                == other.WlxSwitchDesktopToUser.map(|f| f as usize)
            && self.WlxSwitchDesktopToWinlogon.map(|f| f as usize)
                == other.WlxSwitchDesktopToWinlogon.map(|f| f as usize)
            && self.WlxChangePasswordNotify.map(|f| f as usize)
                == other.WlxChangePasswordNotify.map(|f| f as usize)
            && self.WlxGetSourceDesktop.map(|f| f as usize)
                == other.WlxGetSourceDesktop.map(|f| f as usize)
            && self.WlxSetReturnDesktop.map(|f| f as usize)
                == other.WlxSetReturnDesktop.map(|f| f as usize)
            && self.WlxCreateUserDesktop.map(|f| f as usize)
                == other.WlxCreateUserDesktop.map(|f| f as usize)
            && self.WlxChangePasswordNotifyEx.map(|f| f as usize)
                == other.WlxChangePasswordNotifyEx.map(|f| f as usize)
            && self.WlxCloseUserDesktop.map(|f| f as usize)
                == other.WlxCloseUserDesktop.map(|f| f as usize)
            && self.WlxSetOption.map(|f| f as usize) == other.WlxSetOption.map(|f| f as usize)
            && self.WlxGetOption.map(|f| f as usize) == other.WlxGetOption.map(|f| f as usize)
            && self.WlxWin31Migrate.map(|f| f as usize) == other.WlxWin31Migrate.map(|f| f as usize)
            && self.WlxQueryClientCredentials.map(|f| f as usize)
                == other.WlxQueryClientCredentials.map(|f| f as usize)
            && self.WlxQueryInetConnectorCredentials.map(|f| f as usize)
                == other.WlxQueryInetConnectorCredentials.map(|f| f as usize)
            && self.WlxDisconnect.map(|f| f as usize) == other.WlxDisconnect.map(|f| f as usize)
            && self.WlxQueryTerminalServicesData.map(|f| f as usize)
                == other.WlxQueryTerminalServicesData.map(|f| f as usize)
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::cmp::Eq for WLX_DISPATCH_VERSION_1_3 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_UI_WindowsAndMessaging"
))]
unsafe impl ::windows::runtime::Abi for WLX_DISPATCH_VERSION_1_3 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_System_SystemServices",
    feature = "Win32_UI_WindowsAndMessaging"
))]
pub struct WLX_DISPATCH_VERSION_1_4 {
    pub WlxUseCtrlAltDel: ::std::option::Option<PWLX_USE_CTRL_ALT_DEL>,
    pub WlxSetContextPointer: ::std::option::Option<PWLX_SET_CONTEXT_POINTER>,
    pub WlxSasNotify: ::std::option::Option<PWLX_SAS_NOTIFY>,
    pub WlxSetTimeout: ::std::option::Option<PWLX_SET_TIMEOUT>,
    pub WlxAssignShellProtection: ::std::option::Option<PWLX_ASSIGN_SHELL_PROTECTION>,
    pub WlxMessageBox: ::std::option::Option<PWLX_MESSAGE_BOX>,
    pub WlxDialogBox: ::std::option::Option<PWLX_DIALOG_BOX>,
    pub WlxDialogBoxParam: ::std::option::Option<PWLX_DIALOG_BOX_PARAM>,
    pub WlxDialogBoxIndirect: ::std::option::Option<PWLX_DIALOG_BOX_INDIRECT>,
    pub WlxDialogBoxIndirectParam: ::std::option::Option<PWLX_DIALOG_BOX_INDIRECT_PARAM>,
    pub WlxSwitchDesktopToUser: ::std::option::Option<PWLX_SWITCH_DESKTOP_TO_USER>,
    pub WlxSwitchDesktopToWinlogon: ::std::option::Option<PWLX_SWITCH_DESKTOP_TO_WINLOGON>,
    pub WlxChangePasswordNotify: ::std::option::Option<PWLX_CHANGE_PASSWORD_NOTIFY>,
    pub WlxGetSourceDesktop: ::std::option::Option<PWLX_GET_SOURCE_DESKTOP>,
    pub WlxSetReturnDesktop: ::std::option::Option<PWLX_SET_RETURN_DESKTOP>,
    pub WlxCreateUserDesktop: ::std::option::Option<PWLX_CREATE_USER_DESKTOP>,
    pub WlxChangePasswordNotifyEx: ::std::option::Option<PWLX_CHANGE_PASSWORD_NOTIFY_EX>,
    pub WlxCloseUserDesktop: ::std::option::Option<PWLX_CLOSE_USER_DESKTOP>,
    pub WlxSetOption: ::std::option::Option<PWLX_SET_OPTION>,
    pub WlxGetOption: ::std::option::Option<PWLX_GET_OPTION>,
    pub WlxWin31Migrate: ::std::option::Option<PWLX_WIN31_MIGRATE>,
    pub WlxQueryClientCredentials: ::std::option::Option<PWLX_QUERY_CLIENT_CREDENTIALS>,
    pub WlxQueryInetConnectorCredentials: ::std::option::Option<PWLX_QUERY_IC_CREDENTIALS>,
    pub WlxDisconnect: ::std::option::Option<PWLX_DISCONNECT>,
    pub WlxQueryTerminalServicesData: ::std::option::Option<PWLX_QUERY_TERMINAL_SERVICES_DATA>,
    pub WlxQueryConsoleSwitchCredentials:
        ::std::option::Option<PWLX_QUERY_CONSOLESWITCH_CREDENTIALS>,
    pub WlxQueryTsLogonCredentials: ::std::option::Option<PWLX_QUERY_TS_LOGON_CREDENTIALS>,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_System_SystemServices",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl WLX_DISPATCH_VERSION_1_4 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_System_SystemServices",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::default::Default for WLX_DISPATCH_VERSION_1_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_System_SystemServices",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::fmt::Debug for WLX_DISPATCH_VERSION_1_4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_DISPATCH_VERSION_1_4").finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_System_SystemServices",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::cmp::PartialEq for WLX_DISPATCH_VERSION_1_4 {
    fn eq(&self, other: &Self) -> bool {
        self.WlxUseCtrlAltDel.map(|f| f as usize) == other.WlxUseCtrlAltDel.map(|f| f as usize)
            && self.WlxSetContextPointer.map(|f| f as usize)
                == other.WlxSetContextPointer.map(|f| f as usize)
            && self.WlxSasNotify.map(|f| f as usize) == other.WlxSasNotify.map(|f| f as usize)
            && self.WlxSetTimeout.map(|f| f as usize) == other.WlxSetTimeout.map(|f| f as usize)
            && self.WlxAssignShellProtection.map(|f| f as usize)
                == other.WlxAssignShellProtection.map(|f| f as usize)
            && self.WlxMessageBox.map(|f| f as usize) == other.WlxMessageBox.map(|f| f as usize)
            && self.WlxDialogBox.map(|f| f as usize) == other.WlxDialogBox.map(|f| f as usize)
            && self.WlxDialogBoxParam.map(|f| f as usize)
                == other.WlxDialogBoxParam.map(|f| f as usize)
            && self.WlxDialogBoxIndirect.map(|f| f as usize)
                == other.WlxDialogBoxIndirect.map(|f| f as usize)
            && self.WlxDialogBoxIndirectParam.map(|f| f as usize)
                == other.WlxDialogBoxIndirectParam.map(|f| f as usize)
            && self.WlxSwitchDesktopToUser.map(|f| f as usize)
                == other.WlxSwitchDesktopToUser.map(|f| f as usize)
            && self.WlxSwitchDesktopToWinlogon.map(|f| f as usize)
                == other.WlxSwitchDesktopToWinlogon.map(|f| f as usize)
            && self.WlxChangePasswordNotify.map(|f| f as usize)
                == other.WlxChangePasswordNotify.map(|f| f as usize)
            && self.WlxGetSourceDesktop.map(|f| f as usize)
                == other.WlxGetSourceDesktop.map(|f| f as usize)
            && self.WlxSetReturnDesktop.map(|f| f as usize)
                == other.WlxSetReturnDesktop.map(|f| f as usize)
            && self.WlxCreateUserDesktop.map(|f| f as usize)
                == other.WlxCreateUserDesktop.map(|f| f as usize)
            && self.WlxChangePasswordNotifyEx.map(|f| f as usize)
                == other.WlxChangePasswordNotifyEx.map(|f| f as usize)
            && self.WlxCloseUserDesktop.map(|f| f as usize)
                == other.WlxCloseUserDesktop.map(|f| f as usize)
            && self.WlxSetOption.map(|f| f as usize) == other.WlxSetOption.map(|f| f as usize)
            && self.WlxGetOption.map(|f| f as usize) == other.WlxGetOption.map(|f| f as usize)
            && self.WlxWin31Migrate.map(|f| f as usize) == other.WlxWin31Migrate.map(|f| f as usize)
            && self.WlxQueryClientCredentials.map(|f| f as usize)
                == other.WlxQueryClientCredentials.map(|f| f as usize)
            && self.WlxQueryInetConnectorCredentials.map(|f| f as usize)
                == other.WlxQueryInetConnectorCredentials.map(|f| f as usize)
            && self.WlxDisconnect.map(|f| f as usize) == other.WlxDisconnect.map(|f| f as usize)
            && self.WlxQueryTerminalServicesData.map(|f| f as usize)
                == other.WlxQueryTerminalServicesData.map(|f| f as usize)
            && self.WlxQueryConsoleSwitchCredentials.map(|f| f as usize)
                == other.WlxQueryConsoleSwitchCredentials.map(|f| f as usize)
            && self.WlxQueryTsLogonCredentials.map(|f| f as usize)
                == other.WlxQueryTsLogonCredentials.map(|f| f as usize)
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_System_SystemServices",
    feature = "Win32_UI_WindowsAndMessaging"
))]
impl ::std::cmp::Eq for WLX_DISPATCH_VERSION_1_4 {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops",
    feature = "Win32_System_SystemServices",
    feature = "Win32_UI_WindowsAndMessaging"
))]
unsafe impl ::windows::runtime::Abi for WLX_DISPATCH_VERSION_1_4 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const WLX_DLG_INPUT_TIMEOUT: u32 = 102u32;
pub const WLX_DLG_SAS: u32 = 101u32;
pub const WLX_DLG_SCREEN_SAVER_TIMEOUT: u32 = 103u32;
pub const WLX_DLG_USER_LOGOFF: u32 = 104u32;
pub const WLX_LOGON_OPT_NO_PROFILE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_MPR_NOTIFY_INFO {
    pub pszUserName: super::Foundation::PWSTR,
    pub pszDomain: super::Foundation::PWSTR,
    pub pszPassword: super::Foundation::PWSTR,
    pub pszOldPassword: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WLX_MPR_NOTIFY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WLX_MPR_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WLX_MPR_NOTIFY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_MPR_NOTIFY_INFO")
            .field("pszUserName", &self.pszUserName)
            .field("pszDomain", &self.pszDomain)
            .field("pszPassword", &self.pszPassword)
            .field("pszOldPassword", &self.pszOldPassword)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WLX_MPR_NOTIFY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszUserName == other.pszUserName
            && self.pszDomain == other.pszDomain
            && self.pszPassword == other.pszPassword
            && self.pszOldPassword == other.pszOldPassword
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WLX_MPR_NOTIFY_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WLX_MPR_NOTIFY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
pub struct WLX_NOTIFICATION_INFO {
    pub Size: u32,
    pub Flags: u32,
    pub UserName: super::Foundation::PWSTR,
    pub Domain: super::Foundation::PWSTR,
    pub WindowStation: super::Foundation::PWSTR,
    pub hToken: super::Foundation::HANDLE,
    pub hDesktop: super::System::StationsAndDesktops::HDESK,
    pub pStatusCallback: ::std::option::Option<PFNMSGECALLBACK>,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
impl WLX_NOTIFICATION_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
impl ::std::default::Default for WLX_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
impl ::std::fmt::Debug for WLX_NOTIFICATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_NOTIFICATION_INFO")
            .field("Size", &self.Size)
            .field("Flags", &self.Flags)
            .field("UserName", &self.UserName)
            .field("Domain", &self.Domain)
            .field("WindowStation", &self.WindowStation)
            .field("hToken", &self.hToken)
            .field("hDesktop", &self.hDesktop)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
impl ::std::cmp::PartialEq for WLX_NOTIFICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Flags == other.Flags
            && self.UserName == other.UserName
            && self.Domain == other.Domain
            && self.WindowStation == other.WindowStation
            && self.hToken == other.hToken
            && self.hDesktop == other.hDesktop
            && self.pStatusCallback.map(|f| f as usize) == other.pStatusCallback.map(|f| f as usize)
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
impl ::std::cmp::Eq for WLX_NOTIFICATION_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_StationsAndDesktops"
))]
unsafe impl ::windows::runtime::Abi for WLX_NOTIFICATION_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const WLX_OPTION_CONTEXT_POINTER: u32 = 2u32;
pub const WLX_OPTION_DISPATCH_TABLE_SIZE: u32 = 65539u32;
pub const WLX_OPTION_FORCE_LOGOFF_TIME: u32 = 4u32;
pub const WLX_OPTION_IGNORE_AUTO_LOGON: u32 = 8u32;
pub const WLX_OPTION_NO_SWITCH_ON_SAS: u32 = 9u32;
pub const WLX_OPTION_SMART_CARD_INFO: u32 = 65538u32;
pub const WLX_OPTION_SMART_CARD_PRESENT: u32 = 65537u32;
pub const WLX_OPTION_USE_CTRL_ALT_DEL: u32 = 1u32;
pub const WLX_OPTION_USE_SMART_CARD: u32 = 3u32;
pub const WLX_PROFILE_TYPE_V1_0: u32 = 1u32;
pub const WLX_PROFILE_TYPE_V2_0: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_PROFILE_V1_0 {
    pub dwType: u32,
    pub pszProfile: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WLX_PROFILE_V1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WLX_PROFILE_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WLX_PROFILE_V1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_PROFILE_V1_0")
            .field("dwType", &self.dwType)
            .field("pszProfile", &self.pszProfile)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WLX_PROFILE_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pszProfile == other.pszProfile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WLX_PROFILE_V1_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WLX_PROFILE_V1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_PROFILE_V2_0 {
    pub dwType: u32,
    pub pszProfile: super::Foundation::PWSTR,
    pub pszPolicy: super::Foundation::PWSTR,
    pub pszNetworkDefaultUserProfile: super::Foundation::PWSTR,
    pub pszServerName: super::Foundation::PWSTR,
    pub pszEnvironment: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WLX_PROFILE_V2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WLX_PROFILE_V2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WLX_PROFILE_V2_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_PROFILE_V2_0")
            .field("dwType", &self.dwType)
            .field("pszProfile", &self.pszProfile)
            .field("pszPolicy", &self.pszPolicy)
            .field(
                "pszNetworkDefaultUserProfile",
                &self.pszNetworkDefaultUserProfile,
            )
            .field("pszServerName", &self.pszServerName)
            .field("pszEnvironment", &self.pszEnvironment)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WLX_PROFILE_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.pszProfile == other.pszProfile
            && self.pszPolicy == other.pszPolicy
            && self.pszNetworkDefaultUserProfile == other.pszNetworkDefaultUserProfile
            && self.pszServerName == other.pszServerName
            && self.pszEnvironment == other.pszEnvironment
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WLX_PROFILE_V2_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WLX_PROFILE_V2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WLX_SAS_ACTION_DELAYED_FORCE_LOGOFF: u32 = 16u32;
pub const WLX_SAS_ACTION_FORCE_LOGOFF: u32 = 9u32;
pub const WLX_SAS_ACTION_LOCK_WKSTA: u32 = 3u32;
pub const WLX_SAS_ACTION_LOGOFF: u32 = 4u32;
pub const WLX_SAS_ACTION_LOGON: u32 = 1u32;
pub const WLX_SAS_ACTION_NONE: u32 = 2u32;
pub const WLX_SAS_ACTION_PWD_CHANGED: u32 = 6u32;
pub const WLX_SAS_ACTION_RECONNECTED: u32 = 15u32;
pub const WLX_SAS_ACTION_SHUTDOWN_HIBERNATE: u32 = 14u32;
pub const WLX_SAS_ACTION_SHUTDOWN_SLEEP: u32 = 12u32;
pub const WLX_SAS_ACTION_SHUTDOWN_SLEEP2: u32 = 13u32;
pub const WLX_SAS_ACTION_SWITCH_CONSOLE: u32 = 17u32;
pub const WLX_SAS_ACTION_TASKLIST: u32 = 7u32;
pub const WLX_SAS_ACTION_UNLOCK_WKSTA: u32 = 8u32;
pub const WLX_SAS_TYPE_AUTHENTICATED: u32 = 7u32;
pub const WLX_SAS_TYPE_CTRL_ALT_DEL: u32 = 1u32;
pub const WLX_SAS_TYPE_MAX_MSFT_VALUE: u32 = 127u32;
pub const WLX_SAS_TYPE_SCRNSVR_ACTIVITY: u32 = 3u32;
pub const WLX_SAS_TYPE_SCRNSVR_TIMEOUT: u32 = 2u32;
pub const WLX_SAS_TYPE_SC_FIRST_READER_ARRIVED: u32 = 8u32;
pub const WLX_SAS_TYPE_SC_INSERT: u32 = 5u32;
pub const WLX_SAS_TYPE_SC_LAST_READER_REMOVED: u32 = 9u32;
pub const WLX_SAS_TYPE_SC_REMOVE: u32 = 6u32;
pub const WLX_SAS_TYPE_SWITCHUSER: u32 = 10u32;
pub const WLX_SAS_TYPE_TIMEOUT: u32 = 0u32;
pub const WLX_SAS_TYPE_USER_LOGOFF: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_SC_NOTIFICATION_INFO {
    pub pszCard: super::Foundation::PWSTR,
    pub pszReader: super::Foundation::PWSTR,
    pub pszContainer: super::Foundation::PWSTR,
    pub pszCryptoProvider: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WLX_SC_NOTIFICATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WLX_SC_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WLX_SC_NOTIFICATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_SC_NOTIFICATION_INFO")
            .field("pszCard", &self.pszCard)
            .field("pszReader", &self.pszReader)
            .field("pszContainer", &self.pszContainer)
            .field("pszCryptoProvider", &self.pszCryptoProvider)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WLX_SC_NOTIFICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszCard == other.pszCard
            && self.pszReader == other.pszReader
            && self.pszContainer == other.pszContainer
            && self.pszCryptoProvider == other.pszCryptoProvider
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WLX_SC_NOTIFICATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WLX_SC_NOTIFICATION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WLX_SHUTDOWN_TYPE(pub u32);
pub const WLX_SAS_ACTION_SHUTDOWN: WLX_SHUTDOWN_TYPE = WLX_SHUTDOWN_TYPE(5u32);
pub const WLX_SAS_ACTION_SHUTDOWN_REBOOT: WLX_SHUTDOWN_TYPE = WLX_SHUTDOWN_TYPE(11u32);
pub const WLX_SAS_ACTION_SHUTDOWN_POWER_OFF: WLX_SHUTDOWN_TYPE = WLX_SHUTDOWN_TYPE(10u32);
impl ::std::convert::From<u32> for WLX_SHUTDOWN_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WLX_SHUTDOWN_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WLX_SHUTDOWN_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WLX_SHUTDOWN_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WLX_SHUTDOWN_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WLX_SHUTDOWN_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WLX_SHUTDOWN_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WLX_TERMINAL_SERVICES_DATA {
    pub ProfilePath: [u16; 257],
    pub HomeDir: [u16; 257],
    pub HomeDirDrive: [u16; 4],
}
impl WLX_TERMINAL_SERVICES_DATA {}
impl ::std::default::Default for WLX_TERMINAL_SERVICES_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WLX_TERMINAL_SERVICES_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLX_TERMINAL_SERVICES_DATA")
            .field("ProfilePath", &self.ProfilePath)
            .field("HomeDir", &self.HomeDir)
            .field("HomeDirDrive", &self.HomeDirDrive)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WLX_TERMINAL_SERVICES_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ProfilePath == other.ProfilePath
            && self.HomeDir == other.HomeDir
            && self.HomeDirDrive == other.HomeDirDrive
    }
}
impl ::std::cmp::Eq for WLX_TERMINAL_SERVICES_DATA {}
unsafe impl ::windows::runtime::Abi for WLX_TERMINAL_SERVICES_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WLX_VERSION_1_0: u32 = 65536u32;
pub const WLX_VERSION_1_1: u32 = 65537u32;
pub const WLX_VERSION_1_2: u32 = 65538u32;
pub const WLX_VERSION_1_3: u32 = 65539u32;
pub const WLX_VERSION_1_4: u32 = 65540u32;
pub const WLX_WM_SAS: u32 = 1625u32;
pub const WSS_CERTTRUST_SUPPORT: u32 = 4u32;
pub const WSS_INPUT_FLAG_MASK: u32 = 7u32;
pub const WSS_OBJTRUST_SUPPORT: u32 = 1u32;
pub const WSS_OUTPUT_FLAG_MASK: u32 = 3758096384u32;
pub const WSS_OUT_FILE_SUPPORTS_SEAL: u32 = 536870912u32;
pub const WSS_OUT_HAS_SEALING_INTENT: u32 = 1073741824u32;
pub const WSS_OUT_SEALING_STATUS_VERIFIED: u32 = 2147483648u32;
pub const WSS_SIGTRUST_SUPPORT: u32 = 2u32;
pub const WSS_VERIFY_SEALING: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    pub Anonymous: WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0,
    pub hChainEngine: Cryptography::Core::HCERTCHAINENGINE,
    pub pChainPara: *mut Cryptography::Core::CERT_CHAIN_PARA,
    pub dwFlags: u32,
    pub pvReserved: *mut ::std::ffi::c_void,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
impl WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {}
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {}
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::clone::Clone for WTD_GENERIC_CHAIN_POLICY_DATA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct WTD_GENERIC_CHAIN_POLICY_DATA {
    pub Anonymous: WTD_GENERIC_CHAIN_POLICY_DATA_0,
    pub pSignerChainInfo: *mut WTD_GENERIC_CHAIN_POLICY_CREATE_INFO,
    pub pCounterSignerChainInfo: *mut WTD_GENERIC_CHAIN_POLICY_CREATE_INFO,
    pub pfnPolicyCallback: ::std::option::Option<PFN_WTD_GENERIC_CHAIN_POLICY_CALLBACK>,
    pub pvPolicyArg: *mut ::std::ffi::c_void,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl WTD_GENERIC_CHAIN_POLICY_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_DATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
impl WTD_GENERIC_CHAIN_POLICY_DATA_0 {}
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_DATA_0 {}
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub struct WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    pub Anonymous: WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0,
    pub pChainContext: *mut Cryptography::Core::CERT_CHAIN_CONTEXT,
    pub dwSignerType: u32,
    pub pMsgSignerInfo: *mut Cryptography::Core::CMSG_SIGNER_INFO,
    pub dwError: u32,
    pub cCounterSigner: u32,
    pub rgpCounterSigner: *mut *mut WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
impl WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {}
impl ::std::default::Default for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {}
unsafe impl ::windows::runtime::Abi for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn WTHelperCertCheckValidSignature(
    pprovdata: *mut CRYPT_PROVIDER_DATA,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperCertCheckValidSignature(
                pprovdata: *mut CRYPT_PROVIDER_DATA,
            ) -> ::windows::runtime::HRESULT;
        }
        WTHelperCertCheckValidSignature(::std::mem::transmute(pprovdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn WTHelperCertIsSelfSigned(
    dwencoding: u32,
    pcert: *mut Cryptography::Core::CERT_INFO,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperCertIsSelfSigned(
                dwencoding: u32,
                pcert: *mut Cryptography::Core::CERT_INFO,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(WTHelperCertIsSelfSigned(
            ::std::mem::transmute(dwencoding),
            ::std::mem::transmute(pcert),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn WTHelperGetProvCertFromChain(
    psgnr: *mut CRYPT_PROVIDER_SGNR,
    idxcert: u32,
) -> *mut CRYPT_PROVIDER_CERT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperGetProvCertFromChain(
                psgnr: *mut CRYPT_PROVIDER_SGNR,
                idxcert: u32,
            ) -> *mut CRYPT_PROVIDER_CERT;
        }
        ::std::mem::transmute(WTHelperGetProvCertFromChain(
            ::std::mem::transmute(psgnr),
            ::std::mem::transmute(idxcert),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn WTHelperGetProvPrivateDataFromChain(
    pprovdata: *mut CRYPT_PROVIDER_DATA,
    pgproviderid: *mut ::windows::runtime::GUID,
) -> *mut CRYPT_PROVIDER_PRIVDATA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperGetProvPrivateDataFromChain(
                pprovdata: *mut CRYPT_PROVIDER_DATA,
                pgproviderid: *mut ::windows::runtime::GUID,
            ) -> *mut CRYPT_PROVIDER_PRIVDATA;
        }
        ::std::mem::transmute(WTHelperGetProvPrivateDataFromChain(
            ::std::mem::transmute(pprovdata),
            ::std::mem::transmute(pgproviderid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn WTHelperGetProvSignerFromChain<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    pprovdata: *mut CRYPT_PROVIDER_DATA,
    idxsigner: u32,
    fcountersigner: Param2,
    idxcountersigner: u32,
) -> *mut CRYPT_PROVIDER_SGNR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperGetProvSignerFromChain(
                pprovdata: *mut CRYPT_PROVIDER_DATA,
                idxsigner: u32,
                fcountersigner: super::Foundation::BOOL,
                idxcountersigner: u32,
            ) -> *mut CRYPT_PROVIDER_SGNR;
        }
        ::std::mem::transmute(WTHelperGetProvSignerFromChain(
            ::std::mem::transmute(pprovdata),
            ::std::mem::transmute(idxsigner),
            fcountersigner.into_param().abi(),
            ::std::mem::transmute(idxcountersigner),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn WTHelperProvDataFromStateData<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HANDLE>,
>(
    hstatedata: Param0,
) -> *mut CRYPT_PROVIDER_DATA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WTHelperProvDataFromStateData(
                hstatedata: super::Foundation::HANDLE,
            ) -> *mut CRYPT_PROVIDER_DATA;
        }
        ::std::mem::transmute(WTHelperProvDataFromStateData(hstatedata.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WT_ADD_ACTION_ID_RET_RESULT_FLAG: u32 = 1u32;
pub const WT_CURRENT_VERSION: u32 = 512u32;
pub const WT_TRUSTDBDIALOG_NO_UI_FLAG: u32 = 1u32;
pub const WT_TRUSTDBDIALOG_ONLY_PUB_TAB_FLAG: u32 = 2u32;
pub const WT_TRUSTDBDIALOG_WRITE_IEAK_STORE_FLAG: u32 = 512u32;
pub const WT_TRUSTDBDIALOG_WRITE_LEGACY_REG_FLAG: u32 = 256u32;
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WinVerifyTrust<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HWND>,
>(
    hwnd: Param0,
    pgactionid: *mut ::windows::runtime::GUID,
    pwvtdata: *mut ::std::ffi::c_void,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinVerifyTrust(
                hwnd: super::Foundation::HWND,
                pgactionid: *mut ::windows::runtime::GUID,
                pwvtdata: *mut ::std::ffi::c_void,
            ) -> i32;
        }
        ::std::mem::transmute(WinVerifyTrust(
            hwnd.into_param().abi(),
            ::std::mem::transmute(pgactionid),
            ::std::mem::transmute(pwvtdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn WinVerifyTrustEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::HWND>,
>(
    hwnd: Param0,
    pgactionid: *mut ::windows::runtime::GUID,
    pwintrustdata: *mut WINTRUST_DATA,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinVerifyTrustEx(
                hwnd: super::Foundation::HWND,
                pgactionid: *mut ::windows::runtime::GUID,
                pwintrustdata: *mut WINTRUST_DATA,
            ) -> i32;
        }
        ::std::mem::transmute(WinVerifyTrustEx(
            hwnd.into_param().abi(),
            ::std::mem::transmute(pgactionid),
            ::std::mem::transmute(pwintrustdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WintrustAddActionID(
    pgactionid: *const ::windows::runtime::GUID,
    fdwflags: u32,
    psprovinfo: *const CRYPT_REGISTER_ACTIONID,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustAddActionID(
                pgactionid: *const ::windows::runtime::GUID,
                fdwflags: u32,
                psprovinfo: *const CRYPT_REGISTER_ACTIONID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustAddActionID(
            ::std::mem::transmute(pgactionid),
            ::std::mem::transmute(fdwflags),
            ::std::mem::transmute(psprovinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WintrustAddDefaultForUsage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
>(
    pszusageoid: Param0,
    psdefusage: *const CRYPT_PROVIDER_REGDEFUSAGE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustAddDefaultForUsage(
                pszusageoid: super::Foundation::PSTR,
                psdefusage: *const CRYPT_PROVIDER_REGDEFUSAGE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustAddDefaultForUsage(
            pszusageoid.into_param().abi(),
            ::std::mem::transmute(psdefusage),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WintrustGetDefaultForUsage<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::Foundation::PSTR>,
>(
    dwaction: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION,
    pszusageoid: Param1,
    psusage: *mut CRYPT_PROVIDER_DEFUSAGE,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustGetDefaultForUsage(
                dwaction: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION,
                pszusageoid: super::Foundation::PSTR,
                psusage: *mut CRYPT_PROVIDER_DEFUSAGE,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustGetDefaultForUsage(
            ::std::mem::transmute(dwaction),
            pszusageoid.into_param().abi(),
            ::std::mem::transmute(psusage),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn WintrustGetRegPolicyFlags(pdwpolicyflags: *mut WINTRUST_POLICY_FLAGS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustGetRegPolicyFlags(pdwpolicyflags: *mut WINTRUST_POLICY_FLAGS);
        }
        ::std::mem::transmute(WintrustGetRegPolicyFlags(::std::mem::transmute(
            pdwpolicyflags,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub unsafe fn WintrustLoadFunctionPointers(
    pgactionid: *mut ::windows::runtime::GUID,
    ppfns: *mut CRYPT_PROVIDER_FUNCTIONS,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustLoadFunctionPointers(
                pgactionid: *mut ::windows::runtime::GUID,
                ppfns: *mut ::std::mem::ManuallyDrop<CRYPT_PROVIDER_FUNCTIONS>,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustLoadFunctionPointers(
            ::std::mem::transmute(pgactionid),
            ::std::mem::transmute(ppfns),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WintrustRemoveActionID(
    pgactionid: *const ::windows::runtime::GUID,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustRemoveActionID(
                pgactionid: *const ::windows::runtime::GUID,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustRemoveActionID(::std::mem::transmute(pgactionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WintrustSetDefaultIncludePEPageHashes<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Foundation::BOOL>,
>(
    fincludepepagehashes: Param0,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustSetDefaultIncludePEPageHashes(fincludepepagehashes: super::Foundation::BOOL);
        }
        ::std::mem::transmute(WintrustSetDefaultIncludePEPageHashes(
            fincludepepagehashes.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WintrustSetRegPolicyFlags(
    dwpolicyflags: WINTRUST_POLICY_FLAGS,
) -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WintrustSetRegPolicyFlags(
                dwpolicyflags: WINTRUST_POLICY_FLAGS,
            ) -> super::Foundation::BOOL;
        }
        ::std::mem::transmute(WintrustSetRegPolicyFlags(::std::mem::transmute(
            dwpolicyflags,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const cNodetypeSceAnalysisServices: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1736462535,
        8184,
        4561,
        [175, 251, 0, 192, 79, 185, 132, 249],
    );
pub const cNodetypeSceEventLog: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    752903832,
    19443,
    4561,
    [140, 48, 0, 192, 79, 185, 132, 249],
);
pub const cNodetypeSceTemplateServices: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        614987543,
        7948,
        4561,
        [175, 251, 0, 192, 79, 185, 132, 249],
    );
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type pCryptSIPCreateIndirectData = unsafe extern "system" fn(
    psubjectinfo: *mut SIP_SUBJECTINFO,
    pcbindirectdata: *mut u32,
    pindirectdata: *mut SIP_INDIRECT_DATA,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type pCryptSIPGetCaps = unsafe extern "system" fn(
    psubjinfo: *const SIP_SUBJECTINFO,
    pcaps: *mut SIP_CAP_SET_V3,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type pCryptSIPGetSealedDigest = unsafe extern "system" fn(
    psubjectinfo: *const SIP_SUBJECTINFO,
    psig: *const u8,
    dwsig: u32,
    pbdigest: *mut u8,
    pcbdigest: *mut u32,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type pCryptSIPGetSignedDataMsg = unsafe extern "system" fn(
    psubjectinfo: *mut SIP_SUBJECTINFO,
    pdwencodingtype: *mut u32,
    dwindex: u32,
    pcbsigneddatamsg: *mut u32,
    pbsigneddatamsg: *mut u8,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type pCryptSIPPutSignedDataMsg = unsafe extern "system" fn(
    psubjectinfo: *mut SIP_SUBJECTINFO,
    dwencodingtype: u32,
    pdwindex: *mut u32,
    cbsigneddatamsg: u32,
    pbsigneddatamsg: *mut u8,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type pCryptSIPRemoveSignedDataMsg = unsafe extern "system" fn(
    psubjectinfo: *mut SIP_SUBJECTINFO,
    dwindex: u32,
) -> super::Foundation::BOOL;
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security_Cryptography_Core"
))]
pub type pCryptSIPVerifyIndirectData = unsafe extern "system" fn(
    psubjectinfo: *mut SIP_SUBJECTINFO,
    pindirectdata: *mut SIP_INDIRECT_DATA,
) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type pfnIsFileSupported = unsafe extern "system" fn(
    hfile: super::Foundation::HANDLE,
    pgsubject: *mut ::windows::runtime::GUID,
) -> super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type pfnIsFileSupportedName = unsafe extern "system" fn(
    pwszfilename: super::Foundation::PWSTR,
    pgsubject: *mut ::windows::runtime::GUID,
) -> super::Foundation::BOOL;
