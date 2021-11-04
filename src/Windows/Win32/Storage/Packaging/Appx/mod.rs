#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_BUNDLE_FOOTPRINT_FILE_TYPE(pub i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_FIRST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(1i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_LAST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
impl ::std::convert::From<i32> for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(pub i32);
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_APPLICATION: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(0i32);
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_RESOURCE: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(1i32);
impl ::std::convert::From<i32> for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_CAPABILITIES(pub u32);
pub const APPX_CAPABILITY_INTERNET_CLIENT: APPX_CAPABILITIES = APPX_CAPABILITIES(1u32);
pub const APPX_CAPABILITY_INTERNET_CLIENT_SERVER: APPX_CAPABILITIES = APPX_CAPABILITIES(2u32);
pub const APPX_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: APPX_CAPABILITIES = APPX_CAPABILITIES(4u32);
pub const APPX_CAPABILITY_DOCUMENTS_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(8u32);
pub const APPX_CAPABILITY_PICTURES_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(16u32);
pub const APPX_CAPABILITY_VIDEOS_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(32u32);
pub const APPX_CAPABILITY_MUSIC_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(64u32);
pub const APPX_CAPABILITY_ENTERPRISE_AUTHENTICATION: APPX_CAPABILITIES = APPX_CAPABILITIES(128u32);
pub const APPX_CAPABILITY_SHARED_USER_CERTIFICATES: APPX_CAPABILITIES = APPX_CAPABILITIES(256u32);
pub const APPX_CAPABILITY_REMOVABLE_STORAGE: APPX_CAPABILITIES = APPX_CAPABILITIES(512u32);
pub const APPX_CAPABILITY_APPOINTMENTS: APPX_CAPABILITIES = APPX_CAPABILITIES(1024u32);
pub const APPX_CAPABILITY_CONTACTS: APPX_CAPABILITIES = APPX_CAPABILITIES(2048u32);
impl ::std::convert::From<u32> for APPX_CAPABILITIES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_CAPABILITIES {
    type Abi = Self;
}
impl ::std::ops::BitOr for APPX_CAPABILITIES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for APPX_CAPABILITIES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for APPX_CAPABILITIES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for APPX_CAPABILITIES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for APPX_CAPABILITIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_CAPABILITY_CLASS_TYPE(pub i32);
pub const APPX_CAPABILITY_CLASS_DEFAULT: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(0i32);
pub const APPX_CAPABILITY_CLASS_GENERAL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(1i32);
pub const APPX_CAPABILITY_CLASS_RESTRICTED: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(2i32);
pub const APPX_CAPABILITY_CLASS_WINDOWS: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(4i32);
pub const APPX_CAPABILITY_CLASS_ALL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(7i32);
pub const APPX_CAPABILITY_CLASS_CUSTOM: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(8i32);
impl ::std::convert::From<i32> for APPX_CAPABILITY_CLASS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_CAPABILITY_CLASS_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_COMPRESSION_OPTION(pub i32);
pub const APPX_COMPRESSION_OPTION_NONE: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(0i32);
pub const APPX_COMPRESSION_OPTION_NORMAL: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(1i32);
pub const APPX_COMPRESSION_OPTION_MAXIMUM: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(2i32);
pub const APPX_COMPRESSION_OPTION_FAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(3i32);
pub const APPX_COMPRESSION_OPTION_SUPERFAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(4i32);
impl ::std::convert::From<i32> for APPX_COMPRESSION_OPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_COMPRESSION_OPTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
pub struct APPX_ENCRYPTED_EXEMPTIONS {
    pub count: u32,
    pub plainTextFiles: *mut super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl APPX_ENCRYPTED_EXEMPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for APPX_ENCRYPTED_EXEMPTIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for APPX_ENCRYPTED_EXEMPTIONS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APPX_ENCRYPTED_EXEMPTIONS").field("count", &self.count).field("plainTextFiles", &self.plainTextFiles).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for APPX_ENCRYPTED_EXEMPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.plainTextFiles == other.plainTextFiles
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for APPX_ENCRYPTED_EXEMPTIONS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for APPX_ENCRYPTED_EXEMPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_ENCRYPTED_PACKAGE_OPTIONS(pub u32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_NONE: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(0u32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_DIFFUSION: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(1u32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_PAGE_HASHING: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(2u32);
impl ::std::convert::From<u32> for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Abi = Self;
}
impl ::std::ops::BitOr for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS {
    pub keyLength: u32,
    pub encryptionAlgorithm: super::super::super::Foundation::PWSTR,
    pub useDiffusion: super::super::super::Foundation::BOOL,
    pub blockMapHashAlgorithm: ::std::option::Option<super::super::super::System::Com::IUri>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl APPX_ENCRYPTED_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APPX_ENCRYPTED_PACKAGE_SETTINGS").field("keyLength", &self.keyLength).field("encryptionAlgorithm", &self.encryptionAlgorithm).field("useDiffusion", &self.useDiffusion).field("blockMapHashAlgorithm", &self.blockMapHashAlgorithm).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.keyLength == other.keyLength && self.encryptionAlgorithm == other.encryptionAlgorithm && self.useDiffusion == other.useDiffusion && self.blockMapHashAlgorithm == other.blockMapHashAlgorithm
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for APPX_ENCRYPTED_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    pub keyLength: u32,
    pub encryptionAlgorithm: super::super::super::Foundation::PWSTR,
    pub blockMapHashAlgorithm: ::std::option::Option<super::super::super::System::Com::IUri>,
    pub options: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl APPX_ENCRYPTED_PACKAGE_SETTINGS2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APPX_ENCRYPTED_PACKAGE_SETTINGS2").field("keyLength", &self.keyLength).field("encryptionAlgorithm", &self.encryptionAlgorithm).field("blockMapHashAlgorithm", &self.blockMapHashAlgorithm).field("options", &self.options).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn eq(&self, other: &Self) -> bool {
        self.keyLength == other.keyLength && self.encryptionAlgorithm == other.encryptionAlgorithm && self.blockMapHashAlgorithm == other.blockMapHashAlgorithm && self.options == other.options
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_FOOTPRINT_FILE_TYPE(pub i32);
pub const APPX_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(1i32);
pub const APPX_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(2i32);
pub const APPX_FOOTPRINT_FILE_TYPE_CODEINTEGRITY: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(3i32);
pub const APPX_FOOTPRINT_FILE_TYPE_CONTENTGROUPMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(4i32);
impl ::std::convert::From<i32> for APPX_FOOTPRINT_FILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_FOOTPRINT_FILE_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub struct APPX_KEY_INFO {
    pub keyLength: u32,
    pub keyIdLength: u32,
    pub key: *mut u8,
    pub keyId: *mut u8,
}
impl APPX_KEY_INFO {}
impl ::std::default::Default for APPX_KEY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for APPX_KEY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APPX_KEY_INFO").field("keyLength", &self.keyLength).field("keyIdLength", &self.keyIdLength).field("key", &self.key).field("keyId", &self.keyId).finish()
    }
}
impl ::std::cmp::PartialEq for APPX_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.keyLength == other.keyLength && self.keyIdLength == other.keyIdLength && self.key == other.key && self.keyId == other.keyId
    }
}
impl ::std::cmp::Eq for APPX_KEY_INFO {}
unsafe impl ::windows::runtime::Abi for APPX_KEY_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_PACKAGE_ARCHITECTURE(pub i32);
pub const APPX_PACKAGE_ARCHITECTURE_X86: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(0i32);
pub const APPX_PACKAGE_ARCHITECTURE_ARM: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(5i32);
pub const APPX_PACKAGE_ARCHITECTURE_X64: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(9i32);
pub const APPX_PACKAGE_ARCHITECTURE_NEUTRAL: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(11i32);
pub const APPX_PACKAGE_ARCHITECTURE_ARM64: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(12i32);
impl ::std::convert::From<i32> for APPX_PACKAGE_ARCHITECTURE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_PACKAGE_ARCHITECTURE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_PACKAGE_ARCHITECTURE2(pub i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X86: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(0i32);
pub const APPX_PACKAGE_ARCHITECTURE2_ARM: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(5i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(9i32);
pub const APPX_PACKAGE_ARCHITECTURE2_NEUTRAL: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(11i32);
pub const APPX_PACKAGE_ARCHITECTURE2_ARM64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(12i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X86_ON_ARM64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(14i32);
pub const APPX_PACKAGE_ARCHITECTURE2_UNKNOWN: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(65535i32);
impl ::std::convert::From<i32> for APPX_PACKAGE_ARCHITECTURE2 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_PACKAGE_ARCHITECTURE2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(pub u32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_NONE: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(0u32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_SKIP_VALIDATION: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(1u32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_LOCALIZED: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(2u32);
impl ::std::convert::From<u32> for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Abi = Self;
}
impl ::std::ops::BitOr for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(pub i32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION_APPEND_DELTA: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(0i32);
impl ::std::convert::From<i32> for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct APPX_PACKAGE_SETTINGS {
    pub forceZip32: super::super::super::Foundation::BOOL,
    pub hashMethod: ::std::option::Option<super::super::super::System::Com::IUri>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl APPX_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for APPX_PACKAGE_SETTINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for APPX_PACKAGE_SETTINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APPX_PACKAGE_SETTINGS").field("forceZip32", &self.forceZip32).field("hashMethod", &self.hashMethod).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for APPX_PACKAGE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.forceZip32 == other.forceZip32 && self.hashMethod == other.hashMethod
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for APPX_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for APPX_PACKAGE_SETTINGS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    pub inputStream: ::std::option::Option<super::super::super::System::Com::IStream>,
    pub fileName: super::super::super::Foundation::PWSTR,
    pub contentType: super::super::super::Foundation::PWSTR,
    pub compressionOption: APPX_COMPRESSION_OPTION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl APPX_PACKAGE_WRITER_PAYLOAD_STREAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::default::Default for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::fmt::Debug for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APPX_PACKAGE_WRITER_PAYLOAD_STREAM").field("inputStream", &self.inputStream).field("fileName", &self.fileName).field("contentType", &self.contentType).field("compressionOption", &self.compressionOption).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::PartialEq for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn eq(&self, other: &Self) -> bool {
        self.inputStream == other.inputStream && self.fileName == other.fileName && self.contentType == other.contentType && self.compressionOption == other.compressionOption
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::std::cmp::Eq for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_PACKAGING_CONTEXT_CHANGE_TYPE(pub i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_START: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(0i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_CHANGE: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(1i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_DETAILS: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(2i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_END: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(3i32);
impl ::std::convert::From<i32> for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn ActivatePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows::runtime::Result<usize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ActivatePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, cookie: *mut usize) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        ActivatePackageVirtualizationContext(::std::mem::transmute(context), &mut result__).from_abi::<usize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPackageDependency<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagedependencyid: Param0, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut *mut PACKAGEDEPENDENCY_CONTEXT__, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddPackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut *mut PACKAGEDEPENDENCY_CONTEXT__, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        AddPackageDependency(packagedependencyid.into_param().abi(), ::std::mem::transmute(rank), ::std::mem::transmute(options), ::std::mem::transmute(packagedependencycontext), ::std::mem::transmute(packagefullname)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AddPackageDependencyOptions(pub i32);
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = AddPackageDependencyOptions(0i32);
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = AddPackageDependencyOptions(1i32);
impl ::std::convert::From<i32> for AddPackageDependencyOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AddPackageDependencyOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyClrCompat(pub i32);
pub const AppPolicyClrCompat_Other: AppPolicyClrCompat = AppPolicyClrCompat(0i32);
pub const AppPolicyClrCompat_ClassicDesktop: AppPolicyClrCompat = AppPolicyClrCompat(1i32);
pub const AppPolicyClrCompat_Universal: AppPolicyClrCompat = AppPolicyClrCompat(2i32);
pub const AppPolicyClrCompat_PackagedDesktop: AppPolicyClrCompat = AppPolicyClrCompat(3i32);
impl ::std::convert::From<i32> for AppPolicyClrCompat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppPolicyClrCompat {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyCreateFileAccess(pub i32);
pub const AppPolicyCreateFileAccess_Full: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(0i32);
pub const AppPolicyCreateFileAccess_Limited: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(1i32);
impl ::std::convert::From<i32> for AppPolicyCreateFileAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppPolicyCreateFileAccess {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetClrCompat<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyClrCompat) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetClrCompat(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyClrCompat) -> i32;
        }
        ::std::mem::transmute(AppPolicyGetClrCompat(processtoken.into_param().abi(), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetCreateFileAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyCreateFileAccess) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetCreateFileAccess(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyCreateFileAccess) -> i32;
        }
        ::std::mem::transmute(AppPolicyGetCreateFileAccess(processtoken.into_param().abi(), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetLifecycleManagement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyLifecycleManagement) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetLifecycleManagement(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyLifecycleManagement) -> i32;
        }
        ::std::mem::transmute(AppPolicyGetLifecycleManagement(processtoken.into_param().abi(), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetMediaFoundationCodecLoading<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyMediaFoundationCodecLoading) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetMediaFoundationCodecLoading(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyMediaFoundationCodecLoading) -> i32;
        }
        ::std::mem::transmute(AppPolicyGetMediaFoundationCodecLoading(processtoken.into_param().abi(), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetProcessTerminationMethod<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyProcessTerminationMethod) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetProcessTerminationMethod(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyProcessTerminationMethod) -> i32;
        }
        ::std::mem::transmute(AppPolicyGetProcessTerminationMethod(processtoken.into_param().abi(), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetShowDeveloperDiagnostic<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyShowDeveloperDiagnostic) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetShowDeveloperDiagnostic(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyShowDeveloperDiagnostic) -> i32;
        }
        ::std::mem::transmute(AppPolicyGetShowDeveloperDiagnostic(processtoken.into_param().abi(), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetThreadInitializationType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyThreadInitializationType) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetThreadInitializationType(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyThreadInitializationType) -> i32;
        }
        ::std::mem::transmute(AppPolicyGetThreadInitializationType(processtoken.into_param().abi(), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetWindowingModel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyWindowingModel) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetWindowingModel(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyWindowingModel) -> i32;
        }
        ::std::mem::transmute(AppPolicyGetWindowingModel(processtoken.into_param().abi(), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyLifecycleManagement(pub i32);
pub const AppPolicyLifecycleManagement_Unmanaged: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(0i32);
pub const AppPolicyLifecycleManagement_Managed: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(1i32);
impl ::std::convert::From<i32> for AppPolicyLifecycleManagement {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppPolicyLifecycleManagement {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyMediaFoundationCodecLoading(pub i32);
pub const AppPolicyMediaFoundationCodecLoading_All: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(0i32);
pub const AppPolicyMediaFoundationCodecLoading_InboxOnly: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(1i32);
impl ::std::convert::From<i32> for AppPolicyMediaFoundationCodecLoading {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppPolicyMediaFoundationCodecLoading {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyProcessTerminationMethod(pub i32);
pub const AppPolicyProcessTerminationMethod_ExitProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(0i32);
pub const AppPolicyProcessTerminationMethod_TerminateProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(1i32);
impl ::std::convert::From<i32> for AppPolicyProcessTerminationMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppPolicyProcessTerminationMethod {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyShowDeveloperDiagnostic(pub i32);
pub const AppPolicyShowDeveloperDiagnostic_None: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(0i32);
pub const AppPolicyShowDeveloperDiagnostic_ShowUI: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(1i32);
impl ::std::convert::From<i32> for AppPolicyShowDeveloperDiagnostic {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppPolicyShowDeveloperDiagnostic {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyThreadInitializationType(pub i32);
pub const AppPolicyThreadInitializationType_None: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(0i32);
pub const AppPolicyThreadInitializationType_InitializeWinRT: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(1i32);
impl ::std::convert::From<i32> for AppPolicyThreadInitializationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppPolicyThreadInitializationType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyWindowingModel(pub i32);
pub const AppPolicyWindowingModel_None: AppPolicyWindowingModel = AppPolicyWindowingModel(0i32);
pub const AppPolicyWindowingModel_Universal: AppPolicyWindowingModel = AppPolicyWindowingModel(1i32);
pub const AppPolicyWindowingModel_ClassicDesktop: AppPolicyWindowingModel = AppPolicyWindowingModel(2i32);
pub const AppPolicyWindowingModel_ClassicPhone: AppPolicyWindowingModel = AppPolicyWindowingModel(3i32);
impl ::std::convert::From<i32> for AppPolicyWindowingModel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppPolicyWindowingModel {
    type Abi = Self;
}
pub const AppxBundleFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(932054086, 21380, 17335, [136, 119, 231, 219, 221, 136, 52, 70]);
pub const AppxEncryptionFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3697692637, 55400, 18158, [135, 128, 141, 25, 108, 183, 57, 247]);
pub const AppxFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1480761664, 65439, 16742, [143, 92, 98, 245, 183, 176, 199, 129]);
pub const AppxPackageEditor: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4026856138, 44732, 19213, [191, 88, 229, 22, 213, 188, 192, 171]);
pub const AppxPackagingDiagnosticEventSinkManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1355418182, 5512, 16737, [142, 210, 239, 158, 70, 156, 237, 93]);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckIsMSIXPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckIsMSIXPackage(packagefullname: super::super::super::Foundation::PWSTR, ismsixpackage: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CheckIsMSIXPackage(packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn ClosePackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClosePackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE) -> i32;
        }
        ::std::mem::transmute(ClosePackageInfo(::std::mem::transmute(packageinforeference)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CreatePackageDependencyOptions(pub i32);
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = CreatePackageDependencyOptions(0i32);
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: CreatePackageDependencyOptions = CreatePackageDependencyOptions(1i32);
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = CreatePackageDependencyOptions(2i32);
impl ::std::convert::From<i32> for CreatePackageDependencyOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CreatePackageDependencyOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePackageVirtualizationContext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0) -> ::windows::runtime::Result<*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePackageVirtualizationContext(packagefamilyname: super::super::super::Foundation::PWSTR, context: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreatePackageVirtualizationContext(packagefamilyname.into_param().abi(), &mut result__).from_abi::<*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DX_FEATURE_LEVEL(pub i32);
pub const DX_FEATURE_LEVEL_UNSPECIFIED: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(0i32);
pub const DX_FEATURE_LEVEL_9: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(1i32);
pub const DX_FEATURE_LEVEL_10: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(2i32);
pub const DX_FEATURE_LEVEL_11: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(3i32);
impl ::std::convert::From<i32> for DX_FEATURE_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DX_FEATURE_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn DeactivatePackageVirtualizationContext(cookie: usize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeactivatePackageVirtualizationContext(cookie: usize);
        }
        ::std::mem::transmute(DeactivatePackageVirtualizationContext(::std::mem::transmute(cookie)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePackageDependency<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagedependencyid: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeletePackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        DeletePackageDependency(packagedependencyid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn DuplicatePackageVirtualizationContext(sourcecontext: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows::runtime::Result<*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DuplicatePackageVirtualizationContext(sourcecontext: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, destcontext: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DuplicatePackageVirtualizationContext(::std::mem::transmute(sourcecontext), &mut result__).from_abi::<*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindPackagesByPackageFamily<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0, packagefilters: u32, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR, packageproperties: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindPackagesByPackageFamily(packagefamilyname: super::super::super::Foundation::PWSTR, packagefilters: u32, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR, packageproperties: *mut u32) -> i32;
        }
        ::std::mem::transmute(FindPackagesByPackageFamily(packagefamilyname.into_param().abi(), ::std::mem::transmute(packagefilters), ::std::mem::transmute(count), ::std::mem::transmute(packagefullnames), ::std::mem::transmute(bufferlength), ::std::mem::transmute(buffer), ::std::mem::transmute(packageproperties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FormatApplicationUserModelId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0, packagerelativeapplicationid: Param1, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FormatApplicationUserModelId(packagefamilyname: super::super::super::Foundation::PWSTR, packagerelativeapplicationid: super::super::super::Foundation::PWSTR, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(FormatApplicationUserModelId(packagefamilyname.into_param().abi(), packagerelativeapplicationid.into_param().abi(), ::std::mem::transmute(applicationusermodelidlength), ::std::mem::transmute(applicationusermodelid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetApplicationUserModelId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hprocess: Param0, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetApplicationUserModelId(hprocess: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetApplicationUserModelId(hprocess.into_param().abi(), ::std::mem::transmute(applicationusermodelidlength), ::std::mem::transmute(applicationusermodelid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetApplicationUserModelIdFromToken<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(token: Param0, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetApplicationUserModelIdFromToken(token: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetApplicationUserModelIdFromToken(token.into_param().abi(), ::std::mem::transmute(applicationusermodelidlength), ::std::mem::transmute(applicationusermodelid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetCurrentApplicationUserModelId(::std::mem::transmute(applicationusermodelidlength), ::std::mem::transmute(applicationusermodelid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetCurrentPackageFamilyName(::std::mem::transmute(packagefamilynamelength), ::std::mem::transmute(packagefamilyname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetCurrentPackageFullName(::std::mem::transmute(packagefullnamelength), ::std::mem::transmute(packagefullname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn GetCurrentPackageId(bufferlength: *mut u32, buffer: *mut u8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageId(bufferlength: *mut u32, buffer: *mut u8) -> i32;
        }
        ::std::mem::transmute(GetCurrentPackageId(::std::mem::transmute(bufferlength), ::std::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetCurrentPackageInfo(::std::mem::transmute(flags), ::std::mem::transmute(bufferlength), ::std::mem::transmute(buffer), ::std::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn GetCurrentPackageInfo2(flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageInfo2(flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetCurrentPackageInfo2(::std::mem::transmute(flags), ::std::mem::transmute(packagepathtype), ::std::mem::transmute(bufferlength), ::std::mem::transmute(buffer), ::std::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackagePath(pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackagePath(pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetCurrentPackagePath(::std::mem::transmute(pathlength), ::std::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetCurrentPackagePath2(::std::mem::transmute(packagepathtype), ::std::mem::transmute(pathlength), ::std::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn GetCurrentPackageVirtualizationContext() -> *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageVirtualizationContext() -> *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__;
        }
        ::std::mem::transmute(GetCurrentPackageVirtualizationContext())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetIdForPackageDependencyContext(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIdForPackageDependencyContext(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__, packagedependencyid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetIdForPackageDependencyContext(::std::mem::transmute(packagedependencycontext), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn GetPackageApplicationIds(packageinforeference: *const _PACKAGE_INFO_REFERENCE, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageApplicationIds(packageinforeference: *const _PACKAGE_INFO_REFERENCE, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetPackageApplicationIds(::std::mem::transmute(packageinforeference), ::std::mem::transmute(bufferlength), ::std::mem::transmute(buffer), ::std::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hprocess: Param0, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageFamilyName(hprocess: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetPackageFamilyName(hprocess.into_param().abi(), ::std::mem::transmute(packagefamilynamelength), ::std::mem::transmute(packagefamilyname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFamilyNameFromToken<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(token: Param0, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageFamilyNameFromToken(token: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetPackageFamilyNameFromToken(token.into_param().abi(), ::std::mem::transmute(packagefamilynamelength), ::std::mem::transmute(packagefamilyname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFullName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hprocess: Param0, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageFullName(hprocess: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetPackageFullName(hprocess.into_param().abi(), ::std::mem::transmute(packagefullnamelength), ::std::mem::transmute(packagefullname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFullNameFromToken<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(token: Param0, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageFullNameFromToken(token: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetPackageFullNameFromToken(token.into_param().abi(), ::std::mem::transmute(packagefullnamelength), ::std::mem::transmute(packagefullname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hprocess: Param0, bufferlength: *mut u32, buffer: *mut u8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageId(hprocess: super::super::super::Foundation::HANDLE, bufferlength: *mut u32, buffer: *mut u8) -> i32;
        }
        ::std::mem::transmute(GetPackageId(hprocess.into_param().abi(), ::std::mem::transmute(bufferlength), ::std::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn GetPackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetPackageInfo(::std::mem::transmute(packageinforeference), ::std::mem::transmute(flags), ::std::mem::transmute(bufferlength), ::std::mem::transmute(buffer), ::std::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn GetPackageInfo2(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageInfo2(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetPackageInfo2(::std::mem::transmute(packageinforeference), ::std::mem::transmute(flags), ::std::mem::transmute(packagepathtype), ::std::mem::transmute(bufferlength), ::std::mem::transmute(buffer), ::std::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: u32, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: u32, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetPackagePath(::std::mem::transmute(packageid), ::std::mem::transmute(reserved), ::std::mem::transmute(pathlength), ::std::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagePathByFullName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackagePathByFullName(packagefullname: super::super::super::Foundation::PWSTR, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetPackagePathByFullName(packagefullname.into_param().abi(), ::std::mem::transmute(pathlength), ::std::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagePathByFullName2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackagePathByFullName2(packagefullname: super::super::super::Foundation::PWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetPackagePathByFullName2(packagefullname.into_param().abi(), ::std::mem::transmute(packagepathtype), ::std::mem::transmute(pathlength), ::std::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagesByPackageFamily<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackagesByPackageFamily(packagefamilyname: super::super::super::Foundation::PWSTR, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetPackagesByPackageFamily(packagefamilyname.into_param().abi(), ::std::mem::transmute(count), ::std::mem::transmute(packagefullnames), ::std::mem::transmute(bufferlength), ::std::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessesInVirtualizationContext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0, count: *mut u32, processes: *mut *mut super::super::super::Foundation::HANDLE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessesInVirtualizationContext(packagefamilyname: super::super::super::Foundation::PWSTR, count: *mut u32, processes: *mut *mut super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        GetProcessesInVirtualizationContext(packagefamilyname.into_param().abi(), ::std::mem::transmute(count), ::std::mem::transmute(processes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetResolvedPackageFullNameForPackageDependency<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagedependencyid: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetResolvedPackageFullNameForPackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetResolvedPackageFullNameForPackageDependency(packagedependencyid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStagedPackageOrigin<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, origin: *mut PackageOrigin) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStagedPackageOrigin(packagefullname: super::super::super::Foundation::PWSTR, origin: *mut PackageOrigin) -> i32;
        }
        ::std::mem::transmute(GetStagedPackageOrigin(packagefullname.into_param().abi(), ::std::mem::transmute(origin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStagedPackagePathByFullName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStagedPackagePathByFullName(packagefullname: super::super::super::Foundation::PWSTR, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetStagedPackagePathByFullName(packagefullname.into_param().abi(), ::std::mem::transmute(pathlength), ::std::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStagedPackagePathByFullName2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStagedPackagePathByFullName2(packagefullname: super::super::super::Foundation::PWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(GetStagedPackagePathByFullName2(packagefullname.into_param().abi(), ::std::mem::transmute(packagepathtype), ::std::mem::transmute(pathlength), ::std::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBlockMapBlock(pub ::windows::runtime::IUnknown);
impl IAppxBlockMapBlock {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetHash(&self, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(buffersize), ::std::mem::transmute(buffer)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCompressedSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBlockMapBlock {
    type Vtable = IAppxBlockMapBlock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1976514864, 12868, 20448, [168, 200, 224, 188, 178, 112, 184, 137]);
}
impl ::std::convert::From<IAppxBlockMapBlock> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBlockMapBlock) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBlockMapBlock> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBlockMapBlock) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBlockMapBlock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBlockMapBlock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapBlock_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, size: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBlockMapBlocksEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxBlockMapBlocksEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxBlockMapBlock> {
        let mut result__: <IAppxBlockMapBlock as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapBlock>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBlockMapBlocksEnumerator {
    type Vtable = IAppxBlockMapBlocksEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1799527259, 14063, 18334, [185, 235, 12, 20, 130, 180, 158, 22]);
}
impl ::std::convert::From<IAppxBlockMapBlocksEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBlockMapBlocksEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBlockMapBlocksEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBlockMapBlocksEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBlockMapBlocksEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBlockMapBlocksEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapBlocksEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, block: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBlockMapFile(pub ::windows::runtime::IUnknown);
impl IAppxBlockMapFile {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetBlocks(&self) -> ::windows::runtime::Result<IAppxBlockMapBlocksEnumerator> {
        let mut result__: <IAppxBlockMapBlocksEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapBlocksEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetLocalFileHeaderSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetUncompressedSize(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn ValidateFileHash<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filestream: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), filestream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBlockMapFile {
    type Vtable = IAppxBlockMapFile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(662074028, 20323, 17089, [138, 188, 190, 174, 54, 0, 235, 89]);
}
impl ::std::convert::From<IAppxBlockMapFile> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBlockMapFile) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBlockMapFile> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBlockMapFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBlockMapFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBlockMapFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapFile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blocks: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lfhsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, size: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filestream: ::windows::runtime::RawPtr, isvalid: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBlockMapFilesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxBlockMapFilesEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxBlockMapFile> {
        let mut result__: <IAppxBlockMapFile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapFile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBlockMapFilesEnumerator {
    type Vtable = IAppxBlockMapFilesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(45635234, 16994, 16496, [186, 203, 26, 140, 187, 196, 35, 5]);
}
impl ::std::convert::From<IAppxBlockMapFilesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBlockMapFilesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBlockMapFilesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBlockMapFilesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBlockMapFilesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBlockMapFilesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapFilesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBlockMapReader(pub ::windows::runtime::IUnknown);
impl IAppxBlockMapReader {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::runtime::Result<IAppxBlockMapFile> {
        let mut result__: <IAppxBlockMapFile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filename.into_param().abi(), &mut result__).from_abi::<IAppxBlockMapFile>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetFiles(&self) -> ::windows::runtime::Result<IAppxBlockMapFilesEnumerator> {
        let mut result__: <IAppxBlockMapFilesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapFilesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn GetHashMethod(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IUri> {
        let mut result__: <super::super::super::System::Com::IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBlockMapReader {
    type Vtable = IAppxBlockMapReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1593756049, 48291, 17105, [158, 194, 233, 45, 96, 158, 194, 42]);
}
impl ::std::convert::From<IAppxBlockMapReader> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBlockMapReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBlockMapReader> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBlockMapReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBlockMapReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBlockMapReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, file: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hashmethod: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blockmapstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleFactory(pub ::windows::runtime::IUnknown);
impl IAppxBundleFactory {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreateBundleWriter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, outputstream: Param0, bundleversion: u64) -> ::windows::runtime::Result<IAppxBundleWriter> {
        let mut result__: <IAppxBundleWriter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), outputstream.into_param().abi(), ::std::mem::transmute(bundleversion), &mut result__).from_abi::<IAppxBundleWriter>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreateBundleReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::runtime::Result<IAppxBundleReader> {
        let mut result__: <IAppxBundleReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxBundleReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreateBundleManifestReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::runtime::Result<IAppxBundleManifestReader> {
        let mut result__: <IAppxBundleManifestReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxBundleManifestReader>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleFactory {
    type Vtable = IAppxBundleFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3148240996, 38495, 19039, [133, 95, 240, 116, 189, 191, 58, 123]);
}
impl ::std::convert::From<IAppxBundleFactory> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, bundleversion: u64, bundlewriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, bundlereader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, manifestreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleManifestOptionalBundleInfo(pub ::windows::runtime::IUnknown);
impl IAppxBundleManifestOptionalBundleInfo {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageId(&self) -> ::windows::runtime::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetFileName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageInfoItems(&self) -> ::windows::runtime::Result<IAppxBundleManifestPackageInfoEnumerator> {
        let mut result__: <IAppxBundleManifestPackageInfoEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestPackageInfoEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleManifestOptionalBundleInfo {
    type Vtable = IAppxBundleManifestOptionalBundleInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1364980456, 48304, 19817, [140, 72, 227, 131, 20, 123, 110, 18]);
}
impl ::std::convert::From<IAppxBundleManifestOptionalBundleInfo> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleManifestOptionalBundleInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleManifestOptionalBundleInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleManifestOptionalBundleInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleManifestOptionalBundleInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleManifestOptionalBundleInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestOptionalBundleInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageinfoitems: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxBundleManifestOptionalBundleInfoEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxBundleManifestOptionalBundleInfo> {
        let mut result__: <IAppxBundleManifestOptionalBundleInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestOptionalBundleInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleManifestOptionalBundleInfoEnumerator {
    type Vtable = IAppxBundleManifestOptionalBundleInfoEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2585233299, 63870, 18092, [170, 202, 221, 91, 164, 193, 119, 200]);
}
impl ::std::convert::From<IAppxBundleManifestOptionalBundleInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleManifestOptionalBundleInfoEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleManifestOptionalBundleInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleManifestOptionalBundleInfoEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleManifestOptionalBundleInfoEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleManifestOptionalBundleInfoEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, optionalbundle: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleManifestPackageInfo(pub ::windows::runtime::IUnknown);
impl IAppxBundleManifestPackageInfo {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageType(&self) -> ::windows::runtime::Result<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE> {
        let mut result__: <APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageId(&self) -> ::windows::runtime::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetFileName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetOffset(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetSize(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetResources(&self) -> ::windows::runtime::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__: <IAppxManifestQualifiedResourcesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestQualifiedResourcesEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleManifestPackageInfo {
    type Vtable = IAppxBundleManifestPackageInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1422722753, 9871, 16571, [142, 210, 117, 122, 158, 186, 236, 141]);
}
impl ::std::convert::From<IAppxBundleManifestPackageInfo> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleManifestPackageInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleManifestPackageInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleManifestPackageInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleManifestPackageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleManifestPackageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagetype: *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, offset: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, size: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleManifestPackageInfo2(pub ::windows::runtime::IUnknown);
impl IAppxBundleManifestPackageInfo2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetIsPackageReference(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetIsNonQualifiedResourcePackage(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetIsDefaultApplicablePackage(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleManifestPackageInfo2 {
    type Vtable = IAppxBundleManifestPackageInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1153608892, 45775, 19659, [187, 219, 156, 109, 168, 195, 188, 158]);
}
impl ::std::convert::From<IAppxBundleManifestPackageInfo2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleManifestPackageInfo2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleManifestPackageInfo2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleManifestPackageInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleManifestPackageInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleManifestPackageInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ispackagereference: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isdefaultapplicablepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleManifestPackageInfo3(pub ::windows::runtime::IUnknown);
impl IAppxBundleManifestPackageInfo3 {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetTargetDeviceFamilies(&self) -> ::windows::runtime::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        let mut result__: <IAppxManifestTargetDeviceFamiliesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestTargetDeviceFamiliesEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleManifestPackageInfo3 {
    type Vtable = IAppxBundleManifestPackageInfo3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1806125976, 47988, 17046, [128, 208, 95, 66, 86, 169, 150, 117]);
}
impl ::std::convert::From<IAppxBundleManifestPackageInfo3> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleManifestPackageInfo3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleManifestPackageInfo3> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleManifestPackageInfo3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleManifestPackageInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleManifestPackageInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetdevicefamilies: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleManifestPackageInfo4(pub ::windows::runtime::IUnknown);
impl IAppxBundleManifestPackageInfo4 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetIsStub(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleManifestPackageInfo4 {
    type Vtable = IAppxBundleManifestPackageInfo4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1571221821, 43175, 17714, [133, 124, 19, 147, 214, 89, 55, 29]);
}
impl ::std::convert::From<IAppxBundleManifestPackageInfo4> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleManifestPackageInfo4) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleManifestPackageInfo4> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleManifestPackageInfo4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleManifestPackageInfo4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleManifestPackageInfo4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isstub: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleManifestPackageInfoEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxBundleManifestPackageInfoEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxBundleManifestPackageInfo> {
        let mut result__: <IAppxBundleManifestPackageInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestPackageInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleManifestPackageInfoEnumerator {
    type Vtable = IAppxBundleManifestPackageInfoEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4189607662, 18854, 19993, [178, 176, 106, 36, 6, 214, 58, 50]);
}
impl ::std::convert::From<IAppxBundleManifestPackageInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleManifestPackageInfoEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleManifestPackageInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleManifestPackageInfoEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleManifestPackageInfoEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleManifestPackageInfoEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfoEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleManifestReader(pub ::windows::runtime::IUnknown);
impl IAppxBundleManifestReader {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageId(&self) -> ::windows::runtime::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageInfoItems(&self) -> ::windows::runtime::Result<IAppxBundleManifestPackageInfoEnumerator> {
        let mut result__: <IAppxBundleManifestPackageInfoEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestPackageInfoEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleManifestReader {
    type Vtable = IAppxBundleManifestReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3473849281, 52377, 16646, [145, 235, 230, 116, 98, 224, 79, 176]);
}
impl ::std::convert::From<IAppxBundleManifestReader> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleManifestReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleManifestReader> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleManifestReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleManifestReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleManifestReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageinfoitems: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifeststream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleManifestReader2(pub ::windows::runtime::IUnknown);
impl IAppxBundleManifestReader2 {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetOptionalBundles(&self) -> ::windows::runtime::Result<IAppxBundleManifestOptionalBundleInfoEnumerator> {
        let mut result__: <IAppxBundleManifestOptionalBundleInfoEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestOptionalBundleInfoEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleManifestReader2 {
    type Vtable = IAppxBundleManifestReader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1427627888, 831, 19186, [130, 19, 135, 215, 102, 128, 92, 2]);
}
impl ::std::convert::From<IAppxBundleManifestReader2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleManifestReader2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleManifestReader2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleManifestReader2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleManifestReader2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleManifestReader2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, optionalbundles: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleReader(pub ::windows::runtime::IUnknown);
impl IAppxBundleReader {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetFootprintFile(&self, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE) -> ::windows::runtime::Result<IAppxFile> {
        let mut result__: <IAppxFile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(filetype), &mut result__).from_abi::<IAppxFile>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetBlockMap(&self) -> ::windows::runtime::Result<IAppxBlockMapReader> {
        let mut result__: <IAppxBlockMapReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapReader>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetManifest(&self) -> ::windows::runtime::Result<IAppxBundleManifestReader> {
        let mut result__: <IAppxBundleManifestReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestReader>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPayloadPackages(&self) -> ::windows::runtime::Result<IAppxFilesEnumerator> {
        let mut result__: <IAppxFilesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxFilesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPayloadPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::runtime::Result<IAppxFile> {
        let mut result__: <IAppxFile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), filename.into_param().abi(), &mut result__).from_abi::<IAppxFile>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleReader {
    type Vtable = IAppxBundleReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3715479744, 47734, 17328, [174, 15, 104, 101, 106, 29, 197, 200]);
}
impl ::std::convert::From<IAppxBundleReader> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleReader> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE, footprintfile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blockmapreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifestreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, payloadpackages: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, payloadpackage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleWriter(pub ::windows::runtime::IUnknown);
impl IAppxBundleWriter {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddPayloadPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, packagestream: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filename.into_param().abi(), packagestream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleWriter {
    type Vtable = IAppxBundleWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3963908072, 49132, 19556, [171, 79, 73, 240, 56, 240, 198, 210]);
}
impl ::std::convert::From<IAppxBundleWriter> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleWriter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleWriter> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleWriter2(pub ::windows::runtime::IUnknown);
impl IAppxBundleWriter2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddExternalPackageReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, inputstream: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleWriter2 {
    type Vtable = IAppxBundleWriter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1838147953, 460, 18848, [182, 133, 35, 56, 81, 39, 153, 98]);
}
impl ::std::convert::From<IAppxBundleWriter2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleWriter2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleWriter2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleWriter2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleWriter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleWriter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleWriter3(pub ::windows::runtime::IUnknown);
impl IAppxBundleWriter3 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddPackageReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, inputstream: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn Close<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, hashmethodstring: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), hashmethodstring.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleWriter3 {
    type Vtable = IAppxBundleWriter3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2909868370, 63849, 16787, [130, 213, 157, 223, 39, 134, 210, 26]);
}
impl ::std::convert::From<IAppxBundleWriter3> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleWriter3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleWriter3> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleWriter3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleWriter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleWriter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hashmethodstring: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxBundleWriter4(pub ::windows::runtime::IUnknown);
impl IAppxBundleWriter4 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddPayloadPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, filename: Param0, packagestream: Param1, isdefaultapplicablepackage: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filename.into_param().abi(), packagestream.into_param().abi(), isdefaultapplicablepackage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddPackageReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, filename: Param0, inputstream: Param1, isdefaultapplicablepackage: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi(), isdefaultapplicablepackage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddExternalPackageReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, filename: Param0, inputstream: Param1, isdefaultapplicablepackage: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi(), isdefaultapplicablepackage.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxBundleWriter4 {
    type Vtable = IAppxBundleWriter4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2631521571, 20489, 19457, [152, 130, 220, 2, 159, 189, 71, 163]);
}
impl ::std::convert::From<IAppxBundleWriter4> for ::windows::runtime::IUnknown {
    fn from(value: IAppxBundleWriter4) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxBundleWriter4> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxBundleWriter4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxBundleWriter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxBundleWriter4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::runtime::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::runtime::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::runtime::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxContentGroup(pub ::windows::runtime::IUnknown);
impl IAppxContentGroup {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetFiles(&self) -> ::windows::runtime::Result<IAppxContentGroupFilesEnumerator> {
        let mut result__: <IAppxContentGroupFilesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroupFilesEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxContentGroup {
    type Vtable = IAppxContentGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(848258152, 49231, 20028, [182, 250, 107, 141, 39, 243, 0, 58]);
}
impl ::std::convert::From<IAppxContentGroup> for ::windows::runtime::IUnknown {
    fn from(value: IAppxContentGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxContentGroup> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxContentGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxContentGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxContentGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, groupname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxContentGroupFilesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxContentGroupFilesEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxContentGroupFilesEnumerator {
    type Vtable = IAppxContentGroupFilesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(436839165, 29760, 17643, [140, 132, 132, 130, 5, 166, 161, 204]);
}
impl ::std::convert::From<IAppxContentGroupFilesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxContentGroupFilesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxContentGroupFilesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxContentGroupFilesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxContentGroupFilesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxContentGroupFilesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupFilesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxContentGroupMapReader(pub ::windows::runtime::IUnknown);
impl IAppxContentGroupMapReader {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetRequiredGroup(&self) -> ::windows::runtime::Result<IAppxContentGroup> {
        let mut result__: <IAppxContentGroup as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroup>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetAutomaticGroups(&self) -> ::windows::runtime::Result<IAppxContentGroupsEnumerator> {
        let mut result__: <IAppxContentGroupsEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroupsEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxContentGroupMapReader {
    type Vtable = IAppxContentGroupMapReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1099376344, 56729, 20317, [152, 134, 21, 122, 221, 32, 222, 1]);
}
impl ::std::convert::From<IAppxContentGroupMapReader> for ::windows::runtime::IUnknown {
    fn from(value: IAppxContentGroupMapReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxContentGroupMapReader> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxContentGroupMapReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxContentGroupMapReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxContentGroupMapReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupMapReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requiredgroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, automaticgroupsenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxContentGroupMapWriter(pub ::windows::runtime::IUnknown);
impl IAppxContentGroupMapWriter {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn AddAutomaticGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, groupname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), groupname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn AddAutomaticFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxContentGroupMapWriter {
    type Vtable = IAppxContentGroupMapWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3497703286, 43486, 18328, [140, 20, 61, 179, 30, 104, 124, 120]);
}
impl ::std::convert::From<IAppxContentGroupMapWriter> for ::windows::runtime::IUnknown {
    fn from(value: IAppxContentGroupMapWriter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxContentGroupMapWriter> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxContentGroupMapWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxContentGroupMapWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxContentGroupMapWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupMapWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, groupname: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxContentGroupsEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxContentGroupsEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxContentGroup> {
        let mut result__: <IAppxContentGroup as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroup>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxContentGroupsEnumerator {
    type Vtable = IAppxContentGroupsEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(845472887, 5841, 19811, [130, 62, 125, 41, 132, 105, 102, 52]);
}
impl ::std::convert::From<IAppxContentGroupsEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxContentGroupsEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxContentGroupsEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxContentGroupsEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxContentGroupsEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxContentGroupsEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupsEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxEncryptedBundleWriter(pub ::windows::runtime::IUnknown);
impl IAppxEncryptedBundleWriter {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddPayloadPackageEncrypted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, packagestream: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filename.into_param().abi(), packagestream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxEncryptedBundleWriter {
    type Vtable = IAppxEncryptedBundleWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2159054895, 31728, 16663, [184, 198, 66, 121, 239, 129, 238, 119]);
}
impl ::std::convert::From<IAppxEncryptedBundleWriter> for ::windows::runtime::IUnknown {
    fn from(value: IAppxEncryptedBundleWriter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxEncryptedBundleWriter> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxEncryptedBundleWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxEncryptedBundleWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxEncryptedBundleWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxEncryptedBundleWriter2(pub ::windows::runtime::IUnknown);
impl IAppxEncryptedBundleWriter2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddExternalPackageReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, inputstream: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxEncryptedBundleWriter2 {
    type Vtable = IAppxEncryptedBundleWriter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3863264898, 61690, 17080, [169, 86, 141, 28, 180, 142, 227, 121]);
}
impl ::std::convert::From<IAppxEncryptedBundleWriter2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxEncryptedBundleWriter2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxEncryptedBundleWriter2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxEncryptedBundleWriter2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxEncryptedBundleWriter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxEncryptedBundleWriter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxEncryptedBundleWriter3(pub ::windows::runtime::IUnknown);
impl IAppxEncryptedBundleWriter3 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddPayloadPackageEncrypted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, filename: Param0, packagestream: Param1, isdefaultapplicablepackage: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filename.into_param().abi(), packagestream.into_param().abi(), isdefaultapplicablepackage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddExternalPackageReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, filename: Param0, inputstream: Param1, isdefaultapplicablepackage: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi(), isdefaultapplicablepackage.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxEncryptedBundleWriter3 {
    type Vtable = IAppxEncryptedBundleWriter3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(221568691, 23726, 19923, [151, 124, 80, 73, 50, 165, 29, 49]);
}
impl ::std::convert::From<IAppxEncryptedBundleWriter3> for ::windows::runtime::IUnknown {
    fn from(value: IAppxEncryptedBundleWriter3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxEncryptedBundleWriter3> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxEncryptedBundleWriter3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxEncryptedBundleWriter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxEncryptedBundleWriter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::runtime::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::runtime::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxEncryptedPackageWriter(pub ::windows::runtime::IUnknown);
impl IAppxEncryptedPackageWriter {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddPayloadFileEncrypted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, compressionoption: APPX_COMPRESSION_OPTION, inputstream: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filename.into_param().abi(), ::std::mem::transmute(compressionoption), inputstream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxEncryptedPackageWriter {
    type Vtable = IAppxEncryptedPackageWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4097641227, 4985, 16610, [155, 41, 104, 46, 162, 191, 66, 175]);
}
impl ::std::convert::From<IAppxEncryptedPackageWriter> for ::windows::runtime::IUnknown {
    fn from(value: IAppxEncryptedPackageWriter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxEncryptedPackageWriter> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxEncryptedPackageWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxEncryptedPackageWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxEncryptedPackageWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedPackageWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxEncryptedPackageWriter2(pub ::windows::runtime::IUnknown);
impl IAppxEncryptedPackageWriter2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddPayloadFilesEncrypted(&self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(filecount), ::std::mem::transmute(payloadfiles), ::std::mem::transmute(memorylimit)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxEncryptedPackageWriter2 {
    type Vtable = IAppxEncryptedPackageWriter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1044862023, 14885, 16565, [138, 210, 249, 83, 174, 80, 201, 45]);
}
impl ::std::convert::From<IAppxEncryptedPackageWriter2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxEncryptedPackageWriter2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxEncryptedPackageWriter2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxEncryptedPackageWriter2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxEncryptedPackageWriter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxEncryptedPackageWriter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedPackageWriter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filecount: u32, payloadfiles: *const ::std::mem::ManuallyDrop<APPX_PACKAGE_WRITER_PAYLOAD_STREAM>, memorylimit: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxEncryptionFactory(pub ::windows::runtime::IUnknown);
impl IAppxEncryptionFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn EncryptPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::std::mem::transmute(settings), ::std::mem::transmute(keyinfo), ::std::mem::transmute(exemptedfiles)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn DecryptPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, keyinfo: *const APPX_KEY_INFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::std::mem::transmute(keyinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateEncryptedPackageWriter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, outputstream: Param0, manifeststream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::runtime::Result<IAppxEncryptedPackageWriter> {
        let mut result__: <IAppxEncryptedPackageWriter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), outputstream.into_param().abi(), manifeststream.into_param().abi(), ::std::mem::transmute(settings), ::std::mem::transmute(keyinfo), ::std::mem::transmute(exemptedfiles), &mut result__).from_abi::<IAppxEncryptedPackageWriter>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreateEncryptedPackageReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, keyinfo: *const APPX_KEY_INFO) -> ::windows::runtime::Result<IAppxPackageReader> {
        let mut result__: <IAppxPackageReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), ::std::mem::transmute(keyinfo), &mut result__).from_abi::<IAppxPackageReader>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn EncryptBundle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::std::mem::transmute(settings), ::std::mem::transmute(keyinfo), ::std::mem::transmute(exemptedfiles)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn DecryptBundle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, keyinfo: *const APPX_KEY_INFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::std::mem::transmute(keyinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateEncryptedBundleWriter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, outputstream: Param0, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::runtime::Result<IAppxEncryptedBundleWriter> {
        let mut result__: <IAppxEncryptedBundleWriter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), outputstream.into_param().abi(), ::std::mem::transmute(bundleversion), ::std::mem::transmute(settings), ::std::mem::transmute(keyinfo), ::std::mem::transmute(exemptedfiles), &mut result__).from_abi::<IAppxEncryptedBundleWriter>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreateEncryptedBundleReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, keyinfo: *const APPX_KEY_INFO) -> ::windows::runtime::Result<IAppxBundleReader> {
        let mut result__: <IAppxBundleReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), ::std::mem::transmute(keyinfo), &mut result__).from_abi::<IAppxBundleReader>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxEncryptionFactory {
    type Vtable = IAppxEncryptionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2162745421, 35976, 17582, [160, 17, 124, 173, 246, 251, 46, 114]);
}
impl ::std::convert::From<IAppxEncryptionFactory> for ::windows::runtime::IUnknown {
    fn from(value: IAppxEncryptionFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxEncryptionFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxEncryptionFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxEncryptionFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxEncryptionFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, settings: *const ::std::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, keyinfo: *const APPX_KEY_INFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, manifeststream: ::windows::runtime::RawPtr, settings: *const ::std::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, keyinfo: *const APPX_KEY_INFO, packagereader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, settings: *const ::std::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, keyinfo: *const APPX_KEY_INFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, bundleversion: u64, settings: *const ::std::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, keyinfo: *const APPX_KEY_INFO, bundlereader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxEncryptionFactory2(pub ::windows::runtime::IUnknown);
impl IAppxEncryptionFactory2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateEncryptedPackageWriter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(
        &self,
        outputstream: Param0,
        manifeststream: Param1,
        contentgroupmapstream: Param2,
        settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS,
        keyinfo: *const APPX_KEY_INFO,
        exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS,
    ) -> ::windows::runtime::Result<IAppxEncryptedPackageWriter> {
        let mut result__: <IAppxEncryptedPackageWriter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), outputstream.into_param().abi(), manifeststream.into_param().abi(), contentgroupmapstream.into_param().abi(), ::std::mem::transmute(settings), ::std::mem::transmute(keyinfo), ::std::mem::transmute(exemptedfiles), &mut result__).from_abi::<IAppxEncryptedPackageWriter>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxEncryptionFactory2 {
    type Vtable = IAppxEncryptionFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3249610478, 50362, 19122, [165, 93, 208, 21, 254, 143, 246, 79]);
}
impl ::std::convert::From<IAppxEncryptionFactory2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxEncryptionFactory2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxEncryptionFactory2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxEncryptionFactory2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxEncryptionFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxEncryptionFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, manifeststream: ::windows::runtime::RawPtr, contentgroupmapstream: ::windows::runtime::RawPtr, settings: *const ::std::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxEncryptionFactory3(pub ::windows::runtime::IUnknown);
impl IAppxEncryptionFactory3 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn EncryptPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::std::mem::transmute(settings), ::std::mem::transmute(keyinfo), ::std::mem::transmute(exemptedfiles)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateEncryptedPackageWriter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(
        &self,
        outputstream: Param0,
        manifeststream: Param1,
        contentgroupmapstream: Param2,
        settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2,
        keyinfo: *const APPX_KEY_INFO,
        exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS,
    ) -> ::windows::runtime::Result<IAppxEncryptedPackageWriter> {
        let mut result__: <IAppxEncryptedPackageWriter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), outputstream.into_param().abi(), manifeststream.into_param().abi(), contentgroupmapstream.into_param().abi(), ::std::mem::transmute(settings), ::std::mem::transmute(keyinfo), ::std::mem::transmute(exemptedfiles), &mut result__).from_abi::<IAppxEncryptedPackageWriter>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn EncryptBundle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::std::mem::transmute(settings), ::std::mem::transmute(keyinfo), ::std::mem::transmute(exemptedfiles)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateEncryptedBundleWriter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, outputstream: Param0, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::runtime::Result<IAppxEncryptedBundleWriter> {
        let mut result__: <IAppxEncryptedBundleWriter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), outputstream.into_param().abi(), ::std::mem::transmute(bundleversion), ::std::mem::transmute(settings), ::std::mem::transmute(keyinfo), ::std::mem::transmute(exemptedfiles), &mut result__).from_abi::<IAppxEncryptedBundleWriter>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxEncryptionFactory3 {
    type Vtable = IAppxEncryptionFactory3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(166578743, 52580, 18390, [183, 232, 28, 177, 29, 79, 126, 5]);
}
impl ::std::convert::From<IAppxEncryptionFactory3> for ::windows::runtime::IUnknown {
    fn from(value: IAppxEncryptionFactory3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxEncryptionFactory3> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxEncryptionFactory3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxEncryptionFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxEncryptionFactory3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, settings: *const ::std::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, manifeststream: ::windows::runtime::RawPtr, contentgroupmapstream: ::windows::runtime::RawPtr, settings: *const ::std::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, settings: *const ::std::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, bundleversion: u64, settings: *const ::std::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxEncryptionFactory4(pub ::windows::runtime::IUnknown);
impl IAppxEncryptionFactory4 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn EncryptPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::std::mem::transmute(settings), ::std::mem::transmute(keyinfo), ::std::mem::transmute(exemptedfiles), ::std::mem::transmute(memorylimit)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxEncryptionFactory4 {
    type Vtable = IAppxEncryptionFactory4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2826527007, 4861, 16894, [133, 213, 6, 174, 119, 155, 186, 245]);
}
impl ::std::convert::From<IAppxEncryptionFactory4> for ::windows::runtime::IUnknown {
    fn from(value: IAppxEncryptionFactory4) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxEncryptionFactory4> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxEncryptionFactory4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxEncryptionFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxEncryptionFactory4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, settings: *const ::std::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxFactory(pub ::windows::runtime::IUnknown);
impl IAppxFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreatePackageWriter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, outputstream: Param0, settings: *const APPX_PACKAGE_SETTINGS) -> ::windows::runtime::Result<IAppxPackageWriter> {
        let mut result__: <IAppxPackageWriter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), outputstream.into_param().abi(), ::std::mem::transmute(settings), &mut result__).from_abi::<IAppxPackageWriter>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreatePackageReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::runtime::Result<IAppxPackageReader> {
        let mut result__: <IAppxPackageReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxPackageReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreateManifestReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::runtime::Result<IAppxManifestReader> {
        let mut result__: <IAppxManifestReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxManifestReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreateBlockMapReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::runtime::Result<IAppxBlockMapReader> {
        let mut result__: <IAppxBlockMapReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxBlockMapReader>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateValidatedBlockMapReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, blockmapstream: Param0, signaturefilename: Param1) -> ::windows::runtime::Result<IAppxBlockMapReader> {
        let mut result__: <IAppxBlockMapReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), blockmapstream.into_param().abi(), signaturefilename.into_param().abi(), &mut result__).from_abi::<IAppxBlockMapReader>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxFactory {
    type Vtable = IAppxFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3199813897, 58449, 17291, [181, 167, 215, 158, 118, 123, 117, 216]);
}
impl ::std::convert::From<IAppxFactory> for ::windows::runtime::IUnknown {
    fn from(value: IAppxFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, settings: *const ::std::mem::ManuallyDrop<APPX_PACKAGE_SETTINGS>, packagewriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, packagereader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, manifestreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, blockmapreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blockmapstream: ::windows::runtime::RawPtr, signaturefilename: super::super::super::Foundation::PWSTR, blockmapreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxFactory2(pub ::windows::runtime::IUnknown);
impl IAppxFactory2 {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreateContentGroupMapReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::runtime::Result<IAppxContentGroupMapReader> {
        let mut result__: <IAppxContentGroupMapReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxContentGroupMapReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreateSourceContentGroupMapReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::runtime::Result<IAppxSourceContentGroupMapReader> {
        let mut result__: <IAppxSourceContentGroupMapReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxSourceContentGroupMapReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreateContentGroupMapWriter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, stream: Param0) -> ::windows::runtime::Result<IAppxContentGroupMapWriter> {
        let mut result__: <IAppxContentGroupMapWriter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), stream.into_param().abi(), &mut result__).from_abi::<IAppxContentGroupMapWriter>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxFactory2 {
    type Vtable = IAppxFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4046745074, 49794, 20002, [185, 24, 116, 58, 146, 154, 141, 85]);
}
impl ::std::convert::From<IAppxFactory2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxFactory2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxFactory2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxFactory2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, contentgroupmapreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, reader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, contentgroupmapwriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxFile(pub ::windows::runtime::IUnknown);
impl IAppxFile {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCompressionOption(&self) -> ::windows::runtime::Result<APPX_COMPRESSION_OPTION> {
        let mut result__: <APPX_COMPRESSION_OPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<APPX_COMPRESSION_OPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetContentType(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetSize(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxFile {
    type Vtable = IAppxFile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2447344251, 38141, 18063, [130, 123, 87, 244, 27, 47, 111, 46]);
}
impl ::std::convert::From<IAppxFile> for ::windows::runtime::IUnknown {
    fn from(value: IAppxFile) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxFile> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compressionoption: *mut APPX_COMPRESSION_OPTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, size: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxFilesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxFilesEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxFile> {
        let mut result__: <IAppxFile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxFile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxFilesEnumerator {
    type Vtable = IAppxFilesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4027051695, 38961, 16668, [152, 71, 145, 124, 220, 98, 209, 254]);
}
impl ::std::convert::From<IAppxFilesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxFilesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxFilesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxFilesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxFilesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxFilesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFilesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestApplication(pub ::windows::runtime::IUnknown);
impl IAppxManifestApplication {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetStringValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetAppUserModelId(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestApplication {
    type Vtable = IAppxManifestApplication_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1571331060, 14195, 18110, [182, 80, 126, 116, 72, 99, 183, 232]);
}
impl ::std::convert::From<IAppxManifestApplication> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestApplication) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestApplication> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestApplication) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestApplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestApplication {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appusermodelid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestApplicationsEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestApplicationsEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxManifestApplication> {
        let mut result__: <IAppxManifestApplication as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestApplication>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestApplicationsEnumerator {
    type Vtable = IAppxManifestApplicationsEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2662901082, 61515, 19725, [128, 141, 104, 97, 133, 212, 132, 122]);
}
impl ::std::convert::From<IAppxManifestApplicationsEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestApplicationsEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestApplicationsEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestApplicationsEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestApplicationsEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestApplicationsEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestApplicationsEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, application: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestCapabilitiesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestCapabilitiesEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestCapabilitiesEnumerator {
    type Vtable = IAppxManifestCapabilitiesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(298984024, 62576, 17089, [178, 145, 131, 97, 197, 67, 126, 65]);
}
impl ::std::convert::From<IAppxManifestCapabilitiesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestCapabilitiesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestCapabilitiesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestCapabilitiesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestCapabilitiesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestCapabilitiesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestCapabilitiesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capability: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestDeviceCapabilitiesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestDeviceCapabilitiesEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestDeviceCapabilitiesEnumerator {
    type Vtable = IAppxManifestDeviceCapabilitiesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(807421249, 17019, 18972, [186, 207, 101, 91, 244, 99, 165, 64]);
}
impl ::std::convert::From<IAppxManifestDeviceCapabilitiesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestDeviceCapabilitiesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestDeviceCapabilitiesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestDeviceCapabilitiesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestDeviceCapabilitiesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestDeviceCapabilitiesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDeviceCapabilitiesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicecapability: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestDriverConstraint(pub ::windows::runtime::IUnknown);
impl IAppxManifestDriverConstraint {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetMinVersion(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetMinDate(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestDriverConstraint {
    type Vtable = IAppxManifestDriverConstraint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3224485604, 48076, 18666, [162, 55, 195, 64, 69, 200, 10, 7]);
}
impl ::std::convert::From<IAppxManifestDriverConstraint> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestDriverConstraint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestDriverConstraint> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestDriverConstraint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestDriverConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestDriverConstraint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverConstraint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minversion: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mindate: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestDriverConstraintsEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestDriverConstraintsEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxManifestDriverConstraint> {
        let mut result__: <IAppxManifestDriverConstraint as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDriverConstraint>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestDriverConstraintsEnumerator {
    type Vtable = IAppxManifestDriverConstraintsEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3556946641, 62976, 18912, [149, 230, 151, 93, 141, 161, 61, 137]);
}
impl ::std::convert::From<IAppxManifestDriverConstraintsEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestDriverConstraintsEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestDriverConstraintsEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestDriverConstraintsEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestDriverConstraintsEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestDriverConstraintsEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverConstraintsEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, driverconstraint: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestDriverDependenciesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestDriverDependenciesEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxManifestDriverDependency> {
        let mut result__: <IAppxManifestDriverDependency as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDriverDependency>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestDriverDependenciesEnumerator {
    type Vtable = IAppxManifestDriverDependenciesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4261649842, 18047, 18261, [132, 4, 143, 94, 182, 134, 91, 51]);
}
impl ::std::convert::From<IAppxManifestDriverDependenciesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestDriverDependenciesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestDriverDependenciesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestDriverDependenciesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestDriverDependenciesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestDriverDependenciesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverDependenciesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, driverdependency: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestDriverDependency(pub ::windows::runtime::IUnknown);
impl IAppxManifestDriverDependency {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetDriverConstraints(&self) -> ::windows::runtime::Result<IAppxManifestDriverConstraintsEnumerator> {
        let mut result__: <IAppxManifestDriverConstraintsEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDriverConstraintsEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestDriverDependency {
    type Vtable = IAppxManifestDriverDependency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(303090580, 23186, 17922, [190, 36, 121, 243, 24, 175, 74, 249]);
}
impl ::std::convert::From<IAppxManifestDriverDependency> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestDriverDependency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestDriverDependency> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestDriverDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestDriverDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestDriverDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverDependency_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, driverconstraints: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestHostRuntimeDependenciesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestHostRuntimeDependenciesEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxManifestHostRuntimeDependency> {
        let mut result__: <IAppxManifestHostRuntimeDependency as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestHostRuntimeDependency>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestHostRuntimeDependenciesEnumerator {
    type Vtable = IAppxManifestHostRuntimeDependenciesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1680320070, 32585, 17214, [177, 166, 13, 163, 9, 246, 136, 90]);
}
impl ::std::convert::From<IAppxManifestHostRuntimeDependenciesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestHostRuntimeDependenciesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestHostRuntimeDependenciesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestHostRuntimeDependenciesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestHostRuntimeDependenciesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestHostRuntimeDependenciesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependenciesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hostruntimedependency: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestHostRuntimeDependency(pub ::windows::runtime::IUnknown);
impl IAppxManifestHostRuntimeDependency {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPublisher(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetMinVersion(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestHostRuntimeDependency {
    type Vtable = IAppxManifestHostRuntimeDependency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(878039604, 33812, 16653, [149, 199, 123, 53, 37, 91, 131, 145]);
}
impl ::std::convert::From<IAppxManifestHostRuntimeDependency> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestHostRuntimeDependency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestHostRuntimeDependency> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestHostRuntimeDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestHostRuntimeDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestHostRuntimeDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependency_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minversion: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestHostRuntimeDependency2(pub ::windows::runtime::IUnknown);
impl IAppxManifestHostRuntimeDependency2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestHostRuntimeDependency2 {
    type Vtable = IAppxManifestHostRuntimeDependency2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3262063528, 60944, 19158, [184, 152, 43, 77, 122, 235, 254, 106]);
}
impl ::std::convert::From<IAppxManifestHostRuntimeDependency2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestHostRuntimeDependency2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestHostRuntimeDependency2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestHostRuntimeDependency2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestHostRuntimeDependency2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestHostRuntimeDependency2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependency2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestMainPackageDependenciesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestMainPackageDependenciesEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxManifestMainPackageDependency> {
        let mut result__: <IAppxManifestMainPackageDependency as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestMainPackageDependency>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestMainPackageDependenciesEnumerator {
    type Vtable = IAppxManifestMainPackageDependenciesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2845593344, 20946, 20239, [186, 70, 126, 213, 37, 94, 189, 255]);
}
impl ::std::convert::From<IAppxManifestMainPackageDependenciesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestMainPackageDependenciesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestMainPackageDependenciesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestMainPackageDependenciesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestMainPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestMainPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestMainPackageDependenciesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mainpackagedependency: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestMainPackageDependency(pub ::windows::runtime::IUnknown);
impl IAppxManifestMainPackageDependency {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPublisher(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestMainPackageDependency {
    type Vtable = IAppxManifestMainPackageDependency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(97542428, 48169, 18133, [151, 226, 132, 185, 199, 155, 216, 174]);
}
impl ::std::convert::From<IAppxManifestMainPackageDependency> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestMainPackageDependency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestMainPackageDependency> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestMainPackageDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestMainPackageDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestMainPackageDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestMainPackageDependency_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestOSPackageDependenciesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestOSPackageDependenciesEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxManifestOSPackageDependency> {
        let mut result__: <IAppxManifestOSPackageDependency as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestOSPackageDependency>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestOSPackageDependenciesEnumerator {
    type Vtable = IAppxManifestOSPackageDependenciesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3092131779, 63724, 19393, [138, 226, 21, 99, 70, 245, 255, 234]);
}
impl ::std::convert::From<IAppxManifestOSPackageDependenciesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestOSPackageDependenciesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestOSPackageDependenciesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestOSPackageDependenciesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestOSPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestOSPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOSPackageDependenciesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ospackagedependency: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestOSPackageDependency(pub ::windows::runtime::IUnknown);
impl IAppxManifestOSPackageDependency {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestOSPackageDependency {
    type Vtable = IAppxManifestOSPackageDependency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(357144046, 21670, 20244, [172, 151, 216, 207, 5, 25, 100, 75]);
}
impl ::std::convert::From<IAppxManifestOSPackageDependency> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestOSPackageDependency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestOSPackageDependency> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestOSPackageDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestOSPackageDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestOSPackageDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOSPackageDependency_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, version: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestOptionalPackageInfo(pub ::windows::runtime::IUnknown);
impl IAppxManifestOptionalPackageInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetIsOptionalPackage(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetMainPackageName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestOptionalPackageInfo {
    type Vtable = IAppxManifestOptionalPackageInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(640975997, 23389, 20453, [162, 67, 0, 47, 249, 94, 220, 126]);
}
impl ::std::convert::From<IAppxManifestOptionalPackageInfo> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestOptionalPackageInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestOptionalPackageInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestOptionalPackageInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestOptionalPackageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestOptionalPackageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOptionalPackageInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isoptionalpackage: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mainpackagename: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestPackageDependenciesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestPackageDependenciesEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxManifestPackageDependency> {
        let mut result__: <IAppxManifestPackageDependency as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageDependency>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestPackageDependenciesEnumerator {
    type Vtable = IAppxManifestPackageDependenciesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3023813881, 26022, 17117, [186, 192, 140, 103, 65, 231, 245, 164]);
}
impl ::std::convert::From<IAppxManifestPackageDependenciesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestPackageDependenciesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestPackageDependenciesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestPackageDependenciesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependenciesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependency: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestPackageDependency(pub ::windows::runtime::IUnknown);
impl IAppxManifestPackageDependency {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPublisher(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetMinVersion(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestPackageDependency {
    type Vtable = IAppxManifestPackageDependency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3834932057, 29502, 17392, [167, 36, 59, 222, 76, 18, 133, 160]);
}
impl ::std::convert::From<IAppxManifestPackageDependency> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestPackageDependency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestPackageDependency> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestPackageDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestPackageDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestPackageDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minversion: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestPackageDependency2(pub ::windows::runtime::IUnknown);
impl IAppxManifestPackageDependency2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPublisher(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetMinVersion(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetMaxMajorVersionTested(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestPackageDependency2 {
    type Vtable = IAppxManifestPackageDependency2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3718297363, 62463, 18899, [137, 138, 39, 134, 120, 12, 93, 152]);
}
impl ::std::convert::From<IAppxManifestPackageDependency2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestPackageDependency2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestPackageDependency2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestPackageDependency2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestPackageDependency2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestPackageDependency2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IAppxManifestPackageDependency2> for IAppxManifestPackageDependency {
    fn from(value: IAppxManifestPackageDependency2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAppxManifestPackageDependency2> for IAppxManifestPackageDependency {
    fn from(value: &IAppxManifestPackageDependency2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestPackageDependency> for IAppxManifestPackageDependency2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestPackageDependency> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestPackageDependency> for &IAppxManifestPackageDependency2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestPackageDependency> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minversion: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxmajorversiontested: *mut u16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestPackageDependency3(pub ::windows::runtime::IUnknown);
impl IAppxManifestPackageDependency3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetIsOptional(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestPackageDependency3 {
    type Vtable = IAppxManifestPackageDependency3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(449143668, 24984, 19819, [146, 228, 116, 157, 90, 184, 168, 149]);
}
impl ::std::convert::From<IAppxManifestPackageDependency3> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestPackageDependency3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestPackageDependency3> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestPackageDependency3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestPackageDependency3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestPackageDependency3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isoptional: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestPackageId(pub ::windows::runtime::IUnknown);
impl IAppxManifestPackageId {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetArchitecture(&self) -> ::windows::runtime::Result<APPX_PACKAGE_ARCHITECTURE> {
        let mut result__: <APPX_PACKAGE_ARCHITECTURE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<APPX_PACKAGE_ARCHITECTURE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPublisher(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetResourceId(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn ComparePublisher<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, other: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), other.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPackageFullName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestPackageId {
    type Vtable = IAppxManifestPackageId_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(675078871, 29011, 19089, [150, 73, 122, 15, 114, 64, 148, 95]);
}
impl ::std::convert::From<IAppxManifestPackageId> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestPackageId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestPackageId> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestPackageId) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestPackageId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestPackageId {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageId_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageversion: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, other: super::super::super::Foundation::PWSTR, issame: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestPackageId2(pub ::windows::runtime::IUnknown);
impl IAppxManifestPackageId2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetArchitecture(&self) -> ::windows::runtime::Result<APPX_PACKAGE_ARCHITECTURE> {
        let mut result__: <APPX_PACKAGE_ARCHITECTURE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<APPX_PACKAGE_ARCHITECTURE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPublisher(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetResourceId(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn ComparePublisher<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, other: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), other.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPackageFullName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetArchitecture2(&self) -> ::windows::runtime::Result<APPX_PACKAGE_ARCHITECTURE2> {
        let mut result__: <APPX_PACKAGE_ARCHITECTURE2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<APPX_PACKAGE_ARCHITECTURE2>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestPackageId2 {
    type Vtable = IAppxManifestPackageId2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(576100765, 54807, 17137, [136, 14, 11, 164, 84, 35, 25, 213]);
}
impl ::std::convert::From<IAppxManifestPackageId2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestPackageId2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestPackageId2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestPackageId2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestPackageId2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestPackageId2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IAppxManifestPackageId2> for IAppxManifestPackageId {
    fn from(value: IAppxManifestPackageId2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAppxManifestPackageId2> for IAppxManifestPackageId {
    fn from(value: &IAppxManifestPackageId2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestPackageId> for IAppxManifestPackageId2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestPackageId> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestPackageId> for &IAppxManifestPackageId2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestPackageId> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageId2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageversion: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, other: super::super::super::Foundation::PWSTR, issame: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, architecture: *mut APPX_PACKAGE_ARCHITECTURE2) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestProperties(pub ::windows::runtime::IUnknown);
impl IAppxManifestProperties {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetBoolValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetStringValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestProperties {
    type Vtable = IAppxManifestProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(66778701, 62063, 19244, [170, 247, 143, 231, 120, 155, 139, 202]);
}
impl ::std::convert::From<IAppxManifestProperties> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestQualifiedResource(pub ::windows::runtime::IUnknown);
impl IAppxManifestQualifiedResource {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetScale(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetDXFeatureLevel(&self) -> ::windows::runtime::Result<DX_FEATURE_LEVEL> {
        let mut result__: <DX_FEATURE_LEVEL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DX_FEATURE_LEVEL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestQualifiedResource {
    type Vtable = IAppxManifestQualifiedResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(995337367, 15452, 18641, [158, 163, 187, 126, 172, 140, 215, 212]);
}
impl ::std::convert::From<IAppxManifestQualifiedResource> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestQualifiedResource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestQualifiedResource> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestQualifiedResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestQualifiedResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestQualifiedResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestQualifiedResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scale: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dxfeaturelevel: *mut DX_FEATURE_LEVEL) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestQualifiedResourcesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestQualifiedResourcesEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxManifestQualifiedResource> {
        let mut result__: <IAppxManifestQualifiedResource as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestQualifiedResource>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestQualifiedResourcesEnumerator {
    type Vtable = IAppxManifestQualifiedResourcesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2398531070, 14178, 19087, [147, 115, 47, 197, 212, 68, 200, 210]);
}
impl ::std::convert::From<IAppxManifestQualifiedResourcesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestQualifiedResourcesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestQualifiedResourcesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestQualifiedResourcesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestQualifiedResourcesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestQualifiedResourcesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestQualifiedResourcesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestReader(pub ::windows::runtime::IUnknown);
impl IAppxManifestReader {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageId(&self) -> ::windows::runtime::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<IAppxManifestProperties> {
        let mut result__: <IAppxManifestProperties as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestProperties>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::runtime::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestPackageDependenciesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageDependenciesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::runtime::Result<APPX_CAPABILITIES> {
        let mut result__: <APPX_CAPABILITIES as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<APPX_CAPABILITIES>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetResources(&self) -> ::windows::runtime::Result<IAppxManifestResourcesEnumerator> {
        let mut result__: <IAppxManifestResourcesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestResourcesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::runtime::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestDeviceCapabilitiesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDeviceCapabilitiesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPrerequisite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetApplications(&self) -> ::windows::runtime::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__: <IAppxManifestApplicationsEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestApplicationsEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestReader {
    type Vtable = IAppxManifestReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1310445896, 21920, 17536, [163, 209, 21, 84, 71, 16, 99, 124]);
}
impl ::std::convert::From<IAppxManifestReader> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestReader> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencies: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilities: *mut APPX_CAPABILITIES) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicecapabilities: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applications: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifeststream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestReader2(pub ::windows::runtime::IUnknown);
impl IAppxManifestReader2 {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageId(&self) -> ::windows::runtime::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<IAppxManifestProperties> {
        let mut result__: <IAppxManifestProperties as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestProperties>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::runtime::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestPackageDependenciesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageDependenciesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::runtime::Result<APPX_CAPABILITIES> {
        let mut result__: <APPX_CAPABILITIES as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<APPX_CAPABILITIES>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetResources(&self) -> ::windows::runtime::Result<IAppxManifestResourcesEnumerator> {
        let mut result__: <IAppxManifestResourcesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestResourcesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::runtime::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestDeviceCapabilitiesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDeviceCapabilitiesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPrerequisite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetApplications(&self) -> ::windows::runtime::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__: <IAppxManifestApplicationsEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestApplicationsEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetQualifiedResources(&self) -> ::windows::runtime::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__: <IAppxManifestQualifiedResourcesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestQualifiedResourcesEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestReader2 {
    type Vtable = IAppxManifestReader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3496961980, 45853, 20154, [168, 175, 99, 142, 115, 231, 123, 77]);
}
impl ::std::convert::From<IAppxManifestReader2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestReader2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestReader2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestReader2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestReader2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestReader2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IAppxManifestReader2> for IAppxManifestReader {
    fn from(value: IAppxManifestReader2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAppxManifestReader2> for IAppxManifestReader {
    fn from(value: &IAppxManifestReader2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader> for IAppxManifestReader2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader> for &IAppxManifestReader2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencies: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilities: *mut APPX_CAPABILITIES) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicecapabilities: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applications: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifeststream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestReader3(pub ::windows::runtime::IUnknown);
impl IAppxManifestReader3 {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageId(&self) -> ::windows::runtime::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<IAppxManifestProperties> {
        let mut result__: <IAppxManifestProperties as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestProperties>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::runtime::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestPackageDependenciesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageDependenciesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::runtime::Result<APPX_CAPABILITIES> {
        let mut result__: <APPX_CAPABILITIES as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<APPX_CAPABILITIES>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetResources(&self) -> ::windows::runtime::Result<IAppxManifestResourcesEnumerator> {
        let mut result__: <IAppxManifestResourcesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestResourcesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::runtime::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestDeviceCapabilitiesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDeviceCapabilitiesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPrerequisite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetApplications(&self) -> ::windows::runtime::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__: <IAppxManifestApplicationsEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestApplicationsEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetQualifiedResources(&self) -> ::windows::runtime::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__: <IAppxManifestQualifiedResourcesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestQualifiedResourcesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCapabilitiesByCapabilityClass(&self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> ::windows::runtime::Result<IAppxManifestCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestCapabilitiesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(capabilityclass), &mut result__).from_abi::<IAppxManifestCapabilitiesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetTargetDeviceFamilies(&self) -> ::windows::runtime::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        let mut result__: <IAppxManifestTargetDeviceFamiliesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestTargetDeviceFamiliesEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestReader3 {
    type Vtable = IAppxManifestReader3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3292013995, 27063, 16394, [151, 9, 204, 55, 245, 167, 45, 36]);
}
impl ::std::convert::From<IAppxManifestReader3> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestReader3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestReader3> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestReader3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestReader3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestReader3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IAppxManifestReader3> for IAppxManifestReader2 {
    fn from(value: IAppxManifestReader3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAppxManifestReader3> for IAppxManifestReader2 {
    fn from(value: &IAppxManifestReader3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader2> for IAppxManifestReader3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader2> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader2> for &IAppxManifestReader3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IAppxManifestReader3> for IAppxManifestReader {
    fn from(value: IAppxManifestReader3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAppxManifestReader3> for IAppxManifestReader {
    fn from(value: &IAppxManifestReader3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader> for IAppxManifestReader3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader> for &IAppxManifestReader3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencies: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilities: *mut APPX_CAPABILITIES) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicecapabilities: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applications: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifeststream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetdevicefamilies: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestReader4(pub ::windows::runtime::IUnknown);
impl IAppxManifestReader4 {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageId(&self) -> ::windows::runtime::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<IAppxManifestProperties> {
        let mut result__: <IAppxManifestProperties as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestProperties>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::runtime::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestPackageDependenciesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageDependenciesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::runtime::Result<APPX_CAPABILITIES> {
        let mut result__: <APPX_CAPABILITIES as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<APPX_CAPABILITIES>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetResources(&self) -> ::windows::runtime::Result<IAppxManifestResourcesEnumerator> {
        let mut result__: <IAppxManifestResourcesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestResourcesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::runtime::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestDeviceCapabilitiesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDeviceCapabilitiesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPrerequisite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetApplications(&self) -> ::windows::runtime::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__: <IAppxManifestApplicationsEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestApplicationsEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn GetStream(&self) -> ::windows::runtime::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetQualifiedResources(&self) -> ::windows::runtime::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__: <IAppxManifestQualifiedResourcesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestQualifiedResourcesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCapabilitiesByCapabilityClass(&self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> ::windows::runtime::Result<IAppxManifestCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestCapabilitiesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(capabilityclass), &mut result__).from_abi::<IAppxManifestCapabilitiesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetTargetDeviceFamilies(&self) -> ::windows::runtime::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        let mut result__: <IAppxManifestTargetDeviceFamiliesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestTargetDeviceFamiliesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetOptionalPackageInfo(&self) -> ::windows::runtime::Result<IAppxManifestOptionalPackageInfo> {
        let mut result__: <IAppxManifestOptionalPackageInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestOptionalPackageInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestReader4 {
    type Vtable = IAppxManifestReader4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1165605756, 29725, 16737, [181, 161, 71, 189, 59, 120, 173, 155]);
}
impl ::std::convert::From<IAppxManifestReader4> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestReader4) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestReader4> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestReader4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestReader4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestReader4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IAppxManifestReader4> for IAppxManifestReader3 {
    fn from(value: IAppxManifestReader4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAppxManifestReader4> for IAppxManifestReader3 {
    fn from(value: &IAppxManifestReader4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader3> for IAppxManifestReader4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader3> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader3> for &IAppxManifestReader4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader3> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IAppxManifestReader4> for IAppxManifestReader2 {
    fn from(value: IAppxManifestReader4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAppxManifestReader4> for IAppxManifestReader2 {
    fn from(value: &IAppxManifestReader4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader2> for IAppxManifestReader4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader2> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader2> for &IAppxManifestReader4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IAppxManifestReader4> for IAppxManifestReader {
    fn from(value: IAppxManifestReader4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAppxManifestReader4> for IAppxManifestReader {
    fn from(value: &IAppxManifestReader4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader> for IAppxManifestReader4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppxManifestReader> for &IAppxManifestReader4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppxManifestReader> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packageproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dependencies: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilities: *mut APPX_CAPABILITIES) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicecapabilities: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applications: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifeststream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetdevicefamilies: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, optionalpackageinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestReader5(pub ::windows::runtime::IUnknown);
impl IAppxManifestReader5 {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetMainPackageDependencies(&self) -> ::windows::runtime::Result<IAppxManifestMainPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestMainPackageDependenciesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestMainPackageDependenciesEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestReader5 {
    type Vtable = IAppxManifestReader5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2373640498, 42640, 19456, [183, 90, 106, 174, 31, 234, 172, 128]);
}
impl ::std::convert::From<IAppxManifestReader5> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestReader5) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestReader5> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestReader5) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestReader5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestReader5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mainpackagedependencies: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestReader6(pub ::windows::runtime::IUnknown);
impl IAppxManifestReader6 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetIsNonQualifiedResourcePackage(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestReader6 {
    type Vtable = IAppxManifestReader6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(887008420, 54208, 20030, [179, 18, 228, 38, 37, 227, 128, 126]);
}
impl ::std::convert::From<IAppxManifestReader6> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestReader6) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestReader6> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestReader6) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestReader6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestReader6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestReader7(pub ::windows::runtime::IUnknown);
impl IAppxManifestReader7 {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetDriverDependencies(&self) -> ::windows::runtime::Result<IAppxManifestDriverDependenciesEnumerator> {
        let mut result__: <IAppxManifestDriverDependenciesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDriverDependenciesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetOSPackageDependencies(&self) -> ::windows::runtime::Result<IAppxManifestOSPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestOSPackageDependenciesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestOSPackageDependenciesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetHostRuntimeDependencies(&self) -> ::windows::runtime::Result<IAppxManifestHostRuntimeDependenciesEnumerator> {
        let mut result__: <IAppxManifestHostRuntimeDependenciesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestHostRuntimeDependenciesEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestReader7 {
    type Vtable = IAppxManifestReader7_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2399039271, 3296, 18824, [179, 45, 115, 142, 182, 61, 179, 183]);
}
impl ::std::convert::From<IAppxManifestReader7> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestReader7) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestReader7> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestReader7) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestReader7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestReader7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader7_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, driverdependencies: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ospackagedependencies: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hostruntimedependencies: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestResourcesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestResourcesEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestResourcesEnumerator {
    type Vtable = IAppxManifestResourcesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3729652669, 34842, 18619, [133, 140, 214, 242, 186, 234, 230, 237]);
}
impl ::std::convert::From<IAppxManifestResourcesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestResourcesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestResourcesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestResourcesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestResourcesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestResourcesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestResourcesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestTargetDeviceFamiliesEnumerator(pub ::windows::runtime::IUnknown);
impl IAppxManifestTargetDeviceFamiliesEnumerator {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IAppxManifestTargetDeviceFamily> {
        let mut result__: <IAppxManifestTargetDeviceFamily as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestTargetDeviceFamily>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestTargetDeviceFamiliesEnumerator {
    type Vtable = IAppxManifestTargetDeviceFamiliesEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911441718, 10148, 18312, [136, 192, 115, 56, 25, 87, 80, 23]);
}
impl ::std::convert::From<IAppxManifestTargetDeviceFamiliesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestTargetDeviceFamiliesEnumerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestTargetDeviceFamiliesEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestTargetDeviceFamiliesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestTargetDeviceFamiliesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestTargetDeviceFamiliesEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestTargetDeviceFamiliesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetdevicefamily: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxManifestTargetDeviceFamily(pub ::windows::runtime::IUnknown);
impl IAppxManifestTargetDeviceFamily {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetMinVersion(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetMaxVersionTested(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxManifestTargetDeviceFamily {
    type Vtable = IAppxManifestTargetDeviceFamily_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2425467035, 51413, 20273, [134, 135, 163, 56, 37, 159, 174, 251]);
}
impl ::std::convert::From<IAppxManifestTargetDeviceFamily> for ::windows::runtime::IUnknown {
    fn from(value: IAppxManifestTargetDeviceFamily) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxManifestTargetDeviceFamily> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxManifestTargetDeviceFamily) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxManifestTargetDeviceFamily {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxManifestTargetDeviceFamily {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestTargetDeviceFamily_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minversion: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxversiontested: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxPackageEditor(pub ::windows::runtime::IUnknown);
impl IAppxPackageEditor {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, workingdirectory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), workingdirectory.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn CreateDeltaPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, updatedpackagestream: Param0, baselinepackagestream: Param1, deltapackagestream: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), updatedpackagestream.into_param().abi(), baselinepackagestream.into_param().abi(), deltapackagestream.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateDeltaPackageUsingBaselineBlockMap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(
        &self,
        updatedpackagestream: Param0,
        baselineblockmapstream: Param1,
        baselinepackagefullname: Param2,
        deltapackagestream: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), updatedpackagestream.into_param().abi(), baselineblockmapstream.into_param().abi(), baselinepackagefullname.into_param().abi(), deltapackagestream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn UpdatePackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, baselinepackagestream: Param0, deltapackagestream: Param1, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), baselinepackagestream.into_param().abi(), deltapackagestream.into_param().abi(), ::std::mem::transmute(updateoption)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn UpdateEncryptedPackage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, baselineencryptedpackagestream: Param0, deltapackagestream: Param1, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), baselineencryptedpackagestream.into_param().abi(), deltapackagestream.into_param().abi(), ::std::mem::transmute(updateoption), ::std::mem::transmute(settings), ::std::mem::transmute(keyinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn UpdatePackageManifest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, packagestream: Param0, updatedmanifeststream: Param1, ispackageencrypted: Param2, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), packagestream.into_param().abi(), updatedmanifeststream.into_param().abi(), ispackageencrypted.into_param().abi(), ::std::mem::transmute(options)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxPackageEditor {
    type Vtable = IAppxPackageEditor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3803035356, 24177, 17430, [134, 182, 134, 229, 245, 41, 26, 107]);
}
impl ::std::convert::From<IAppxPackageEditor> for ::windows::runtime::IUnknown {
    fn from(value: IAppxPackageEditor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxPackageEditor> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxPackageEditor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxPackageEditor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxPackageEditor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageEditor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, workingdirectory: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updatedpackagestream: ::windows::runtime::RawPtr, baselinepackagestream: ::windows::runtime::RawPtr, deltapackagestream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updatedpackagestream: ::windows::runtime::RawPtr, baselineblockmapstream: ::windows::runtime::RawPtr, baselinepackagefullname: super::super::super::Foundation::PWSTR, deltapackagestream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baselinepackagestream: ::windows::runtime::RawPtr, deltapackagestream: ::windows::runtime::RawPtr, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baselineencryptedpackagestream: ::windows::runtime::RawPtr, deltapackagestream: ::windows::runtime::RawPtr, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const ::std::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagestream: ::windows::runtime::RawPtr, updatedmanifeststream: ::windows::runtime::RawPtr, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxPackageReader(pub ::windows::runtime::IUnknown);
impl IAppxPackageReader {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetBlockMap(&self) -> ::windows::runtime::Result<IAppxBlockMapReader> {
        let mut result__: <IAppxBlockMapReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapReader>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetFootprintFile(&self, r#type: APPX_FOOTPRINT_FILE_TYPE) -> ::windows::runtime::Result<IAppxFile> {
        let mut result__: <IAppxFile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type), &mut result__).from_abi::<IAppxFile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn GetPayloadFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::runtime::Result<IAppxFile> {
        let mut result__: <IAppxFile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), filename.into_param().abi(), &mut result__).from_abi::<IAppxFile>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetPayloadFiles(&self) -> ::windows::runtime::Result<IAppxFilesEnumerator> {
        let mut result__: <IAppxFilesEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxFilesEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetManifest(&self) -> ::windows::runtime::Result<IAppxManifestReader> {
        let mut result__: <IAppxManifestReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestReader>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxPackageReader {
    type Vtable = IAppxPackageReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3049559632, 39356, 18460, [154, 52, 61, 83, 164, 16, 103, 8]);
}
impl ::std::convert::From<IAppxPackageReader> for ::windows::runtime::IUnknown {
    fn from(value: IAppxPackageReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxPackageReader> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxPackageReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxPackageReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxPackageReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blockmapreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: APPX_FOOTPRINT_FILE_TYPE, file: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, file: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filesenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifestreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxPackageWriter(pub ::windows::runtime::IUnknown);
impl IAppxPackageWriter {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddPayloadFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, contenttype: Param1, compressionoption: APPX_COMPRESSION_OPTION, inputstream: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filename.into_param().abi(), contenttype.into_param().abi(), ::std::mem::transmute(compressionoption), inputstream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn Close<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, manifest: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), manifest.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxPackageWriter {
    type Vtable = IAppxPackageWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2426004283, 9327, 16868, [136, 26, 0, 142, 182, 19, 248, 88]);
}
impl ::std::convert::From<IAppxPackageWriter> for ::windows::runtime::IUnknown {
    fn from(value: IAppxPackageWriter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxPackageWriter> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxPackageWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxPackageWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxPackageWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::super::Foundation::PWSTR, contenttype: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifest: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxPackageWriter2(pub ::windows::runtime::IUnknown);
impl IAppxPackageWriter2 {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_System_Com`*"]
    pub unsafe fn Close<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, manifest: Param0, contentgroupmap: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), manifest.into_param().abi(), contentgroupmap.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxPackageWriter2 {
    type Vtable = IAppxPackageWriter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(754304253, 58700, 20133, [186, 78, 248, 196, 177, 5, 168, 200]);
}
impl ::std::convert::From<IAppxPackageWriter2> for ::windows::runtime::IUnknown {
    fn from(value: IAppxPackageWriter2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxPackageWriter2> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxPackageWriter2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxPackageWriter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxPackageWriter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, manifest: ::windows::runtime::RawPtr, contentgroupmap: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxPackageWriter3(pub ::windows::runtime::IUnknown);
impl IAppxPackageWriter3 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddPayloadFiles(&self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(filecount), ::std::mem::transmute(payloadfiles), ::std::mem::transmute(memorylimit)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxPackageWriter3 {
    type Vtable = IAppxPackageWriter3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2822417619, 16832, 17665, [184, 163, 116, 22, 79, 80, 178, 253]);
}
impl ::std::convert::From<IAppxPackageWriter3> for ::windows::runtime::IUnknown {
    fn from(value: IAppxPackageWriter3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxPackageWriter3> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxPackageWriter3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxPackageWriter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxPackageWriter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filecount: u32, payloadfiles: *const ::std::mem::ManuallyDrop<APPX_PACKAGE_WRITER_PAYLOAD_STREAM>, memorylimit: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxPackagingDiagnosticEventSink(pub ::windows::runtime::IUnknown);
impl IAppxPackagingDiagnosticEventSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn ReportContextChange<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: Param2, contextmessage: Param3, detailsmessage: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(changetype), ::std::mem::transmute(contextid), contextname.into_param().abi(), contextmessage.into_param().abi(), detailsmessage.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    pub unsafe fn ReportError<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, errormessage: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), errormessage.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxPackagingDiagnosticEventSink {
    type Vtable = IAppxPackagingDiagnosticEventSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(388209991, 27355, 17874, [128, 246, 249, 203, 195, 191, 5, 157]);
}
impl ::std::convert::From<IAppxPackagingDiagnosticEventSink> for ::windows::runtime::IUnknown {
    fn from(value: IAppxPackagingDiagnosticEventSink) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxPackagingDiagnosticEventSink> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxPackagingDiagnosticEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxPackagingDiagnosticEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxPackagingDiagnosticEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackagingDiagnosticEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: super::super::super::Foundation::PSTR, contextmessage: super::super::super::Foundation::PWSTR, detailsmessage: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, errormessage: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxPackagingDiagnosticEventSinkManager(pub ::windows::runtime::IUnknown);
impl IAppxPackagingDiagnosticEventSinkManager {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn SetSinkForProcess<'a, Param0: ::windows::runtime::IntoParam<'a, IAppxPackagingDiagnosticEventSink>>(&self, sink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), sink.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAppxPackagingDiagnosticEventSinkManager {
    type Vtable = IAppxPackagingDiagnosticEventSinkManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(915818746, 42987, 18697, [161, 93, 105, 84, 160, 120, 241, 138]);
}
impl ::std::convert::From<IAppxPackagingDiagnosticEventSinkManager> for ::windows::runtime::IUnknown {
    fn from(value: IAppxPackagingDiagnosticEventSinkManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxPackagingDiagnosticEventSinkManager> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxPackagingDiagnosticEventSinkManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxPackagingDiagnosticEventSinkManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxPackagingDiagnosticEventSinkManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackagingDiagnosticEventSinkManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAppxSourceContentGroupMapReader(pub ::windows::runtime::IUnknown);
impl IAppxSourceContentGroupMapReader {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetRequiredGroup(&self) -> ::windows::runtime::Result<IAppxContentGroup> {
        let mut result__: <IAppxContentGroup as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroup>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub unsafe fn GetAutomaticGroups(&self) -> ::windows::runtime::Result<IAppxContentGroupsEnumerator> {
        let mut result__: <IAppxContentGroupsEnumerator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroupsEnumerator>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppxSourceContentGroupMapReader {
    type Vtable = IAppxSourceContentGroupMapReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4079581469, 21515, 19103, [188, 117, 50, 130, 183, 215, 49, 147]);
}
impl ::std::convert::From<IAppxSourceContentGroupMapReader> for ::windows::runtime::IUnknown {
    fn from(value: IAppxSourceContentGroupMapReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAppxSourceContentGroupMapReader> for ::windows::runtime::IUnknown {
    fn from(value: &IAppxSourceContentGroupMapReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppxSourceContentGroupMapReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppxSourceContentGroupMapReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxSourceContentGroupMapReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requiredgroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, automaticgroupsenumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenPackageInfoByFullName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPackageInfoByFullName(packagefullname: super::super::super::Foundation::PWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32;
        }
        ::std::mem::transmute(OpenPackageInfoByFullName(packagefullname.into_param().abi(), ::std::mem::transmute(reserved), ::std::mem::transmute(packageinforeference)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenPackageInfoByFullNameForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSID>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(usersid: Param0, packagefullname: Param1, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPackageInfoByFullNameForUser(usersid: super::super::super::Foundation::PSID, packagefullname: super::super::super::Foundation::PWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32;
        }
        ::std::mem::transmute(OpenPackageInfoByFullNameForUser(usersid.into_param().abi(), packagefullname.into_param().abi(), ::std::mem::transmute(reserved), ::std::mem::transmute(packageinforeference)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub struct PACKAGEDEPENDENCY_CONTEXT__ {
    pub unused: i32,
}
impl PACKAGEDEPENDENCY_CONTEXT__ {}
impl ::std::default::Default for PACKAGEDEPENDENCY_CONTEXT__ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PACKAGEDEPENDENCY_CONTEXT__ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PACKAGEDEPENDENCY_CONTEXT__").field("unused", &self.unused).finish()
    }
}
impl ::std::cmp::PartialEq for PACKAGEDEPENDENCY_CONTEXT__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::std::cmp::Eq for PACKAGEDEPENDENCY_CONTEXT__ {}
unsafe impl ::windows::runtime::Abi for PACKAGEDEPENDENCY_CONTEXT__ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_DEPENDENCY_RANK_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_ALL_LOADED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_BUNDLE: u32 = 128u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_DIRECT: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_DYNAMIC: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_HEAD: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_HOSTRUNTIME: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_IS_IN_RELATED_SET: u32 = 262144u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_OPTIONAL: u32 = 131072u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_RESOURCE: u32 = 64u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_STATIC: u32 = 524288u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: super::super::super::Foundation::PWSTR,
    pub publisher: super::super::super::Foundation::PWSTR,
    pub resourceId: super::super::super::Foundation::PWSTR,
    pub publisherId: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PACKAGE_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PACKAGE_ID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PACKAGE_ID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PACKAGE_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: super::super::super::Foundation::PWSTR,
    pub packageFullName: super::super::super::Foundation::PWSTR,
    pub packageFamilyName: super::super::super::Foundation::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(feature = "Win32_Foundation")]
impl PACKAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PACKAGE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PACKAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PACKAGE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_INFORMATION_BASIC: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_INFORMATION_FULL: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_BUNDLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_DEVELOPMENT_MODE: u32 = 65536u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_DYNAMIC: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_FRAMEWORK: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_HOSTRUNTIME: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_IS_IN_RELATED_SET: u32 = 262144u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_OPTIONAL: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_RESOURCE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_STATIC: u32 = 524288u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl PACKAGE_VERSION {}
impl ::std::default::Default for PACKAGE_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PACKAGE_VERSION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PACKAGE_VERSION {}
unsafe impl ::windows::runtime::Abi for PACKAGE_VERSION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub union PACKAGE_VERSION_0 {
    pub Version: u64,
    pub Anonymous: PACKAGE_VERSION_0_0,
}
impl PACKAGE_VERSION_0 {}
impl ::std::default::Default for PACKAGE_VERSION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PACKAGE_VERSION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PACKAGE_VERSION_0 {}
unsafe impl ::windows::runtime::Abi for PACKAGE_VERSION_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
impl PACKAGE_VERSION_0_0 {}
impl ::std::default::Default for PACKAGE_VERSION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PACKAGE_VERSION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Revision", &self.Revision).field("Build", &self.Build).field("Minor", &self.Minor).field("Major", &self.Major).finish()
    }
}
impl ::std::cmp::PartialEq for PACKAGE_VERSION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.Build == other.Build && self.Minor == other.Minor && self.Major == other.Major
    }
}
impl ::std::cmp::Eq for PACKAGE_VERSION_0_0 {}
unsafe impl ::windows::runtime::Abi for PACKAGE_VERSION_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub struct PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    pub unused: i32,
}
impl PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {}
impl ::std::default::Default for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__").field("unused", &self.unused).finish()
    }
}
impl ::std::cmp::PartialEq for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::std::cmp::Eq for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {}
unsafe impl ::windows::runtime::Abi for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageDependencyLifetimeKind(pub i32);
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(0i32);
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(1i32);
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(2i32);
impl ::std::convert::From<i32> for PackageDependencyLifetimeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PackageDependencyLifetimeKind {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageDependencyProcessorArchitectures(pub i32);
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(0i32);
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(1i32);
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(2i32);
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(4i32);
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(8i32);
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(16i32);
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(32i32);
impl ::std::convert::From<i32> for PackageDependencyProcessorArchitectures {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PackageDependencyProcessorArchitectures {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageFamilyNameFromFullName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackageFamilyNameFromFullName(packagefullname: super::super::super::Foundation::PWSTR, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(PackageFamilyNameFromFullName(packagefullname.into_param().abi(), ::std::mem::transmute(packagefamilynamelength), ::std::mem::transmute(packagefamilyname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(PackageFamilyNameFromId(::std::mem::transmute(packageid), ::std::mem::transmute(packagefamilynamelength), ::std::mem::transmute(packagefamilyname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(PackageFullNameFromId(::std::mem::transmute(packageid), ::std::mem::transmute(packagefullnamelength), ::std::mem::transmute(packagefullname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageIdFromFullName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, flags: u32, bufferlength: *mut u32, buffer: *mut u8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackageIdFromFullName(packagefullname: super::super::super::Foundation::PWSTR, flags: u32, bufferlength: *mut u32, buffer: *mut u8) -> i32;
        }
        ::std::mem::transmute(PackageIdFromFullName(packagefullname.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(bufferlength), ::std::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageNameAndPublisherIdFromFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0, packagenamelength: *mut u32, packagename: super::super::super::Foundation::PWSTR, packagepublisheridlength: *mut u32, packagepublisherid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackageNameAndPublisherIdFromFamilyName(packagefamilyname: super::super::super::Foundation::PWSTR, packagenamelength: *mut u32, packagename: super::super::super::Foundation::PWSTR, packagepublisheridlength: *mut u32, packagepublisherid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(PackageNameAndPublisherIdFromFamilyName(packagefamilyname.into_param().abi(), ::std::mem::transmute(packagenamelength), ::std::mem::transmute(packagename), ::std::mem::transmute(packagepublisheridlength), ::std::mem::transmute(packagepublisherid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageOrigin(pub i32);
pub const PackageOrigin_Unknown: PackageOrigin = PackageOrigin(0i32);
pub const PackageOrigin_Unsigned: PackageOrigin = PackageOrigin(1i32);
pub const PackageOrigin_Inbox: PackageOrigin = PackageOrigin(2i32);
pub const PackageOrigin_Store: PackageOrigin = PackageOrigin(3i32);
pub const PackageOrigin_DeveloperUnsigned: PackageOrigin = PackageOrigin(4i32);
pub const PackageOrigin_DeveloperSigned: PackageOrigin = PackageOrigin(5i32);
pub const PackageOrigin_LineOfBusiness: PackageOrigin = PackageOrigin(6i32);
impl ::std::convert::From<i32> for PackageOrigin {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PackageOrigin {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackagePathType(pub i32);
pub const PackagePathType_Install: PackagePathType = PackagePathType(0i32);
pub const PackagePathType_Mutable: PackagePathType = PackagePathType(1i32);
pub const PackagePathType_Effective: PackagePathType = PackagePathType(2i32);
pub const PackagePathType_MachineExternal: PackagePathType = PackagePathType(3i32);
pub const PackagePathType_UserExternal: PackagePathType = PackagePathType(4i32);
pub const PackagePathType_EffectiveExternal: PackagePathType = PackagePathType(5i32);
impl ::std::convert::From<i32> for PackagePathType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PackagePathType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ParseApplicationUserModelId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(applicationusermodelid: Param0, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR, packagerelativeapplicationidlength: *mut u32, packagerelativeapplicationid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ParseApplicationUserModelId(applicationusermodelid: super::super::super::Foundation::PWSTR, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR, packagerelativeapplicationidlength: *mut u32, packagerelativeapplicationid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(ParseApplicationUserModelId(applicationusermodelid.into_param().abi(), ::std::mem::transmute(packagefamilynamelength), ::std::mem::transmute(packagefamilyname), ::std::mem::transmute(packagerelativeapplicationidlength), ::std::mem::transmute(packagerelativeapplicationid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn ReleasePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleasePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__);
        }
        ::std::mem::transmute(ReleasePackageVirtualizationContext(::std::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
#[inline]
pub unsafe fn RemovePackageDependency(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemovePackageDependency(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows::runtime::HRESULT;
        }
        RemovePackageDependency(::std::mem::transmute(packagedependencycontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryCreatePackageDependency<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSID>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, PACKAGE_VERSION>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
    user: Param0,
    packagefamilyname: Param1,
    minversion: Param2,
    packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures,
    lifetimekind: PackageDependencyLifetimeKind,
    lifetimeartifact: Param5,
    options: CreatePackageDependencyOptions,
) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TryCreatePackageDependency(user: super::super::super::Foundation::PSID, packagefamilyname: super::super::super::Foundation::PWSTR, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: super::super::super::Foundation::PWSTR, options: CreatePackageDependencyOptions, packagedependencyid: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        TryCreatePackageDependency(user.into_param().abi(), packagefamilyname.into_param().abi(), minversion.into_param().abi(), ::std::mem::transmute(packagedependencyprocessorarchitectures), ::std::mem::transmute(lifetimekind), lifetimeartifact.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyApplicationUserModelId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(applicationusermodelid: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyApplicationUserModelId(applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(VerifyApplicationUserModelId(applicationusermodelid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyPackageFamilyName(packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(VerifyPackageFamilyName(packagefamilyname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageFullName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyPackageFullName(packagefullname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(VerifyPackageFullName(packagefullname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageId(packageid: *const PACKAGE_ID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyPackageId(packageid: *const PACKAGE_ID) -> i32;
        }
        ::std::mem::transmute(VerifyPackageId(::std::mem::transmute(packageid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageRelativeApplicationId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagerelativeapplicationid: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyPackageRelativeApplicationId(packagerelativeapplicationid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::std::mem::transmute(VerifyPackageRelativeApplicationId(packagerelativeapplicationid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub struct _PACKAGE_INFO_REFERENCE {
    pub reserved: *mut ::std::ffi::c_void,
}
impl _PACKAGE_INFO_REFERENCE {}
impl ::std::default::Default for _PACKAGE_INFO_REFERENCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _PACKAGE_INFO_REFERENCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_PACKAGE_INFO_REFERENCE").field("reserved", &self.reserved).finish()
    }
}
impl ::std::cmp::PartialEq for _PACKAGE_INFO_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl ::std::cmp::Eq for _PACKAGE_INFO_REFERENCE {}
unsafe impl ::windows::runtime::Abi for _PACKAGE_INFO_REFERENCE {
    type Abi = Self;
}
