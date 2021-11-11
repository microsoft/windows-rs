#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
pub struct SAFER_CODE_PROPERTIES_V1 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: super::super::Foundation::PWSTR,
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
impl SAFER_CODE_PROPERTIES_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_CODE_PROPERTIES_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_CODE_PROPERTIES_V1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for SAFER_CODE_PROPERTIES_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwCheckFlags == other.dwCheckFlags && self.ImagePath == other.ImagePath && self.hImageFileHandle == other.hImageFileHandle && self.UrlZoneId == other.UrlZoneId && self.ImageHash == other.ImageHash && self.dwImageHashSize == other.dwImageHashSize && self.ImageSize == other.ImageSize && self.HashAlgorithm == other.HashAlgorithm && self.pByteBlock == other.pByteBlock && self.hWndParent == other.hWndParent && self.dwWVTUIChoice == other.dwWVTUIChoice
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_CODE_PROPERTIES_V1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_CODE_PROPERTIES_V1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
pub struct SAFER_CODE_PROPERTIES_V2 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: super::super::Foundation::PWSTR,
    pub hImageFileHandle: super::super::Foundation::HANDLE,
    pub UrlZoneId: u32,
    pub ImageHash: [u8; 64],
    pub dwImageHashSize: u32,
    pub ImageSize: i64,
    pub HashAlgorithm: u32,
    pub pByteBlock: *mut u8,
    pub hWndParent: super::super::Foundation::HWND,
    pub dwWVTUIChoice: u32,
    pub PackageMoniker: super::super::Foundation::PWSTR,
    pub PackagePublisher: super::super::Foundation::PWSTR,
    pub PackageName: super::super::Foundation::PWSTR,
    pub PackageVersion: u64,
    pub PackageIsFramework: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_CODE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_CODE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_CODE_PROPERTIES_V2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for SAFER_CODE_PROPERTIES_V2 {
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
impl ::core::cmp::Eq for SAFER_CODE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_CODE_PROPERTIES_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(pub u32);
pub const SAFER_TOKEN_NULL_IF_EQUAL: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(1u32);
pub const SAFER_TOKEN_COMPARE_ONLY: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(2u32);
pub const SAFER_TOKEN_MAKE_INERT: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(4u32);
pub const SAFER_TOKEN_WANT_FLAGS: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS = SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS(8u32);
impl ::core::convert::From<u32> for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_APPX_PACKAGE: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_AUTHENTICODE: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_IMAGEHASH: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_IMAGEPATH: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_IMAGEPATH_NT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_NOSIGNEDHASH: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_CRITERIA_URLZONE: u32 = 16u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
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
impl ::core::default::Default for SAFER_HASH_IDENTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_HASH_IDENTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for SAFER_HASH_IDENTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.Description == other.Description && self.FriendlyName == other.FriendlyName && self.HashSize == other.HashSize && self.ImageHash == other.ImageHash && self.HashAlgorithm == other.HashAlgorithm && self.ImageSize == other.ImageSize && self.dwSaferFlags == other.dwSaferFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_HASH_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_HASH_IDENTIFICATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
pub struct SAFER_HASH_IDENTIFICATION2 {
    pub hashIdentification: SAFER_HASH_IDENTIFICATION,
    pub HashSize: u32,
    pub ImageHash: [u8; 64],
    pub HashAlgorithm: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_HASH_IDENTIFICATION2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_HASH_IDENTIFICATION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_HASH_IDENTIFICATION2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SAFER_HASH_IDENTIFICATION2").field("hashIdentification", &self.hashIdentification).field("HashSize", &self.HashSize).field("ImageHash", &self.ImageHash).field("HashAlgorithm", &self.HashAlgorithm).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAFER_HASH_IDENTIFICATION2 {
    fn eq(&self, other: &Self) -> bool {
        self.hashIdentification == other.hashIdentification && self.HashSize == other.HashSize && self.ImageHash == other.ImageHash && self.HashAlgorithm == other.HashAlgorithm
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_HASH_IDENTIFICATION2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_HASH_IDENTIFICATION2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
pub struct SAFER_IDENTIFICATION_HEADER {
    pub dwIdentificationType: SAFER_IDENTIFICATION_TYPES,
    pub cbStructSize: u32,
    pub IdentificationGuid: ::windows::core::GUID,
    pub lastModified: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_IDENTIFICATION_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_IDENTIFICATION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_IDENTIFICATION_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SAFER_IDENTIFICATION_HEADER").field("dwIdentificationType", &self.dwIdentificationType).field("cbStructSize", &self.cbStructSize).field("IdentificationGuid", &self.IdentificationGuid).field("lastModified", &self.lastModified).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAFER_IDENTIFICATION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdentificationType == other.dwIdentificationType && self.cbStructSize == other.cbStructSize && self.IdentificationGuid == other.IdentificationGuid && self.lastModified == other.lastModified
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_IDENTIFICATION_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_IDENTIFICATION_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SAFER_IDENTIFICATION_TYPES(pub i32);
pub const SaferIdentityDefault: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(0i32);
pub const SaferIdentityTypeImageName: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(1i32);
pub const SaferIdentityTypeImageHash: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(2i32);
pub const SaferIdentityTypeUrlZone: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(3i32);
pub const SaferIdentityTypeCertificate: SAFER_IDENTIFICATION_TYPES = SAFER_IDENTIFICATION_TYPES(4i32);
impl ::core::convert::From<i32> for SAFER_IDENTIFICATION_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SAFER_IDENTIFICATION_TYPES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVELID_CONSTRAINED: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVELID_DISALLOWED: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVELID_FULLYTRUSTED: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVELID_NORMALUSER: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVELID_UNTRUSTED: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_LEVEL_OPEN: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_MAX_DESCRIPTION_SIZE: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_MAX_FRIENDLYNAME_SIZE: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_MAX_HASH_SIZE: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SAFER_OBJECT_INFO_CLASS(pub i32);
pub const SaferObjectLevelId: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(1i32);
pub const SaferObjectScopeId: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(2i32);
pub const SaferObjectFriendlyName: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(3i32);
pub const SaferObjectDescription: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(4i32);
pub const SaferObjectBuiltin: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(5i32);
pub const SaferObjectDisallowed: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(6i32);
pub const SaferObjectDisableMaxPrivilege: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(7i32);
pub const SaferObjectInvertDeletedPrivileges: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(8i32);
pub const SaferObjectDeletedPrivileges: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(9i32);
pub const SaferObjectDefaultOwner: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(10i32);
pub const SaferObjectSidsToDisable: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(11i32);
pub const SaferObjectRestrictedSidsInverted: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(12i32);
pub const SaferObjectRestrictedSidsAdded: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(13i32);
pub const SaferObjectAllIdentificationGuids: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(14i32);
pub const SaferObjectSingleIdentification: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(15i32);
pub const SaferObjectExtendedError: SAFER_OBJECT_INFO_CLASS = SAFER_OBJECT_INFO_CLASS(16i32);
impl ::core::convert::From<i32> for SAFER_OBJECT_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SAFER_OBJECT_INFO_CLASS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
pub struct SAFER_PATHNAME_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub Description: [u16; 256],
    pub ImageName: super::super::Foundation::PWSTR,
    pub dwSaferFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_PATHNAME_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_PATHNAME_IDENTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_PATHNAME_IDENTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SAFER_PATHNAME_IDENTIFICATION").field("header", &self.header).field("Description", &self.Description).field("ImageName", &self.ImageName).field("dwSaferFlags", &self.dwSaferFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAFER_PATHNAME_IDENTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.Description == other.Description && self.ImageName == other.ImageName && self.dwSaferFlags == other.dwSaferFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_PATHNAME_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_PATHNAME_IDENTIFICATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_BLOCK_CLIENT_UI: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_HASH_DUPLICATE: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SAFER_POLICY_INFO_CLASS(pub i32);
pub const SaferPolicyLevelList: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(1i32);
pub const SaferPolicyEnableTransparentEnforcement: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(2i32);
pub const SaferPolicyDefaultLevel: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(3i32);
pub const SaferPolicyEvaluateUserScope: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(4i32);
pub const SaferPolicyScopeFlags: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(5i32);
pub const SaferPolicyDefaultLevelFlags: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(6i32);
pub const SaferPolicyAuthenticodeEnabled: SAFER_POLICY_INFO_CLASS = SAFER_POLICY_INFO_CLASS(7i32);
impl ::core::convert::From<i32> for SAFER_POLICY_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SAFER_POLICY_INFO_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_JOBID_CONSTRAINED: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_JOBID_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_JOBID_UNTRUSTED: u32 = 50331648u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_ONLY_AUDIT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_ONLY_EXES: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_SANDBOX_INERT: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_UIFLAGS_HIDDEN: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_UIFLAGS_INFORMATION_PROMPT: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_UIFLAGS_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_POLICY_UIFLAGS_OPTION_PROMPT: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_SCOPEID_MACHINE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_AppLocker`*"]
pub const SAFER_SCOPEID_USER: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
pub struct SAFER_URLZONE_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub UrlZoneId: u32,
    pub dwSaferFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SAFER_URLZONE_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SAFER_URLZONE_IDENTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SAFER_URLZONE_IDENTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SAFER_URLZONE_IDENTIFICATION").field("header", &self.header).field("UrlZoneId", &self.UrlZoneId).field("dwSaferFlags", &self.dwSaferFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SAFER_URLZONE_IDENTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.UrlZoneId == other.UrlZoneId && self.dwSaferFlags == other.dwSaferFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SAFER_URLZONE_IDENTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SAFER_URLZONE_IDENTIFICATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferCloseLevel<'a, Param0: ::windows::core::IntoParam<'a, super::SAFER_LEVEL_HANDLE>>(hlevelhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferCloseLevel(hlevelhandle: super::SAFER_LEVEL_HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SaferCloseLevel(hlevelhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferComputeTokenFromLevel<'a, Param0: ::windows::core::IntoParam<'a, super::SAFER_LEVEL_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(levelhandle: Param0, inaccesstoken: Param1, outaccesstoken: *mut super::super::Foundation::HANDLE, dwflags: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferComputeTokenFromLevel(levelhandle: super::SAFER_LEVEL_HANDLE, inaccesstoken: super::super::Foundation::HANDLE, outaccesstoken: *mut super::super::Foundation::HANDLE, dwflags: SAFER_COMPUTE_TOKEN_FROM_LEVEL_FLAGS, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SaferComputeTokenFromLevel(levelhandle.into_param().abi(), inaccesstoken.into_param().abi(), ::core::mem::transmute(outaccesstoken), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferCreateLevel(dwscopeid: u32, dwlevelid: u32, openflags: u32, plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferCreateLevel(dwscopeid: u32, dwlevelid: u32, openflags: u32, plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SaferCreateLevel(::core::mem::transmute(dwscopeid), ::core::mem::transmute(dwlevelid), ::core::mem::transmute(openflags), ::core::mem::transmute(plevelhandle), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferGetLevelInformation<'a, Param0: ::windows::core::IntoParam<'a, super::SAFER_LEVEL_HANDLE>>(levelhandle: Param0, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *mut ::core::ffi::c_void, dwinbuffersize: u32, lpdwoutbuffersize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferGetLevelInformation(levelhandle: super::SAFER_LEVEL_HANDLE, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *mut ::core::ffi::c_void, dwinbuffersize: u32, lpdwoutbuffersize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SaferGetLevelInformation(levelhandle.into_param().abi(), ::core::mem::transmute(dwinfotype), ::core::mem::transmute(lpquerybuffer), ::core::mem::transmute(dwinbuffersize), ::core::mem::transmute(lpdwoutbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferGetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *mut ::core::ffi::c_void, infobufferretsize: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferGetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *mut ::core::ffi::c_void, infobufferretsize: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SaferGetPolicyInformation(::core::mem::transmute(dwscopeid), ::core::mem::transmute(saferpolicyinfoclass), ::core::mem::transmute(infobuffersize), ::core::mem::transmute(infobuffer), ::core::mem::transmute(infobufferretsize), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferIdentifyLevel(dwnumproperties: u32, pcodeproperties: *const SAFER_CODE_PROPERTIES_V2, plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferIdentifyLevel(dwnumproperties: u32, pcodeproperties: *const SAFER_CODE_PROPERTIES_V2, plevelhandle: *mut super::SAFER_LEVEL_HANDLE, lpreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SaferIdentifyLevel(::core::mem::transmute(dwnumproperties), ::core::mem::transmute(pcodeproperties), ::core::mem::transmute(plevelhandle), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferRecordEventLogEntry<'a, Param0: ::windows::core::IntoParam<'a, super::SAFER_LEVEL_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hlevel: Param0, sztargetpath: Param1, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferRecordEventLogEntry(hlevel: super::SAFER_LEVEL_HANDLE, sztargetpath: super::super::Foundation::PWSTR, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SaferRecordEventLogEntry(hlevel.into_param().abi(), sztargetpath.into_param().abi(), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferSetLevelInformation<'a, Param0: ::windows::core::IntoParam<'a, super::SAFER_LEVEL_HANDLE>>(levelhandle: Param0, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *const ::core::ffi::c_void, dwinbuffersize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferSetLevelInformation(levelhandle: super::SAFER_LEVEL_HANDLE, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *const ::core::ffi::c_void, dwinbuffersize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SaferSetLevelInformation(levelhandle.into_param().abi(), ::core::mem::transmute(dwinfotype), ::core::mem::transmute(lpquerybuffer), ::core::mem::transmute(dwinbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferSetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *const ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferSetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *const ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SaferSetPolicyInformation(::core::mem::transmute(dwscopeid), ::core::mem::transmute(saferpolicyinfoclass), ::core::mem::transmute(infobuffersize), ::core::mem::transmute(infobuffer), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaferiIsExecutableFileType<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(szfullpathname: Param0, bfromshellexecute: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaferiIsExecutableFileType(szfullpathname: super::super::Foundation::PWSTR, bfromshellexecute: super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SaferiIsExecutableFileType(szfullpathname.into_param().abi(), bfromshellexecute.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
