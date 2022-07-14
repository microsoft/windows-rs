#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_CODE_PROPERTIES_V1 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: ::windows::core::PCWSTR,
    pub hImageFileHandle: super::super::Foundation::HANDLE,
    pub UrlZoneId: u32,
    pub ImageHash: [u8; 64],
    pub dwImageHashSize: u32,
    pub ImageSize: i64,
    pub HashAlgorithm: u32,
    pub pByteBlock: *mut u8,
    pub hWndParent: super::super::Foundation::HWND,
    pub dwWVTUIChoice: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SAFER_CODE_PROPERTIES_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SAFER_CODE_PROPERTIES_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_CODE_PROPERTIES_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFER_CODE_PROPERTIES_V1")
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
unsafe impl ::windows::core::Abi for SAFER_CODE_PROPERTIES_V1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAFER_CODE_PROPERTIES_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFER_CODE_PROPERTIES_V1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_CODE_PROPERTIES_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_CODE_PROPERTIES_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_CODE_PROPERTIES_V2 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: ::windows::core::PCWSTR,
    pub hImageFileHandle: super::super::Foundation::HANDLE,
    pub UrlZoneId: u32,
    pub ImageHash: [u8; 64],
    pub dwImageHashSize: u32,
    pub ImageSize: i64,
    pub HashAlgorithm: u32,
    pub pByteBlock: *mut u8,
    pub hWndParent: super::super::Foundation::HWND,
    pub dwWVTUIChoice: u32,
    pub PackageMoniker: ::windows::core::PCWSTR,
    pub PackagePublisher: ::windows::core::PCWSTR,
    pub PackageName: ::windows::core::PCWSTR,
    pub PackageVersion: u64,
    pub PackageIsFramework: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SAFER_CODE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SAFER_CODE_PROPERTIES_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_CODE_PROPERTIES_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFER_CODE_PROPERTIES_V2")
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
unsafe impl ::windows::core::Abi for SAFER_CODE_PROPERTIES_V2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAFER_CODE_PROPERTIES_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFER_CODE_PROPERTIES_V2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_CODE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_CODE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_TOKEN_NULL_IF_EQUAL: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_TOKEN_COMPARE_ONLY: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_TOKEN_MAKE_INERT: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_TOKEN_WANT_FLAGS: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(8u32);
impl ::core::marker::Copy for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {}
impl ::core::clone::Clone for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_CRITERIA_APPX_PACKAGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_CRITERIA_AUTHENTICODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_CRITERIA_IMAGEHASH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_CRITERIA_IMAGEPATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_CRITERIA_IMAGEPATH_NT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_CRITERIA_NOSIGNEDHASH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_CRITERIA_URLZONE: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for SAFER_HASH_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SAFER_HASH_IDENTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_HASH_IDENTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFER_HASH_IDENTIFICATION").field("header", &self.header).field("Description", &self.Description).field("FriendlyName", &self.FriendlyName).field("HashSize", &self.HashSize).field("ImageHash", &self.ImageHash).field("HashAlgorithm", &self.HashAlgorithm).field("ImageSize", &self.ImageSize).field("dwSaferFlags", &self.dwSaferFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_HASH_IDENTIFICATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAFER_HASH_IDENTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFER_HASH_IDENTIFICATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_HASH_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_HASH_IDENTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_HASH_IDENTIFICATION2 {
    pub hashIdentification: SAFER_HASH_IDENTIFICATION,
    pub HashSize: u32,
    pub ImageHash: [u8; 64],
    pub HashAlgorithm: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SAFER_HASH_IDENTIFICATION2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SAFER_HASH_IDENTIFICATION2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_HASH_IDENTIFICATION2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFER_HASH_IDENTIFICATION2").field("hashIdentification", &self.hashIdentification).field("HashSize", &self.HashSize).field("ImageHash", &self.ImageHash).field("HashAlgorithm", &self.HashAlgorithm).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_HASH_IDENTIFICATION2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAFER_HASH_IDENTIFICATION2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFER_HASH_IDENTIFICATION2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_HASH_IDENTIFICATION2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_HASH_IDENTIFICATION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_IDENTIFICATION_HEADER {
    pub dwIdentificationType: SAFER_IDENTIFICATION_TYPES,
    pub cbStructSize: u32,
    pub IdentificationGuid: ::windows::core::GUID,
    pub lastModified: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SAFER_IDENTIFICATION_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SAFER_IDENTIFICATION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_IDENTIFICATION_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFER_IDENTIFICATION_HEADER").field("dwIdentificationType", &self.dwIdentificationType).field("cbStructSize", &self.cbStructSize).field("IdentificationGuid", &self.IdentificationGuid).field("lastModified", &self.lastModified).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_IDENTIFICATION_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAFER_IDENTIFICATION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFER_IDENTIFICATION_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_IDENTIFICATION_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_IDENTIFICATION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SAFER_IDENTIFICATION_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferIdentityDefault: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferIdentityTypeImageName: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferIdentityTypeImageHash: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferIdentityTypeUrlZone: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(3i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferIdentityTypeCertificate: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(4i32);
impl ::core::marker::Copy for SAFER_IDENTIFICATION_TYPES {}
impl ::core::clone::Clone for SAFER_IDENTIFICATION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SAFER_IDENTIFICATION_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SAFER_IDENTIFICATION_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for SAFER_IDENTIFICATION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SAFER_IDENTIFICATION_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_LEVELID_CONSTRAINED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_LEVELID_DISALLOWED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_LEVELID_FULLYTRUSTED: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_LEVELID_NORMALUSER: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_LEVELID_UNTRUSTED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_LEVEL_OPEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_MAX_DESCRIPTION_SIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_MAX_FRIENDLYNAME_SIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_MAX_HASH_SIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SAFER_OBJECT_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectLevelId: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectScopeId: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectFriendlyName: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectDescription: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectBuiltin: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectDisallowed: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectDisableMaxPrivilege: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(7i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectInvertDeletedPrivileges: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(8i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectDeletedPrivileges: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(9i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectDefaultOwner: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(10i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectSidsToDisable: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(11i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectRestrictedSidsInverted: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(12i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectRestrictedSidsAdded: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(13i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectAllIdentificationGuids: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(14i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectSingleIdentification: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(15i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferObjectExtendedError: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(16i32);
impl ::core::marker::Copy for SAFER_OBJECT_INFO_CLASS {}
impl ::core::clone::Clone for SAFER_OBJECT_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SAFER_OBJECT_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SAFER_OBJECT_INFO_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SAFER_OBJECT_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SAFER_OBJECT_INFO_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_PATHNAME_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub Description: [u16; 256],
    pub ImageName: ::windows::core::PWSTR,
    pub dwSaferFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SAFER_PATHNAME_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SAFER_PATHNAME_IDENTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_PATHNAME_IDENTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFER_PATHNAME_IDENTIFICATION").field("header", &self.header).field("Description", &self.Description).field("ImageName", &self.ImageName).field("dwSaferFlags", &self.dwSaferFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_PATHNAME_IDENTIFICATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAFER_PATHNAME_IDENTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFER_PATHNAME_IDENTIFICATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_PATHNAME_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_PATHNAME_IDENTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_BLOCK_CLIENT_UI: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_HASH_DUPLICATE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SAFER_POLICY_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferPolicyLevelList: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferPolicyEnableTransparentEnforcement: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferPolicyDefaultLevel: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferPolicyEvaluateUserScope: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferPolicyScopeFlags: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferPolicyDefaultLevelFlags: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SaferPolicyAuthenticodeEnabled: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(7i32);
impl ::core::marker::Copy for SAFER_POLICY_INFO_CLASS {}
impl ::core::clone::Clone for SAFER_POLICY_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SAFER_POLICY_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SAFER_POLICY_INFO_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SAFER_POLICY_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SAFER_POLICY_INFO_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_JOBID_CONSTRAINED: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_JOBID_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_JOBID_UNTRUSTED: u32 = 50331648u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_ONLY_AUDIT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_ONLY_EXES: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_SANDBOX_INERT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_UIFLAGS_HIDDEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_UIFLAGS_INFORMATION_PROMPT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_UIFLAGS_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_POLICY_UIFLAGS_OPTION_PROMPT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_SCOPEID_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SAFER_SCOPEID_USER: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SAFER_URLZONE_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub UrlZoneId: u32,
    pub dwSaferFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SAFER_URLZONE_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SAFER_URLZONE_IDENTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_URLZONE_IDENTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFER_URLZONE_IDENTIFICATION").field("header", &self.header).field("UrlZoneId", &self.UrlZoneId).field("dwSaferFlags", &self.dwSaferFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_URLZONE_IDENTIFICATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAFER_URLZONE_IDENTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFER_URLZONE_IDENTIFICATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_URLZONE_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_URLZONE_IDENTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SRP_POLICY_APPX: &str = "APPX";
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SRP_POLICY_DLL: &str = "DLL";
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SRP_POLICY_EXE: &str = "EXE";
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SRP_POLICY_MANAGEDINSTALLER: &str = "MANAGEDINSTALLER";
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SRP_POLICY_MSI: &str = "MSI";
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SRP_POLICY_NOV2: &str = "IGNORESRPV2";
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SRP_POLICY_SCRIPT: &str = "SCRIPT";
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SRP_POLICY_SHELL: &str = "SHELL";
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SRP_POLICY_WLDPCONFIGCI: &str = "WLDPCONFIGCI";
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SRP_POLICY_WLDPMSI: &str = "WLDPMSI";
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`*"]
pub const SRP_POLICY_WLDPSCRIPT: &str = "WLDPSCRIPT";
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferCloseLevel<'a, P0>(hlevelhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::SAFER_LEVEL_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SaferCloseLevel(hlevelhandle: super::SAFER_LEVEL_HANDLE) -> super::super::Foundation::BOOL;
    }
    SaferCloseLevel(hlevelhandle.into())
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferComputeTokenFromLevel<'a, P0, P1>(levelhandle: P0, inaccesstoken: P1, outaccesstoken: *mut super::super::Foundation::HANDLE, dwflags: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::SAFER_LEVEL_HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SaferComputeTokenFromLevel(levelhandle: super::SAFER_LEVEL_HANDLE, inaccesstoken: super::super::Foundation::HANDLE, outaccesstoken: *mut super::super::Foundation::HANDLE, dwflags: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    SaferComputeTokenFromLevel(levelhandle.into(), inaccesstoken.into(), ::core::mem::transmute(outaccesstoken), dwflags, ::core::mem::transmute(lpreserved))
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferCreateLevel(dwscopeid: u32, dwlevelid: u32, openflags: u32, plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SaferCreateLevel(dwscopeid: u32, dwlevelid: u32, openflags: u32, plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    SaferCreateLevel(dwscopeid, dwlevelid, openflags, ::core::mem::transmute(plevelhandle), ::core::mem::transmute(lpreserved))
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferGetLevelInformation<'a, P0>(levelhandle: P0, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *mut ::core::ffi::c_void, dwinbuffersize: u32, lpdwoutbuffersize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::SAFER_LEVEL_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SaferGetLevelInformation(levelhandle: super::SAFER_LEVEL_HANDLE, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *mut ::core::ffi::c_void, dwinbuffersize: u32, lpdwoutbuffersize: *mut u32) -> super::super::Foundation::BOOL;
    }
    SaferGetLevelInformation(levelhandle.into(), dwinfotype, ::core::mem::transmute(lpquerybuffer), dwinbuffersize, ::core::mem::transmute(lpdwoutbuffersize))
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferGetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *mut ::core::ffi::c_void, infobufferretsize: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SaferGetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *mut ::core::ffi::c_void, infobufferretsize: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    SaferGetPolicyInformation(dwscopeid, saferpolicyinfoclass, infobuffersize, ::core::mem::transmute(infobuffer), ::core::mem::transmute(infobufferretsize), ::core::mem::transmute(lpreserved))
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferIdentifyLevel(pcodeproperties: &[SAFER_CODE_PROPERTIES_V2], plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SaferIdentifyLevel(dwnumproperties: u32, pcodeproperties: *const SAFER_CODE_PROPERTIES_V2, plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    SaferIdentifyLevel(pcodeproperties.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pcodeproperties)), ::core::mem::transmute(plevelhandle), ::core::mem::transmute(lpreserved))
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferRecordEventLogEntry<'a, P0, P1>(hlevel: P0, sztargetpath: P1, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::SAFER_LEVEL_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SaferRecordEventLogEntry(hlevel: super::SAFER_LEVEL_HANDLE, sztargetpath: ::windows::core::PCWSTR, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    SaferRecordEventLogEntry(hlevel.into(), sztargetpath.into(), ::core::mem::transmute(lpreserved))
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferSetLevelInformation<'a, P0>(levelhandle: P0, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *const ::core::ffi::c_void, dwinbuffersize: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::SAFER_LEVEL_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SaferSetLevelInformation(levelhandle: super::SAFER_LEVEL_HANDLE, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *const ::core::ffi::c_void, dwinbuffersize: u32) -> super::super::Foundation::BOOL;
    }
    SaferSetLevelInformation(levelhandle.into(), dwinfotype, ::core::mem::transmute(lpquerybuffer), dwinbuffersize)
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferSetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *const ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SaferSetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *const ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    SaferSetPolicyInformation(dwscopeid, saferpolicyinfoclass, infobuffersize, ::core::mem::transmute(infobuffer), ::core::mem::transmute(lpreserved))
}
#[doc = "*Required features: `\"Win32_Security_AppLocker\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferiIsExecutableFileType<'a, P0, P1>(szfullpathname: P0, bfromshellexecute: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SaferiIsExecutableFileType(szfullpathname: ::windows::core::PCWSTR, bfromshellexecute: super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOL;
    }
    SaferiIsExecutableFileType(szfullpathname.into(), bfromshellexecute.into())
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
