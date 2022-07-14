#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTXA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: ::windows::core::PCSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: ::windows::core::PCSTR,
    pub lpResourceName: ::windows::core::PCSTR,
    pub lpApplicationName: ::windows::core::PCSTR,
    pub hModule: super::super::Foundation::HINSTANCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTCTXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTCTXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ACTCTXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTXA").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("lpSource", &self.lpSource).field("wProcessorArchitecture", &self.wProcessorArchitecture).field("wLangId", &self.wLangId).field("lpAssemblyDirectory", &self.lpAssemblyDirectory).field("lpResourceName", &self.lpResourceName).field("lpApplicationName", &self.lpApplicationName).field("hModule", &self.hModule).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACTCTXA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACTCTXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTCTXA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACTCTXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTCTXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTXW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: ::windows::core::PCWSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: ::windows::core::PCWSTR,
    pub lpResourceName: ::windows::core::PCWSTR,
    pub lpApplicationName: ::windows::core::PCWSTR,
    pub hModule: super::super::Foundation::HINSTANCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTCTXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTCTXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ACTCTXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTXW").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("lpSource", &self.lpSource).field("wProcessorArchitecture", &self.wProcessorArchitecture).field("wLangId", &self.wLangId).field("lpAssemblyDirectory", &self.lpAssemblyDirectory).field("lpResourceName", &self.lpResourceName).field("lpApplicationName", &self.lpApplicationName).field("hModule", &self.hModule).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACTCTXW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACTCTXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTCTXW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACTCTXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTCTXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACTCTX_COMPATIBILITY_ELEMENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MAXVERSIONTESTED: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(3i32);
impl ::core::marker::Copy for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {}
impl ::core::clone::Clone for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTCTX_COMPATIBILITY_ELEMENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACTCTX_REQUESTED_RUN_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_UNSPECIFIED: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_AS_INVOKER: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_REQUIRE_ADMIN: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_NUMBERS: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(4i32);
impl ::core::marker::Copy for ACTCTX_REQUESTED_RUN_LEVEL {}
impl ::core::clone::Clone for ACTCTX_REQUESTED_RUN_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTCTX_REQUESTED_RUN_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ACTCTX_REQUESTED_RUN_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACTCTX_REQUESTED_RUN_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTCTX_REQUESTED_RUN_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
pub struct ACTCTX_SECTION_KEYED_DATA {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut ::core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut ::core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut ::core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
    pub ulFlags: u32,
    pub AssemblyMetadata: super::WindowsProgramming::ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::fmt::Debug for ACTCTX_SECTION_KEYED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTX_SECTION_KEYED_DATA")
            .field("cbSize", &self.cbSize)
            .field("ulDataFormatVersion", &self.ulDataFormatVersion)
            .field("lpData", &self.lpData)
            .field("ulLength", &self.ulLength)
            .field("lpSectionGlobalData", &self.lpSectionGlobalData)
            .field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength)
            .field("lpSectionBase", &self.lpSectionBase)
            .field("ulSectionTotalLength", &self.ulSectionTotalLength)
            .field("hActCtx", &self.hActCtx)
            .field("ulAssemblyRosterIndex", &self.ulAssemblyRosterIndex)
            .field("ulFlags", &self.ulFlags)
            .field("AssemblyMetadata", &self.AssemblyMetadata)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
unsafe impl ::windows::core::Abi for ACTCTX_SECTION_KEYED_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTCTX_SECTION_KEYED_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::cmp::Eq for ACTCTX_SECTION_KEYED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::default::Default for ACTCTX_SECTION_KEYED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulEncodedAssemblyIdentityLength: u32,
    pub ulManifestPathType: u32,
    pub ulManifestPathLength: u32,
    pub liManifestLastWriteTime: i64,
    pub ulPolicyPathType: u32,
    pub ulPolicyPathLength: u32,
    pub liPolicyLastWriteTime: i64,
    pub ulMetadataSatelliteRosterIndex: u32,
    pub ulManifestVersionMajor: u32,
    pub ulManifestVersionMinor: u32,
    pub ulPolicyVersionMajor: u32,
    pub ulPolicyVersionMinor: u32,
    pub ulAssemblyDirectoryNameLength: u32,
    pub lpAssemblyEncodedAssemblyIdentity: ::windows::core::PCWSTR,
    pub lpAssemblyManifestPath: ::windows::core::PCWSTR,
    pub lpAssemblyPolicyPath: ::windows::core::PCWSTR,
    pub lpAssemblyDirectoryName: ::windows::core::PCWSTR,
    pub ulFileCount: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION")
            .field("ulFlags", &self.ulFlags)
            .field("ulEncodedAssemblyIdentityLength", &self.ulEncodedAssemblyIdentityLength)
            .field("ulManifestPathType", &self.ulManifestPathType)
            .field("ulManifestPathLength", &self.ulManifestPathLength)
            .field("liManifestLastWriteTime", &self.liManifestLastWriteTime)
            .field("ulPolicyPathType", &self.ulPolicyPathType)
            .field("ulPolicyPathLength", &self.ulPolicyPathLength)
            .field("liPolicyLastWriteTime", &self.liPolicyLastWriteTime)
            .field("ulMetadataSatelliteRosterIndex", &self.ulMetadataSatelliteRosterIndex)
            .field("ulManifestVersionMajor", &self.ulManifestVersionMajor)
            .field("ulManifestVersionMinor", &self.ulManifestVersionMinor)
            .field("ulPolicyVersionMajor", &self.ulPolicyVersionMajor)
            .field("ulPolicyVersionMinor", &self.ulPolicyVersionMinor)
            .field("ulAssemblyDirectoryNameLength", &self.ulAssemblyDirectoryNameLength)
            .field("lpAssemblyEncodedAssemblyIdentity", &self.lpAssemblyEncodedAssemblyIdentity)
            .field("lpAssemblyManifestPath", &self.lpAssemblyManifestPath)
            .field("lpAssemblyPolicyPath", &self.lpAssemblyPolicyPath)
            .field("lpAssemblyDirectoryName", &self.lpAssemblyDirectoryName)
            .field("ulFileCount", &self.ulFileCount)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {}
impl ::core::default::Default for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    pub ElementCount: u32,
    pub Elements: [COMPATIBILITY_CONTEXT_ELEMENT; 1],
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION").field("ElementCount", &self.ElementCount).field("Elements", &self.Elements).finish()
    }
}
unsafe impl ::windows::core::Abi for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {}
impl ::core::default::Default for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    pub dwFlags: u32,
    pub ulFormatVersion: u32,
    pub ulAssemblyCount: u32,
    pub ulRootManifestPathType: u32,
    pub ulRootManifestPathChars: u32,
    pub ulRootConfigurationPathType: u32,
    pub ulRootConfigurationPathChars: u32,
    pub ulAppDirPathType: u32,
    pub ulAppDirPathChars: u32,
    pub lpRootManifestPath: ::windows::core::PCWSTR,
    pub lpRootConfigurationPath: ::windows::core::PCWSTR,
    pub lpAppDirPath: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_DETAILED_INFORMATION")
            .field("dwFlags", &self.dwFlags)
            .field("ulFormatVersion", &self.ulFormatVersion)
            .field("ulAssemblyCount", &self.ulAssemblyCount)
            .field("ulRootManifestPathType", &self.ulRootManifestPathType)
            .field("ulRootManifestPathChars", &self.ulRootManifestPathChars)
            .field("ulRootConfigurationPathType", &self.ulRootConfigurationPathType)
            .field("ulRootConfigurationPathChars", &self.ulRootConfigurationPathChars)
            .field("ulAppDirPathType", &self.ulAppDirPathType)
            .field("ulAppDirPathChars", &self.ulAppDirPathChars)
            .field("lpRootManifestPath", &self.lpRootManifestPath)
            .field("lpRootConfigurationPath", &self.lpRootConfigurationPath)
            .field("lpAppDirPath", &self.lpAppDirPath)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTIVATION_CONTEXT_DETAILED_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_DETAILED_INFORMATION {}
impl ::core::default::Default for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_QUERY_INDEX {
    pub ulAssemblyIndex: u32,
    pub ulFileIndexInAssembly: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_QUERY_INDEX {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_QUERY_INDEX").field("ulAssemblyIndex", &self.ulAssemblyIndex).field("ulFileIndexInAssembly", &self.ulFileIndexInAssembly).finish()
    }
}
unsafe impl ::windows::core::Abi for ACTIVATION_CONTEXT_QUERY_INDEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTIVATION_CONTEXT_QUERY_INDEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_QUERY_INDEX {}
impl ::core::default::Default for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    pub ulFlags: u32,
    pub RunLevel: ACTCTX_REQUESTED_RUN_LEVEL,
    pub UiAccess: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION").field("ulFlags", &self.ulFlags).field("RunLevel", &self.RunLevel).field("UiAccess", &self.UiAccess).finish()
    }
}
unsafe impl ::windows::core::Abi for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {}
impl ::core::default::Default for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADVERTISEFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ADVERTISEFLAGS_MACHINEASSIGN: ADVERTISEFLAGS = ADVERTISEFLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ADVERTISEFLAGS_USERASSIGN: ADVERTISEFLAGS = ADVERTISEFLAGS(1i32);
impl ::core::marker::Copy for ADVERTISEFLAGS {}
impl ::core::clone::Clone for ADVERTISEFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADVERTISEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ADVERTISEFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ADVERTISEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADVERTISEFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_FAIL_IF_CLOSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_FAIL_IF_EXACT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_TEST_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_VALID_FLAGS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ASM_BIND_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_FORCE_CACHE_INSTALL: ASM_BIND_FLAGS = ASM_BIND_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_RFS_INTEGRITY_CHECK: ASM_BIND_FLAGS = ASM_BIND_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_RFS_MODULE_CHECK: ASM_BIND_FLAGS = ASM_BIND_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_BINPATH_PROBE_ONLY: ASM_BIND_FLAGS = ASM_BIND_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_SHARED_BINPATH_HINT: ASM_BIND_FLAGS = ASM_BIND_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_PARENT_ASM_HINT: ASM_BIND_FLAGS = ASM_BIND_FLAGS(32u32);
impl ::core::marker::Copy for ASM_BIND_FLAGS {}
impl ::core::clone::Clone for ASM_BIND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ASM_BIND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ASM_BIND_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ASM_BIND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASM_BIND_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ASM_BIND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ASM_BIND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ASM_BIND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ASM_BIND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ASM_BIND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ASM_CMP_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_NAME: ASM_CMP_FLAGS = ASM_CMP_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_MAJOR_VERSION: ASM_CMP_FLAGS = ASM_CMP_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_MINOR_VERSION: ASM_CMP_FLAGS = ASM_CMP_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_BUILD_NUMBER: ASM_CMP_FLAGS = ASM_CMP_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_REVISION_NUMBER: ASM_CMP_FLAGS = ASM_CMP_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_PUBLIC_KEY_TOKEN: ASM_CMP_FLAGS = ASM_CMP_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_CULTURE: ASM_CMP_FLAGS = ASM_CMP_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_CUSTOM: ASM_CMP_FLAGS = ASM_CMP_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_ALL: ASM_CMP_FLAGS = ASM_CMP_FLAGS(255i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_DEFAULT: ASM_CMP_FLAGS = ASM_CMP_FLAGS(256i32);
impl ::core::marker::Copy for ASM_CMP_FLAGS {}
impl ::core::clone::Clone for ASM_CMP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ASM_CMP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ASM_CMP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ASM_CMP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASM_CMP_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ASM_DISPLAY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_VERSION: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_CULTURE: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_PUBLIC_KEY_TOKEN: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_PUBLIC_KEY: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_CUSTOM: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_PROCESSORARCHITECTURE: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_LANGUAGEID: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(64i32);
impl ::core::marker::Copy for ASM_DISPLAY_FLAGS {}
impl ::core::clone::Clone for ASM_DISPLAY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ASM_DISPLAY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ASM_DISPLAY_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ASM_DISPLAY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASM_DISPLAY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ASM_NAME(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_PUBLIC_KEY: ASM_NAME = ASM_NAME(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_PUBLIC_KEY_TOKEN: ASM_NAME = ASM_NAME(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_HASH_VALUE: ASM_NAME = ASM_NAME(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NAME: ASM_NAME = ASM_NAME(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MAJOR_VERSION: ASM_NAME = ASM_NAME(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MINOR_VERSION: ASM_NAME = ASM_NAME(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_BUILD_NUMBER: ASM_NAME = ASM_NAME(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_REVISION_NUMBER: ASM_NAME = ASM_NAME(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CULTURE: ASM_NAME = ASM_NAME(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_PROCESSOR_ID_ARRAY: ASM_NAME = ASM_NAME(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_OSINFO_ARRAY: ASM_NAME = ASM_NAME(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_HASH_ALGID: ASM_NAME = ASM_NAME(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_ALIAS: ASM_NAME = ASM_NAME(12i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CODEBASE_URL: ASM_NAME = ASM_NAME(13i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CODEBASE_LASTMOD: ASM_NAME = ASM_NAME(14i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NULL_PUBLIC_KEY: ASM_NAME = ASM_NAME(15i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NULL_PUBLIC_KEY_TOKEN: ASM_NAME = ASM_NAME(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CUSTOM: ASM_NAME = ASM_NAME(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NULL_CUSTOM: ASM_NAME = ASM_NAME(18i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MVID: ASM_NAME = ASM_NAME(19i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MAX_PARAMS: ASM_NAME = ASM_NAME(20i32);
impl ::core::marker::Copy for ASM_NAME {}
impl ::core::clone::Clone for ASM_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ASM_NAME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ASM_NAME {
    type Abi = Self;
}
impl ::core::fmt::Debug for ASM_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASM_NAME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASSEMBLYINFO_FLAG_INSTALLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASSEMBLYINFO_FLAG_PAYLOADRESIDENT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ASSEMBLY_FILE_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulFilenameLength: u32,
    pub ulPathLength: u32,
    pub lpFileName: ::windows::core::PCWSTR,
    pub lpFilePath: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for ASSEMBLY_FILE_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASSEMBLY_FILE_DETAILED_INFORMATION").field("ulFlags", &self.ulFlags).field("ulFilenameLength", &self.ulFilenameLength).field("ulPathLength", &self.ulPathLength).field("lpFileName", &self.lpFileName).field("lpFilePath", &self.lpFilePath).finish()
    }
}
unsafe impl ::windows::core::Abi for ASSEMBLY_FILE_DETAILED_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ASSEMBLY_FILE_DETAILED_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for ASSEMBLY_FILE_DETAILED_INFORMATION {}
impl ::core::default::Default for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ASSEMBLY_INFO {
    pub cbAssemblyInfo: u32,
    pub dwAssemblyFlags: u32,
    pub uliAssemblySizeInKB: u64,
    pub pszCurrentAssemblyPathBuf: ::windows::core::PWSTR,
    pub cchBuf: u32,
}
impl ::core::marker::Copy for ASSEMBLY_INFO {}
impl ::core::clone::Clone for ASSEMBLY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ASSEMBLY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASSEMBLY_INFO").field("cbAssemblyInfo", &self.cbAssemblyInfo).field("dwAssemblyFlags", &self.dwAssemblyFlags).field("uliAssemblySizeInKB", &self.uliAssemblySizeInKB).field("pszCurrentAssemblyPathBuf", &self.pszCurrentAssemblyPathBuf).field("cchBuf", &self.cchBuf).finish()
    }
}
unsafe impl ::windows::core::Abi for ASSEMBLY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ASSEMBLY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ASSEMBLY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for ASSEMBLY_INFO {}
impl ::core::default::Default for ASSEMBLY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ActivateActCtx<'a, P0>(hactctx: P0, lpcookie: *mut usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ActivateActCtx(hactctx: super::super::Foundation::HANDLE, lpcookie: *mut usize) -> super::super::Foundation::BOOL;
    }
    ActivateActCtx(hactctx.into(), ::core::mem::transmute(lpcookie))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddRefActCtx<'a, P0>(hactctx: P0)
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddRefActCtx(hactctx: super::super::Foundation::HANDLE);
    }
    AddRefActCtx(hactctx.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaA<'a, P0, P1, P2>(applyflags: i64, lpsourcename: P0, lpdeltaname: P1, lptargetname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyDeltaA(applyflags: i64, lpsourcename: ::windows::core::PCSTR, lpdeltaname: ::windows::core::PCSTR, lptargetname: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    ApplyDeltaA(applyflags, lpsourcename.into(), lpdeltaname.into(), lptargetname.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyDeltaB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    }
    ApplyDeltaB(applyflags, ::core::mem::transmute(source), ::core::mem::transmute(delta), ::core::mem::transmute(lptarget))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaGetReverseB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lpreversefiletime: *const super::super::Foundation::FILETIME, lptarget: *mut DELTA_OUTPUT, lptargetreverse: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyDeltaGetReverseB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lpreversefiletime: *const super::super::Foundation::FILETIME, lptarget: *mut DELTA_OUTPUT, lptargetreverse: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    }
    ApplyDeltaGetReverseB(applyflags, ::core::mem::transmute(source), ::core::mem::transmute(delta), ::core::mem::transmute(lpreversefiletime), ::core::mem::transmute(lptarget), ::core::mem::transmute(lptargetreverse))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaProvidedB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut ::core::ffi::c_void, utargetsize: usize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyDeltaProvidedB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut ::core::ffi::c_void, utargetsize: usize) -> super::super::Foundation::BOOL;
    }
    ApplyDeltaProvidedB(applyflags, ::core::mem::transmute(source), ::core::mem::transmute(delta), ::core::mem::transmute(lptarget), utargetsize)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaW<'a, P0, P1, P2>(applyflags: i64, lpsourcename: P0, lpdeltaname: P1, lptargetname: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyDeltaW(applyflags: i64, lpsourcename: ::windows::core::PCWSTR, lpdeltaname: ::windows::core::PCWSTR, lptargetname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    ApplyDeltaW(applyflags, lpsourcename.into(), lpdeltaname.into(), lptargetname.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileA<'a, P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyPatchToFileA(patchfilename: ::windows::core::PCSTR, oldfilename: ::windows::core::PCSTR, newfilename: ::windows::core::PCSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    }
    ApplyPatchToFileA(patchfilename.into(), oldfilename.into(), newfilename.into(), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileByBuffers(patchfilemapped: *const u8, patchfilesize: u32, oldfilemapped: *const u8, oldfilesize: u32, newfilebuffer: *mut *mut u8, newfilebuffersize: u32, newfileactualsize: *mut u32, newfiletime: *mut super::super::Foundation::FILETIME, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyPatchToFileByBuffers(patchfilemapped: *const u8, patchfilesize: u32, oldfilemapped: *const u8, oldfilesize: u32, newfilebuffer: *mut *mut u8, newfilebuffersize: u32, newfileactualsize: *mut u32, newfiletime: *mut super::super::Foundation::FILETIME, applyoptionflags: u32, progresscallback: *mut ::core::ffi::c_void, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    ApplyPatchToFileByBuffers(::core::mem::transmute(patchfilemapped), patchfilesize, ::core::mem::transmute(oldfilemapped), oldfilesize, ::core::mem::transmute(newfilebuffer), newfilebuffersize, ::core::mem::transmute(newfileactualsize), ::core::mem::transmute(newfiletime), applyoptionflags, ::core::mem::transmute(progresscallback), ::core::mem::transmute(callbackcontext))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileByHandles<'a, P0, P1, P2>(patchfilehandle: P0, oldfilehandle: P1, newfilehandle: P2, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyPatchToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    }
    ApplyPatchToFileByHandles(patchfilehandle.into(), oldfilehandle.into(), newfilehandle.into(), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileByHandlesEx<'a, P0, P1, P2>(patchfilehandle: P0, oldfilehandle: P1, newfilehandle: P2, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyPatchToFileByHandlesEx(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32, progresscallback: *mut ::core::ffi::c_void, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    ApplyPatchToFileByHandlesEx(patchfilehandle.into(), oldfilehandle.into(), newfilehandle.into(), applyoptionflags, ::core::mem::transmute(progresscallback), ::core::mem::transmute(callbackcontext))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileExA<'a, P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyPatchToFileExA(patchfilename: ::windows::core::PCSTR, oldfilename: ::windows::core::PCSTR, newfilename: ::windows::core::PCSTR, applyoptionflags: u32, progresscallback: *mut ::core::ffi::c_void, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    ApplyPatchToFileExA(patchfilename.into(), oldfilename.into(), newfilename.into(), applyoptionflags, ::core::mem::transmute(progresscallback), ::core::mem::transmute(callbackcontext))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileExW<'a, P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyPatchToFileExW(patchfilename: ::windows::core::PCWSTR, oldfilename: ::windows::core::PCWSTR, newfilename: ::windows::core::PCWSTR, applyoptionflags: u32, progresscallback: *mut ::core::ffi::c_void, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    ApplyPatchToFileExW(patchfilename.into(), oldfilename.into(), newfilename.into(), applyoptionflags, ::core::mem::transmute(progresscallback), ::core::mem::transmute(callbackcontext))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileW<'a, P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ApplyPatchToFileW(patchfilename: ::windows::core::PCWSTR, oldfilename: ::windows::core::PCWSTR, newfilename: ::windows::core::PCWSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    }
    ApplyPatchToFileW(patchfilename.into(), oldfilename.into(), newfilename.into(), applyoptionflags)
}
pub const CLSID_EvalCom2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e5e1910_8053_4660_b795_6b612e29bc58);
pub const CLSID_MsmMerge2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf94985d5_29f9_4743_9805_99bc3f35b678);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct COMPATIBILITY_CONTEXT_ELEMENT {
    pub Id: ::windows::core::GUID,
    pub Type: ACTCTX_COMPATIBILITY_ELEMENT_TYPE,
    pub MaxVersionTested: u64,
}
impl ::core::marker::Copy for COMPATIBILITY_CONTEXT_ELEMENT {}
impl ::core::clone::Clone for COMPATIBILITY_CONTEXT_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPATIBILITY_CONTEXT_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPATIBILITY_CONTEXT_ELEMENT").field("Id", &self.Id).field("Type", &self.Type).field("MaxVersionTested", &self.MaxVersionTested).finish()
    }
}
unsafe impl ::windows::core::Abi for COMPATIBILITY_CONTEXT_ELEMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPATIBILITY_CONTEXT_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPATIBILITY_CONTEXT_ELEMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPATIBILITY_CONTEXT_ELEMENT {}
impl ::core::default::Default for COMPATIBILITY_CONTEXT_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREATE_ASM_NAME_OBJ_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const CANOF_PARSE_DISPLAY_NAME: CREATE_ASM_NAME_OBJ_FLAGS = CREATE_ASM_NAME_OBJ_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const CANOF_SET_DEFAULT_VALUES: CREATE_ASM_NAME_OBJ_FLAGS = CREATE_ASM_NAME_OBJ_FLAGS(2i32);
impl ::core::marker::Copy for CREATE_ASM_NAME_OBJ_FLAGS {}
impl ::core::clone::Clone for CREATE_ASM_NAME_OBJ_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_ASM_NAME_OBJ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CREATE_ASM_NAME_OBJ_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CREATE_ASM_NAME_OBJ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_ASM_NAME_OBJ_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateActCtxA(pactctx: *const ACTCTXA) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateActCtxA(pactctx: *const ACTCTXA) -> super::super::Foundation::HANDLE;
    }
    let result__ = CreateActCtxA(::core::mem::transmute(pactctx));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateActCtxW(pactctx: *const ACTCTXW) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateActCtxW(pactctx: *const ACTCTXW) -> super::super::Foundation::HANDLE;
    }
    let result__ = CreateActCtxW(::core::mem::transmute(pactctx));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeltaA<'a, P0, P1, P2, P3, P4>(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: P0, lptargetname: P1, lpsourceoptionsname: P2, lptargetoptionsname: P3, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdeltaname: P4) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
    P4: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDeltaA(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: ::windows::core::PCSTR, lptargetname: ::windows::core::PCSTR, lpsourceoptionsname: ::windows::core::PCSTR, lptargetoptionsname: ::windows::core::PCSTR, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdeltaname: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    CreateDeltaA(filetypeset, setflags, resetflags, lpsourcename.into(), lptargetname.into(), lpsourceoptionsname.into(), lptargetoptionsname.into(), ::core::mem::transmute(globaloptions), ::core::mem::transmute(lptargetfiletime), hashalgid, lpdeltaname.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeltaB(filetypeset: i64, setflags: i64, resetflags: i64, source: DELTA_INPUT, target: DELTA_INPUT, sourceoptions: DELTA_INPUT, targetoptions: DELTA_INPUT, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdelta: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDeltaB(filetypeset: i64, setflags: i64, resetflags: i64, source: DELTA_INPUT, target: DELTA_INPUT, sourceoptions: DELTA_INPUT, targetoptions: DELTA_INPUT, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdelta: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    }
    CreateDeltaB(filetypeset, setflags, resetflags, ::core::mem::transmute(source), ::core::mem::transmute(target), ::core::mem::transmute(sourceoptions), ::core::mem::transmute(targetoptions), ::core::mem::transmute(globaloptions), ::core::mem::transmute(lptargetfiletime), hashalgid, ::core::mem::transmute(lpdelta))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeltaW<'a, P0, P1, P2, P3, P4>(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: P0, lptargetname: P1, lpsourceoptionsname: P2, lptargetoptionsname: P3, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdeltaname: P4) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
    P4: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDeltaW(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: ::windows::core::PCWSTR, lptargetname: ::windows::core::PCWSTR, lpsourceoptionsname: ::windows::core::PCWSTR, lptargetoptionsname: ::windows::core::PCWSTR, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdeltaname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    CreateDeltaW(filetypeset, setflags, resetflags, lpsourcename.into(), lptargetname.into(), lpsourceoptionsname.into(), lptargetoptionsname.into(), ::core::mem::transmute(globaloptions), ::core::mem::transmute(lptargetfiletime), hashalgid, lpdeltaname.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileA<'a, P0, P1, P2>(oldfilename: P0, newfilename: P1, patchfilename: P2, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePatchFileA(oldfilename: ::windows::core::PCSTR, newfilename: ::windows::core::PCSTR, patchfilename: ::windows::core::PCSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL;
    }
    CreatePatchFileA(oldfilename.into(), newfilename.into(), patchfilename.into(), optionflags, ::core::mem::transmute(optiondata))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileByHandles<'a, P0, P1, P2>(oldfilehandle: P0, newfilehandle: P1, patchfilehandle: P2, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePatchFileByHandles(oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, patchfilehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL;
    }
    CreatePatchFileByHandles(oldfilehandle.into(), newfilehandle.into(), patchfilehandle.into(), optionflags, ::core::mem::transmute(optiondata))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileByHandlesEx<'a, P0, P1>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_H], newfilehandle: P0, patchfilehandle: P1, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePatchFileByHandlesEx(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_H, newfilehandle: super::super::Foundation::HANDLE, patchfilehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: *mut ::core::ffi::c_void, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    CreatePatchFileByHandlesEx(oldfileinfoarray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(oldfileinfoarray)), newfilehandle.into(), patchfilehandle.into(), optionflags, ::core::mem::transmute(optiondata), ::core::mem::transmute(progresscallback), ::core::mem::transmute(callbackcontext))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileExA<'a, P0, P1>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_A], newfilename: P0, patchfilename: P1, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePatchFileExA(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_A, newfilename: ::windows::core::PCSTR, patchfilename: ::windows::core::PCSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: *mut ::core::ffi::c_void, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    CreatePatchFileExA(oldfileinfoarray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(oldfileinfoarray)), newfilename.into(), patchfilename.into(), optionflags, ::core::mem::transmute(optiondata), ::core::mem::transmute(progresscallback), ::core::mem::transmute(callbackcontext))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileExW<'a, P0, P1>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_W], newfilename: P0, patchfilename: P1, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePatchFileExW(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_W, newfilename: ::windows::core::PCWSTR, patchfilename: ::windows::core::PCWSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: *mut ::core::ffi::c_void, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    CreatePatchFileExW(oldfileinfoarray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(oldfileinfoarray)), newfilename.into(), patchfilename.into(), optionflags, ::core::mem::transmute(optiondata), ::core::mem::transmute(progresscallback), ::core::mem::transmute(callbackcontext))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileW<'a, P0, P1, P2>(oldfilename: P0, newfilename: P1, patchfilename: P2, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePatchFileW(oldfilename: ::windows::core::PCWSTR, newfilename: ::windows::core::PCWSTR, patchfilename: ::windows::core::PCWSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL;
    }
    CreatePatchFileW(oldfilename.into(), newfilename.into(), patchfilename.into(), optionflags, ::core::mem::transmute(optiondata))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DEFAULT_DISK_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DEFAULT_FILE_SEQUENCE_START: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DEFAULT_MINIMUM_REQUIRED_MSI_VERSION: u32 = 100u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct DELTA_HASH {
    pub HashSize: u32,
    pub HashValue: [u8; 32],
}
impl ::core::marker::Copy for DELTA_HASH {}
impl ::core::clone::Clone for DELTA_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DELTA_HASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DELTA_HASH").field("HashSize", &self.HashSize).field("HashValue", &self.HashValue).finish()
    }
}
unsafe impl ::windows::core::Abi for DELTA_HASH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DELTA_HASH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DELTA_HASH>()) == 0 }
    }
}
impl ::core::cmp::Eq for DELTA_HASH {}
impl ::core::default::Default for DELTA_HASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DELTA_HEADER_INFO {
    pub FileTypeSet: i64,
    pub FileType: i64,
    pub Flags: i64,
    pub TargetSize: usize,
    pub TargetFileTime: super::super::Foundation::FILETIME,
    pub TargetHashAlgId: u32,
    pub TargetHash: DELTA_HASH,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_HEADER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_HEADER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DELTA_HEADER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DELTA_HEADER_INFO").field("FileTypeSet", &self.FileTypeSet).field("FileType", &self.FileType).field("Flags", &self.Flags).field("TargetSize", &self.TargetSize).field("TargetFileTime", &self.TargetFileTime).field("TargetHashAlgId", &self.TargetHashAlgId).field("TargetHash", &self.TargetHash).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DELTA_HEADER_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DELTA_HEADER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DELTA_HEADER_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DELTA_HEADER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DELTA_HEADER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DELTA_INPUT {
    pub Anonymous: DELTA_INPUT_0,
    pub uSize: usize,
    pub Editable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DELTA_INPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DELTA_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DELTA_INPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DELTA_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DELTA_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DELTA_INPUT_0 {
    pub lpcStart: *const ::core::ffi::c_void,
    pub lpStart: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_INPUT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DELTA_INPUT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DELTA_INPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DELTA_INPUT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DELTA_INPUT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DELTA_INPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DELTA_MAX_HASH_SIZE: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct DELTA_OUTPUT {
    pub lpStart: *mut ::core::ffi::c_void,
    pub uSize: usize,
}
impl ::core::marker::Copy for DELTA_OUTPUT {}
impl ::core::clone::Clone for DELTA_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DELTA_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DELTA_OUTPUT").field("lpStart", &self.lpStart).field("uSize", &self.uSize).finish()
    }
}
unsafe impl ::windows::core::Abi for DELTA_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DELTA_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DELTA_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DELTA_OUTPUT {}
impl ::core::default::Default for DELTA_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeactivateActCtx(dwflags: u32, ulcookie: usize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeactivateActCtx(dwflags: u32, ulcookie: usize) -> super::super::Foundation::BOOL;
    }
    DeactivateActCtx(dwflags, ulcookie)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeltaFree(lpmemory: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeltaFree(lpmemory: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    DeltaFree(::core::mem::transmute(lpmemory))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeltaNormalizeProvidedB(filetypeset: i64, normalizeflags: i64, normalizeoptions: DELTA_INPUT, lpsource: *mut ::core::ffi::c_void, usourcesize: usize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeltaNormalizeProvidedB(filetypeset: i64, normalizeflags: i64, normalizeoptions: DELTA_INPUT, lpsource: *mut ::core::ffi::c_void, usourcesize: usize) -> super::super::Foundation::BOOL;
    }
    DeltaNormalizeProvidedB(filetypeset, normalizeflags, ::core::mem::transmute(normalizeoptions), ::core::mem::transmute(lpsource), usourcesize)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_BIGGER_THAN_COMPRESSED: u32 = 3222155525u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_CORRUPT: u32 = 3222159618u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_DECODE_FAILURE: u32 = 3222159617u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_ENCODE_FAILURE: u32 = 3222155521u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_IMAGEHLP_FAILURE: u32 = 3222155526u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_INVALID_OPTIONS: u32 = 3222155522u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_NEWER_FORMAT: u32 = 3222159619u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_NOT_AVAILABLE: u32 = 3222159622u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_NOT_NECESSARY: u32 = 3222159621u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_RETAIN_RANGES_DIFFER: u32 = 3222155524u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_SAME_FILE: u32 = 3222155523u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_WRONG_FILE: u32 = 3222159620u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_API_PATCHING_SYMBOL_FLAGS: u32 = 3222163725u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_FAMILY_RANGE_NAME: u32 = 3222163801u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_FILE_SEQUENCE_START: u32 = 3222163770u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_GUIDS_TO_REPLACE: u32 = 3222163721u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_DISKID: u32 = 3222163773u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_FILESEQSTART: u32 = 3222163774u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_NAME: u32 = 3222163748u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_SRC_PROP: u32 = 3222163750u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_MAJOR_VERSION: u32 = 3222163853u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_PATCH_GUID: u32 = 3222163720u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_PRODUCTVERSION_VALIDATION: u32 = 3222163844u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_SEQUENCE: u32 = 3222163848u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_SUPERCEDENCE: u32 = 3222163847u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET: u32 = 3222163849u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_NAME: u32 = 3222163736u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_CODE: u32 = 3222163834u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_VERSION: u32 = 3222163835u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADED: u32 = 3222163776u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADE_CODE: u32 = 3222163836u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_PRODUCT_CODE_LIST: u32 = 3222163722u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TGT_UPD_IMAGES: u32 = 3222163846u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TRANSFORMSET: u32 = 3222163845u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_FAMILY: u32 = 3222163775u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_NAME: u32 = 3222163728u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_CODE: u32 = 3222163831u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_VERSION: u32 = 3222163832u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_UPGRADE_CODE: u32 = 3222163833u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_VERSION_STRING: u32 = 3222163852u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BASE: u32 = 3222163713u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANNOT_CREATE_TABLE: u32 = 3222163841u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANNOT_RUN_MAKECAB: u32 = 3222163782u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANNOT_WRITE_DDF: u32 = 3222163781u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_COPY_FILE_TO_TEMP_FOLDER: u32 = 3222163771u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_ONE_PATCH_FILE: u32 = 3222163772u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_PATCH_FILE: u32 = 3222163718u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO: u32 = 3222163828u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO_POUND: u32 = 3222163830u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_TEMP_FOLDER: u32 = 3222163715u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_DELETE_TEMP_FOLDER: u32 = 3222163974u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_GENERATE_SEQUENCEINFO_MAJORUPGD: u32 = 3222163842u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM: u32 = 3222163827u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM_POUND: u32 = 3222163829u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_OVERWRITE_PATCH: u32 = 3222163717u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_READ_FILE: u32 = 3222163978u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CREATEFILE_LOG_FAILED: u32 = 3222163861u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUPLICATE_SEQUENCE_RECORD: u32 = 3222163858u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_IMAGE_FAMILY_NAME: u32 = 3222163749u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_TARGET_IMAGE_NAME: u32 = 3222163737u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_TARGET_IMAGE_PACKCODE: u32 = 3222163777u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_NAME: u32 = 3222163729u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_PACKCODE: u32 = 3222163795u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_ERROR_WRITING_TO_LOG: u32 = 3222163864u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXECUTE_VIEW: u32 = 3222163870u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_FAMILY_FIELD: u32 = 3222163756u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_LENGTHS: u32 = 3222163814u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_OFFSETS: u32 = 3222163812u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_RETAIN_OFFSETS: u32 = 3222163817u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BLANK_FILE_TABLE_KEY: u32 = 3222163755u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BLANK_PATH_TO_FILE: u32 = 3222163758u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_IGNORE_COUNT_MISMATCH: u32 = 3222163815u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_FILE_TABLE_KEY: u32 = 3222163754u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_LENGTHS: u32 = 3222163813u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_OFFSETS: u32 = 3222163811u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_PATH_TO_FILE: u32 = 3222163757u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_RETAIN_OFFSETS: u32 = 3222163816u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_MISSING_FILE: u32 = 3222163759u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAILED_CREATE_TRANSFORM: u32 = 3222163973u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAILED_EXPAND_PATH: u32 = 3222163872u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_LENGTHS: u32 = 3222163809u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_OFFSETS: u32 = 3222163806u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_FILE_TABLE_KEY: u32 = 3222163803u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_LENGTHS: u32 = 3222163808u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_OFFSETS: u32 = 3222163805u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_COUNT_MISMATCH: u32 = 3222163810u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_FILE_TABLE_KEY: u32 = 3222163802u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_LENGTHS: u32 = 3222163807u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_OFFSETS: u32 = 3222163804u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_NAME_TOO_LONG: u32 = 3222163800u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_IMAGE_FAMILY_NAME_TOO_LONG: u32 = 3222163747u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_IMAGE_PATH_NOT_EXIST: u32 = 3222163988u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INTERNAL_ERROR: u32 = 3222163969u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_LOG_LEVEL: u32 = 3222163862u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_MAJOR_VERSION: u32 = 3222163990u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PARAMETER: u32 = 3222163860u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PATCHMETADATA_PROP: u32 = 3222163856u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PATCH_TYPE_SEQUENCING: u32 = 3222163977u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_EXTERNALFILES: u32 = 3222163982u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_FAMILYFILERANGES: u32 = 3222163992u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_IMAGEFAMILIES: u32 = 3222163983u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_PATCHSEQUENCE: u32 = 3222163984u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_PROPERTIES: u32 = 3222163991u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_PROPERTY: u32 = 3222163970u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_TARGETFILES_OPTIONALDATA: u32 = 3222163985u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_TARGETIMAGES: u32 = 3222163971u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILESTOIGNORE: u32 = 3222163980u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILES_OPTIONALDATA: u32 = 3222163986u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDIMAGES: u32 = 3222163981u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_RANGE_ELEMENT: u32 = 3222163989u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_SUPERCEDENCE: u32 = 3222163857u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_SUPERSEDENCE_VALUE: u32 = 3222163976u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_UI_LEVEL: u32 = 3222163863u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_LAX_VALIDATION_FLAGS: u32 = 3222163972u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MAJOR_UPGD_WITHOUT_SEQUENCING: u32 = 3222163843u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MATCHED_PRODUCT_VERSIONS: u32 = 3222163837u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222163779u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222163780u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_DIRECTORY_TABLE: u32 = 3222163975u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_PATCHMETADATA: u32 = 3222163987u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_PATCH_GUID: u32 = 3222163719u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_PATCH_PATH: u32 = 3222163716u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_NO_UPGRADED_IMAGES_TO_PATCH: u32 = 3222163723u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_NULL_PATCHFAMILY: u32 = 3222163850u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_NULL_SEQUENCE_NUMBER: u32 = 3222163851u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OBSOLETION_WITH_MSI30: u32 = 3222163839u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222163840u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222163838u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OODS_COPYING_MSI: u32 = 3222163726u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OPEN_VIEW: u32 = 3222163869u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OUT_OF_MEMORY: u32 = 3222163865u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_PATCHMETADATA_PROP_NOT_SET: u32 = 3222163855u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_PCP_BAD_FORMAT: u32 = 3222163714u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_PCP_DOESNT_EXIST: u32 = 3222163713u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_SEQUENCING_BAD_TARGET: u32 = 3222163854u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_BAD_PROD_CODE_VAL: u32 = 3222163744u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_BAD_PROD_VALIDATE: u32 = 3222163743u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_COMPRESSED: u32 = 3222163742u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_NAME_TOO_LONG: u32 = 3222163735u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_EMPTY: u32 = 3222163739u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_EXIST: u32 = 3222163740u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_MSI: u32 = 3222163741u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_TOO_LONG: u32 = 3222163738u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_MISSING_SRC_FILES: u32 = 3222163746u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_WRONG_PRODUCT_VERSION_COMP: u32 = 3222163979u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_LENGTHS: u32 = 3222163822u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_OFFSETS: u32 = 3222163820u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_RETAIN_OFFSETS: u32 = 3222163825u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_TARGET_FIELD: u32 = 3222163791u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163789u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_IGNORE_COUNT_MISMATCH: u32 = 3222163823u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163788u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_LENGTHS: u32 = 3222163821u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_OFFSETS: u32 = 3222163819u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_RETAIN_OFFSETS: u32 = 3222163824u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163790u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_BAD_UPGRADED_FIELD: u32 = 3222163778u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163752u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163751u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163753u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_BAD_FILE_TABLE_KEY: u32 = 3222163799u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_BAD_UPGRADED_FIELD: u32 = 3222163796u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_BLANK_FILE_TABLE_KEY: u32 = 3222163798u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_LONG_FILE_TABLE_KEY: u32 = 3222163797u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UNKNOWN_ERROR: u32 = 3222163866u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UNKNOWN_INFO: u32 = 3222163867u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UNKNOWN_WARN: u32 = 3222163868u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_COMPRESSED: u32 = 3222163734u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_NAME_TOO_LONG: u32 = 3222163727u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_EXIST: u32 = 3222163793u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_MSI: u32 = 3222163794u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_TOO_LONG: u32 = 3222163792u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_EMPTY: u32 = 3222163731u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_EXIST: u32 = 3222163732u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_MSI: u32 = 3222163733u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_TOO_LONG: u32 = 3222163730u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_MISSING_SRC_FILES: u32 = 3222163745u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_VIEW_FETCH: u32 = 3222163871u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_WRITE_SUMMARY_PROPERTIES: u32 = 3222163787u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_WRONG_PATCHMETADATA_STRD_PROP: u32 = 3222163859u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_ROLLBACK_DISABLED: u32 = 1653u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtractPatchHeaderToFileA<'a, P0, P1>(patchfilename: P0, patchheaderfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExtractPatchHeaderToFileA(patchfilename: ::windows::core::PCSTR, patchheaderfilename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    ExtractPatchHeaderToFileA(patchfilename.into(), patchheaderfilename.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtractPatchHeaderToFileByHandles<'a, P0, P1>(patchfilehandle: P0, patchheaderfilehandle: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExtractPatchHeaderToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, patchheaderfilehandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    ExtractPatchHeaderToFileByHandles(patchfilehandle.into(), patchheaderfilehandle.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtractPatchHeaderToFileW<'a, P0, P1>(patchfilename: P0, patchheaderfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExtractPatchHeaderToFileW(patchfilename: ::windows::core::PCWSTR, patchheaderfilename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    ExtractPatchHeaderToFileW(patchfilename.into(), patchheaderfilename.into())
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct FUSION_INSTALL_REFERENCE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub guidScheme: ::windows::core::GUID,
    pub szIdentifier: ::windows::core::PCWSTR,
    pub szNonCannonicalData: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for FUSION_INSTALL_REFERENCE {}
impl ::core::clone::Clone for FUSION_INSTALL_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FUSION_INSTALL_REFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FUSION_INSTALL_REFERENCE").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("guidScheme", &self.guidScheme).field("szIdentifier", &self.szIdentifier).field("szNonCannonicalData", &self.szNonCannonicalData).finish()
    }
}
unsafe impl ::windows::core::Abi for FUSION_INSTALL_REFERENCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FUSION_INSTALL_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FUSION_INSTALL_REFERENCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for FUSION_INSTALL_REFERENCE {}
impl ::core::default::Default for FUSION_INSTALL_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FUSION_REFCOUNT_FILEPATH_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb02f9d65_fb77_4f7a_afa5_b391309f11c9);
pub const FUSION_REFCOUNT_OPAQUE_STRING_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ec93463_b0c3_45e1_8364_327e96aea856);
pub const FUSION_REFCOUNT_UNINSTALL_SUBKEY_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cedc215_ac4b_488b_93c0_a50a49cb2fb8);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FindActCtxSectionGuid(dwflags: u32, lpextensionguid: *const ::windows::core::GUID, ulsectionid: u32, lpguidtofind: *const ::windows::core::GUID, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindActCtxSectionGuid(dwflags: u32, lpextensionguid: *const ::windows::core::GUID, ulsectionid: u32, lpguidtofind: *const ::windows::core::GUID, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    }
    FindActCtxSectionGuid(dwflags, ::core::mem::transmute(lpextensionguid), ulsectionid, ::core::mem::transmute(lpguidtofind), ::core::mem::transmute(returneddata))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FindActCtxSectionStringA<'a, P0>(dwflags: u32, lpextensionguid: *const ::windows::core::GUID, ulsectionid: u32, lpstringtofind: P0, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindActCtxSectionStringA(dwflags: u32, lpextensionguid: *const ::windows::core::GUID, ulsectionid: u32, lpstringtofind: ::windows::core::PCSTR, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    }
    FindActCtxSectionStringA(dwflags, ::core::mem::transmute(lpextensionguid), ulsectionid, lpstringtofind.into(), ::core::mem::transmute(returneddata))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FindActCtxSectionStringW<'a, P0>(dwflags: u32, lpextensionguid: *const ::windows::core::GUID, ulsectionid: u32, lpstringtofind: P0, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindActCtxSectionStringW(dwflags: u32, lpextensionguid: *const ::windows::core::GUID, ulsectionid: u32, lpstringtofind: ::windows::core::PCWSTR, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    }
    FindActCtxSectionStringW(dwflags, ::core::mem::transmute(lpextensionguid), ulsectionid, lpstringtofind.into(), ::core::mem::transmute(returneddata))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentActCtx(lphactctx: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCurrentActCtx(lphactctx: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    GetCurrentActCtx(::core::mem::transmute(lphactctx))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaInfoA<'a, P0>(lpdeltaname: P0, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDeltaInfoA(lpdeltaname: ::windows::core::PCSTR, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    }
    GetDeltaInfoA(lpdeltaname.into(), ::core::mem::transmute(lpheaderinfo))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaInfoB(delta: DELTA_INPUT, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDeltaInfoB(delta: DELTA_INPUT, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    }
    GetDeltaInfoB(::core::mem::transmute(delta), ::core::mem::transmute(lpheaderinfo))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaInfoW<'a, P0>(lpdeltaname: P0, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDeltaInfoW(lpdeltaname: ::windows::core::PCWSTR, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    }
    GetDeltaInfoW(lpdeltaname.into(), ::core::mem::transmute(lpheaderinfo))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaSignatureA<'a, P0>(filetypeset: i64, hashalgid: u32, lpsourcename: P0, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDeltaSignatureA(filetypeset: i64, hashalgid: u32, lpsourcename: ::windows::core::PCSTR, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    }
    GetDeltaSignatureA(filetypeset, hashalgid, lpsourcename.into(), ::core::mem::transmute(lphash))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaSignatureB(filetypeset: i64, hashalgid: u32, source: DELTA_INPUT, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDeltaSignatureB(filetypeset: i64, hashalgid: u32, source: DELTA_INPUT, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    }
    GetDeltaSignatureB(filetypeset, hashalgid, ::core::mem::transmute(source), ::core::mem::transmute(lphash))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaSignatureW<'a, P0>(filetypeset: i64, hashalgid: u32, lpsourcename: P0, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDeltaSignatureW(filetypeset: i64, hashalgid: u32, lpsourcename: ::windows::core::PCWSTR, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    }
    GetDeltaSignatureW(filetypeset, hashalgid, lpsourcename.into(), ::core::mem::transmute(lphash))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureA<'a, P0>(filename: P0, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangearray: &[PATCH_IGNORE_RANGE], retainrangearray: &[PATCH_RETAIN_RANGE], signaturebuffersize: u32, signaturebuffer: ::windows::core::PSTR) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetFilePatchSignatureA(filename: ::windows::core::PCSTR, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: ::windows::core::PSTR) -> super::super::Foundation::BOOL;
    }
    GetFilePatchSignatureA(filename.into(), optionflags, ::core::mem::transmute(optiondata), ignorerangearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ignorerangearray)), retainrangearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(retainrangearray)), signaturebuffersize, ::core::mem::transmute(signaturebuffer))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureByBuffer(filebufferwritable: *mut u8, filesize: u32, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangearray: &[PATCH_IGNORE_RANGE], retainrangearray: &[PATCH_RETAIN_RANGE], signaturebuffersize: u32, signaturebuffer: ::windows::core::PSTR) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetFilePatchSignatureByBuffer(filebufferwritable: *mut u8, filesize: u32, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: ::windows::core::PSTR) -> super::super::Foundation::BOOL;
    }
    GetFilePatchSignatureByBuffer(::core::mem::transmute(filebufferwritable), filesize, optionflags, ::core::mem::transmute(optiondata), ignorerangearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ignorerangearray)), retainrangearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(retainrangearray)), signaturebuffersize, ::core::mem::transmute(signaturebuffer))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureByHandle<'a, P0>(filehandle: P0, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangearray: &[PATCH_IGNORE_RANGE], retainrangearray: &[PATCH_RETAIN_RANGE], signaturebuffersize: u32, signaturebuffer: ::windows::core::PSTR) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetFilePatchSignatureByHandle(filehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: ::windows::core::PSTR) -> super::super::Foundation::BOOL;
    }
    GetFilePatchSignatureByHandle(filehandle.into(), optionflags, ::core::mem::transmute(optiondata), ignorerangearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ignorerangearray)), retainrangearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(retainrangearray)), signaturebuffersize, ::core::mem::transmute(signaturebuffer))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureW<'a, P0>(filename: P0, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangearray: &[PATCH_IGNORE_RANGE], retainrangearray: &[PATCH_RETAIN_RANGE], signaturebuffersize: u32, signaturebuffer: ::windows::core::PWSTR) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetFilePatchSignatureW(filename: ::windows::core::PCWSTR, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: ::windows::core::PWSTR) -> super::super::Foundation::BOOL;
    }
    GetFilePatchSignatureW(filename.into(), optionflags, ::core::mem::transmute(optiondata), ignorerangearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ignorerangearray)), retainrangearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(retainrangearray)), signaturebuffersize, ::core::mem::transmute(signaturebuffer))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_ADMIN: &str = "ADMIN";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_ADVERTISE: &str = "ADVERTISE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_COLLECTUSERINFO: &str = "CollectUserInfo";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_FIRSTRUN: &str = "FirstRun";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_INSTALL: &str = "INSTALL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_SEQUENCE: &str = "SEQUENCE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_ALREADY_INSTALLED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_INSTALLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_REFRESHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_FLAG_REFRESH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IASSEMBLYCACHE_UNINSTALL_DISPOSITION(pub u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(1u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_STILL_IN_USE: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(2u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_ALREADY_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(3u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_DELETE_PENDING: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(4u32);
impl ::core::marker::Copy for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {}
impl ::core::clone::Clone for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    type Abi = Self;
}
impl ::core::fmt::Debug for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IASSEMBLYCACHE_UNINSTALL_DISPOSITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IAssemblyCache(::windows::core::IUnknown);
impl IAssemblyCache {
    pub unsafe fn UninstallAssembly<'a, P0>(&self, dwflags: u32, pszassemblyname: P0, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).UninstallAssembly)(::windows::core::Interface::as_raw(self), dwflags, pszassemblyname.into(), ::core::mem::transmute(prefdata), ::core::mem::transmute(puldisposition)).ok()
    }
    pub unsafe fn QueryAssemblyInfo<'a, P0>(&self, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: P0, pasminfo: *mut ASSEMBLY_INFO) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).QueryAssemblyInfo)(::windows::core::Interface::as_raw(self), dwflags, pszassemblyname.into(), ::core::mem::transmute(pasminfo)).ok()
    }
    pub unsafe fn CreateAssemblyCacheItem<'a, P0>(&self, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut ::core::option::Option<IAssemblyCacheItem>, pszassemblyname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CreateAssemblyCacheItem)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(pvreserved), ::core::mem::transmute(ppasmitem), pszassemblyname.into()).ok()
    }
    pub unsafe fn Reserved(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Reserved)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn InstallAssembly<'a, P0>(&self, dwflags: u32, pszmanifestfilepath: P0, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).InstallAssembly)(::windows::core::Interface::as_raw(self), dwflags, pszmanifestfilepath.into(), ::core::mem::transmute(prefdata)).ok()
    }
}
impl ::core::convert::From<IAssemblyCache> for ::windows::core::IUnknown {
    fn from(value: IAssemblyCache) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAssemblyCache> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAssemblyCache) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAssemblyCache> for ::windows::core::IUnknown {
    fn from(value: &IAssemblyCache) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAssemblyCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAssemblyCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAssemblyCache {}
impl ::core::fmt::Debug for IAssemblyCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAssemblyCache").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAssemblyCache {
    type Vtable = IAssemblyCache_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe707dcde_d1cd_11d2_bab9_00c04f8eceae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyCache_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub UninstallAssembly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszassemblyname: ::windows::core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows::core::HRESULT,
    pub QueryAssemblyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: ::windows::core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> ::windows::core::HRESULT,
    pub CreateAssemblyCacheItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut *mut ::core::ffi::c_void, pszassemblyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Reserved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InstallAssembly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszmanifestfilepath: ::windows::core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IAssemblyCacheItem(::windows::core::IUnknown);
impl IAssemblyCacheItem {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream<'a, P0>(&self, dwflags: u32, pszstreamname: P0, dwformat: u32, dwformatflags: u32, ppistream: *mut ::core::option::Option<super::Com::IStream>, pulimaxsize: *mut u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CreateStream)(::windows::core::Interface::as_raw(self), dwflags, pszstreamname.into(), dwformat, dwformatflags, ::core::mem::transmute(ppistream), ::core::mem::transmute(pulimaxsize)).ok()
    }
    pub unsafe fn Commit(&self, dwflags: u32, puldisposition: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(puldisposition)).ok()
    }
    pub unsafe fn AbortItem(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AbortItem)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IAssemblyCacheItem> for ::windows::core::IUnknown {
    fn from(value: IAssemblyCacheItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAssemblyCacheItem> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAssemblyCacheItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAssemblyCacheItem> for ::windows::core::IUnknown {
    fn from(value: &IAssemblyCacheItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAssemblyCacheItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAssemblyCacheItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAssemblyCacheItem {}
impl ::core::fmt::Debug for IAssemblyCacheItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAssemblyCacheItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAssemblyCacheItem {
    type Vtable = IAssemblyCacheItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e3aaeb4_d1cd_11d2_bab9_00c04f8eceae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyCacheItem_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszstreamname: ::windows::core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut *mut ::core::ffi::c_void, pulimaxsize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStream: usize,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, puldisposition: *mut u32) -> ::windows::core::HRESULT,
    pub AbortItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IAssemblyName(::windows::core::IUnknown);
impl IAssemblyName {
    pub unsafe fn SetProperty(&self, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), propertyid, ::core::mem::transmute(pvproperty), cbproperty).ok()
    }
    pub unsafe fn GetProperty(&self, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), propertyid, ::core::mem::transmute(pvproperty), ::core::mem::transmute(pcbproperty)).ok()
    }
    pub unsafe fn Finalize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finalize)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDisplayName(&self, szdisplayname: ::windows::core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(szdisplayname), ::core::mem::transmute(pccdisplayname), dwdisplayflags).ok()
    }
    pub unsafe fn Reserved<'a, P0, P1, P2>(&self, refiid: *const ::windows::core::GUID, punkreserved1: P0, punkreserved2: P1, szreserved: P2, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Reserved)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(refiid), punkreserved1.into().abi(), punkreserved2.into().abi(), szreserved.into(), llreserved, ::core::mem::transmute(pvreserved), cbreserved, ::core::mem::transmute(ppreserved)).ok()
    }
    pub unsafe fn GetName(&self, lpcwbuffer: *mut u32, pwzname: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpcwbuffer), ::core::mem::transmute(pwzname)).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwversionhi), ::core::mem::transmute(pdwversionlow)).ok()
    }
    pub unsafe fn IsEqual<'a, P0>(&self, pname: P0, dwcmpflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAssemblyName>>,
    {
        (::windows::core::Interface::vtable(self).IsEqual)(::windows::core::Interface::as_raw(self), pname.into().abi(), dwcmpflags).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IAssemblyName> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAssemblyName>(result__)
    }
}
impl ::core::convert::From<IAssemblyName> for ::windows::core::IUnknown {
    fn from(value: IAssemblyName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAssemblyName> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAssemblyName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAssemblyName> for ::windows::core::IUnknown {
    fn from(value: &IAssemblyName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAssemblyName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAssemblyName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAssemblyName {}
impl ::core::fmt::Debug for IAssemblyName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAssemblyName").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAssemblyName {
    type Vtable = IAssemblyName_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd193bc0_b4bc_11d2_9833_00c04fc31d2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyName_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows::core::HRESULT,
    pub Finalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szdisplayname: ::windows::core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows::core::HRESULT,
    pub Reserved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, punkreserved1: *mut ::core::ffi::c_void, punkreserved2: *mut ::core::ffi::c_void, szreserved: ::windows::core::PCWSTR, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcwbuffer: *mut u32, pwzname: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::core::ffi::c_void, dwcmpflags: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IEnumMsmDependency(::windows::core::IUnknown);
impl IEnumMsmDependency {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, cfetch: u32, rgmsmdependencies: *mut ::core::option::Option<IMsmDependency>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cfetch, ::core::mem::transmute(rgmsmdependencies), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cskip: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cskip).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumMsmDependency> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumMsmDependency>(result__)
    }
}
impl ::core::convert::From<IEnumMsmDependency> for ::windows::core::IUnknown {
    fn from(value: IEnumMsmDependency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumMsmDependency> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumMsmDependency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumMsmDependency> for ::windows::core::IUnknown {
    fn from(value: &IEnumMsmDependency) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumMsmDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumMsmDependency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumMsmDependency {}
impl ::core::fmt::Debug for IEnumMsmDependency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMsmDependency").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumMsmDependency {
    type Vtable = IEnumMsmDependency_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adda82c_2c26_11d2_ad65_00a0c9af11a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMsmDependency_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmdependencies: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pemsmdependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IEnumMsmError(::windows::core::IUnknown);
impl IEnumMsmError {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, cfetch: u32, rgmsmerrors: *mut ::core::option::Option<IMsmError>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cfetch, ::core::mem::transmute(rgmsmerrors), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cskip: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cskip).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumMsmError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumMsmError>(result__)
    }
}
impl ::core::convert::From<IEnumMsmError> for ::windows::core::IUnknown {
    fn from(value: IEnumMsmError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumMsmError> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumMsmError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumMsmError> for ::windows::core::IUnknown {
    fn from(value: &IEnumMsmError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumMsmError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumMsmError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumMsmError {}
impl ::core::fmt::Debug for IEnumMsmError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMsmError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumMsmError {
    type Vtable = IEnumMsmError_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adda829_2c26_11d2_ad65_00a0c9af11a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMsmError_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmerrors: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pemsmerrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IEnumMsmString(::windows::core::IUnknown);
impl IEnumMsmString {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, cfetch: u32, rgbstrstrings: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cfetch, ::core::mem::transmute(rgbstrstrings), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cskip: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cskip).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumMsmString> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumMsmString>(result__)
    }
}
impl ::core::convert::From<IEnumMsmString> for ::windows::core::IUnknown {
    fn from(value: IEnumMsmString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumMsmString> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumMsmString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumMsmString> for ::windows::core::IUnknown {
    fn from(value: &IEnumMsmString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumMsmString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumMsmString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumMsmString {}
impl ::core::fmt::Debug for IEnumMsmString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMsmString").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumMsmString {
    type Vtable = IEnumMsmString_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adda826_2c26_11d2_ad65_00a0c9af11a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMsmString_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfetch: u32, rgbstrstrings: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pemsmstrings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmDependencies(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmDependencies {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, item: i32) -> ::windows::core::Result<IMsmDependency> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), item, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMsmDependency>(result__)
    }
    pub unsafe fn Count(&self, count: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmDependencies> for ::windows::core::IUnknown {
    fn from(value: IMsmDependencies) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmDependencies> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMsmDependencies) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmDependencies> for ::windows::core::IUnknown {
    fn from(value: &IMsmDependencies) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmDependencies> for super::Com::IDispatch {
    fn from(value: IMsmDependencies) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmDependencies> for &'a super::Com::IDispatch {
    fn from(value: &'a IMsmDependencies) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmDependencies> for super::Com::IDispatch {
    fn from(value: &IMsmDependencies) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmDependencies {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmDependencies {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmDependencies {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmDependencies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmDependencies").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMsmDependencies {
    type Vtable = IMsmDependencies_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adda82d_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmDependencies_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmDependency(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmDependency {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Module(&self, module: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Module)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(module)).ok()
    }
    pub unsafe fn Language(&self, language: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Language)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(language)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self, version: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Version)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(version)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmDependency> for ::windows::core::IUnknown {
    fn from(value: IMsmDependency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmDependency> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMsmDependency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmDependency> for ::windows::core::IUnknown {
    fn from(value: &IMsmDependency) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmDependency> for super::Com::IDispatch {
    fn from(value: IMsmDependency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmDependency> for &'a super::Com::IDispatch {
    fn from(value: &'a IMsmDependency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmDependency> for super::Com::IDispatch {
    fn from(value: &IMsmDependency) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmDependency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmDependency {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmDependency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmDependency").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMsmDependency {
    type Vtable = IMsmDependency_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adda82b_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmDependency_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Module: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Module: usize,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Version: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmError(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmError {
    pub unsafe fn Type(&self, errortype: *mut msmErrorType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(errortype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self, errorpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Path)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(errorpath)).ok()
    }
    pub unsafe fn Language(&self, errorlanguage: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Language)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(errorlanguage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DatabaseTable(&self, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DatabaseTable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(errortable)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DatabaseKeys(&self) -> ::windows::core::Result<IMsmStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DatabaseKeys)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMsmStrings>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleTable(&self, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModuleTable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(errortable)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModuleKeys(&self) -> ::windows::core::Result<IMsmStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ModuleKeys)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMsmStrings>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmError> for ::windows::core::IUnknown {
    fn from(value: IMsmError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmError> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMsmError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmError> for ::windows::core::IUnknown {
    fn from(value: &IMsmError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmError> for super::Com::IDispatch {
    fn from(value: IMsmError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmError> for &'a super::Com::IDispatch {
    fn from(value: &'a IMsmError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmError> for super::Com::IDispatch {
    fn from(value: &IMsmError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmError {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmError").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMsmError {
    type Vtable = IMsmError_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adda828_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmError_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errortype: *mut msmErrorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorlanguage: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DatabaseTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DatabaseTable: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DatabaseKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorkeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DatabaseKeys: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ModuleTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errortable: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModuleTable: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ModuleKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorkeys: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModuleKeys: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmErrors(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmErrors {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, item: i32) -> ::windows::core::Result<IMsmError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), item, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMsmError>(result__)
    }
    pub unsafe fn Count(&self, count: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmErrors> for ::windows::core::IUnknown {
    fn from(value: IMsmErrors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmErrors> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMsmErrors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmErrors> for ::windows::core::IUnknown {
    fn from(value: &IMsmErrors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmErrors> for super::Com::IDispatch {
    fn from(value: IMsmErrors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmErrors> for &'a super::Com::IDispatch {
    fn from(value: &'a IMsmErrors) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmErrors> for super::Com::IDispatch {
    fn from(value: &IMsmErrors) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmErrors {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmErrors {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmErrors {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmErrors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmErrors").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMsmErrors {
    type Vtable = IMsmErrors_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adda82a_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmErrors_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmGetFiles(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmGetFiles {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModuleFiles(&self) -> ::windows::core::Result<IMsmStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ModuleFiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMsmStrings>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmGetFiles> for ::windows::core::IUnknown {
    fn from(value: IMsmGetFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmGetFiles> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMsmGetFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmGetFiles> for ::windows::core::IUnknown {
    fn from(value: &IMsmGetFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmGetFiles> for super::Com::IDispatch {
    fn from(value: IMsmGetFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmGetFiles> for &'a super::Com::IDispatch {
    fn from(value: &'a IMsmGetFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmGetFiles> for super::Com::IDispatch {
    fn from(value: &IMsmGetFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmGetFiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmGetFiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmGetFiles {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmGetFiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmGetFiles").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMsmGetFiles {
    type Vtable = IMsmGetFiles_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7041ae26_2d78_11d2_888a_00a0c981b015);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmGetFiles_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ModuleFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModuleFiles: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmMerge(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmMerge {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenDatabase<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).OpenDatabase)(::windows::core::Interface::as_raw(self), path.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenModule<'a, P0>(&self, path: P0, language: i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).OpenModule)(::windows::core::Interface::as_raw(self), path.into().abi(), language).ok()
    }
    pub unsafe fn CloseDatabase(&self, commit: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseDatabase)(::windows::core::Interface::as_raw(self), commit).ok()
    }
    pub unsafe fn CloseModule(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseModule)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenLog<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).OpenLog)(::windows::core::Interface::as_raw(self), path.into().abi()).ok()
    }
    pub unsafe fn CloseLog(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseLog)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Log<'a, P0>(&self, message: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).Log)(::windows::core::Interface::as_raw(self), message.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Errors(&self) -> ::windows::core::Result<IMsmErrors> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Errors)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMsmErrors>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Dependencies(&self) -> ::windows::core::Result<IMsmDependencies> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Dependencies)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMsmDependencies>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Merge<'a, P0, P1>(&self, feature: P0, redirectdir: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).Merge)(::windows::core::Interface::as_raw(self), feature.into().abi(), redirectdir.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, P0>(&self, feature: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self), feature.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtractCAB<'a, P0>(&self, filename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ExtractCAB)(::windows::core::Interface::as_raw(self), filename.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtractFiles<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ExtractFiles)(::windows::core::Interface::as_raw(self), path.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmMerge> for ::windows::core::IUnknown {
    fn from(value: IMsmMerge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmMerge> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMsmMerge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmMerge> for ::windows::core::IUnknown {
    fn from(value: &IMsmMerge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmMerge> for super::Com::IDispatch {
    fn from(value: IMsmMerge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmMerge> for &'a super::Com::IDispatch {
    fn from(value: &'a IMsmMerge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmMerge> for super::Com::IDispatch {
    fn from(value: &IMsmMerge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmMerge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmMerge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmMerge {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmMerge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmMerge").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMsmMerge {
    type Vtable = IMsmMerge_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adda82e_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmMerge_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenDatabase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenDatabase: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenModule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, language: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenModule: usize,
    pub CloseDatabase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commit: i16) -> ::windows::core::HRESULT,
    pub CloseModule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenLog: usize,
    pub CloseLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Log: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Errors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Errors: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Dependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Dependencies: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Merge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, redirectdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Merge: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtractCAB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtractCAB: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtractFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtractFiles: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmStrings(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmStrings {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_Item(&self, item: i32, r#return: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), item, ::core::mem::transmute(r#return)).ok()
    }
    pub unsafe fn Count(&self, count: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmStrings> for ::windows::core::IUnknown {
    fn from(value: IMsmStrings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmStrings> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMsmStrings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmStrings> for ::windows::core::IUnknown {
    fn from(value: &IMsmStrings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMsmStrings> for super::Com::IDispatch {
    fn from(value: IMsmStrings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMsmStrings> for &'a super::Com::IDispatch {
    fn from(value: &'a IMsmStrings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMsmStrings> for super::Com::IDispatch {
    fn from(value: &IMsmStrings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmStrings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmStrings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmStrings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmStrings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmStrings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMsmStrings {
    type Vtable = IMsmStrings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adda827_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmStrings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_BASE: u32 = 3222229249u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_I: u32 = 3222229251u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_II: u32 = 3222229256u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_III: u32 = 3222229257u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_IV: u32 = 3222229258u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_I_VALIDATION: u32 = 3222229250u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_V: u32 = 3222229259u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_GENERATING_METADATA: u32 = 3222229265u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PASSED_MAIN_CONTROL: u32 = 3222229249u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PATCHCACHE_FILEINFO_FAILURE: u32 = 3222229267u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PATCHCACHE_PCI_READFAILURE: u32 = 3222229268u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PATCHCACHE_PCI_WRITEFAILURE: u32 = 3222229269u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PCP_PATH: u32 = 3222229252u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PROPERTY: u32 = 3222229255u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_SET_OPTIONS: u32 = 3222229254u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_SUCCESSFUL_PATCH_CREATION: u32 = 3222229271u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_TEMP_DIR: u32 = 3222229253u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_TEMP_DIR_CLEANUP: u32 = 3222229266u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_USING_USER_MSI_FOR_PATCH_TABLES: u32 = 3222229270u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLFEATUREATTRIBUTE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORLOCAL: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORSOURCE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FOLLOWPARENT: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORADVERTISE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_DISALLOWADVERTISE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_NOUNSUPPORTEDADVERTISE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(32i32);
impl ::core::marker::Copy for INSTALLFEATUREATTRIBUTE {}
impl ::core::clone::Clone for INSTALLFEATUREATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLFEATUREATTRIBUTE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSTALLFEATUREATTRIBUTE {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTALLFEATUREATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLFEATUREATTRIBUTE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLLEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLEVEL_DEFAULT: INSTALLLEVEL = INSTALLLEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLEVEL_MINIMUM: INSTALLLEVEL = INSTALLLEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLEVEL_MAXIMUM: INSTALLLEVEL = INSTALLLEVEL(65535i32);
impl ::core::marker::Copy for INSTALLLEVEL {}
impl ::core::clone::Clone for INSTALLLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSTALLLEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTALLLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLLEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLLOGATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGATTRIBUTES_APPEND: INSTALLLOGATTRIBUTES = INSTALLLOGATTRIBUTES(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGATTRIBUTES_FLUSHEACHLINE: INSTALLLOGATTRIBUTES = INSTALLLOGATTRIBUTES(2i32);
impl ::core::marker::Copy for INSTALLLOGATTRIBUTES {}
impl ::core::clone::Clone for INSTALLLOGATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLLOGATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSTALLLOGATTRIBUTES {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTALLLOGATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLLOGATTRIBUTES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLMESSAGE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_FATALEXIT: INSTALLMESSAGE = INSTALLMESSAGE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_ERROR: INSTALLMESSAGE = INSTALLMESSAGE(16777216i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_WARNING: INSTALLMESSAGE = INSTALLMESSAGE(33554432i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_USER: INSTALLMESSAGE = INSTALLMESSAGE(50331648i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INFO: INSTALLMESSAGE = INSTALLMESSAGE(67108864i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_FILESINUSE: INSTALLMESSAGE = INSTALLMESSAGE(83886080i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_RESOLVESOURCE: INSTALLMESSAGE = INSTALLMESSAGE(100663296i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_OUTOFDISKSPACE: INSTALLMESSAGE = INSTALLMESSAGE(117440512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_ACTIONSTART: INSTALLMESSAGE = INSTALLMESSAGE(134217728i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_ACTIONDATA: INSTALLMESSAGE = INSTALLMESSAGE(150994944i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_PROGRESS: INSTALLMESSAGE = INSTALLMESSAGE(167772160i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_COMMONDATA: INSTALLMESSAGE = INSTALLMESSAGE(184549376i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INITIALIZE: INSTALLMESSAGE = INSTALLMESSAGE(201326592i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_TERMINATE: INSTALLMESSAGE = INSTALLMESSAGE(218103808i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_SHOWDIALOG: INSTALLMESSAGE = INSTALLMESSAGE(234881024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_PERFORMANCE: INSTALLMESSAGE = INSTALLMESSAGE(251658240i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_RMFILESINUSE: INSTALLMESSAGE = INSTALLMESSAGE(419430400i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INSTALLSTART: INSTALLMESSAGE = INSTALLMESSAGE(436207616i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INSTALLEND: INSTALLMESSAGE = INSTALLMESSAGE(452984832i32);
impl ::core::marker::Copy for INSTALLMESSAGE {}
impl ::core::clone::Clone for INSTALLMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLMESSAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSTALLMESSAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTALLMESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLMESSAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_TYPEMASK: i32 = -16777216i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLMODE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_NODETECTION_ANY: INSTALLMODE = INSTALLMODE(-4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_NOSOURCERESOLUTION: INSTALLMODE = INSTALLMODE(-3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_NODETECTION: INSTALLMODE = INSTALLMODE(-2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_EXISTING: INSTALLMODE = INSTALLMODE(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_DEFAULT: INSTALLMODE = INSTALLMODE(0i32);
impl ::core::marker::Copy for INSTALLMODE {}
impl ::core::clone::Clone for INSTALLMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSTALLMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTALLMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLOGMODE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_FATALEXIT: INSTALLOGMODE = INSTALLOGMODE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_ERROR: INSTALLOGMODE = INSTALLOGMODE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_WARNING: INSTALLOGMODE = INSTALLOGMODE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_USER: INSTALLOGMODE = INSTALLOGMODE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INFO: INSTALLOGMODE = INSTALLOGMODE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_RESOLVESOURCE: INSTALLOGMODE = INSTALLOGMODE(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_OUTOFDISKSPACE: INSTALLOGMODE = INSTALLOGMODE(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_ACTIONSTART: INSTALLOGMODE = INSTALLOGMODE(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_ACTIONDATA: INSTALLOGMODE = INSTALLOGMODE(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_COMMONDATA: INSTALLOGMODE = INSTALLOGMODE(2048i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_PROPERTYDUMP: INSTALLOGMODE = INSTALLOGMODE(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_VERBOSE: INSTALLOGMODE = INSTALLOGMODE(4096i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_EXTRADEBUG: INSTALLOGMODE = INSTALLOGMODE(8192i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_LOGONLYONERROR: INSTALLOGMODE = INSTALLOGMODE(16384i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_LOGPERFORMANCE: INSTALLOGMODE = INSTALLOGMODE(32768i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_PROGRESS: INSTALLOGMODE = INSTALLOGMODE(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INITIALIZE: INSTALLOGMODE = INSTALLOGMODE(4096i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_TERMINATE: INSTALLOGMODE = INSTALLOGMODE(8192i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_SHOWDIALOG: INSTALLOGMODE = INSTALLOGMODE(16384i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_FILESINUSE: INSTALLOGMODE = INSTALLOGMODE(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_RMFILESINUSE: INSTALLOGMODE = INSTALLOGMODE(33554432i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INSTALLSTART: INSTALLOGMODE = INSTALLOGMODE(67108864i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INSTALLEND: INSTALLOGMODE = INSTALLOGMODE(134217728i32);
impl ::core::marker::Copy for INSTALLOGMODE {}
impl ::core::clone::Clone for INSTALLOGMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLOGMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSTALLOGMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTALLOGMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLOGMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_ASSIGNMENTTYPE: &str = "AssignmentType";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_AUTHORIZED_LUA_APP: &str = "AuthorizedLUAApp";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_DISKPROMPT: &str = "DiskPrompt";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_DISPLAYNAME: &str = "DisplayName";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_HELPLINK: &str = "HelpLink";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_HELPTELEPHONE: &str = "HelpTelephone";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLDATE: &str = "InstallDate";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLEDLANGUAGE: &str = "InstalledLanguage";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLEDPRODUCTNAME: &str = "InstalledProductName";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLLOCATION: &str = "InstallLocation";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLSOURCE: &str = "InstallSource";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTANCETYPE: &str = "InstanceType";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LANGUAGE: &str = "Language";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LASTUSEDSOURCE: &str = "LastUsedSource";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LASTUSEDTYPE: &str = "LastUsedType";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LOCALPACKAGE: &str = "LocalPackage";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LUAENABLED: &str = "LUAEnabled";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_MEDIAPACKAGEPATH: &str = "MediaPackagePath";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_MOREINFOURL: &str = "MoreInfoURL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PACKAGECODE: &str = "PackageCode";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PACKAGENAME: &str = "PackageName";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PATCHSTATE: &str = "State";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PATCHTYPE: &str = "PatchType";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTICON: &str = "ProductIcon";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTID: &str = "ProductID";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTNAME: &str = "ProductName";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTSTATE: &str = "State";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PUBLISHER: &str = "Publisher";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_REGCOMPANY: &str = "RegCompany";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_REGOWNER: &str = "RegOwner";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_TRANSFORMS: &str = "Transforms";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_UNINSTALLABLE: &str = "Uninstallable";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_URLINFOABOUT: &str = "URLInfoAbout";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_URLUPDATEINFO: &str = "URLUpdateInfo";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSION: &str = "Version";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSIONMAJOR: &str = "VersionMajor";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSIONMINOR: &str = "VersionMinor";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSIONSTRING: &str = "VersionString";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLSTATE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_NOTUSED: INSTALLSTATE = INSTALLSTATE(-7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_BADCONFIG: INSTALLSTATE = INSTALLSTATE(-6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_INCOMPLETE: INSTALLSTATE = INSTALLSTATE(-5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_SOURCEABSENT: INSTALLSTATE = INSTALLSTATE(-4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_MOREDATA: INSTALLSTATE = INSTALLSTATE(-3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_INVALIDARG: INSTALLSTATE = INSTALLSTATE(-2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_UNKNOWN: INSTALLSTATE = INSTALLSTATE(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_BROKEN: INSTALLSTATE = INSTALLSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_ADVERTISED: INSTALLSTATE = INSTALLSTATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_REMOVED: INSTALLSTATE = INSTALLSTATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_ABSENT: INSTALLSTATE = INSTALLSTATE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_LOCAL: INSTALLSTATE = INSTALLSTATE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_SOURCE: INSTALLSTATE = INSTALLSTATE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_DEFAULT: INSTALLSTATE = INSTALLSTATE(5i32);
impl ::core::marker::Copy for INSTALLSTATE {}
impl ::core::clone::Clone for INSTALLSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSTALLSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTALLSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLTYPE_DEFAULT: INSTALLTYPE = INSTALLTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLTYPE_NETWORK_IMAGE: INSTALLTYPE = INSTALLTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLTYPE_SINGLE_INSTANCE: INSTALLTYPE = INSTALLTYPE(2i32);
impl ::core::marker::Copy for INSTALLTYPE {}
impl ::core::clone::Clone for INSTALLTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSTALLTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTALLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLUILEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_NOCHANGE: INSTALLUILEVEL = INSTALLUILEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_DEFAULT: INSTALLUILEVEL = INSTALLUILEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_NONE: INSTALLUILEVEL = INSTALLUILEVEL(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_BASIC: INSTALLUILEVEL = INSTALLUILEVEL(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_REDUCED: INSTALLUILEVEL = INSTALLUILEVEL(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_FULL: INSTALLUILEVEL = INSTALLUILEVEL(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_ENDDIALOG: INSTALLUILEVEL = INSTALLUILEVEL(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_PROGRESSONLY: INSTALLUILEVEL = INSTALLUILEVEL(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_HIDECANCEL: INSTALLUILEVEL = INSTALLUILEVEL(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_SOURCERESONLY: INSTALLUILEVEL = INSTALLUILEVEL(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_UACONLY: INSTALLUILEVEL = INSTALLUILEVEL(512i32);
impl ::core::marker::Copy for INSTALLUILEVEL {}
impl ::core::clone::Clone for INSTALLUILEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLUILEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSTALLUILEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTALLUILEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLUILEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLUI_HANDLERA = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: ::windows::core::PCSTR) -> i32>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLUI_HANDLERW = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: ::windows::core::PCWSTR) -> i32>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMApplicationInfo(::windows::core::IUnknown);
impl IPMApplicationInfo {
    pub unsafe fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProductID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn InstanceID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).InstanceID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn OfferID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).OfferID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefaultTask(&self, pdefaulttask: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DefaultTask)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdefaulttask)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AppTitle(&self, papptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AppTitle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(papptitle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IconPath(&self, pappiconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IconPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pappiconpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotificationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NotificationState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn AppInstallType(&self) -> ::windows::core::Result<PM_APPLICATION_INSTALL_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AppInstallType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PM_APPLICATION_INSTALL_TYPE>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<PM_APPLICATION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PM_APPLICATION_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRevoked(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsRevoked)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateAvailable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).UpdateAvailable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallDate(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).InstallDate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUninstallable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsUninstallable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThemable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsThemable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTrial(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsTrial)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallPath(&self, pinstallpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InstallPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinstallpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DataRoot(&self, pdataroot: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DataRoot)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdataroot)).ok()
    }
    pub unsafe fn Genre(&self) -> ::windows::core::Result<PM_APP_GENRE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Genre)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PM_APP_GENRE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Publisher(&self, ppublisher: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Publisher)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppublisher)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Author(&self, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Author)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pauthor)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdescription)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Version)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pversion)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_InvocationInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn AppPlatMajorVersion(&self) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AppPlatMajorVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    pub unsafe fn AppPlatMinorVersion(&self) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AppPlatMinorVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    pub unsafe fn PublisherID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PublisherID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMultiCore(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsMultiCore)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SID(&self, psid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psid)).ok()
    }
    pub unsafe fn AppPlatMajorVersionLightUp(&self) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AppPlatMajorVersionLightUp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    pub unsafe fn AppPlatMinorVersionLightUp(&self) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AppPlatMinorVersionLightUp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_UpdateAvailable<'a, P0>(&self, isupdateavailable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_UpdateAvailable)(::windows::core::Interface::as_raw(self), isupdateavailable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_NotificationState<'a, P0>(&self, isnotified: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_NotificationState)(::windows::core::Interface::as_raw(self), isnotified.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IconPath<'a, P0>(&self, appiconpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).set_IconPath)(::windows::core::Interface::as_raw(self), appiconpath.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_UninstallableState<'a, P0>(&self, isuninstallable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_UninstallableState)(::windows::core::Interface::as_raw(self), isuninstallable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinableOnKidZone(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsPinableOnKidZone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOriginallyPreInstalled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOriginallyPreInstalled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInstallOnSD(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsInstallOnSD)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOptoutOnSD(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOptoutOnSD)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOptoutBackupRestore(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOptoutBackupRestore)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_EnterpriseDisabled<'a, P0>(&self, isdisabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_EnterpriseDisabled)(::windows::core::Interface::as_raw(self), isdisabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_EnterpriseUninstallable<'a, P0>(&self, isuninstallable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_EnterpriseUninstallable)(::windows::core::Interface::as_raw(self), isuninstallable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnterpriseDisabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnterpriseDisabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnterpriseUninstallable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnterpriseUninstallable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVisibleOnAppList(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsVisibleOnAppList)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInboxApp(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsInboxApp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn StorageID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StorageID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartAppBlob(&self, pblob: *mut PM_STARTAPPBLOB) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartAppBlob)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblob)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMovable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsMovable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn DeploymentAppEnumerationHubFilter(&self) -> ::windows::core::Result<PM_TILE_HUBTYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DeploymentAppEnumerationHubFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PM_TILE_HUBTYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModifiedDate(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ModifiedDate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOriginallyRestored(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOriginallyRestored)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShouldDeferMdilBind(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ShouldDeferMdilBind)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFullyPreInstall(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsFullyPreInstall)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsMdilMaintenanceNeeded<'a, P0>(&self, fismdilmaintenanceneeded: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_IsMdilMaintenanceNeeded)(::windows::core::Interface::as_raw(self), fismdilmaintenanceneeded.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_Title<'a, P0>(&self, apptitle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).set_Title)(::windows::core::Interface::as_raw(self), apptitle.into().abi()).ok()
    }
}
impl ::core::convert::From<IPMApplicationInfo> for ::windows::core::IUnknown {
    fn from(value: IPMApplicationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMApplicationInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMApplicationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMApplicationInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMApplicationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMApplicationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMApplicationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMApplicationInfo {}
impl ::core::fmt::Debug for IPMApplicationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMApplicationInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMApplicationInfo {
    type Vtable = IPMApplicationInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50afb58a_438c_4088_9789_f8c4899829c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMApplicationInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub InstanceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OfferID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pofferid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DefaultTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdefaulttask: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DefaultTask: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AppTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AppTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IconPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappiconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IconPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NotificationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotificationState: usize,
    pub AppInstallType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut PM_APPLICATION_STATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisrevoked: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRevoked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisupdateavailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateAvailable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstalldate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallDate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUninstallable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisuninstallable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUninstallable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsThemable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsThemable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistrial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTrial: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstallpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DataRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataroot: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DataRoot: usize,
    pub Genre: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgenre: *mut PM_APP_GENRE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Publisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppublisher: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Publisher: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Author: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Version: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_InvocationInfo: usize,
    pub AppPlatMajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows::core::HRESULT,
    pub AppPlatMinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows::core::HRESULT,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppublisherid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMultiCore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pismulticore: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMultiCore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SID: usize,
    pub AppPlatMajorVersionLightUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows::core::HRESULT,
    pub AppPlatMinorVersionLightUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub set_UpdateAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isupdateavailable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_UpdateAvailable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_NotificationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isnotified: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_NotificationState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_IconPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appiconpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_IconPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_UninstallableState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_UninstallableState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinableOnKidZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pispinable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinableOnKidZone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOriginallyPreInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pispreinstalled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOriginallyPreInstalled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInstallOnSD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisinstallonsd: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInstallOnSD: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOptoutOnSD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisoptoutonsd: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOptoutOnSD: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOptoutBackupRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisoptoutbackuprestore: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOptoutBackupRestore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_EnterpriseDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isdisabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_EnterpriseDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_EnterpriseUninstallable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_EnterpriseUninstallable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnterpriseDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnterpriseDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnterpriseUninstallable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isuninstallable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnterpriseUninstallable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVisibleOnAppList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisvisible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVisibleOnAppList: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInboxApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisinboxapp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInboxApp: usize,
    pub StorageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstorageid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StartAppBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTAPPBLOB) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartAppBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMovable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pismovable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMovable: usize,
    pub DeploymentAppEnumerationHubFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: *mut PM_TILE_HUBTYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ModifiedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmodifieddate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModifiedDate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOriginallyRestored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisrestored: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOriginallyRestored: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShouldDeferMdilBind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfdefermdilbind: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShouldDeferMdilBind: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFullyPreInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisfullypreinstall: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFullyPreInstall: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_IsMdilMaintenanceNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_IsMdilMaintenanceNeeded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apptitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_Title: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMApplicationInfoEnumerator(::windows::core::IUnknown);
impl IPMApplicationInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::core::Result<IPMApplicationInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMApplicationInfo>(result__)
    }
}
impl ::core::convert::From<IPMApplicationInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: IPMApplicationInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMApplicationInfoEnumerator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMApplicationInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMApplicationInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IPMApplicationInfoEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMApplicationInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMApplicationInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMApplicationInfoEnumerator {}
impl ::core::fmt::Debug for IPMApplicationInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMApplicationInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMApplicationInfoEnumerator {
    type Vtable = IPMApplicationInfoEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ec42a96_4d46_4dc6_a3d9_a7acaac0f5fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMApplicationInfoEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMBackgroundServiceAgentInfo(::windows::core::IUnknown);
impl IPMBackgroundServiceAgentInfo {
    pub unsafe fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProductID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskID(&self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TaskID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptaskid)).ok()
    }
    pub unsafe fn BSAID(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BSAID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BGSpecifier(&self, pbgspecifier: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BGSpecifier)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbgspecifier)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BGName(&self, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BGName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbgname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BGSource(&self, pbgsource: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BGSource)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbgsource)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BGType(&self, pbgtype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BGType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbgtype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPeriodic(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsPeriodic)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsScheduled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsScheduled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsScheduleAllowed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsScheduleAllowed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdescription)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLaunchOnBoot(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsLaunchOnBoot)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsScheduled<'a, P0>(&self, isscheduled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_IsScheduled)(::windows::core::Interface::as_raw(self), isscheduled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsScheduleAllowed<'a, P0>(&self, isscheduleallowed: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_IsScheduleAllowed)(::windows::core::Interface::as_raw(self), isscheduleallowed.into()).ok()
    }
}
impl ::core::convert::From<IPMBackgroundServiceAgentInfo> for ::windows::core::IUnknown {
    fn from(value: IPMBackgroundServiceAgentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMBackgroundServiceAgentInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMBackgroundServiceAgentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMBackgroundServiceAgentInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMBackgroundServiceAgentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMBackgroundServiceAgentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMBackgroundServiceAgentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMBackgroundServiceAgentInfo {}
impl ::core::fmt::Debug for IPMBackgroundServiceAgentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMBackgroundServiceAgentInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMBackgroundServiceAgentInfo {
    type Vtable = IPMBackgroundServiceAgentInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a8b46da_928c_4879_998c_09dc96f3d490);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundServiceAgentInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskID: usize,
    pub BSAID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsaid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BGSpecifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgspecifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BGSpecifier: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BGName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BGName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BGSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BGSource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BGType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgtype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BGType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPeriodic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisperiodic: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPeriodic: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsScheduled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisscheduled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsScheduled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsScheduleAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisscheduleallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsScheduleAllowed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLaunchOnBoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plaunchonboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLaunchOnBoot: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_IsScheduled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isscheduled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_IsScheduled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_IsScheduleAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isscheduleallowed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_IsScheduleAllowed: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMBackgroundServiceAgentInfoEnumerator(::windows::core::IUnknown);
impl IPMBackgroundServiceAgentInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::core::Result<IPMBackgroundServiceAgentInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMBackgroundServiceAgentInfo>(result__)
    }
}
impl ::core::convert::From<IPMBackgroundServiceAgentInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: IPMBackgroundServiceAgentInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMBackgroundServiceAgentInfoEnumerator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMBackgroundServiceAgentInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMBackgroundServiceAgentInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IPMBackgroundServiceAgentInfoEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMBackgroundServiceAgentInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMBackgroundServiceAgentInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMBackgroundServiceAgentInfoEnumerator {}
impl ::core::fmt::Debug for IPMBackgroundServiceAgentInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMBackgroundServiceAgentInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMBackgroundServiceAgentInfoEnumerator {
    type Vtable = IPMBackgroundServiceAgentInfoEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18eb2072_ab56_43b3_872c_beafb7a6b391);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundServiceAgentInfoEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbsainfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMBackgroundWorkerInfo(::windows::core::IUnknown);
impl IPMBackgroundWorkerInfo {
    pub unsafe fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProductID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskID(&self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TaskID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptaskid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BGName(&self, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BGName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbgname)).ok()
    }
    pub unsafe fn MaxStartupLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MaxStartupLatency)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn ExpectedRuntime(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ExpectedRuntime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBootWorker(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsBootWorker)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IPMBackgroundWorkerInfo> for ::windows::core::IUnknown {
    fn from(value: IPMBackgroundWorkerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMBackgroundWorkerInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMBackgroundWorkerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMBackgroundWorkerInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMBackgroundWorkerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMBackgroundWorkerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMBackgroundWorkerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMBackgroundWorkerInfo {}
impl ::core::fmt::Debug for IPMBackgroundWorkerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMBackgroundWorkerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMBackgroundWorkerInfo {
    type Vtable = IPMBackgroundWorkerInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7dd4531b_d3bf_4b6b_94f3_69c098b1497d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundWorkerInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BGName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BGName: usize,
    pub MaxStartupLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaxstartuplatency: *mut u32) -> ::windows::core::HRESULT,
    pub ExpectedRuntime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexpectedruntime: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsBootWorker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisbootworker: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsBootWorker: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMBackgroundWorkerInfoEnumerator(::windows::core::IUnknown);
impl IPMBackgroundWorkerInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::core::Result<IPMBackgroundWorkerInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMBackgroundWorkerInfo>(result__)
    }
}
impl ::core::convert::From<IPMBackgroundWorkerInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: IPMBackgroundWorkerInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMBackgroundWorkerInfoEnumerator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMBackgroundWorkerInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMBackgroundWorkerInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IPMBackgroundWorkerInfoEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMBackgroundWorkerInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMBackgroundWorkerInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMBackgroundWorkerInfoEnumerator {}
impl ::core::fmt::Debug for IPMBackgroundWorkerInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMBackgroundWorkerInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMBackgroundWorkerInfoEnumerator {
    type Vtable = IPMBackgroundWorkerInfoEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87f479f8_90d8_4ec7_92b9_72787e2f636b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundWorkerInfoEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbwinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMDeploymentManager(::windows::core::IUnknown);
impl IPMDeploymentManager {
    pub unsafe fn ReportDownloadBegin(&self, productid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportDownloadBegin)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    pub unsafe fn ReportDownloadProgress(&self, productid: ::windows::core::GUID, usprogress: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportDownloadProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), usprogress).ok()
    }
    pub unsafe fn ReportDownloadComplete(&self, productid: ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportDownloadComplete)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), hrresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginInstall)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinstallinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginUpdate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pupdateinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDeployPackage(&self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginDeployPackage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinstallinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginUpdateDeployedPackageLegacy(&self, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginUpdateDeployedPackageLegacy)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pupdateinfo)).ok()
    }
    pub unsafe fn BeginUninstall(&self, productid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginUninstall)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginEnterpriseAppInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginEnterpriseAppInstall)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinstallinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginEnterpriseAppUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginEnterpriseAppUpdate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pupdateinfo)).ok()
    }
    pub unsafe fn BeginUpdateLicense(&self, productid: ::windows::core::GUID, offerid: ::windows::core::GUID, pblicense: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginUpdateLicense)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(offerid), ::core::mem::transmute(::windows::core::as_ptr_or_null(pblicense)), pblicense.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLicenseChallenge<'a, P0>(&self, packagepath: P0, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).GetLicenseChallenge)(::windows::core::Interface::as_raw(self), packagepath.into().abi(), ::core::mem::transmute(ppbchallenge), ::core::mem::transmute(pcbchallenge), ::core::mem::transmute(ppbkid), ::core::mem::transmute(pcbkid), ::core::mem::transmute(ppbdeviceid), ::core::mem::transmute(pcbdeviceid), ::core::mem::transmute(ppbsaltvalue), ::core::mem::transmute(pcbsaltvalue), ::core::mem::transmute(ppbkgvvalue), ::core::mem::transmute(pcbkgvvalue)).ok()
    }
    pub unsafe fn GetLicenseChallengeByProductID(&self, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLicenseChallengeByProductID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(ppbchallenge), ::core::mem::transmute(pcblicense)).ok()
    }
    pub unsafe fn GetLicenseChallengeByProductID2(&self, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLicenseChallengeByProductID2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(ppbchallenge), ::core::mem::transmute(pcblicense), ::core::mem::transmute(ppbkid), ::core::mem::transmute(pcbkid), ::core::mem::transmute(ppbdeviceid), ::core::mem::transmute(pcbdeviceid), ::core::mem::transmute(ppbsaltvalue), ::core::mem::transmute(pcbsaltvalue), ::core::mem::transmute(ppbkgvvalue), ::core::mem::transmute(pcbkgvvalue)).ok()
    }
    pub unsafe fn RevokeLicense(&self, productid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RevokeLicense)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RebindMdilBinaries(&self, productid: ::windows::core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RebindMdilBinaries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(filenames)).ok()
    }
    pub unsafe fn RebindAllMdilBinaries(&self, productid: ::windows::core::GUID, instanceid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RebindAllMdilBinaries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(instanceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegenerateXbf(&self, productid: ::windows::core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegenerateXbf)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(assemblypaths)).ok()
    }
    pub unsafe fn GenerateXbfForCurrentLocale(&self, productid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateXbfForCurrentLocale)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginProvision<'a, P0>(&self, productid: ::windows::core::GUID, xmlpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).BeginProvision)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), xmlpath.into().abi()).ok()
    }
    pub unsafe fn BeginDeprovision(&self, productid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginDeprovision)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    pub unsafe fn ReindexSQLCEDatabases(&self, productid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReindexSQLCEDatabases)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    pub unsafe fn SetApplicationsNeedMaintenance(&self, requiredmaintenanceoperations: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SetApplicationsNeedMaintenance)(::windows::core::Interface::as_raw(self), requiredmaintenanceoperations, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn UpdateChamberProfile(&self, productid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateChamberProfile)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnterprisePolicyIsApplicationAllowed<'a, P0>(&self, productid: ::windows::core::GUID, publishername: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnterprisePolicyIsApplicationAllowed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), publishername.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginUpdateDeployedPackage(&self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginUpdateDeployedPackage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pupdateinfo)).ok()
    }
    pub unsafe fn ReportRestoreCancelled(&self, productid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportRestoreCancelled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResolveResourceString<'a, P0>(&self, resourcestring: P0, presolvedresourcestring: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ResolveResourceString)(::windows::core::Interface::as_raw(self), resourcestring.into(), ::core::mem::transmute(presolvedresourcestring)).ok()
    }
    pub unsafe fn UpdateCapabilitiesForModernApps(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateCapabilitiesForModernApps)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReportDownloadStatusUpdate(&self, productid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportDownloadStatusUpdate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    pub unsafe fn BeginUninstallWithOptions(&self, productid: ::windows::core::GUID, removaloptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginUninstallWithOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), removaloptions).ok()
    }
    pub unsafe fn BindDeferredMdilBinaries(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BindDeferredMdilBinaries)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateXamlLightupXbfForCurrentLocale<'a, P0>(&self, packagefamilyname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).GenerateXamlLightupXbfForCurrentLocale)(::windows::core::Interface::as_raw(self), packagefamilyname.into().abi()).ok()
    }
    pub unsafe fn AddLicenseForAppx(&self, productid: ::windows::core::GUID, pblicense: &[u8], pbplayreadyheader: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddLicenseForAppx)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(::windows::core::as_ptr_or_null(pblicense)), pblicense.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbplayreadyheader)), pbplayreadyheader.len() as _).ok()
    }
    pub unsafe fn FixJunctionsForAppsOnSDCard(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FixJunctionsForAppsOnSDCard)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPMDeploymentManager> for ::windows::core::IUnknown {
    fn from(value: IPMDeploymentManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMDeploymentManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMDeploymentManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMDeploymentManager> for ::windows::core::IUnknown {
    fn from(value: &IPMDeploymentManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMDeploymentManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMDeploymentManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMDeploymentManager {}
impl ::core::fmt::Debug for IPMDeploymentManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMDeploymentManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMDeploymentManager {
    type Vtable = IPMDeploymentManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35f785fa_1979_4a8b_bc8f_fd70eb0d1544);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMDeploymentManager_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ReportDownloadBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ReportDownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, usprogress: u16) -> ::windows::core::HRESULT,
    pub ReportDownloadComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginInstall: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginUpdate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginDeployPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginDeployPackage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginUpdateDeployedPackageLegacy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginUpdateDeployedPackageLegacy: usize,
    pub BeginUninstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginEnterpriseAppInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginEnterpriseAppInstall: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginEnterpriseAppUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginEnterpriseAppUpdate: usize,
    pub BeginUpdateLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, offerid: ::windows::core::GUID, pblicense: *const u8, cblicense: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLicenseChallenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLicenseChallenge: usize,
    pub GetLicenseChallengeByProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows::core::HRESULT,
    pub GetLicenseChallengeByProductID2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows::core::HRESULT,
    pub RevokeLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RebindMdilBinaries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RebindMdilBinaries: usize,
    pub RebindAllMdilBinaries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, instanceid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RegenerateXbf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegenerateXbf: usize,
    pub GenerateXbfForCurrentLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginProvision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, xmlpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginProvision: usize,
    pub BeginDeprovision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ReindexSQLCEDatabases: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetApplicationsNeedMaintenance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredmaintenanceoperations: u32, pcapplications: *mut u32) -> ::windows::core::HRESULT,
    pub UpdateChamberProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnterprisePolicyIsApplicationAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, publishername: ::windows::core::PCWSTR, pisallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnterprisePolicyIsApplicationAllowed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginUpdateDeployedPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginUpdateDeployedPackage: usize,
    pub ReportRestoreCancelled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ResolveResourceString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcestring: ::windows::core::PCWSTR, presolvedresourcestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResolveResourceString: usize,
    pub UpdateCapabilitiesForModernApps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportDownloadStatusUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub BeginUninstallWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, removaloptions: u32) -> ::windows::core::HRESULT,
    pub BindDeferredMdilBinaries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GenerateXamlLightupXbfForCurrentLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GenerateXamlLightupXbfForCurrentLocale: usize,
    pub AddLicenseForAppx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> ::windows::core::HRESULT,
    pub FixJunctionsForAppsOnSDCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMEnumerationManager(::windows::core::IUnknown);
impl IPMEnumerationManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AllApplications<'a, P0>(&self, ppappenum: *mut ::core::option::Option<IPMApplicationInfoEnumerator>, filter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PM_ENUM_FILTER>>,
    {
        (::windows::core::Interface::vtable(self).get_AllApplications)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppappenum), filter.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AllTiles<'a, P0>(&self, pptileenum: *mut ::core::option::Option<IPMTileInfoEnumerator>, filter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PM_ENUM_FILTER>>,
    {
        (::windows::core::Interface::vtable(self).get_AllTiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pptileenum), filter.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AllTasks<'a, P0>(&self, pptaskenum: *mut ::core::option::Option<IPMTaskInfoEnumerator>, filter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PM_ENUM_FILTER>>,
    {
        (::windows::core::Interface::vtable(self).get_AllTasks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pptaskenum), filter.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AllExtensions<'a, P0>(&self, ppextensionenum: *mut ::core::option::Option<IPMExtensionInfoEnumerator>, filter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PM_ENUM_FILTER>>,
    {
        (::windows::core::Interface::vtable(self).get_AllExtensions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppextensionenum), filter.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AllBackgroundServiceAgents<'a, P0>(&self, ppbsaenum: *mut ::core::option::Option<IPMBackgroundServiceAgentInfoEnumerator>, filter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PM_ENUM_FILTER>>,
    {
        (::windows::core::Interface::vtable(self).get_AllBackgroundServiceAgents)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppbsaenum), filter.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AllBackgroundWorkers<'a, P0>(&self, ppbswenum: *mut ::core::option::Option<IPMBackgroundWorkerInfoEnumerator>, filter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PM_ENUM_FILTER>>,
    {
        (::windows::core::Interface::vtable(self).get_AllBackgroundWorkers)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppbswenum), filter.into().abi()).ok()
    }
    pub unsafe fn get_ApplicationInfo(&self, productid: ::windows::core::GUID) -> ::windows::core::Result<IPMApplicationInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_ApplicationInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMApplicationInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_TileInfo<'a, P0>(&self, productid: ::windows::core::GUID, tileid: P0) -> ::windows::core::Result<IPMTileInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_TileInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), tileid.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMTileInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_TaskInfo<'a, P0>(&self, productid: ::windows::core::GUID, taskid: P0) -> ::windows::core::Result<IPMTaskInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_TaskInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), taskid.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMTaskInfo>(result__)
    }
    pub unsafe fn get_TaskInfoEx<'a, P0>(&self, productid: ::windows::core::GUID, taskid: P0) -> ::windows::core::Result<IPMTaskInfo>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_TaskInfoEx)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), taskid.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMTaskInfo>(result__)
    }
    pub unsafe fn get_BackgroundServiceAgentInfo(&self, bsaid: u32) -> ::windows::core::Result<IPMBackgroundServiceAgentInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_BackgroundServiceAgentInfo)(::windows::core::Interface::as_raw(self), bsaid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMBackgroundServiceAgentInfo>(result__)
    }
    pub unsafe fn AllLiveTileJobs(&self) -> ::windows::core::Result<IPMLiveTileJobInfoEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AllLiveTileJobs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMLiveTileJobInfoEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_LiveTileJob<'a, P0>(&self, productid: ::windows::core::GUID, tileid: P0, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE) -> ::windows::core::Result<IPMLiveTileJobInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_LiveTileJob)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), tileid.into().abi(), recurrencetype, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMLiveTileJobInfo>(result__)
    }
    pub unsafe fn get_ApplicationInfoExternal(&self, productid: ::windows::core::GUID) -> ::windows::core::Result<IPMApplicationInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_ApplicationInfoExternal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMApplicationInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_FileHandlerGenericLogo<'a, P0>(&self, filetype: P0, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).get_FileHandlerGenericLogo)(::windows::core::Interface::as_raw(self), filetype.into().abi(), logosize, ::core::mem::transmute(plogo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_ApplicationInfoFromAccessClaims<'a, P0, P1>(&self, sysappid0: P0, sysappid1: P1) -> ::windows::core::Result<IPMApplicationInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_ApplicationInfoFromAccessClaims)(::windows::core::Interface::as_raw(self), sysappid0.into().abi(), sysappid1.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMApplicationInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_StartTileEnumeratorBlob<'a, P0>(&self, filter: P0, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PM_ENUM_FILTER>>,
    {
        (::windows::core::Interface::vtable(self).get_StartTileEnumeratorBlob)(::windows::core::Interface::as_raw(self), filter.into().abi(), ::core::mem::transmute(pctiles), ::core::mem::transmute(pptileblobs)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_StartAppEnumeratorBlob<'a, P0>(&self, filter: P0, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PM_ENUM_FILTER>>,
    {
        (::windows::core::Interface::vtable(self).get_StartAppEnumeratorBlob)(::windows::core::Interface::as_raw(self), filter.into().abi(), ::core::mem::transmute(pcapps), ::core::mem::transmute(ppappblobs)).ok()
    }
}
impl ::core::convert::From<IPMEnumerationManager> for ::windows::core::IUnknown {
    fn from(value: IPMEnumerationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMEnumerationManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMEnumerationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMEnumerationManager> for ::windows::core::IUnknown {
    fn from(value: &IPMEnumerationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMEnumerationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMEnumerationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMEnumerationManager {}
impl ::core::fmt::Debug for IPMEnumerationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMEnumerationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMEnumerationManager {
    type Vtable = IPMEnumerationManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x698d57c2_292d_4cf3_b73c_d95a6922ed9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMEnumerationManager_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AllApplications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppappenum: *mut *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AllApplications: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AllTiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptileenum: *mut *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AllTiles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AllTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptaskenum: *mut *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AllTasks: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AllExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextensionenum: *mut *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AllExtensions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AllBackgroundServiceAgents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbsaenum: *mut *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AllBackgroundServiceAgents: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AllBackgroundWorkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbswenum: *mut *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AllBackgroundWorkers: usize,
    pub get_ApplicationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_TileInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, tileid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptileinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_TileInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_TaskInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, taskid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_TaskInfo: usize,
    pub get_TaskInfoEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, taskid: ::windows::core::PCWSTR, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_BackgroundServiceAgentInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsaid: u32, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllLiveTileJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplivetilejobenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_LiveTileJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, tileid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE, pplivetilejobinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_LiveTileJob: usize,
    pub get_ApplicationInfoExternal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows::core::GUID, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_FileHandlerGenericLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_FileHandlerGenericLogo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_ApplicationInfoFromAccessClaims: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sysappid0: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sysappid1: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_ApplicationInfoFromAccessClaims: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_StartTileEnumeratorBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_StartTileEnumeratorBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_StartAppEnumeratorBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: ::core::mem::ManuallyDrop<PM_ENUM_FILTER>, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_StartAppEnumeratorBlob: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionCachedFileUpdaterInfo(::windows::core::IUnknown);
impl IPMExtensionCachedFileUpdaterInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsUpdates(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportsUpdates)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IPMExtensionCachedFileUpdaterInfo> for ::windows::core::IUnknown {
    fn from(value: IPMExtensionCachedFileUpdaterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMExtensionCachedFileUpdaterInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMExtensionCachedFileUpdaterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMExtensionCachedFileUpdaterInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMExtensionCachedFileUpdaterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMExtensionCachedFileUpdaterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMExtensionCachedFileUpdaterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionCachedFileUpdaterInfo {}
impl ::core::fmt::Debug for IPMExtensionCachedFileUpdaterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionCachedFileUpdaterInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMExtensionCachedFileUpdaterInfo {
    type Vtable = IPMExtensionCachedFileUpdaterInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2d77509_4e58_4ba9_af7e_b642e370e1b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionCachedFileUpdaterInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psupportsupdates: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsUpdates: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionContractInfo(::windows::core::IUnknown);
impl IPMExtensionContractInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_InvocationInfo(&self, paumid: *mut super::super::Foundation::BSTR, pargs: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_InvocationInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(paumid), ::core::mem::transmute(pargs)).ok()
    }
}
impl ::core::convert::From<IPMExtensionContractInfo> for ::windows::core::IUnknown {
    fn from(value: IPMExtensionContractInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMExtensionContractInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMExtensionContractInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMExtensionContractInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMExtensionContractInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMExtensionContractInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMExtensionContractInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionContractInfo {}
impl ::core::fmt::Debug for IPMExtensionContractInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionContractInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMExtensionContractInfo {
    type Vtable = IPMExtensionContractInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5666373_7ba1_467c_b819_b175db1c295b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionContractInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paumid: *mut super::super::Foundation::BSTR, pargs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_InvocationInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionFileExtensionInfo(::windows::core::IUnknown);
impl IPMExtensionFileExtensionInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisplayName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdisplayname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_Logo(&self, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_Logo)(::windows::core::Interface::as_raw(self), logosize, ::core::mem::transmute(plogo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_ContentType<'a, P0>(&self, filetype: P0, pcontenttype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).get_ContentType)(::windows::core::Interface::as_raw(self), filetype.into().abi(), ::core::mem::transmute(pcontenttype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_FileType<'a, P0>(&self, contenttype: P0, pfiletype: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).get_FileType)(::windows::core::Interface::as_raw(self), contenttype.into().abi(), ::core::mem::transmute(pfiletype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_InvocationInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AllFileTypes(&self, pcbtypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_AllFileTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcbtypes), ::core::mem::transmute(pptypes)).ok()
    }
}
impl ::core::convert::From<IPMExtensionFileExtensionInfo> for ::windows::core::IUnknown {
    fn from(value: IPMExtensionFileExtensionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMExtensionFileExtensionInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMExtensionFileExtensionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMExtensionFileExtensionInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMExtensionFileExtensionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMExtensionFileExtensionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMExtensionFileExtensionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionFileExtensionInfo {}
impl ::core::fmt::Debug for IPMExtensionFileExtensionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionFileExtensionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMExtensionFileExtensionInfo {
    type Vtable = IPMExtensionFileExtensionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b87cb6c_0b88_4989_a4ec_033714f710d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionFileExtensionInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logosize: PM_LOGO_SIZE, plogo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Logo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcontenttype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_ContentType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_FileType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfiletype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_FileType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_InvocationInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbtypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AllFileTypes: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionFileOpenPickerInfo(::windows::core::IUnknown);
impl IPMExtensionFileOpenPickerInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_AllFileTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pctypes), ::core::mem::transmute(pptypes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsAllFileTypes(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportsAllFileTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IPMExtensionFileOpenPickerInfo> for ::windows::core::IUnknown {
    fn from(value: IPMExtensionFileOpenPickerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMExtensionFileOpenPickerInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMExtensionFileOpenPickerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMExtensionFileOpenPickerInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMExtensionFileOpenPickerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMExtensionFileOpenPickerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMExtensionFileOpenPickerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionFileOpenPickerInfo {}
impl ::core::fmt::Debug for IPMExtensionFileOpenPickerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionFileOpenPickerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMExtensionFileOpenPickerInfo {
    type Vtable = IPMExtensionFileOpenPickerInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dc91d25_9606_420c_9a78_e034a3418345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionFileOpenPickerInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AllFileTypes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsAllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsAllFileTypes: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionFileSavePickerInfo(::windows::core::IUnknown);
impl IPMExtensionFileSavePickerInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_AllFileTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pctypes), ::core::mem::transmute(pptypes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsAllFileTypes(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportsAllFileTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IPMExtensionFileSavePickerInfo> for ::windows::core::IUnknown {
    fn from(value: IPMExtensionFileSavePickerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMExtensionFileSavePickerInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMExtensionFileSavePickerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMExtensionFileSavePickerInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMExtensionFileSavePickerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMExtensionFileSavePickerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMExtensionFileSavePickerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionFileSavePickerInfo {}
impl ::core::fmt::Debug for IPMExtensionFileSavePickerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionFileSavePickerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMExtensionFileSavePickerInfo {
    type Vtable = IPMExtensionFileSavePickerInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38005cba_f81a_493e_a0f8_922c8680da43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionFileSavePickerInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AllFileTypes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsAllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsAllFileTypes: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionInfo(::windows::core::IUnknown);
impl IPMExtensionInfo {
    pub unsafe fn SupplierPID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupplierPID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupplierTaskID(&self, psuppliertid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SupplierTaskID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psuppliertid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Title)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptitle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IconPath(&self, piconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IconPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(piconpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtraFile(&self, pfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExtraFile)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfilepath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_InvocationInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
}
impl ::core::convert::From<IPMExtensionInfo> for ::windows::core::IUnknown {
    fn from(value: IPMExtensionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMExtensionInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMExtensionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMExtensionInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMExtensionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMExtensionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMExtensionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionInfo {}
impl ::core::fmt::Debug for IPMExtensionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMExtensionInfo {
    type Vtable = IPMExtensionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49acde79_9788_4d0a_8aa0_1746afdb9e9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SupplierPID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psupplierpid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SupplierTaskID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psuppliertid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupplierTaskID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Title: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IconPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piconpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IconPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtraFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtraFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_InvocationInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionInfoEnumerator(::windows::core::IUnknown);
impl IPMExtensionInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::core::Result<IPMExtensionInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMExtensionInfo>(result__)
    }
}
impl ::core::convert::From<IPMExtensionInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: IPMExtensionInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMExtensionInfoEnumerator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMExtensionInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMExtensionInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IPMExtensionInfoEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMExtensionInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMExtensionInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionInfoEnumerator {}
impl ::core::fmt::Debug for IPMExtensionInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMExtensionInfoEnumerator {
    type Vtable = IPMExtensionInfoEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x403b9e82_1171_4573_8e6f_6f33f39b83dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionInfoEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextensioninfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionProtocolInfo(::windows::core::IUnknown);
impl IPMExtensionProtocolInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Protocol(&self, pprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Protocol)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprotocol)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_InvocationInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
}
impl ::core::convert::From<IPMExtensionProtocolInfo> for ::windows::core::IUnknown {
    fn from(value: IPMExtensionProtocolInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMExtensionProtocolInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMExtensionProtocolInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMExtensionProtocolInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMExtensionProtocolInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMExtensionProtocolInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMExtensionProtocolInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionProtocolInfo {}
impl ::core::fmt::Debug for IPMExtensionProtocolInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionProtocolInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMExtensionProtocolInfo {
    type Vtable = IPMExtensionProtocolInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e3fa036_51eb_4453_baff_b8d8e4b46c8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionProtocolInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Protocol: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_InvocationInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionShareTargetInfo(::windows::core::IUnknown);
impl IPMExtensionShareTargetInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_AllFileTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pctypes), ::core::mem::transmute(pptypes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_AllDataFormats(&self, pcdataformats: *mut u32, ppdataformats: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_AllDataFormats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdataformats), ::core::mem::transmute(ppdataformats)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsAllFileTypes(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportsAllFileTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IPMExtensionShareTargetInfo> for ::windows::core::IUnknown {
    fn from(value: IPMExtensionShareTargetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMExtensionShareTargetInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMExtensionShareTargetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMExtensionShareTargetInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMExtensionShareTargetInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMExtensionShareTargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMExtensionShareTargetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionShareTargetInfo {}
impl ::core::fmt::Debug for IPMExtensionShareTargetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionShareTargetInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMExtensionShareTargetInfo {
    type Vtable = IPMExtensionShareTargetInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5471f48b_c65c_4656_8c70_242e31195fea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionShareTargetInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AllFileTypes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AllDataFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdataformats: *mut u32, ppdataformats: *mut *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AllDataFormats: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsAllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsAllFileTypes: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMLiveTileJobInfo(::windows::core::IUnknown);
impl IPMLiveTileJobInfo {
    pub unsafe fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProductID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TileID(&self, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TileID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptileid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NextSchedule(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NextSchedule)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_NextSchedule(&self, ftnextschedule: super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_NextSchedule)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ftnextschedule)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartSchedule(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartSchedule)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_StartSchedule(&self, ftstartschedule: super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_StartSchedule)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ftstartschedule)).ok()
    }
    pub unsafe fn IntervalDuration(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IntervalDuration)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn set_IntervalDuration(&self, ulintervalduration: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_IntervalDuration)(::windows::core::Interface::as_raw(self), ulintervalduration).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunForever(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RunForever)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_RunForever<'a, P0>(&self, frunforever: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_RunForever)(::windows::core::Interface::as_raw(self), frunforever.into()).ok()
    }
    pub unsafe fn MaxRunCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MaxRunCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn set_MaxRunCount(&self, ulmaxruncount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_MaxRunCount)(::windows::core::Interface::as_raw(self), ulmaxruncount).ok()
    }
    pub unsafe fn RunCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RunCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn set_RunCount(&self, ulruncount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_RunCount)(::windows::core::Interface::as_raw(self), ulruncount).ok()
    }
    pub unsafe fn RecurrenceType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RecurrenceType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn set_RecurrenceType(&self, ulrecurrencetype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_RecurrenceType)(::windows::core::Interface::as_raw(self), ulrecurrencetype).ok()
    }
    pub unsafe fn get_TileXML(&self, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_TileXML)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptilexml), ::core::mem::transmute(pcbtilexml)).ok()
    }
    pub unsafe fn set_TileXML(&self, ptilexml: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_TileXML)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(ptilexml)), ptilexml.len() as _).ok()
    }
    pub unsafe fn get_UrlXML(&self, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_UrlXML)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(purlxml), ::core::mem::transmute(pcburlxml)).ok()
    }
    pub unsafe fn set_UrlXML(&self, purlxml: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_UrlXML)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(purlxml)), purlxml.len() as _).ok()
    }
    pub unsafe fn AttemptCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AttemptCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn set_AttemptCount(&self, ulattemptcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_AttemptCount)(::windows::core::Interface::as_raw(self), ulattemptcount).ok()
    }
    pub unsafe fn DownloadState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DownloadState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn set_DownloadState(&self, uldownloadstate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_DownloadState)(::windows::core::Interface::as_raw(self), uldownloadstate).ok()
    }
}
impl ::core::convert::From<IPMLiveTileJobInfo> for ::windows::core::IUnknown {
    fn from(value: IPMLiveTileJobInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMLiveTileJobInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMLiveTileJobInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMLiveTileJobInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMLiveTileJobInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMLiveTileJobInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMLiveTileJobInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMLiveTileJobInfo {}
impl ::core::fmt::Debug for IPMLiveTileJobInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMLiveTileJobInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMLiveTileJobInfo {
    type Vtable = IPMLiveTileJobInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6009a81f_4710_4697_b5f6_2208f6057b8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMLiveTileJobInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TileID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TileID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NextSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnextschedule: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NextSchedule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_NextSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ftnextschedule: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_NextSchedule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstartschedule: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartSchedule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_StartSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ftstartschedule: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_StartSchedule: usize,
    pub IntervalDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pintervalduration: *mut u32) -> ::windows::core::HRESULT,
    pub set_IntervalDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulintervalduration: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunForever: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isrunforever: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunForever: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_RunForever: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frunforever: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_RunForever: usize,
    pub MaxRunCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaxruncount: *mut u32) -> ::windows::core::HRESULT,
    pub set_MaxRunCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulmaxruncount: u32) -> ::windows::core::HRESULT,
    pub RunCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pruncount: *mut u32) -> ::windows::core::HRESULT,
    pub set_RunCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulruncount: u32) -> ::windows::core::HRESULT,
    pub RecurrenceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precurrencetype: *mut u32) -> ::windows::core::HRESULT,
    pub set_RecurrenceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulrecurrencetype: u32) -> ::windows::core::HRESULT,
    pub get_TileXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows::core::HRESULT,
    pub set_TileXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptilexml: *const u8, cbtilexml: u32) -> ::windows::core::HRESULT,
    pub get_UrlXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows::core::HRESULT,
    pub set_UrlXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, purlxml: *const u8, cburlxml: u32) -> ::windows::core::HRESULT,
    pub AttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattemptcount: *mut u32) -> ::windows::core::HRESULT,
    pub set_AttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulattemptcount: u32) -> ::windows::core::HRESULT,
    pub DownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdownloadstate: *mut u32) -> ::windows::core::HRESULT,
    pub set_DownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uldownloadstate: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMLiveTileJobInfoEnumerator(::windows::core::IUnknown);
impl IPMLiveTileJobInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::core::Result<IPMLiveTileJobInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMLiveTileJobInfo>(result__)
    }
}
impl ::core::convert::From<IPMLiveTileJobInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: IPMLiveTileJobInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMLiveTileJobInfoEnumerator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMLiveTileJobInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMLiveTileJobInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IPMLiveTileJobInfoEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMLiveTileJobInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMLiveTileJobInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMLiveTileJobInfoEnumerator {}
impl ::core::fmt::Debug for IPMLiveTileJobInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMLiveTileJobInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMLiveTileJobInfoEnumerator {
    type Vtable = IPMLiveTileJobInfoEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc042582_9415_4f36_9f99_06f104c07c03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMLiveTileJobInfoEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplivetilejobinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTaskInfo(::windows::core::IUnknown);
impl IPMTaskInfo {
    pub unsafe fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProductID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskID(&self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TaskID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptaskid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NavigationPage(&self, pnavigationpage: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NavigationPage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pnavigationpage)).ok()
    }
    pub unsafe fn TaskTransition(&self) -> ::windows::core::Result<PM_TASK_TRANSITION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TaskTransition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PM_TASK_TRANSITION>(result__)
    }
    pub unsafe fn RuntimeType(&self) -> ::windows::core::Result<PACKMAN_RUNTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RuntimeType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PACKMAN_RUNTIME>(result__)
    }
    pub unsafe fn ActivationPolicy(&self) -> ::windows::core::Result<PM_ACTIVATION_POLICY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ActivationPolicy)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PM_ACTIVATION_POLICY>(result__)
    }
    pub unsafe fn TaskType(&self) -> ::windows::core::Result<PM_TASK_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TaskType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PM_TASK_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_InvocationInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImagePath(&self, pimagepath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ImagePath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pimagepath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImageParams(&self, pimageparams: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ImageParams)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pimageparams)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallRootFolder(&self, pinstallrootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InstallRootFolder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinstallrootfolder)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DataRootFolder(&self, pdatarootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DataRootFolder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdatarootfolder)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSingleInstanceHost(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsSingleInstanceHost)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInteropEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsInteropEnabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn ApplicationState(&self) -> ::windows::core::Result<PM_APPLICATION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ApplicationState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PM_APPLICATION_STATE>(result__)
    }
    pub unsafe fn InstallType(&self) -> ::windows::core::Result<PM_APPLICATION_INSTALL_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).InstallType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PM_APPLICATION_INSTALL_TYPE>(result__)
    }
    pub unsafe fn get_Version(&self, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_Version)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptargetmajorversion), ::core::mem::transmute(ptargetminorversion)).ok()
    }
    pub unsafe fn BitsPerPixel(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BitsPerPixel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressesDehydration(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SuppressesDehydration)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BackgroundExecutionAbilities(&self, pbackgroundexecutionabilities: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BackgroundExecutionAbilities)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbackgroundexecutionabilities)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOptedForExtendedMem(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOptedForExtendedMem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IPMTaskInfo> for ::windows::core::IUnknown {
    fn from(value: IPMTaskInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMTaskInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMTaskInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMTaskInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMTaskInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMTaskInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMTaskInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTaskInfo {}
impl ::core::fmt::Debug for IPMTaskInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTaskInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMTaskInfo {
    type Vtable = IPMTaskInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf1d8c33_1bf5_4ee0_b549_6b9dd3834942);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTaskInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NavigationPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnavigationpage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NavigationPage: usize,
    pub TaskTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptasktransition: *mut PM_TASK_TRANSITION) -> ::windows::core::HRESULT,
    pub RuntimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pruntimetype: *mut PACKMAN_RUNTIME) -> ::windows::core::HRESULT,
    pub ActivationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactivationpolicy: *mut PM_ACTIVATION_POLICY) -> ::windows::core::HRESULT,
    pub TaskType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptasktype: *mut PM_TASK_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_InvocationInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImagePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimagepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImagePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImageParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageparams: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImageParams: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallRootFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstallrootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallRootFolder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DataRootFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatarootfolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DataRootFolder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSingleInstanceHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pissingleinstancehost: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSingleInstanceHost: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInteropEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisinteropenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInteropEnabled: usize,
    pub ApplicationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papplicationstate: *mut PM_APPLICATION_STATE) -> ::windows::core::HRESULT,
    pub InstallType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows::core::HRESULT,
    pub get_Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows::core::HRESULT,
    pub BitsPerPixel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitsperpixel: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SuppressesDehydration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psuppressesdehydration: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SuppressesDehydration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BackgroundExecutionAbilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbackgroundexecutionabilities: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BackgroundExecutionAbilities: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOptedForExtendedMem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisoptedin: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOptedForExtendedMem: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTaskInfoEnumerator(::windows::core::IUnknown);
impl IPMTaskInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::core::Result<IPMTaskInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMTaskInfo>(result__)
    }
}
impl ::core::convert::From<IPMTaskInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: IPMTaskInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMTaskInfoEnumerator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMTaskInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMTaskInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IPMTaskInfoEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMTaskInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMTaskInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTaskInfoEnumerator {}
impl ::core::fmt::Debug for IPMTaskInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTaskInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMTaskInfoEnumerator {
    type Vtable = IPMTaskInfoEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0630b0f8_0bbc_4821_be74_c7995166ed2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTaskInfoEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTileInfo(::windows::core::IUnknown);
impl IPMTileInfo {
    pub unsafe fn ProductID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProductID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TileID(&self, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TileID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptileid)).ok()
    }
    pub unsafe fn TemplateType(&self) -> ::windows::core::Result<TILE_TEMPLATE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TemplateType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TILE_TEMPLATE_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_HubPinnedState)(::windows::core::Interface::as_raw(self), hubtype, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn get_HubPosition(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_HubPosition)(::windows::core::Interface::as_raw(self), hubtype, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsNotified(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsNotified)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsDefault)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskID(&self, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TaskID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptaskid)).ok()
    }
    pub unsafe fn TileType(&self) -> ::windows::core::Result<PM_STARTTILE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TileType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PM_STARTTILE_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThemable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsThemable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn get_PropertyById(&self, propid: u32) -> ::windows::core::Result<IPMTilePropertyInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_PropertyById)(::windows::core::Interface::as_raw(self), propid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMTilePropertyInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_InvocationInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn PropertyEnum(&self) -> ::windows::core::Result<IPMTilePropertyEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PropertyEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMTilePropertyEnumerator>(result__)
    }
    pub unsafe fn get_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows::core::Result<PM_TILE_SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_HubTileSize)(::windows::core::Interface::as_raw(self), hubtype, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PM_TILE_SIZE>(result__)
    }
    pub unsafe fn set_HubPosition(&self, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_HubPosition)(::windows::core::Interface::as_raw(self), hubtype, position).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_NotifiedState<'a, P0>(&self, notified: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_NotifiedState)(::windows::core::Interface::as_raw(self), notified.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_HubPinnedState<'a, P0>(&self, hubtype: PM_TILE_HUBTYPE, pinned: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_HubPinnedState)(::windows::core::Interface::as_raw(self), hubtype, pinned.into()).ok()
    }
    pub unsafe fn set_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).set_HubTileSize)(::windows::core::Interface::as_raw(self), hubtype, size).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_InvocationInfo<'a, P0, P1>(&self, taskname: P0, taskparameters: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).set_InvocationInfo)(::windows::core::Interface::as_raw(self), taskname.into().abi(), taskparameters.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartTileBlob(&self, pblob: *mut PM_STARTTILEBLOB) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartTileBlob)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblob)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRestoring(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsRestoring)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAutoRestoreDisabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsAutoRestoreDisabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsRestoring<'a, P0>(&self, restoring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_IsRestoring)(::windows::core::Interface::as_raw(self), restoring.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsAutoRestoreDisabled<'a, P0>(&self, autorestoredisabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).set_IsAutoRestoreDisabled)(::windows::core::Interface::as_raw(self), autorestoredisabled.into()).ok()
    }
}
impl ::core::convert::From<IPMTileInfo> for ::windows::core::IUnknown {
    fn from(value: IPMTileInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMTileInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMTileInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMTileInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMTileInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMTileInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMTileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTileInfo {}
impl ::core::fmt::Debug for IPMTileInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTileInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMTileInfo {
    type Vtable = IPMTileInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1604833_2b08_4001_82cd_183ad734f752);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTileInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TileID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptileid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TileID: usize,
    pub TemplateType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptemplatetype: *mut TILE_TEMPLATE_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_HubPinnedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, ppinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_HubPinnedState: usize,
    pub get_HubPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pposition: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsNotified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsNotified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisdefault: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDefault: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TaskID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptaskid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TaskID: usize,
    pub TileType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstarttiletype: *mut PM_STARTTILE_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsThemable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsThemable: usize,
    pub get_PropertyById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32, pppropinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut super::super::Foundation::BSTR, pparameters: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_InvocationInfo: usize,
    pub PropertyEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptilepropenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub get_HubTileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, psize: *mut PM_TILE_SIZE) -> ::windows::core::HRESULT,
    pub set_HubPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub set_NotifiedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notified: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_NotifiedState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_HubPinnedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_HubPinnedState: usize,
    pub set_HubTileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub set_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskparameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_InvocationInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartTileBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTTILEBLOB) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartTileBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRestoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisrestoring: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRestoring: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAutoRestoreDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisautorestoredisabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAutoRestoreDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_IsRestoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restoring: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_IsRestoring: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_IsAutoRestoreDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autorestoredisabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_IsAutoRestoreDisabled: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTileInfoEnumerator(::windows::core::IUnknown);
impl IPMTileInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::core::Result<IPMTileInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMTileInfo>(result__)
    }
}
impl ::core::convert::From<IPMTileInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: IPMTileInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMTileInfoEnumerator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMTileInfoEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMTileInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IPMTileInfoEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMTileInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMTileInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTileInfoEnumerator {}
impl ::core::fmt::Debug for IPMTileInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTileInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMTileInfoEnumerator {
    type Vtable = IPMTileInfoEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xded83065_e462_4b2c_acb5_e39cea61c874);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTileInfoEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptileinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTilePropertyEnumerator(::windows::core::IUnknown);
impl IPMTilePropertyEnumerator {
    pub unsafe fn Next(&self) -> ::windows::core::Result<IPMTilePropertyInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPMTilePropertyInfo>(result__)
    }
}
impl ::core::convert::From<IPMTilePropertyEnumerator> for ::windows::core::IUnknown {
    fn from(value: IPMTilePropertyEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMTilePropertyEnumerator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMTilePropertyEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMTilePropertyEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IPMTilePropertyEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMTilePropertyEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMTilePropertyEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTilePropertyEnumerator {}
impl ::core::fmt::Debug for IPMTilePropertyEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTilePropertyEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMTilePropertyEnumerator {
    type Vtable = IPMTilePropertyEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc4cd629_9047_4250_aac8_930e47812421);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTilePropertyEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTilePropertyInfo(::windows::core::IUnknown);
impl IPMTilePropertyInfo {
    pub unsafe fn PropertyID(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PropertyID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyValue(&self, ppropvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PropertyValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppropvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_Property<'a, P0>(&self, propvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).set_Property)(::windows::core::Interface::as_raw(self), propvalue.into().abi()).ok()
    }
}
impl ::core::convert::From<IPMTilePropertyInfo> for ::windows::core::IUnknown {
    fn from(value: IPMTilePropertyInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IPMTilePropertyInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPMTilePropertyInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPMTilePropertyInfo> for ::windows::core::IUnknown {
    fn from(value: &IPMTilePropertyInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IPMTilePropertyInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPMTilePropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTilePropertyInfo {}
impl ::core::fmt::Debug for IPMTilePropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTilePropertyInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPMTilePropertyInfo {
    type Vtable = IPMTilePropertyInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c2b8017_1efa_42a7_86c0_6d4b640bf528);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTilePropertyInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub PropertyID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PropertyValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_Property: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ACTION: &str = "ACTION";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ADMINTOOLS_FOLDER: &str = "AdminToolsFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ADMINUSER: &str = "AdminUser";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ADMIN_PROPERTIES: &str = "AdminProperties";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_AFTERREBOOT: &str = "AFTERREBOOT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ALLOWEDPROPERTIES: &str = "SecureCustomProperties";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ALLUSERS: &str = "ALLUSERS";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_APPDATA_FOLDER: &str = "AppDataFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARM: &str = "Arm";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARM64: &str = "Arm64";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPAUTHORIZEDCDFPREFIX: &str = "ARPAUTHORIZEDCDFPREFIX";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPCOMMENTS: &str = "ARPCOMMENTS";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPCONTACT: &str = "ARPCONTACT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPHELPLINK: &str = "ARPHELPLINK";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPHELPTELEPHONE: &str = "ARPHELPTELEPHONE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPINSTALLLOCATION: &str = "ARPINSTALLLOCATION";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPNOMODIFY: &str = "ARPNOMODIFY";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPNOREMOVE: &str = "ARPNOREMOVE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPNOREPAIR: &str = "ARPNOREPAIR";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPPRODUCTICON: &str = "ARPPRODUCTICON";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPREADME: &str = "ARPREADME";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSETTINGSIDENTIFIER: &str = "MSIARPSETTINGSIDENTIFIER";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSHIMFLAGS: &str = "SHIMFLAGS";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSHIMSERVICEPACKLEVEL: &str = "SHIMSERVICEPACKLEVEL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSHIMVERSIONNT: &str = "SHIMVERSIONNT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSIZE: &str = "ARPSIZE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSYSTEMCOMPONENT: &str = "ARPSYSTEMCOMPONENT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPURLINFOABOUT: &str = "ARPURLINFOABOUT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPURLUPDATEINFO: &str = "ARPURLUPDATEINFO";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_AVAILABLEFREEREG: &str = "AVAILABLEFREEREG";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_BORDERSIDE: &str = "BorderSide";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_BORDERTOP: &str = "BorderTop";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CAPTIONHEIGHT: &str = "CaptionHeight";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CARRYINGNDP: &str = "CARRYINGNDP";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CHECKCRCS: &str = "MSICHECKCRCS";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COLORBITS: &str = "ColorBits";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMMONAPPDATA_FOLDER: &str = "CommonAppDataFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMMONFILES64_FOLDER: &str = "CommonFiles64Folder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMMONFILES_FOLDER: &str = "CommonFilesFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPANYNAME: &str = "COMPANYNAME";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPONENTADDDEFAULT: &str = "COMPADDDEFAULT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPONENTADDLOCAL: &str = "COMPADDLOCAL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPONENTADDSOURCE: &str = "COMPADDSOURCE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPUTERNAME: &str = "ComputerName";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COSTINGCOMPLETE: &str = "CostingComplete";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CUSTOMACTIONDATA: &str = "CustomActionData";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DATE: &str = "Date";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DATETIME: &str = "DateTime";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DEFAULTUIFONT: &str = "DefaultUIFont";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DESKTOP_FOLDER: &str = "DesktopFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DISABLEADVTSHORTCUTS: &str = "DISABLEADVTSHORTCUTS";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DISABLEROLLBACK: &str = "DISABLEROLLBACK";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DISKPROMPT: &str = "DiskPrompt";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ENABLEUSERCONTROL: &str = "EnableUserControl";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ENFORCE_UPGRADE_COMPONENT_RULES: &str = "MSIENFORCEUPGRADECOMPONENTRULES";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_EXECUTEACTION: &str = "EXECUTEACTION";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_EXECUTEMODE: &str = "EXECUTEMODE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FAVORITES_FOLDER: &str = "FavoritesFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADDDEFAULT: &str = "ADDDEFAULT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADDLOCAL: &str = "ADDLOCAL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADDSOURCE: &str = "ADDSOURCE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADVERTISE: &str = "ADVERTISE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREREMOVE: &str = "REMOVE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FILEADDDEFAULT: &str = "FILEADDDEFAULT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FILEADDLOCAL: &str = "FILEADDLOCAL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FILEADDSOURCE: &str = "FILEADDSOURCE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FONTS_FOLDER: &str = "FontsFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_HIDDEN_PROPERTIES: &str = "MsiHiddenProperties";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_HIDECANCEL: &str = "MsiUIHideCancel";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_IA64: &str = "IA64";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLED: &str = "Installed";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLLANGUAGE: &str = "ProductLanguage";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLLEVEL: &str = "INSTALLLEVEL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLPERUSER: &str = "MSIINSTALLPERUSER";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INTEL: &str = "Intel";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INTEL64: &str = "Intel64";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INTERNALINSTALLEDPERUSER: &str = "MSIINTERNALINSTALLEDPERUSER";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ISADMINPACKAGE: &str = "IsAdminPackage";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LEFTUNIT: &str = "LeftUnit";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LIMITUI: &str = "LIMITUI";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LOCALAPPDATA_FOLDER: &str = "LocalAppDataFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LOGACTION: &str = "LOGACTION";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LOGONUSER: &str = "LogonUser";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MANUFACTURER: &str = "Manufacturer";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIAMD64: &str = "MsiAMD64";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIDISABLEEEUI: &str = "MSIDISABLEEEUI";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIDISABLELUAPATCHING: &str = "MSIDISABLELUAPATCHING";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIINSTANCEGUID: &str = "MSIINSTANCEGUID";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSILOGFILELOCATION: &str = "MsiLogFileLocation";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSILOGGINGMODE: &str = "MsiLogging";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSINEWINSTANCE: &str = "MSINEWINSTANCE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSINODISABLEMEDIA: &str = "MSINODISABLEMEDIA";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIPACKAGEDOWNLOADLOCALCOPY: &str = "MSIPACKAGEDOWNLOADLOCALCOPY";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIPATCHDOWNLOADLOCALCOPY: &str = "MSIPATCHDOWNLOADLOCALCOPY";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIPATCHREMOVE: &str = "MSIPATCHREMOVE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSITABLETPC: &str = "MsiTabletPC";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIX64: &str = "Msix64";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_FASTINSTALL: &str = "MSIFASTINSTALL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_REBOOT_PENDING: &str = "MsiSystemRebootPending";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_CONTROL: &str = "MSIRESTARTMANAGERCONTROL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_DISABLE_RESTART: &str = "MSIDISABLERMRESTART";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_SESSION_KEY: &str = "MsiRestartManagerSessionKey";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_SHUTDOWN: &str = "MSIRMSHUTDOWN";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_UAC_DEPLOYMENT_COMPLIANT: &str = "MSIDEPLOYMENTCOMPLIANT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_UNINSTALL_SUPERSEDED_COMPONENTS: &str = "MSIUNINSTALLSUPERSEDEDCOMPONENTS";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_USE_REAL_ADMIN_DETECTION: &str = "MSIUSEREALADMINDETECTION";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MYPICTURES_FOLDER: &str = "MyPicturesFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NETASSEMBLYSUPPORT: &str = "MsiNetAssemblySupport";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NETHOOD_FOLDER: &str = "NetHoodFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NOCOMPANYNAME: &str = "NOCOMPANYNAME";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NOUSERNAME: &str = "NOUSERNAME";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTPRODUCTTYPE: &str = "MsiNTProductType";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEBACKOFFICE: &str = "MsiNTSuiteBackOffice";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEDATACENTER: &str = "MsiNTSuiteDataCenter";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEENTERPRISE: &str = "MsiNTSuiteEnterprise";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEPERSONAL: &str = "MsiNTSuitePersonal";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITESMALLBUSINESS: &str = "MsiNTSuiteSmallBusiness";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITESMALLBUSINESSRESTRICTED: &str = "MsiNTSuiteSmallBusinessRestricted";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEWEBSERVER: &str = "MsiNTSuiteWebServer";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_OLEADVTSUPPORT: &str = "OLEAdvtSupport";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_OUTOFDISKSPACE: &str = "OutOfDiskSpace";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_OUTOFNORBDISKSPACE: &str = "OutOfNoRbDiskSpace";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCH: &str = "PATCH";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCHNEWPACKAGECODE: &str = "PATCHNEWPACKAGECODE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCHNEWSUMMARYCOMMENTS: &str = "PATCHNEWSUMMARYCOMMENTS";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCHNEWSUMMARYSUBJECT: &str = "PATCHNEWSUMMARYSUBJECT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PERSONAL_FOLDER: &str = "PersonalFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PHYSICALMEMORY: &str = "PhysicalMemory";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PIDKEY: &str = "PIDKEY";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PIDTEMPLATE: &str = "PIDTemplate";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRESELECTED: &str = "Preselected";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER: &str = "PRIMARYFOLDER";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_PATH: &str = "PrimaryVolumePath";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_SPACEAVAILABLE: &str = "PrimaryVolumeSpaceAvailable";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_SPACEREMAINING: &str = "PrimaryVolumeSpaceRemaining";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_SPACEREQUIRED: &str = "PrimaryVolumeSpaceRequired";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRINTHOOD_FOLDER: &str = "PrintHoodFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIVILEGED: &str = "Privileged";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTCODE: &str = "ProductCode";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTID: &str = "ProductID";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTLANGUAGE: &str = "PRODUCTLANGUAGE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTNAME: &str = "ProductName";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTSTATE: &str = "ProductState";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTVERSION: &str = "ProductVersion";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRAMFILES64_FOLDER: &str = "ProgramFiles64Folder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRAMFILES_FOLDER: &str = "ProgramFilesFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRAMMENU_FOLDER: &str = "ProgramMenuFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRESSONLY: &str = "MsiUIProgressOnly";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROMPTROLLBACKCOST: &str = "PROMPTROLLBACKCOST";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REBOOT: &str = "REBOOT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REBOOTPROMPT: &str = "REBOOTPROMPT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RECENT_FOLDER: &str = "RecentFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REDIRECTEDDLLSUPPORT: &str = "RedirectedDllSupport";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REINSTALL: &str = "REINSTALL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REINSTALLMODE: &str = "REINSTALLMODE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REMOTEADMINTS: &str = "RemoteAdminTS";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REPLACEDINUSEFILES: &str = "ReplacedInUseFiles";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RESTRICTEDUSERCONTROL: &str = "RestrictedUserControl";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RESUME: &str = "RESUME";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ROLLBACKDISABLED: &str = "RollbackDisabled";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ROOTDRIVE: &str = "ROOTDRIVE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RUNNINGELEVATED: &str = "MsiRunningElevated";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SCREENX: &str = "ScreenX";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SCREENY: &str = "ScreenY";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SENDTO_FOLDER: &str = "SendToFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SEQUENCE: &str = "SEQUENCE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SERVICEPACKLEVEL: &str = "ServicePackLevel";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SERVICEPACKLEVELMINOR: &str = "ServicePackLevelMinor";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SHAREDWINDOWS: &str = "SharedWindows";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SHELLADVTSUPPORT: &str = "ShellAdvtSupport";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SHORTFILENAMES: &str = "SHORTFILENAMES";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SOURCEDIR: &str = "SourceDir";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SOURCELIST: &str = "SOURCELIST";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SOURCERESONLY: &str = "MsiUISourceResOnly";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_STARTMENU_FOLDER: &str = "StartMenuFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_STARTUP_FOLDER: &str = "StartupFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEM16_FOLDER: &str = "System16Folder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEM64_FOLDER: &str = "System64Folder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEMLANGUAGEID: &str = "SystemLanguageID";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEM_FOLDER: &str = "SystemFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TARGETDIR: &str = "TARGETDIR";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMPLATE_AMD64: &str = "AMD64";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMPLATE_FOLDER: &str = "TemplateFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMPLATE_X64: &str = "x64";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMP_FOLDER: &str = "TempFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TERMSERVER: &str = "TerminalServer";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEXTHEIGHT: &str = "TextHeight";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEXTHEIGHT_CORRECTION: &str = "TextHeightCorrection";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEXTINTERNALLEADING: &str = "TextInternalLeading";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TIME: &str = "Time";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRANSFORMS: &str = "TRANSFORMS";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRANSFORMSATSOURCE: &str = "TRANSFORMSATSOURCE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRANSFORMSSECURE: &str = "TRANSFORMSSECURE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRUEADMINUSER: &str = "MsiTrueAdminUser";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TTCSUPPORT: &str = "TTCSupport";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_UACONLY: &str = "MsiUIUACOnly";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_UPDATESTARTED: &str = "UpdateStarted";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_UPGRADECODE: &str = "UpgradeCode";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_USERLANGUAGEID: &str = "UserLanguageID";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_USERNAME: &str = "USERNAME";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_USERSID: &str = "UserSID";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VERSION9X: &str = "Version9X";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VERSIONNT: &str = "VersionNT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VERSIONNT64: &str = "VersionNT64";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VIRTUALMEMORY: &str = "VirtualMemory";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WIN32ASSEMBLYSUPPORT: &str = "MsiWin32AssemblySupport";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WINDOWSBUILD: &str = "WindowsBuild";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WINDOWS_FOLDER: &str = "WindowsFolder";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WINDOWS_VOLUME: &str = "WindowsVolume";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_EXECUTEMODE_NONE: &str = "NONE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_EXECUTEMODE_SCRIPT: &str = "SCRIPT";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_FEATURE_ALL: &str = "ALL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_MSI_RM_CONTROL_DISABLE: &str = "Disable";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_MSI_RM_CONTROL_DISABLESHUTDOWN: &str = "DisableShutdown";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_RBCOST_FAIL: &str = "F";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_RBCOST_PROMPT: &str = "P";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_RBCOST_SILENT: &str = "D";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE__CARRYINGNDP_URTREINSTALL: &str = "URTREINSTALL";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE__CARRYINGNDP_URTUPGRADE: &str = "URTUPGRADE";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IValidate(::windows::core::IUnknown);
impl IValidate {
    pub unsafe fn OpenDatabase<'a, P0>(&self, szdatabase: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OpenDatabase)(::windows::core::Interface::as_raw(self), szdatabase.into()).ok()
    }
    pub unsafe fn OpenCUB<'a, P0>(&self, szcubfile: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OpenCUB)(::windows::core::Interface::as_raw(self), szcubfile.into()).ok()
    }
    pub unsafe fn CloseDatabase(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseDatabase)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CloseCUB(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseCUB)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplay(&self, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdisplayfunction), ::core::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatus(&self, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstatusfunction), ::core::mem::transmute(pcontext)).ok()
    }
    pub unsafe fn Validate<'a, P0>(&self, wzices: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Validate)(::windows::core::Interface::as_raw(self), wzices.into()).ok()
    }
}
impl ::core::convert::From<IValidate> for ::windows::core::IUnknown {
    fn from(value: IValidate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IValidate> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IValidate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IValidate> for ::windows::core::IUnknown {
    fn from(value: &IValidate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IValidate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IValidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IValidate {}
impl ::core::fmt::Debug for IValidate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValidate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IValidate {
    type Vtable = IValidate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe482e5c6_e31e_4143_a2e6_dbc3d8e4b8d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValidate_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OpenDatabase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szdatabase: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OpenCUB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szcubfile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub CloseDatabase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseCUB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplayfunction: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatusfunction: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStatus: usize,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzices: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
pub const LIBID_MsmMergeTypeLib: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adda82f_2c26_11d2_ad65_00a0c9af11a6);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGERR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGINFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGNONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGPERFMESSAGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_NO_LOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_SETUPAPI_APPLOG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_SETUPAPI_DEVLOG: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGWARN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDISPLAYVAL = ::core::option::Option<unsafe extern "system" fn(pcontext: *mut ::core::ffi::c_void, uitype: RESULTTYPES, szwval: ::windows::core::PCWSTR, szwdescription: ::windows::core::PCWSTR, szwlocation: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPEVALCOMCALLBACK = ::core::option::Option<unsafe extern "system" fn(istatus: STATUSTYPES, szdata: ::windows::core::PCWSTR, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MAX_FEATURE_CHARS: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MAX_GUID_CHARS: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIADVERTISEOPTIONFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIADVERTISEOPTIONFLAGS_INSTANCE: MSIADVERTISEOPTIONFLAGS = MSIADVERTISEOPTIONFLAGS(1i32);
impl ::core::marker::Copy for MSIADVERTISEOPTIONFLAGS {}
impl ::core::clone::Clone for MSIADVERTISEOPTIONFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIADVERTISEOPTIONFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSIADVERTISEOPTIONFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSIADVERTISEOPTIONFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIADVERTISEOPTIONFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIARCHITECTUREFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_X86: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_IA64: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_AMD64: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_ARM: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(8i32);
impl ::core::marker::Copy for MSIARCHITECTUREFLAGS {}
impl ::core::clone::Clone for MSIARCHITECTUREFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIARCHITECTUREFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSIARCHITECTUREFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSIARCHITECTUREFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIARCHITECTUREFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIASSEMBLYINFO(pub u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIASSEMBLYINFO_NETASSEMBLY: MSIASSEMBLYINFO = MSIASSEMBLYINFO(0u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIASSEMBLYINFO_WIN32ASSEMBLY: MSIASSEMBLYINFO = MSIASSEMBLYINFO(1u32);
impl ::core::marker::Copy for MSIASSEMBLYINFO {}
impl ::core::clone::Clone for MSIASSEMBLYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIASSEMBLYINFO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSIASSEMBLYINFO {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSIASSEMBLYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIASSEMBLYINFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSICODE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICODE_PRODUCT: MSICODE = MSICODE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICODE_PATCH: MSICODE = MSICODE(1073741824i32);
impl ::core::marker::Copy for MSICODE {}
impl ::core::clone::Clone for MSICODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSICODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSICODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSICODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSICODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSICOLINFO(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOLINFO_NAMES: MSICOLINFO = MSICOLINFO(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOLINFO_TYPES: MSICOLINFO = MSICOLINFO(1i32);
impl ::core::marker::Copy for MSICOLINFO {}
impl ::core::clone::Clone for MSICOLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSICOLINFO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSICOLINFO {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSICOLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSICOLINFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSICONDITION(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_FALSE: MSICONDITION = MSICONDITION(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_TRUE: MSICONDITION = MSICONDITION(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_NONE: MSICONDITION = MSICONDITION(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_ERROR: MSICONDITION = MSICONDITION(3i32);
impl ::core::marker::Copy for MSICONDITION {}
impl ::core::clone::Clone for MSICONDITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSICONDITION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSICONDITION {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSICONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSICONDITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSICOSTTREE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_SELFONLY: MSICOSTTREE = MSICOSTTREE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_CHILDREN: MSICOSTTREE = MSICOSTTREE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_PARENTS: MSICOSTTREE = MSICOSTTREE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_RESERVED: MSICOSTTREE = MSICOSTTREE(3i32);
impl ::core::marker::Copy for MSICOSTTREE {}
impl ::core::clone::Clone for MSICOSTTREE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSICOSTTREE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSICOSTTREE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSICOSTTREE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSICOSTTREE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIDBERROR(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_INVALIDARG: MSIDBERROR = MSIDBERROR(-3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_MOREDATA: MSIDBERROR = MSIDBERROR(-2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_FUNCTIONERROR: MSIDBERROR = MSIDBERROR(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_NOERROR: MSIDBERROR = MSIDBERROR(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_DUPLICATEKEY: MSIDBERROR = MSIDBERROR(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_REQUIRED: MSIDBERROR = MSIDBERROR(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADLINK: MSIDBERROR = MSIDBERROR(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_OVERFLOW: MSIDBERROR = MSIDBERROR(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_UNDERFLOW: MSIDBERROR = MSIDBERROR(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_NOTINSET: MSIDBERROR = MSIDBERROR(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADVERSION: MSIDBERROR = MSIDBERROR(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCASE: MSIDBERROR = MSIDBERROR(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADGUID: MSIDBERROR = MSIDBERROR(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADWILDCARD: MSIDBERROR = MSIDBERROR(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADIDENTIFIER: MSIDBERROR = MSIDBERROR(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADLANGUAGE: MSIDBERROR = MSIDBERROR(12i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADFILENAME: MSIDBERROR = MSIDBERROR(13i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADPATH: MSIDBERROR = MSIDBERROR(14i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCONDITION: MSIDBERROR = MSIDBERROR(15i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADFORMATTED: MSIDBERROR = MSIDBERROR(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADTEMPLATE: MSIDBERROR = MSIDBERROR(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADDEFAULTDIR: MSIDBERROR = MSIDBERROR(18i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADREGPATH: MSIDBERROR = MSIDBERROR(19i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCUSTOMSOURCE: MSIDBERROR = MSIDBERROR(20i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADPROPERTY: MSIDBERROR = MSIDBERROR(21i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_MISSINGDATA: MSIDBERROR = MSIDBERROR(22i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCATEGORY: MSIDBERROR = MSIDBERROR(23i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADKEYTABLE: MSIDBERROR = MSIDBERROR(24i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADMAXMINVALUES: MSIDBERROR = MSIDBERROR(25i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCABINET: MSIDBERROR = MSIDBERROR(26i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADSHORTCUT: MSIDBERROR = MSIDBERROR(27i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_STRINGOVERFLOW: MSIDBERROR = MSIDBERROR(28i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADLOCALIZEATTRIB: MSIDBERROR = MSIDBERROR(29i32);
impl ::core::marker::Copy for MSIDBERROR {}
impl ::core::clone::Clone for MSIDBERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIDBERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSIDBERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSIDBERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIDBERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIDBSTATE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBSTATE_ERROR: MSIDBSTATE = MSIDBSTATE(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBSTATE_READ: MSIDBSTATE = MSIDBSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBSTATE_WRITE: MSIDBSTATE = MSIDBSTATE(1i32);
impl ::core::marker::Copy for MSIDBSTATE {}
impl ::core::clone::Clone for MSIDBSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIDBSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSIDBSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSIDBSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIDBSTATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct MSIFILEHASHINFO {
    pub dwFileHashInfoSize: u32,
    pub dwData: [u32; 4],
}
impl ::core::marker::Copy for MSIFILEHASHINFO {}
impl ::core::clone::Clone for MSIFILEHASHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSIFILEHASHINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSIFILEHASHINFO").field("dwFileHashInfoSize", &self.dwFileHashInfoSize).field("dwData", &self.dwData).finish()
    }
}
unsafe impl ::windows::core::Abi for MSIFILEHASHINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MSIFILEHASHINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSIFILEHASHINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MSIFILEHASHINFO {}
impl ::core::default::Default for MSIFILEHASHINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIHANDLE(pub u32);
impl MSIHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0
    }
}
impl ::core::default::Default for MSIHANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for MSIHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for MSIHANDLE {}
impl ::core::fmt::Debug for MSIHANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIHANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<MSIHANDLE>> for MSIHANDLE {
    fn from(optional: ::core::option::Option<MSIHANDLE>) -> MSIHANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for MSIHANDLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIINSTALLCONTEXT(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_FIRSTVISIBLE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_NONE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_USERMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_USERUNMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_MACHINE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_ALL: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_ALLUSERMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(8i32);
impl ::core::marker::Copy for MSIINSTALLCONTEXT {}
impl ::core::clone::Clone for MSIINSTALLCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIINSTALLCONTEXT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSIINSTALLCONTEXT {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSIINSTALLCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIINSTALLCONTEXT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIMODIFY(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_SEEK: MSIMODIFY = MSIMODIFY(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_REFRESH: MSIMODIFY = MSIMODIFY(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_INSERT: MSIMODIFY = MSIMODIFY(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_UPDATE: MSIMODIFY = MSIMODIFY(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_ASSIGN: MSIMODIFY = MSIMODIFY(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_REPLACE: MSIMODIFY = MSIMODIFY(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_MERGE: MSIMODIFY = MSIMODIFY(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_DELETE: MSIMODIFY = MSIMODIFY(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_INSERT_TEMPORARY: MSIMODIFY = MSIMODIFY(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE: MSIMODIFY = MSIMODIFY(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE_NEW: MSIMODIFY = MSIMODIFY(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE_FIELD: MSIMODIFY = MSIMODIFY(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE_DELETE: MSIMODIFY = MSIMODIFY(11i32);
impl ::core::marker::Copy for MSIMODIFY {}
impl ::core::clone::Clone for MSIMODIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIMODIFY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSIMODIFY {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSIMODIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIMODIFY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIOPENPACKAGEFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIOPENPACKAGEFLAGS_IGNOREMACHINESTATE: MSIOPENPACKAGEFLAGS = MSIOPENPACKAGEFLAGS(1i32);
impl ::core::marker::Copy for MSIOPENPACKAGEFLAGS {}
impl ::core::clone::Clone for MSIOPENPACKAGEFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIOPENPACKAGEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSIOPENPACKAGEFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSIOPENPACKAGEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIOPENPACKAGEFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIPATCHDATATYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCH_DATATYPE_PATCHFILE: MSIPATCHDATATYPE = MSIPATCHDATATYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCH_DATATYPE_XMLPATH: MSIPATCHDATATYPE = MSIPATCHDATATYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCH_DATATYPE_XMLBLOB: MSIPATCHDATATYPE = MSIPATCHDATATYPE(2i32);
impl ::core::marker::Copy for MSIPATCHDATATYPE {}
impl ::core::clone::Clone for MSIPATCHDATATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIPATCHDATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSIPATCHDATATYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSIPATCHDATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIPATCHDATATYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct MSIPATCHSEQUENCEINFOA {
    pub szPatchData: ::windows::core::PCSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
impl ::core::marker::Copy for MSIPATCHSEQUENCEINFOA {}
impl ::core::clone::Clone for MSIPATCHSEQUENCEINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSIPATCHSEQUENCEINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSIPATCHSEQUENCEINFOA").field("szPatchData", &self.szPatchData).field("ePatchDataType", &self.ePatchDataType).field("dwOrder", &self.dwOrder).field("uStatus", &self.uStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for MSIPATCHSEQUENCEINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MSIPATCHSEQUENCEINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSIPATCHSEQUENCEINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MSIPATCHSEQUENCEINFOA {}
impl ::core::default::Default for MSIPATCHSEQUENCEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct MSIPATCHSEQUENCEINFOW {
    pub szPatchData: ::windows::core::PCWSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
impl ::core::marker::Copy for MSIPATCHSEQUENCEINFOW {}
impl ::core::clone::Clone for MSIPATCHSEQUENCEINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSIPATCHSEQUENCEINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSIPATCHSEQUENCEINFOW").field("szPatchData", &self.szPatchData).field("ePatchDataType", &self.ePatchDataType).field("dwOrder", &self.dwOrder).field("uStatus", &self.uStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for MSIPATCHSEQUENCEINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MSIPATCHSEQUENCEINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSIPATCHSEQUENCEINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MSIPATCHSEQUENCEINFOW {}
impl ::core::default::Default for MSIPATCHSEQUENCEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIPATCHSTATE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_INVALID: MSIPATCHSTATE = MSIPATCHSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_APPLIED: MSIPATCHSTATE = MSIPATCHSTATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_SUPERSEDED: MSIPATCHSTATE = MSIPATCHSTATE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_OBSOLETED: MSIPATCHSTATE = MSIPATCHSTATE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_REGISTERED: MSIPATCHSTATE = MSIPATCHSTATE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_ALL: MSIPATCHSTATE = MSIPATCHSTATE(15i32);
impl ::core::marker::Copy for MSIPATCHSTATE {}
impl ::core::clone::Clone for MSIPATCHSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIPATCHSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSIPATCHSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSIPATCHSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIPATCHSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIRUNMODE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ADMIN: MSIRUNMODE = MSIRUNMODE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ADVERTISE: MSIRUNMODE = MSIRUNMODE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_MAINTENANCE: MSIRUNMODE = MSIRUNMODE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ROLLBACKENABLED: MSIRUNMODE = MSIRUNMODE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_LOGENABLED: MSIRUNMODE = MSIRUNMODE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_OPERATIONS: MSIRUNMODE = MSIRUNMODE(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_REBOOTATEND: MSIRUNMODE = MSIRUNMODE(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_REBOOTNOW: MSIRUNMODE = MSIRUNMODE(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_CABINET: MSIRUNMODE = MSIRUNMODE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_SOURCESHORTNAMES: MSIRUNMODE = MSIRUNMODE(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_TARGETSHORTNAMES: MSIRUNMODE = MSIRUNMODE(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_RESERVED11: MSIRUNMODE = MSIRUNMODE(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_WINDOWS9X: MSIRUNMODE = MSIRUNMODE(12i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ZAWENABLED: MSIRUNMODE = MSIRUNMODE(13i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_RESERVED14: MSIRUNMODE = MSIRUNMODE(14i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_RESERVED15: MSIRUNMODE = MSIRUNMODE(15i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_SCHEDULED: MSIRUNMODE = MSIRUNMODE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ROLLBACK: MSIRUNMODE = MSIRUNMODE(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_COMMIT: MSIRUNMODE = MSIRUNMODE(18i32);
impl ::core::marker::Copy for MSIRUNMODE {}
impl ::core::clone::Clone for MSIRUNMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIRUNMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSIRUNMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSIRUNMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIRUNMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSISOURCETYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_UNKNOWN: MSISOURCETYPE = MSISOURCETYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_NETWORK: MSISOURCETYPE = MSISOURCETYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_URL: MSISOURCETYPE = MSISOURCETYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_MEDIA: MSISOURCETYPE = MSISOURCETYPE(4i32);
impl ::core::marker::Copy for MSISOURCETYPE {}
impl ::core::clone::Clone for MSISOURCETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSISOURCETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSISOURCETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSISOURCETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSISOURCETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSITRANSACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTION_CHAIN_EMBEDDEDUI: MSITRANSACTION = MSITRANSACTION(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTION_JOIN_EXISTING_EMBEDDEDUI: MSITRANSACTION = MSITRANSACTION(2i32);
impl ::core::marker::Copy for MSITRANSACTION {}
impl ::core::clone::Clone for MSITRANSACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSITRANSACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSITRANSACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSITRANSACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSITRANSACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSITRANSACTIONSTATE(pub u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTIONSTATE_ROLLBACK: MSITRANSACTIONSTATE = MSITRANSACTIONSTATE(0u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTIONSTATE_COMMIT: MSITRANSACTIONSTATE = MSITRANSACTIONSTATE(1u32);
impl ::core::marker::Copy for MSITRANSACTIONSTATE {}
impl ::core::clone::Clone for MSITRANSACTIONSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSITRANSACTIONSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSITRANSACTIONSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSITRANSACTIONSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSITRANSACTIONSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSITRANSFORM_ERROR(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_ADDEXISTINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_DELMISSINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_ADDEXISTINGTABLE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_DELMISSINGTABLE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_UPDATEMISSINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_CHANGECODEPAGE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_VIEWTRANSFORM: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_NONE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(0i32);
impl ::core::marker::Copy for MSITRANSFORM_ERROR {}
impl ::core::clone::Clone for MSITRANSFORM_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSITRANSFORM_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSITRANSFORM_ERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSITRANSFORM_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSITRANSFORM_ERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSITRANSFORM_VALIDATE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_LANGUAGE: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_PRODUCT: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_PLATFORM: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_MAJORVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_MINORVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_UPDATEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWLESSBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWLESSEQUALBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWEQUALBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWGREATEREQUALBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWGREATERBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_UPGRADECODE: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(2048i32);
impl ::core::marker::Copy for MSITRANSFORM_VALIDATE {}
impl ::core::clone::Clone for MSITRANSFORM_VALIDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSITRANSFORM_VALIDATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSITRANSFORM_VALIDATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSITRANSFORM_VALIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSITRANSFORM_VALIDATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSI_INVALID_HASH_IS_FATAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSI_NULL_INTEGER: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiAdvertiseProductA<'a, P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiAdvertiseProductA(szpackagepath: ::windows::core::PCSTR, szscriptfilepath: ::windows::core::PCSTR, sztransforms: ::windows::core::PCSTR, lgidlanguage: u16) -> u32;
    }
    MsiAdvertiseProductA(szpackagepath.into(), szscriptfilepath.into(), sztransforms.into(), lgidlanguage)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiAdvertiseProductExA<'a, P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiAdvertiseProductExA(szpackagepath: ::windows::core::PCSTR, szscriptfilepath: ::windows::core::PCSTR, sztransforms: ::windows::core::PCSTR, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32;
    }
    MsiAdvertiseProductExA(szpackagepath.into(), szscriptfilepath.into(), sztransforms.into(), lgidlanguage, dwplatform, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiAdvertiseProductExW<'a, P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiAdvertiseProductExW(szpackagepath: ::windows::core::PCWSTR, szscriptfilepath: ::windows::core::PCWSTR, sztransforms: ::windows::core::PCWSTR, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32;
    }
    MsiAdvertiseProductExW(szpackagepath.into(), szscriptfilepath.into(), sztransforms.into(), lgidlanguage, dwplatform, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiAdvertiseProductW<'a, P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiAdvertiseProductW(szpackagepath: ::windows::core::PCWSTR, szscriptfilepath: ::windows::core::PCWSTR, sztransforms: ::windows::core::PCWSTR, lgidlanguage: u16) -> u32;
    }
    MsiAdvertiseProductW(szpackagepath.into(), szscriptfilepath.into(), sztransforms.into(), lgidlanguage)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiAdvertiseScriptA<'a, P0, P1>(szscriptfile: P0, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiAdvertiseScriptA(szscriptfile: ::windows::core::PCSTR, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: super::super::Foundation::BOOL) -> u32;
    }
    MsiAdvertiseScriptA(szscriptfile.into(), dwflags, ::core::mem::transmute(phregdata), fremoveitems.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiAdvertiseScriptW<'a, P0, P1>(szscriptfile: P0, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiAdvertiseScriptW(szscriptfile: ::windows::core::PCWSTR, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: super::super::Foundation::BOOL) -> u32;
    }
    MsiAdvertiseScriptW(szscriptfile.into(), dwflags, ::core::mem::transmute(phregdata), fremoveitems.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiApplyMultiplePatchesA<'a, P0, P1, P2>(szpatchpackages: P0, szproductcode: P1, szpropertieslist: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiApplyMultiplePatchesA(szpatchpackages: ::windows::core::PCSTR, szproductcode: ::windows::core::PCSTR, szpropertieslist: ::windows::core::PCSTR) -> u32;
    }
    MsiApplyMultiplePatchesA(szpatchpackages.into(), szproductcode.into(), szpropertieslist.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiApplyMultiplePatchesW<'a, P0, P1, P2>(szpatchpackages: P0, szproductcode: P1, szpropertieslist: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiApplyMultiplePatchesW(szpatchpackages: ::windows::core::PCWSTR, szproductcode: ::windows::core::PCWSTR, szpropertieslist: ::windows::core::PCWSTR) -> u32;
    }
    MsiApplyMultiplePatchesW(szpatchpackages.into(), szproductcode.into(), szpropertieslist.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiApplyPatchA<'a, P0, P1, P2>(szpatchpackage: P0, szinstallpackage: P1, einstalltype: INSTALLTYPE, szcommandline: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiApplyPatchA(szpatchpackage: ::windows::core::PCSTR, szinstallpackage: ::windows::core::PCSTR, einstalltype: INSTALLTYPE, szcommandline: ::windows::core::PCSTR) -> u32;
    }
    MsiApplyPatchA(szpatchpackage.into(), szinstallpackage.into(), einstalltype, szcommandline.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiApplyPatchW<'a, P0, P1, P2>(szpatchpackage: P0, szinstallpackage: P1, einstalltype: INSTALLTYPE, szcommandline: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiApplyPatchW(szpatchpackage: ::windows::core::PCWSTR, szinstallpackage: ::windows::core::PCWSTR, einstalltype: INSTALLTYPE, szcommandline: ::windows::core::PCWSTR) -> u32;
    }
    MsiApplyPatchW(szpatchpackage.into(), szinstallpackage.into(), einstalltype, szcommandline.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiBeginTransactionA<'a, P0>(szname: P0, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiBeginTransactionA(szname: ::windows::core::PCSTR, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MsiBeginTransactionA(szname.into(), dwtransactionattributes, ::core::mem::transmute(phtransactionhandle), ::core::mem::transmute(phchangeofownerevent))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiBeginTransactionW<'a, P0>(szname: P0, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiBeginTransactionW(szname: ::windows::core::PCWSTR, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MsiBeginTransactionW(szname.into(), dwtransactionattributes, ::core::mem::transmute(phtransactionhandle), ::core::mem::transmute(phchangeofownerevent))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCloseAllHandles() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiCloseAllHandles() -> u32;
    }
    MsiCloseAllHandles()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCloseHandle<'a, P0>(hany: P0) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiCloseHandle(hany: MSIHANDLE) -> u32;
    }
    MsiCloseHandle(hany.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCollectUserInfoA<'a, P0>(szproduct: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiCollectUserInfoA(szproduct: ::windows::core::PCSTR) -> u32;
    }
    MsiCollectUserInfoA(szproduct.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCollectUserInfoW<'a, P0>(szproduct: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiCollectUserInfoW(szproduct: ::windows::core::PCWSTR) -> u32;
    }
    MsiCollectUserInfoW(szproduct.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureFeatureA<'a, P0, P1>(szproduct: P0, szfeature: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiConfigureFeatureA(szproduct: ::windows::core::PCSTR, szfeature: ::windows::core::PCSTR, einstallstate: INSTALLSTATE) -> u32;
    }
    MsiConfigureFeatureA(szproduct.into(), szfeature.into(), einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureFeatureW<'a, P0, P1>(szproduct: P0, szfeature: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiConfigureFeatureW(szproduct: ::windows::core::PCWSTR, szfeature: ::windows::core::PCWSTR, einstallstate: INSTALLSTATE) -> u32;
    }
    MsiConfigureFeatureW(szproduct.into(), szfeature.into(), einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureProductA<'a, P0>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiConfigureProductA(szproduct: ::windows::core::PCSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32;
    }
    MsiConfigureProductA(szproduct.into(), iinstalllevel, einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureProductExA<'a, P0, P1>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiConfigureProductExA(szproduct: ::windows::core::PCSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: ::windows::core::PCSTR) -> u32;
    }
    MsiConfigureProductExA(szproduct.into(), iinstalllevel, einstallstate, szcommandline.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureProductExW<'a, P0, P1>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiConfigureProductExW(szproduct: ::windows::core::PCWSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: ::windows::core::PCWSTR) -> u32;
    }
    MsiConfigureProductExW(szproduct.into(), iinstalllevel, einstallstate, szcommandline.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureProductW<'a, P0>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiConfigureProductW(szproduct: ::windows::core::PCWSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32;
    }
    MsiConfigureProductW(szproduct.into(), iinstalllevel, einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCreateRecord(cparams: u32) -> MSIHANDLE {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiCreateRecord(cparams: u32) -> MSIHANDLE;
    }
    MsiCreateRecord(cparams)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoA<'a, P0, P1, P2>(hdatabase: P0, hdatabasereference: P1, sztransformfile: P2, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<MSIHANDLE>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiCreateTransformSummaryInfoA(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: ::windows::core::PCSTR, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32;
    }
    MsiCreateTransformSummaryInfoA(hdatabase.into(), hdatabasereference.into(), sztransformfile.into(), ierrorconditions, ivalidation)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoW<'a, P0, P1, P2>(hdatabase: P0, hdatabasereference: P1, sztransformfile: P2, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<MSIHANDLE>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiCreateTransformSummaryInfoW(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: ::windows::core::PCWSTR, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32;
    }
    MsiCreateTransformSummaryInfoW(hdatabase.into(), hdatabasereference.into(), sztransformfile.into(), ierrorconditions, ivalidation)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseApplyTransformA<'a, P0, P1>(hdatabase: P0, sztransformfile: P1, ierrorconditions: MSITRANSFORM_ERROR) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseApplyTransformA(hdatabase: MSIHANDLE, sztransformfile: ::windows::core::PCSTR, ierrorconditions: MSITRANSFORM_ERROR) -> u32;
    }
    MsiDatabaseApplyTransformA(hdatabase.into(), sztransformfile.into(), ierrorconditions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseApplyTransformW<'a, P0, P1>(hdatabase: P0, sztransformfile: P1, ierrorconditions: MSITRANSFORM_ERROR) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseApplyTransformW(hdatabase: MSIHANDLE, sztransformfile: ::windows::core::PCWSTR, ierrorconditions: MSITRANSFORM_ERROR) -> u32;
    }
    MsiDatabaseApplyTransformW(hdatabase.into(), sztransformfile.into(), ierrorconditions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseCommit<'a, P0>(hdatabase: P0) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseCommit(hdatabase: MSIHANDLE) -> u32;
    }
    MsiDatabaseCommit(hdatabase.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseExportA<'a, P0, P1, P2, P3>(hdatabase: P0, sztablename: P1, szfolderpath: P2, szfilename: P3) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseExportA(hdatabase: MSIHANDLE, sztablename: ::windows::core::PCSTR, szfolderpath: ::windows::core::PCSTR, szfilename: ::windows::core::PCSTR) -> u32;
    }
    MsiDatabaseExportA(hdatabase.into(), sztablename.into(), szfolderpath.into(), szfilename.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseExportW<'a, P0, P1, P2, P3>(hdatabase: P0, sztablename: P1, szfolderpath: P2, szfilename: P3) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseExportW(hdatabase: MSIHANDLE, sztablename: ::windows::core::PCWSTR, szfolderpath: ::windows::core::PCWSTR, szfilename: ::windows::core::PCWSTR) -> u32;
    }
    MsiDatabaseExportW(hdatabase.into(), sztablename.into(), szfolderpath.into(), szfilename.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformA<'a, P0, P1, P2>(hdatabase: P0, hdatabasereference: P1, sztransformfile: P2, ireserved1: i32, ireserved2: i32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<MSIHANDLE>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseGenerateTransformA(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: ::windows::core::PCSTR, ireserved1: i32, ireserved2: i32) -> u32;
    }
    MsiDatabaseGenerateTransformA(hdatabase.into(), hdatabasereference.into(), sztransformfile.into(), ireserved1, ireserved2)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformW<'a, P0, P1, P2>(hdatabase: P0, hdatabasereference: P1, sztransformfile: P2, ireserved1: i32, ireserved2: i32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<MSIHANDLE>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseGenerateTransformW(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: ::windows::core::PCWSTR, ireserved1: i32, ireserved2: i32) -> u32;
    }
    MsiDatabaseGenerateTransformW(hdatabase.into(), hdatabasereference.into(), sztransformfile.into(), ireserved1, ireserved2)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysA<'a, P0, P1>(hdatabase: P0, sztablename: P1, phrecord: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseGetPrimaryKeysA(hdatabase: MSIHANDLE, sztablename: ::windows::core::PCSTR, phrecord: *mut MSIHANDLE) -> u32;
    }
    MsiDatabaseGetPrimaryKeysA(hdatabase.into(), sztablename.into(), ::core::mem::transmute(phrecord))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysW<'a, P0, P1>(hdatabase: P0, sztablename: P1, phrecord: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseGetPrimaryKeysW(hdatabase: MSIHANDLE, sztablename: ::windows::core::PCWSTR, phrecord: *mut MSIHANDLE) -> u32;
    }
    MsiDatabaseGetPrimaryKeysW(hdatabase.into(), sztablename.into(), ::core::mem::transmute(phrecord))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseImportA<'a, P0, P1, P2>(hdatabase: P0, szfolderpath: P1, szfilename: P2) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseImportA(hdatabase: MSIHANDLE, szfolderpath: ::windows::core::PCSTR, szfilename: ::windows::core::PCSTR) -> u32;
    }
    MsiDatabaseImportA(hdatabase.into(), szfolderpath.into(), szfilename.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseImportW<'a, P0, P1, P2>(hdatabase: P0, szfolderpath: P1, szfilename: P2) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseImportW(hdatabase: MSIHANDLE, szfolderpath: ::windows::core::PCWSTR, szfilename: ::windows::core::PCWSTR) -> u32;
    }
    MsiDatabaseImportW(hdatabase.into(), szfolderpath.into(), szfilename.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentA<'a, P0, P1>(hdatabase: P0, sztablename: P1) -> MSICONDITION
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseIsTablePersistentA(hdatabase: MSIHANDLE, sztablename: ::windows::core::PCSTR) -> MSICONDITION;
    }
    MsiDatabaseIsTablePersistentA(hdatabase.into(), sztablename.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentW<'a, P0, P1>(hdatabase: P0, sztablename: P1) -> MSICONDITION
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseIsTablePersistentW(hdatabase: MSIHANDLE, sztablename: ::windows::core::PCWSTR) -> MSICONDITION;
    }
    MsiDatabaseIsTablePersistentW(hdatabase.into(), sztablename.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseMergeA<'a, P0, P1, P2>(hdatabase: P0, hdatabasemerge: P1, sztablename: P2) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<MSIHANDLE>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseMergeA(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: ::windows::core::PCSTR) -> u32;
    }
    MsiDatabaseMergeA(hdatabase.into(), hdatabasemerge.into(), sztablename.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseMergeW<'a, P0, P1, P2>(hdatabase: P0, hdatabasemerge: P1, sztablename: P2) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<MSIHANDLE>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseMergeW(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: ::windows::core::PCWSTR) -> u32;
    }
    MsiDatabaseMergeW(hdatabase.into(), hdatabasemerge.into(), sztablename.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseOpenViewA<'a, P0, P1>(hdatabase: P0, szquery: P1, phview: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseOpenViewA(hdatabase: MSIHANDLE, szquery: ::windows::core::PCSTR, phview: *mut MSIHANDLE) -> u32;
    }
    MsiDatabaseOpenViewA(hdatabase.into(), szquery.into(), ::core::mem::transmute(phview))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseOpenViewW<'a, P0, P1>(hdatabase: P0, szquery: P1, phview: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDatabaseOpenViewW(hdatabase: MSIHANDLE, szquery: ::windows::core::PCWSTR, phview: *mut MSIHANDLE) -> u32;
    }
    MsiDatabaseOpenViewW(hdatabase.into(), szquery.into(), ::core::mem::transmute(phview))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesA<'a, P0>(szproductpackagepath: P0, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOA]) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDetermineApplicablePatchesA(szproductpackagepath: ::windows::core::PCSTR, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOA) -> u32;
    }
    MsiDetermineApplicablePatchesA(szproductpackagepath.into(), ppatchinfo.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppatchinfo)))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesW<'a, P0>(szproductpackagepath: P0, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOW]) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDetermineApplicablePatchesW(szproductpackagepath: ::windows::core::PCWSTR, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOW) -> u32;
    }
    MsiDetermineApplicablePatchesW(szproductpackagepath.into(), ppatchinfo.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppatchinfo)))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDeterminePatchSequenceA<'a, P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOA]) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDeterminePatchSequenceA(szproductcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOA) -> u32;
    }
    MsiDeterminePatchSequenceA(szproductcode.into(), szusersid.into(), dwcontext, ppatchinfo.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppatchinfo)))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDeterminePatchSequenceW<'a, P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOW]) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDeterminePatchSequenceW(szproductcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOW) -> u32;
    }
    MsiDeterminePatchSequenceW(szproductcode.into(), szusersid.into(), dwcontext, ppatchinfo.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppatchinfo)))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDoActionA<'a, P0, P1>(hinstall: P0, szaction: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDoActionA(hinstall: MSIHANDLE, szaction: ::windows::core::PCSTR) -> u32;
    }
    MsiDoActionA(hinstall.into(), szaction.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDoActionW<'a, P0, P1>(hinstall: P0, szaction: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiDoActionW(hinstall: MSIHANDLE, szaction: ::windows::core::PCWSTR) -> u32;
    }
    MsiDoActionW(hinstall.into(), szaction.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnableLogA<'a, P0>(dwlogmode: INSTALLOGMODE, szlogfile: P0, dwlogattributes: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnableLogA(dwlogmode: INSTALLOGMODE, szlogfile: ::windows::core::PCSTR, dwlogattributes: u32) -> u32;
    }
    MsiEnableLogA(dwlogmode, szlogfile.into(), dwlogattributes)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnableLogW<'a, P0>(dwlogmode: INSTALLOGMODE, szlogfile: P0, dwlogattributes: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnableLogW(dwlogmode: INSTALLOGMODE, szlogfile: ::windows::core::PCWSTR, dwlogattributes: u32) -> u32;
    }
    MsiEnableLogW(dwlogmode, szlogfile.into(), dwlogattributes)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnableUIPreview<'a, P0>(hdatabase: P0, phpreview: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnableUIPreview(hdatabase: MSIHANDLE, phpreview: *mut MSIHANDLE) -> u32;
    }
    MsiEnableUIPreview(hdatabase.into(), ::core::mem::transmute(phpreview))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEndTransaction(dwtransactionstate: MSITRANSACTIONSTATE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEndTransaction(dwtransactionstate: MSITRANSACTIONSTATE) -> u32;
    }
    MsiEndTransaction(dwtransactionstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumClientsA<'a, P0>(szcomponent: P0, iproductindex: u32, lpproductbuf: ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumClientsA(szcomponent: ::windows::core::PCSTR, iproductindex: u32, lpproductbuf: ::windows::core::PSTR) -> u32;
    }
    MsiEnumClientsA(szcomponent.into(), iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumClientsExA<'a, P0, P1>(szcomponent: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: ::windows::core::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PSTR, pcchsid: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumClientsExA(szcomponent: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: ::windows::core::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PSTR, pcchsid: *mut u32) -> u32;
    }
    MsiEnumClientsExA(szcomponent.into(), szusersid.into(), dwcontext, dwproductindex, ::core::mem::transmute(szproductbuf), ::core::mem::transmute(pdwinstalledcontext), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumClientsExW<'a, P0, P1>(szcomponent: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: ::windows::core::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PWSTR, pcchsid: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumClientsExW(szcomponent: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: ::windows::core::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PWSTR, pcchsid: *mut u32) -> u32;
    }
    MsiEnumClientsExW(szcomponent.into(), szusersid.into(), dwcontext, dwproductindex, ::core::mem::transmute(szproductbuf), ::core::mem::transmute(pdwinstalledcontext), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumClientsW<'a, P0>(szcomponent: P0, iproductindex: u32, lpproductbuf: ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumClientsW(szcomponent: ::windows::core::PCWSTR, iproductindex: u32, lpproductbuf: ::windows::core::PWSTR) -> u32;
    }
    MsiEnumClientsW(szcomponent.into(), iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentCostsA<'a, P0, P1>(hinstall: P0, szcomponent: P1, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: ::windows::core::PSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumComponentCostsA(hinstall: MSIHANDLE, szcomponent: ::windows::core::PCSTR, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: ::windows::core::PSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32;
    }
    MsiEnumComponentCostsA(hinstall.into(), szcomponent.into(), dwindex, istate, ::core::mem::transmute(szdrivebuf), ::core::mem::transmute(pcchdrivebuf), ::core::mem::transmute(picost), ::core::mem::transmute(pitempcost))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentCostsW<'a, P0, P1>(hinstall: P0, szcomponent: P1, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: ::windows::core::PWSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumComponentCostsW(hinstall: MSIHANDLE, szcomponent: ::windows::core::PCWSTR, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: ::windows::core::PWSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32;
    }
    MsiEnumComponentCostsW(hinstall.into(), szcomponent.into(), dwindex, istate, ::core::mem::transmute(szdrivebuf), ::core::mem::transmute(pcchdrivebuf), ::core::mem::transmute(picost), ::core::mem::transmute(pitempcost))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentQualifiersA<'a, P0>(szcomponent: P0, iindex: u32, lpqualifierbuf: ::windows::core::PSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: ::windows::core::PSTR, pcchapplicationdatabuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumComponentQualifiersA(szcomponent: ::windows::core::PCSTR, iindex: u32, lpqualifierbuf: ::windows::core::PSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: ::windows::core::PSTR, pcchapplicationdatabuf: *mut u32) -> u32;
    }
    MsiEnumComponentQualifiersA(szcomponent.into(), iindex, ::core::mem::transmute(lpqualifierbuf), ::core::mem::transmute(pcchqualifierbuf), ::core::mem::transmute(lpapplicationdatabuf), ::core::mem::transmute(pcchapplicationdatabuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentQualifiersW<'a, P0>(szcomponent: P0, iindex: u32, lpqualifierbuf: ::windows::core::PWSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: ::windows::core::PWSTR, pcchapplicationdatabuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumComponentQualifiersW(szcomponent: ::windows::core::PCWSTR, iindex: u32, lpqualifierbuf: ::windows::core::PWSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: ::windows::core::PWSTR, pcchapplicationdatabuf: *mut u32) -> u32;
    }
    MsiEnumComponentQualifiersW(szcomponent.into(), iindex, ::core::mem::transmute(lpqualifierbuf), ::core::mem::transmute(pcchqualifierbuf), ::core::mem::transmute(lpapplicationdatabuf), ::core::mem::transmute(pcchapplicationdatabuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentsA(icomponentindex: u32, lpcomponentbuf: ::windows::core::PSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumComponentsA(icomponentindex: u32, lpcomponentbuf: ::windows::core::PSTR) -> u32;
    }
    MsiEnumComponentsA(icomponentindex, ::core::mem::transmute(lpcomponentbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentsExA<'a, P0>(szusersid: P0, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: ::windows::core::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PSTR, pcchsid: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumComponentsExA(szusersid: ::windows::core::PCSTR, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: ::windows::core::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PSTR, pcchsid: *mut u32) -> u32;
    }
    MsiEnumComponentsExA(szusersid.into(), dwcontext, dwindex, ::core::mem::transmute(szinstalledcomponentcode), ::core::mem::transmute(pdwinstalledcontext), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentsExW<'a, P0>(szusersid: P0, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: ::windows::core::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PWSTR, pcchsid: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumComponentsExW(szusersid: ::windows::core::PCWSTR, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: ::windows::core::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PWSTR, pcchsid: *mut u32) -> u32;
    }
    MsiEnumComponentsExW(szusersid.into(), dwcontext, dwindex, ::core::mem::transmute(szinstalledcomponentcode), ::core::mem::transmute(pdwinstalledcontext), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentsW(icomponentindex: u32, lpcomponentbuf: ::windows::core::PWSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumComponentsW(icomponentindex: u32, lpcomponentbuf: ::windows::core::PWSTR) -> u32;
    }
    MsiEnumComponentsW(icomponentindex, ::core::mem::transmute(lpcomponentbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumFeaturesA<'a, P0>(szproduct: P0, ifeatureindex: u32, lpfeaturebuf: ::windows::core::PSTR, lpparentbuf: ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumFeaturesA(szproduct: ::windows::core::PCSTR, ifeatureindex: u32, lpfeaturebuf: ::windows::core::PSTR, lpparentbuf: ::windows::core::PSTR) -> u32;
    }
    MsiEnumFeaturesA(szproduct.into(), ifeatureindex, ::core::mem::transmute(lpfeaturebuf), ::core::mem::transmute(lpparentbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumFeaturesW<'a, P0>(szproduct: P0, ifeatureindex: u32, lpfeaturebuf: ::windows::core::PWSTR, lpparentbuf: ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumFeaturesW(szproduct: ::windows::core::PCWSTR, ifeatureindex: u32, lpfeaturebuf: ::windows::core::PWSTR, lpparentbuf: ::windows::core::PWSTR) -> u32;
    }
    MsiEnumFeaturesW(szproduct.into(), ifeatureindex, ::core::mem::transmute(lpfeaturebuf), ::core::mem::transmute(lpparentbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumPatchesA<'a, P0>(szproduct: P0, ipatchindex: u32, lppatchbuf: ::windows::core::PSTR, lptransformsbuf: ::windows::core::PSTR, pcchtransformsbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumPatchesA(szproduct: ::windows::core::PCSTR, ipatchindex: u32, lppatchbuf: ::windows::core::PSTR, lptransformsbuf: ::windows::core::PSTR, pcchtransformsbuf: *mut u32) -> u32;
    }
    MsiEnumPatchesA(szproduct.into(), ipatchindex, ::core::mem::transmute(lppatchbuf), ::core::mem::transmute(lptransformsbuf), ::core::mem::transmute(pcchtransformsbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumPatchesExA<'a, P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: ::windows::core::PSTR, sztargetproductcode: ::windows::core::PSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: ::windows::core::PSTR, pcchtargetusersid: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumPatchesExA(szproductcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: ::windows::core::PSTR, sztargetproductcode: ::windows::core::PSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: ::windows::core::PSTR, pcchtargetusersid: *mut u32) -> u32;
    }
    MsiEnumPatchesExA(szproductcode.into(), szusersid.into(), dwcontext, dwfilter, dwindex, ::core::mem::transmute(szpatchcode), ::core::mem::transmute(sztargetproductcode), ::core::mem::transmute(pdwtargetproductcontext), ::core::mem::transmute(sztargetusersid), ::core::mem::transmute(pcchtargetusersid))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumPatchesExW<'a, P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: ::windows::core::PWSTR, sztargetproductcode: ::windows::core::PWSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: ::windows::core::PWSTR, pcchtargetusersid: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumPatchesExW(szproductcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: ::windows::core::PWSTR, sztargetproductcode: ::windows::core::PWSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: ::windows::core::PWSTR, pcchtargetusersid: *mut u32) -> u32;
    }
    MsiEnumPatchesExW(szproductcode.into(), szusersid.into(), dwcontext, dwfilter, dwindex, ::core::mem::transmute(szpatchcode), ::core::mem::transmute(sztargetproductcode), ::core::mem::transmute(pdwtargetproductcontext), ::core::mem::transmute(sztargetusersid), ::core::mem::transmute(pcchtargetusersid))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumPatchesW<'a, P0>(szproduct: P0, ipatchindex: u32, lppatchbuf: ::windows::core::PWSTR, lptransformsbuf: ::windows::core::PWSTR, pcchtransformsbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumPatchesW(szproduct: ::windows::core::PCWSTR, ipatchindex: u32, lppatchbuf: ::windows::core::PWSTR, lptransformsbuf: ::windows::core::PWSTR, pcchtransformsbuf: *mut u32) -> u32;
    }
    MsiEnumPatchesW(szproduct.into(), ipatchindex, ::core::mem::transmute(lppatchbuf), ::core::mem::transmute(lptransformsbuf), ::core::mem::transmute(pcchtransformsbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumProductsA(iproductindex: u32, lpproductbuf: ::windows::core::PSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumProductsA(iproductindex: u32, lpproductbuf: ::windows::core::PSTR) -> u32;
    }
    MsiEnumProductsA(iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumProductsExA<'a, P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwindex: u32, szinstalledproductcode: ::windows::core::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PSTR, pcchsid: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumProductsExA(szproductcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: u32, dwindex: u32, szinstalledproductcode: ::windows::core::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PSTR, pcchsid: *mut u32) -> u32;
    }
    MsiEnumProductsExA(szproductcode.into(), szusersid.into(), dwcontext, dwindex, ::core::mem::transmute(szinstalledproductcode), ::core::mem::transmute(pdwinstalledcontext), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumProductsExW<'a, P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwindex: u32, szinstalledproductcode: ::windows::core::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PWSTR, pcchsid: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumProductsExW(szproductcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: u32, dwindex: u32, szinstalledproductcode: ::windows::core::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: ::windows::core::PWSTR, pcchsid: *mut u32) -> u32;
    }
    MsiEnumProductsExW(szproductcode.into(), szusersid.into(), dwcontext, dwindex, ::core::mem::transmute(szinstalledproductcode), ::core::mem::transmute(pdwinstalledcontext), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumProductsW(iproductindex: u32, lpproductbuf: ::windows::core::PWSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumProductsW(iproductindex: u32, lpproductbuf: ::windows::core::PWSTR) -> u32;
    }
    MsiEnumProductsW(iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumRelatedProductsA<'a, P0>(lpupgradecode: P0, dwreserved: u32, iproductindex: u32, lpproductbuf: ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumRelatedProductsA(lpupgradecode: ::windows::core::PCSTR, dwreserved: u32, iproductindex: u32, lpproductbuf: ::windows::core::PSTR) -> u32;
    }
    MsiEnumRelatedProductsA(lpupgradecode.into(), dwreserved, iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumRelatedProductsW<'a, P0>(lpupgradecode: P0, dwreserved: u32, iproductindex: u32, lpproductbuf: ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEnumRelatedProductsW(lpupgradecode: ::windows::core::PCWSTR, dwreserved: u32, iproductindex: u32, lpproductbuf: ::windows::core::PWSTR) -> u32;
    }
    MsiEnumRelatedProductsW(lpupgradecode.into(), dwreserved, iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEvaluateConditionA<'a, P0, P1>(hinstall: P0, szcondition: P1) -> MSICONDITION
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEvaluateConditionA(hinstall: MSIHANDLE, szcondition: ::windows::core::PCSTR) -> MSICONDITION;
    }
    MsiEvaluateConditionA(hinstall.into(), szcondition.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEvaluateConditionW<'a, P0, P1>(hinstall: P0, szcondition: P1) -> MSICONDITION
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiEvaluateConditionW(hinstall: MSIHANDLE, szcondition: ::windows::core::PCWSTR) -> MSICONDITION;
    }
    MsiEvaluateConditionW(hinstall.into(), szcondition.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiExtractPatchXMLDataA<'a, P0>(szpatchpath: P0, dwreserved: u32, szxmldata: ::windows::core::PSTR, pcchxmldata: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiExtractPatchXMLDataA(szpatchpath: ::windows::core::PCSTR, dwreserved: u32, szxmldata: ::windows::core::PSTR, pcchxmldata: *mut u32) -> u32;
    }
    MsiExtractPatchXMLDataA(szpatchpath.into(), dwreserved, ::core::mem::transmute(szxmldata), ::core::mem::transmute(pcchxmldata))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiExtractPatchXMLDataW<'a, P0>(szpatchpath: P0, dwreserved: u32, szxmldata: ::windows::core::PWSTR, pcchxmldata: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiExtractPatchXMLDataW(szpatchpath: ::windows::core::PCWSTR, dwreserved: u32, szxmldata: ::windows::core::PWSTR, pcchxmldata: *mut u32) -> u32;
    }
    MsiExtractPatchXMLDataW(szpatchpath.into(), dwreserved, ::core::mem::transmute(szxmldata), ::core::mem::transmute(pcchxmldata))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiFormatRecordA<'a, P0, P1>(hinstall: P0, hrecord: P1, szresultbuf: ::windows::core::PSTR, pcchresultbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiFormatRecordA(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: ::windows::core::PSTR, pcchresultbuf: *mut u32) -> u32;
    }
    MsiFormatRecordA(hinstall.into(), hrecord.into(), ::core::mem::transmute(szresultbuf), ::core::mem::transmute(pcchresultbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiFormatRecordW<'a, P0, P1>(hinstall: P0, hrecord: P1, szresultbuf: ::windows::core::PWSTR, pcchresultbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiFormatRecordW(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: ::windows::core::PWSTR, pcchresultbuf: *mut u32) -> u32;
    }
    MsiFormatRecordW(hinstall.into(), hrecord.into(), ::core::mem::transmute(szresultbuf), ::core::mem::transmute(pcchresultbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetActiveDatabase<'a, P0>(hinstall: P0) -> MSIHANDLE
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetActiveDatabase(hinstall: MSIHANDLE) -> MSIHANDLE;
    }
    MsiGetActiveDatabase(hinstall.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentPathA<'a, P0, P1>(szproduct: P0, szcomponent: P1, lppathbuf: ::windows::core::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetComponentPathA(szproduct: ::windows::core::PCSTR, szcomponent: ::windows::core::PCSTR, lppathbuf: ::windows::core::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    }
    MsiGetComponentPathA(szproduct.into(), szcomponent.into(), ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentPathExA<'a, P0, P1, P2>(szproductcode: P0, szcomponentcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: ::windows::core::PSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetComponentPathExA(szproductcode: ::windows::core::PCSTR, szcomponentcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: ::windows::core::PSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE;
    }
    MsiGetComponentPathExA(szproductcode.into(), szcomponentcode.into(), szusersid.into(), dwcontext, ::core::mem::transmute(lpoutpathbuffer), ::core::mem::transmute(pcchoutpathbuffer))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentPathExW<'a, P0, P1, P2>(szproductcode: P0, szcomponentcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: ::windows::core::PWSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetComponentPathExW(szproductcode: ::windows::core::PCWSTR, szcomponentcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: ::windows::core::PWSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE;
    }
    MsiGetComponentPathExW(szproductcode.into(), szcomponentcode.into(), szusersid.into(), dwcontext, ::core::mem::transmute(lpoutpathbuffer), ::core::mem::transmute(pcchoutpathbuffer))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentPathW<'a, P0, P1>(szproduct: P0, szcomponent: P1, lppathbuf: ::windows::core::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetComponentPathW(szproduct: ::windows::core::PCWSTR, szcomponent: ::windows::core::PCWSTR, lppathbuf: ::windows::core::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    }
    MsiGetComponentPathW(szproduct.into(), szcomponent.into(), ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentStateA<'a, P0, P1>(hinstall: P0, szcomponent: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetComponentStateA(hinstall: MSIHANDLE, szcomponent: ::windows::core::PCSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    }
    MsiGetComponentStateA(hinstall.into(), szcomponent.into(), ::core::mem::transmute(piinstalled), ::core::mem::transmute(piaction))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentStateW<'a, P0, P1>(hinstall: P0, szcomponent: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetComponentStateW(hinstall: MSIHANDLE, szcomponent: ::windows::core::PCWSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    }
    MsiGetComponentStateW(hinstall.into(), szcomponent.into(), ::core::mem::transmute(piinstalled), ::core::mem::transmute(piaction))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetDatabaseState<'a, P0>(hdatabase: P0) -> MSIDBSTATE
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetDatabaseState(hdatabase: MSIHANDLE) -> MSIDBSTATE;
    }
    MsiGetDatabaseState(hdatabase.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureCostA<'a, P0, P1>(hinstall: P0, szfeature: P1, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFeatureCostA(hinstall: MSIHANDLE, szfeature: ::windows::core::PCSTR, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32;
    }
    MsiGetFeatureCostA(hinstall.into(), szfeature.into(), icosttree, istate, ::core::mem::transmute(picost))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureCostW<'a, P0, P1>(hinstall: P0, szfeature: P1, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFeatureCostW(hinstall: MSIHANDLE, szfeature: ::windows::core::PCWSTR, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32;
    }
    MsiGetFeatureCostW(hinstall.into(), szfeature.into(), icosttree, istate, ::core::mem::transmute(picost))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureInfoA<'a, P0, P1>(hproduct: P0, szfeature: P1, lpattributes: *mut u32, lptitlebuf: ::windows::core::PSTR, pcchtitlebuf: *mut u32, lphelpbuf: ::windows::core::PSTR, pcchhelpbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFeatureInfoA(hproduct: MSIHANDLE, szfeature: ::windows::core::PCSTR, lpattributes: *mut u32, lptitlebuf: ::windows::core::PSTR, pcchtitlebuf: *mut u32, lphelpbuf: ::windows::core::PSTR, pcchhelpbuf: *mut u32) -> u32;
    }
    MsiGetFeatureInfoA(hproduct.into(), szfeature.into(), ::core::mem::transmute(lpattributes), ::core::mem::transmute(lptitlebuf), ::core::mem::transmute(pcchtitlebuf), ::core::mem::transmute(lphelpbuf), ::core::mem::transmute(pcchhelpbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureInfoW<'a, P0, P1>(hproduct: P0, szfeature: P1, lpattributes: *mut u32, lptitlebuf: ::windows::core::PWSTR, pcchtitlebuf: *mut u32, lphelpbuf: ::windows::core::PWSTR, pcchhelpbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFeatureInfoW(hproduct: MSIHANDLE, szfeature: ::windows::core::PCWSTR, lpattributes: *mut u32, lptitlebuf: ::windows::core::PWSTR, pcchtitlebuf: *mut u32, lphelpbuf: ::windows::core::PWSTR, pcchhelpbuf: *mut u32) -> u32;
    }
    MsiGetFeatureInfoW(hproduct.into(), szfeature.into(), ::core::mem::transmute(lpattributes), ::core::mem::transmute(lptitlebuf), ::core::mem::transmute(pcchtitlebuf), ::core::mem::transmute(lphelpbuf), ::core::mem::transmute(pcchhelpbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureStateA<'a, P0, P1>(hinstall: P0, szfeature: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFeatureStateA(hinstall: MSIHANDLE, szfeature: ::windows::core::PCSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    }
    MsiGetFeatureStateA(hinstall.into(), szfeature.into(), ::core::mem::transmute(piinstalled), ::core::mem::transmute(piaction))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureStateW<'a, P0, P1>(hinstall: P0, szfeature: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFeatureStateW(hinstall: MSIHANDLE, szfeature: ::windows::core::PCWSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    }
    MsiGetFeatureStateW(hinstall.into(), szfeature.into(), ::core::mem::transmute(piinstalled), ::core::mem::transmute(piaction))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureUsageA<'a, P0, P1>(szproduct: P0, szfeature: P1, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFeatureUsageA(szproduct: ::windows::core::PCSTR, szfeature: ::windows::core::PCSTR, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32;
    }
    MsiGetFeatureUsageA(szproduct.into(), szfeature.into(), ::core::mem::transmute(pdwusecount), ::core::mem::transmute(pwdateused))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureUsageW<'a, P0, P1>(szproduct: P0, szfeature: P1, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFeatureUsageW(szproduct: ::windows::core::PCWSTR, szfeature: ::windows::core::PCWSTR, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32;
    }
    MsiGetFeatureUsageW(szproduct.into(), szfeature.into(), ::core::mem::transmute(pdwusecount), ::core::mem::transmute(pwdateused))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureValidStatesA<'a, P0, P1>(hinstall: P0, szfeature: P1, lpinstallstates: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFeatureValidStatesA(hinstall: MSIHANDLE, szfeature: ::windows::core::PCSTR, lpinstallstates: *mut u32) -> u32;
    }
    MsiGetFeatureValidStatesA(hinstall.into(), szfeature.into(), ::core::mem::transmute(lpinstallstates))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureValidStatesW<'a, P0, P1>(hinstall: P0, szfeature: P1, lpinstallstates: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFeatureValidStatesW(hinstall: MSIHANDLE, szfeature: ::windows::core::PCWSTR, lpinstallstates: *mut u32) -> u32;
    }
    MsiGetFeatureValidStatesW(hinstall.into(), szfeature.into(), ::core::mem::transmute(lpinstallstates))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFileHashA<'a, P0>(szfilepath: P0, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFileHashA(szfilepath: ::windows::core::PCSTR, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32;
    }
    MsiGetFileHashA(szfilepath.into(), dwoptions, ::core::mem::transmute(phash))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFileHashW<'a, P0>(szfilepath: P0, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFileHashW(szfilepath: ::windows::core::PCWSTR, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32;
    }
    MsiGetFileHashW(szfilepath.into(), dwoptions, ::core::mem::transmute(phash))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationA<'a, P0>(szsignedobjectpath: P0, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFileSignatureInformationA(szsignedobjectpath: ::windows::core::PCSTR, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows::core::HRESULT;
    }
    MsiGetFileSignatureInformationA(szsignedobjectpath.into(), dwflags, ::core::mem::transmute(ppccertcontext), ::core::mem::transmute(pbhashdata), ::core::mem::transmute(pcbhashdata)).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationW<'a, P0>(szsignedobjectpath: P0, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFileSignatureInformationW(szsignedobjectpath: ::windows::core::PCWSTR, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows::core::HRESULT;
    }
    MsiGetFileSignatureInformationW(szsignedobjectpath.into(), dwflags, ::core::mem::transmute(ppccertcontext), ::core::mem::transmute(pbhashdata), ::core::mem::transmute(pcbhashdata)).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFileVersionA<'a, P0>(szfilepath: P0, lpversionbuf: ::windows::core::PSTR, pcchversionbuf: *mut u32, lplangbuf: ::windows::core::PSTR, pcchlangbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFileVersionA(szfilepath: ::windows::core::PCSTR, lpversionbuf: ::windows::core::PSTR, pcchversionbuf: *mut u32, lplangbuf: ::windows::core::PSTR, pcchlangbuf: *mut u32) -> u32;
    }
    MsiGetFileVersionA(szfilepath.into(), ::core::mem::transmute(lpversionbuf), ::core::mem::transmute(pcchversionbuf), ::core::mem::transmute(lplangbuf), ::core::mem::transmute(pcchlangbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFileVersionW<'a, P0>(szfilepath: P0, lpversionbuf: ::windows::core::PWSTR, pcchversionbuf: *mut u32, lplangbuf: ::windows::core::PWSTR, pcchlangbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetFileVersionW(szfilepath: ::windows::core::PCWSTR, lpversionbuf: ::windows::core::PWSTR, pcchversionbuf: *mut u32, lplangbuf: ::windows::core::PWSTR, pcchlangbuf: *mut u32) -> u32;
    }
    MsiGetFileVersionW(szfilepath.into(), ::core::mem::transmute(lpversionbuf), ::core::mem::transmute(pcchversionbuf), ::core::mem::transmute(lplangbuf), ::core::mem::transmute(pcchlangbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetLanguage<'a, P0>(hinstall: P0) -> u16
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetLanguage(hinstall: MSIHANDLE) -> u16;
    }
    MsiGetLanguage(hinstall.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetLastErrorRecord() -> MSIHANDLE {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetLastErrorRecord() -> MSIHANDLE;
    }
    MsiGetLastErrorRecord()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetMode<'a, P0>(hinstall: P0, erunmode: MSIRUNMODE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE) -> super::super::Foundation::BOOL;
    }
    MsiGetMode(hinstall.into(), erunmode)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchFileListA<'a, P0, P1>(szproductcode: P0, szpatchpackages: P1, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetPatchFileListA(szproductcode: ::windows::core::PCSTR, szpatchpackages: ::windows::core::PCSTR, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32;
    }
    MsiGetPatchFileListA(szproductcode.into(), szpatchpackages.into(), ::core::mem::transmute(pcfiles), ::core::mem::transmute(pphfilerecords))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchFileListW<'a, P0, P1>(szproductcode: P0, szpatchpackages: P1, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetPatchFileListW(szproductcode: ::windows::core::PCWSTR, szpatchpackages: ::windows::core::PCWSTR, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32;
    }
    MsiGetPatchFileListW(szproductcode.into(), szpatchpackages.into(), ::core::mem::transmute(pcfiles), ::core::mem::transmute(pphfilerecords))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchInfoA<'a, P0, P1>(szpatch: P0, szattribute: P1, lpvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetPatchInfoA(szpatch: ::windows::core::PCSTR, szattribute: ::windows::core::PCSTR, lpvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiGetPatchInfoA(szpatch.into(), szattribute.into(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchInfoExA<'a, P0, P1, P2, P3>(szpatchcode: P0, szproductcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, szproperty: P3, lpvalue: ::windows::core::PSTR, pcchvalue: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetPatchInfoExA(szpatchcode: ::windows::core::PCSTR, szproductcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: ::windows::core::PCSTR, lpvalue: ::windows::core::PSTR, pcchvalue: *mut u32) -> u32;
    }
    MsiGetPatchInfoExA(szpatchcode.into(), szproductcode.into(), szusersid.into(), dwcontext, szproperty.into(), ::core::mem::transmute(lpvalue), ::core::mem::transmute(pcchvalue))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchInfoExW<'a, P0, P1, P2, P3>(szpatchcode: P0, szproductcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, szproperty: P3, lpvalue: ::windows::core::PWSTR, pcchvalue: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetPatchInfoExW(szpatchcode: ::windows::core::PCWSTR, szproductcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: ::windows::core::PCWSTR, lpvalue: ::windows::core::PWSTR, pcchvalue: *mut u32) -> u32;
    }
    MsiGetPatchInfoExW(szpatchcode.into(), szproductcode.into(), szusersid.into(), dwcontext, szproperty.into(), ::core::mem::transmute(lpvalue), ::core::mem::transmute(pcchvalue))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchInfoW<'a, P0, P1>(szpatch: P0, szattribute: P1, lpvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetPatchInfoW(szpatch: ::windows::core::PCWSTR, szattribute: ::windows::core::PCWSTR, lpvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiGetPatchInfoW(szpatch.into(), szattribute.into(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductCodeA<'a, P0>(szcomponent: P0, lpbuf39: ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetProductCodeA(szcomponent: ::windows::core::PCSTR, lpbuf39: ::windows::core::PSTR) -> u32;
    }
    MsiGetProductCodeA(szcomponent.into(), ::core::mem::transmute(lpbuf39))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductCodeW<'a, P0>(szcomponent: P0, lpbuf39: ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetProductCodeW(szcomponent: ::windows::core::PCWSTR, lpbuf39: ::windows::core::PWSTR) -> u32;
    }
    MsiGetProductCodeW(szcomponent.into(), ::core::mem::transmute(lpbuf39))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoA<'a, P0, P1>(szproduct: P0, szattribute: P1, lpvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetProductInfoA(szproduct: ::windows::core::PCSTR, szattribute: ::windows::core::PCSTR, lpvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiGetProductInfoA(szproduct.into(), szattribute.into(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoExA<'a, P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szproperty: P2, szvalue: ::windows::core::PSTR, pcchvalue: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetProductInfoExA(szproductcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: ::windows::core::PCSTR, szvalue: ::windows::core::PSTR, pcchvalue: *mut u32) -> u32;
    }
    MsiGetProductInfoExA(szproductcode.into(), szusersid.into(), dwcontext, szproperty.into(), ::core::mem::transmute(szvalue), ::core::mem::transmute(pcchvalue))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoExW<'a, P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szproperty: P2, szvalue: ::windows::core::PWSTR, pcchvalue: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetProductInfoExW(szproductcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: ::windows::core::PCWSTR, szvalue: ::windows::core::PWSTR, pcchvalue: *mut u32) -> u32;
    }
    MsiGetProductInfoExW(szproductcode.into(), szusersid.into(), dwcontext, szproperty.into(), ::core::mem::transmute(szvalue), ::core::mem::transmute(pcchvalue))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptA<'a, P0>(szscriptfile: P0, lpproductbuf39: ::windows::core::PSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: ::windows::core::PSTR, pcchnamebuf: *mut u32, lppackagebuf: ::windows::core::PSTR, pcchpackagebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetProductInfoFromScriptA(szscriptfile: ::windows::core::PCSTR, lpproductbuf39: ::windows::core::PSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: ::windows::core::PSTR, pcchnamebuf: *mut u32, lppackagebuf: ::windows::core::PSTR, pcchpackagebuf: *mut u32) -> u32;
    }
    MsiGetProductInfoFromScriptA(szscriptfile.into(), ::core::mem::transmute(lpproductbuf39), ::core::mem::transmute(plgidlanguage), ::core::mem::transmute(pdwversion), ::core::mem::transmute(lpnamebuf), ::core::mem::transmute(pcchnamebuf), ::core::mem::transmute(lppackagebuf), ::core::mem::transmute(pcchpackagebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptW<'a, P0>(szscriptfile: P0, lpproductbuf39: ::windows::core::PWSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: ::windows::core::PWSTR, pcchnamebuf: *mut u32, lppackagebuf: ::windows::core::PWSTR, pcchpackagebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetProductInfoFromScriptW(szscriptfile: ::windows::core::PCWSTR, lpproductbuf39: ::windows::core::PWSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: ::windows::core::PWSTR, pcchnamebuf: *mut u32, lppackagebuf: ::windows::core::PWSTR, pcchpackagebuf: *mut u32) -> u32;
    }
    MsiGetProductInfoFromScriptW(szscriptfile.into(), ::core::mem::transmute(lpproductbuf39), ::core::mem::transmute(plgidlanguage), ::core::mem::transmute(pdwversion), ::core::mem::transmute(lpnamebuf), ::core::mem::transmute(pcchnamebuf), ::core::mem::transmute(lppackagebuf), ::core::mem::transmute(pcchpackagebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoW<'a, P0, P1>(szproduct: P0, szattribute: P1, lpvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetProductInfoW(szproduct: ::windows::core::PCWSTR, szattribute: ::windows::core::PCWSTR, lpvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiGetProductInfoW(szproduct.into(), szattribute.into(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductPropertyA<'a, P0, P1>(hproduct: P0, szproperty: P1, lpvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetProductPropertyA(hproduct: MSIHANDLE, szproperty: ::windows::core::PCSTR, lpvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiGetProductPropertyA(hproduct.into(), szproperty.into(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductPropertyW<'a, P0, P1>(hproduct: P0, szproperty: P1, lpvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetProductPropertyW(hproduct: MSIHANDLE, szproperty: ::windows::core::PCWSTR, lpvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiGetProductPropertyW(hproduct.into(), szproperty.into(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPropertyA<'a, P0, P1>(hinstall: P0, szname: P1, szvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetPropertyA(hinstall: MSIHANDLE, szname: ::windows::core::PCSTR, szvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiGetPropertyA(hinstall.into(), szname.into(), ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPropertyW<'a, P0, P1>(hinstall: P0, szname: P1, szvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetPropertyW(hinstall: MSIHANDLE, szname: ::windows::core::PCWSTR, szvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiGetPropertyW(hinstall.into(), szname.into(), ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetShortcutTargetA<'a, P0>(szshortcutpath: P0, szproductcode: ::windows::core::PSTR, szfeatureid: ::windows::core::PSTR, szcomponentcode: ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetShortcutTargetA(szshortcutpath: ::windows::core::PCSTR, szproductcode: ::windows::core::PSTR, szfeatureid: ::windows::core::PSTR, szcomponentcode: ::windows::core::PSTR) -> u32;
    }
    MsiGetShortcutTargetA(szshortcutpath.into(), ::core::mem::transmute(szproductcode), ::core::mem::transmute(szfeatureid), ::core::mem::transmute(szcomponentcode))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetShortcutTargetW<'a, P0>(szshortcutpath: P0, szproductcode: ::windows::core::PWSTR, szfeatureid: ::windows::core::PWSTR, szcomponentcode: ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetShortcutTargetW(szshortcutpath: ::windows::core::PCWSTR, szproductcode: ::windows::core::PWSTR, szfeatureid: ::windows::core::PWSTR, szcomponentcode: ::windows::core::PWSTR) -> u32;
    }
    MsiGetShortcutTargetW(szshortcutpath.into(), ::core::mem::transmute(szproductcode), ::core::mem::transmute(szfeatureid), ::core::mem::transmute(szcomponentcode))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetSourcePathA<'a, P0, P1>(hinstall: P0, szfolder: P1, szpathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetSourcePathA(hinstall: MSIHANDLE, szfolder: ::windows::core::PCSTR, szpathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiGetSourcePathA(hinstall.into(), szfolder.into(), ::core::mem::transmute(szpathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetSourcePathW<'a, P0, P1>(hinstall: P0, szfolder: P1, szpathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetSourcePathW(hinstall: MSIHANDLE, szfolder: ::windows::core::PCWSTR, szpathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiGetSourcePathW(hinstall.into(), szfolder.into(), ::core::mem::transmute(szpathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetSummaryInformationA<'a, P0, P1>(hdatabase: P0, szdatabasepath: P1, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetSummaryInformationA(hdatabase: MSIHANDLE, szdatabasepath: ::windows::core::PCSTR, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32;
    }
    MsiGetSummaryInformationA(hdatabase.into(), szdatabasepath.into(), uiupdatecount, ::core::mem::transmute(phsummaryinfo))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetSummaryInformationW<'a, P0, P1>(hdatabase: P0, szdatabasepath: P1, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetSummaryInformationW(hdatabase: MSIHANDLE, szdatabasepath: ::windows::core::PCWSTR, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32;
    }
    MsiGetSummaryInformationW(hdatabase.into(), szdatabasepath.into(), uiupdatecount, ::core::mem::transmute(phsummaryinfo))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetTargetPathA<'a, P0, P1>(hinstall: P0, szfolder: P1, szpathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetTargetPathA(hinstall: MSIHANDLE, szfolder: ::windows::core::PCSTR, szpathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiGetTargetPathA(hinstall.into(), szfolder.into(), ::core::mem::transmute(szpathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetTargetPathW<'a, P0, P1>(hinstall: P0, szfolder: P1, szpathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetTargetPathW(hinstall: MSIHANDLE, szfolder: ::windows::core::PCWSTR, szpathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiGetTargetPathW(hinstall.into(), szfolder.into(), ::core::mem::transmute(szpathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetUserInfoA<'a, P0>(szproduct: P0, lpusernamebuf: ::windows::core::PSTR, pcchusernamebuf: *mut u32, lporgnamebuf: ::windows::core::PSTR, pcchorgnamebuf: *mut u32, lpserialbuf: ::windows::core::PSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetUserInfoA(szproduct: ::windows::core::PCSTR, lpusernamebuf: ::windows::core::PSTR, pcchusernamebuf: *mut u32, lporgnamebuf: ::windows::core::PSTR, pcchorgnamebuf: *mut u32, lpserialbuf: ::windows::core::PSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE;
    }
    MsiGetUserInfoA(szproduct.into(), ::core::mem::transmute(lpusernamebuf), ::core::mem::transmute(pcchusernamebuf), ::core::mem::transmute(lporgnamebuf), ::core::mem::transmute(pcchorgnamebuf), ::core::mem::transmute(lpserialbuf), ::core::mem::transmute(pcchserialbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetUserInfoW<'a, P0>(szproduct: P0, lpusernamebuf: ::windows::core::PWSTR, pcchusernamebuf: *mut u32, lporgnamebuf: ::windows::core::PWSTR, pcchorgnamebuf: *mut u32, lpserialbuf: ::windows::core::PWSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiGetUserInfoW(szproduct: ::windows::core::PCWSTR, lpusernamebuf: ::windows::core::PWSTR, pcchusernamebuf: *mut u32, lporgnamebuf: ::windows::core::PWSTR, pcchorgnamebuf: *mut u32, lpserialbuf: ::windows::core::PWSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE;
    }
    MsiGetUserInfoW(szproduct.into(), ::core::mem::transmute(lpusernamebuf), ::core::mem::transmute(pcchusernamebuf), ::core::mem::transmute(lporgnamebuf), ::core::mem::transmute(pcchorgnamebuf), ::core::mem::transmute(lpserialbuf), ::core::mem::transmute(pcchserialbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallMissingComponentA<'a, P0, P1>(szproduct: P0, szcomponent: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiInstallMissingComponentA(szproduct: ::windows::core::PCSTR, szcomponent: ::windows::core::PCSTR, einstallstate: INSTALLSTATE) -> u32;
    }
    MsiInstallMissingComponentA(szproduct.into(), szcomponent.into(), einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallMissingComponentW<'a, P0, P1>(szproduct: P0, szcomponent: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiInstallMissingComponentW(szproduct: ::windows::core::PCWSTR, szcomponent: ::windows::core::PCWSTR, einstallstate: INSTALLSTATE) -> u32;
    }
    MsiInstallMissingComponentW(szproduct.into(), szcomponent.into(), einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallMissingFileA<'a, P0, P1>(szproduct: P0, szfile: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiInstallMissingFileA(szproduct: ::windows::core::PCSTR, szfile: ::windows::core::PCSTR) -> u32;
    }
    MsiInstallMissingFileA(szproduct.into(), szfile.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallMissingFileW<'a, P0, P1>(szproduct: P0, szfile: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiInstallMissingFileW(szproduct: ::windows::core::PCWSTR, szfile: ::windows::core::PCWSTR) -> u32;
    }
    MsiInstallMissingFileW(szproduct.into(), szfile.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallProductA<'a, P0, P1>(szpackagepath: P0, szcommandline: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiInstallProductA(szpackagepath: ::windows::core::PCSTR, szcommandline: ::windows::core::PCSTR) -> u32;
    }
    MsiInstallProductA(szpackagepath.into(), szcommandline.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallProductW<'a, P0, P1>(szpackagepath: P0, szcommandline: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiInstallProductW(szpackagepath: ::windows::core::PCWSTR, szcommandline: ::windows::core::PCWSTR) -> u32;
    }
    MsiInstallProductW(szpackagepath.into(), szcommandline.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiIsProductElevatedA<'a, P0>(szproduct: P0, pfelevated: *mut super::super::Foundation::BOOL) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiIsProductElevatedA(szproduct: ::windows::core::PCSTR, pfelevated: *mut super::super::Foundation::BOOL) -> u32;
    }
    MsiIsProductElevatedA(szproduct.into(), ::core::mem::transmute(pfelevated))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiIsProductElevatedW<'a, P0>(szproduct: P0, pfelevated: *mut super::super::Foundation::BOOL) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiIsProductElevatedW(szproduct: ::windows::core::PCWSTR, pfelevated: *mut super::super::Foundation::BOOL) -> u32;
    }
    MsiIsProductElevatedW(szproduct.into(), ::core::mem::transmute(pfelevated))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiJoinTransaction<'a, P0>(htransactionhandle: P0, dwtransactionattributes: u32, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiJoinTransaction(htransactionhandle: MSIHANDLE, dwtransactionattributes: u32, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MsiJoinTransaction(htransactionhandle.into(), dwtransactionattributes, ::core::mem::transmute(phchangeofownerevent))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiLocateComponentA<'a, P0>(szcomponent: P0, lppathbuf: ::windows::core::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiLocateComponentA(szcomponent: ::windows::core::PCSTR, lppathbuf: ::windows::core::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    }
    MsiLocateComponentA(szcomponent.into(), ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiLocateComponentW<'a, P0>(szcomponent: P0, lppathbuf: ::windows::core::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiLocateComponentW(szcomponent: ::windows::core::PCWSTR, lppathbuf: ::windows::core::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    }
    MsiLocateComponentW(szcomponent.into(), ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiNotifySidChangeA<'a, P0, P1>(poldsid: P0, pnewsid: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiNotifySidChangeA(poldsid: ::windows::core::PCSTR, pnewsid: ::windows::core::PCSTR) -> u32;
    }
    MsiNotifySidChangeA(poldsid.into(), pnewsid.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiNotifySidChangeW<'a, P0, P1>(poldsid: P0, pnewsid: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiNotifySidChangeW(poldsid: ::windows::core::PCWSTR, pnewsid: ::windows::core::PCWSTR) -> u32;
    }
    MsiNotifySidChangeW(poldsid.into(), pnewsid.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenDatabaseA<'a, P0, P1>(szdatabasepath: P0, szpersist: P1, phdatabase: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiOpenDatabaseA(szdatabasepath: ::windows::core::PCSTR, szpersist: ::windows::core::PCSTR, phdatabase: *mut MSIHANDLE) -> u32;
    }
    MsiOpenDatabaseA(szdatabasepath.into(), szpersist.into(), ::core::mem::transmute(phdatabase))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenDatabaseW<'a, P0, P1>(szdatabasepath: P0, szpersist: P1, phdatabase: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiOpenDatabaseW(szdatabasepath: ::windows::core::PCWSTR, szpersist: ::windows::core::PCWSTR, phdatabase: *mut MSIHANDLE) -> u32;
    }
    MsiOpenDatabaseW(szdatabasepath.into(), szpersist.into(), ::core::mem::transmute(phdatabase))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenPackageA<'a, P0>(szpackagepath: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiOpenPackageA(szpackagepath: ::windows::core::PCSTR, hproduct: *mut MSIHANDLE) -> u32;
    }
    MsiOpenPackageA(szpackagepath.into(), ::core::mem::transmute(hproduct))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenPackageExA<'a, P0>(szpackagepath: P0, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiOpenPackageExA(szpackagepath: ::windows::core::PCSTR, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32;
    }
    MsiOpenPackageExA(szpackagepath.into(), dwoptions, ::core::mem::transmute(hproduct))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenPackageExW<'a, P0>(szpackagepath: P0, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiOpenPackageExW(szpackagepath: ::windows::core::PCWSTR, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32;
    }
    MsiOpenPackageExW(szpackagepath.into(), dwoptions, ::core::mem::transmute(hproduct))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenPackageW<'a, P0>(szpackagepath: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiOpenPackageW(szpackagepath: ::windows::core::PCWSTR, hproduct: *mut MSIHANDLE) -> u32;
    }
    MsiOpenPackageW(szpackagepath.into(), ::core::mem::transmute(hproduct))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenProductA<'a, P0>(szproduct: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiOpenProductA(szproduct: ::windows::core::PCSTR, hproduct: *mut MSIHANDLE) -> u32;
    }
    MsiOpenProductA(szproduct.into(), ::core::mem::transmute(hproduct))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenProductW<'a, P0>(szproduct: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiOpenProductW(szproduct: ::windows::core::PCWSTR, hproduct: *mut MSIHANDLE) -> u32;
    }
    MsiOpenProductW(szproduct.into(), ::core::mem::transmute(hproduct))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiPreviewBillboardA<'a, P0, P1, P2>(hpreview: P0, szcontrolname: P1, szbillboard: P2) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiPreviewBillboardA(hpreview: MSIHANDLE, szcontrolname: ::windows::core::PCSTR, szbillboard: ::windows::core::PCSTR) -> u32;
    }
    MsiPreviewBillboardA(hpreview.into(), szcontrolname.into(), szbillboard.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiPreviewBillboardW<'a, P0, P1, P2>(hpreview: P0, szcontrolname: P1, szbillboard: P2) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiPreviewBillboardW(hpreview: MSIHANDLE, szcontrolname: ::windows::core::PCWSTR, szbillboard: ::windows::core::PCWSTR) -> u32;
    }
    MsiPreviewBillboardW(hpreview.into(), szcontrolname.into(), szbillboard.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiPreviewDialogA<'a, P0, P1>(hpreview: P0, szdialogname: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiPreviewDialogA(hpreview: MSIHANDLE, szdialogname: ::windows::core::PCSTR) -> u32;
    }
    MsiPreviewDialogA(hpreview.into(), szdialogname.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiPreviewDialogW<'a, P0, P1>(hpreview: P0, szdialogname: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiPreviewDialogW(hpreview: MSIHANDLE, szdialogname: ::windows::core::PCWSTR) -> u32;
    }
    MsiPreviewDialogW(hpreview.into(), szdialogname.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptA<'a, P0, P1, P2, P3, P4>(szscriptfile: P0, sziconfolder: P1, hregdata: P2, fshortcuts: P3, fremoveitems: P4) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::Registry::HKEY>,
    P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    P4: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiProcessAdvertiseScriptA(szscriptfile: ::windows::core::PCSTR, sziconfolder: ::windows::core::PCSTR, hregdata: super::Registry::HKEY, fshortcuts: super::super::Foundation::BOOL, fremoveitems: super::super::Foundation::BOOL) -> u32;
    }
    MsiProcessAdvertiseScriptA(szscriptfile.into(), sziconfolder.into(), hregdata.into(), fshortcuts.into(), fremoveitems.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptW<'a, P0, P1, P2, P3, P4>(szscriptfile: P0, sziconfolder: P1, hregdata: P2, fshortcuts: P3, fremoveitems: P4) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::Registry::HKEY>,
    P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    P4: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiProcessAdvertiseScriptW(szscriptfile: ::windows::core::PCWSTR, sziconfolder: ::windows::core::PCWSTR, hregdata: super::Registry::HKEY, fshortcuts: super::super::Foundation::BOOL, fremoveitems: super::super::Foundation::BOOL) -> u32;
    }
    MsiProcessAdvertiseScriptW(szscriptfile.into(), sziconfolder.into(), hregdata.into(), fshortcuts.into(), fremoveitems.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProcessMessage<'a, P0, P1>(hinstall: P0, emessagetype: INSTALLMESSAGE, hrecord: P1) -> i32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiProcessMessage(hinstall: MSIHANDLE, emessagetype: INSTALLMESSAGE, hrecord: MSIHANDLE) -> i32;
    }
    MsiProcessMessage(hinstall.into(), emessagetype, hrecord.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideAssemblyA<'a, P0, P1>(szassemblyname: P0, szappcontext: P1, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiProvideAssemblyA(szassemblyname: ::windows::core::PCSTR, szappcontext: ::windows::core::PCSTR, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiProvideAssemblyA(szassemblyname.into(), szappcontext.into(), dwinstallmode, dwassemblyinfo, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideAssemblyW<'a, P0, P1>(szassemblyname: P0, szappcontext: P1, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiProvideAssemblyW(szassemblyname: ::windows::core::PCWSTR, szappcontext: ::windows::core::PCWSTR, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiProvideAssemblyW(szassemblyname.into(), szappcontext.into(), dwinstallmode, dwassemblyinfo, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideComponentA<'a, P0, P1, P2>(szproduct: P0, szfeature: P1, szcomponent: P2, dwinstallmode: INSTALLMODE, lppathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiProvideComponentA(szproduct: ::windows::core::PCSTR, szfeature: ::windows::core::PCSTR, szcomponent: ::windows::core::PCSTR, dwinstallmode: INSTALLMODE, lppathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiProvideComponentA(szproduct.into(), szfeature.into(), szcomponent.into(), dwinstallmode, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideComponentW<'a, P0, P1, P2>(szproduct: P0, szfeature: P1, szcomponent: P2, dwinstallmode: INSTALLMODE, lppathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiProvideComponentW(szproduct: ::windows::core::PCWSTR, szfeature: ::windows::core::PCWSTR, szcomponent: ::windows::core::PCWSTR, dwinstallmode: INSTALLMODE, lppathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiProvideComponentW(szproduct.into(), szfeature.into(), szcomponent.into(), dwinstallmode, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentA<'a, P0, P1>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, lppathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiProvideQualifiedComponentA(szcategory: ::windows::core::PCSTR, szqualifier: ::windows::core::PCSTR, dwinstallmode: INSTALLMODE, lppathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiProvideQualifiedComponentA(szcategory.into(), szqualifier.into(), dwinstallmode, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExA<'a, P0, P1, P2>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, szproduct: P2, dwunused1: u32, dwunused2: u32, lppathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiProvideQualifiedComponentExA(szcategory: ::windows::core::PCSTR, szqualifier: ::windows::core::PCSTR, dwinstallmode: INSTALLMODE, szproduct: ::windows::core::PCSTR, dwunused1: u32, dwunused2: u32, lppathbuf: ::windows::core::PSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiProvideQualifiedComponentExA(szcategory.into(), szqualifier.into(), dwinstallmode, szproduct.into(), dwunused1, dwunused2, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExW<'a, P0, P1, P2>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, szproduct: P2, dwunused1: u32, dwunused2: u32, lppathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiProvideQualifiedComponentExW(szcategory: ::windows::core::PCWSTR, szqualifier: ::windows::core::PCWSTR, dwinstallmode: INSTALLMODE, szproduct: ::windows::core::PCWSTR, dwunused1: u32, dwunused2: u32, lppathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiProvideQualifiedComponentExW(szcategory.into(), szqualifier.into(), dwinstallmode, szproduct.into(), dwunused1, dwunused2, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentW<'a, P0, P1>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, lppathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiProvideQualifiedComponentW(szcategory: ::windows::core::PCWSTR, szqualifier: ::windows::core::PCWSTR, dwinstallmode: INSTALLMODE, lppathbuf: ::windows::core::PWSTR, pcchpathbuf: *mut u32) -> u32;
    }
    MsiProvideQualifiedComponentW(szcategory.into(), szqualifier.into(), dwinstallmode, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryComponentStateA<'a, P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: P2, pdwstate: *mut INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiQueryComponentStateA(szproductcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: ::windows::core::PCSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    }
    MsiQueryComponentStateA(szproductcode.into(), szusersid.into(), dwcontext, szcomponentcode.into(), ::core::mem::transmute(pdwstate))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryComponentStateW<'a, P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: P2, pdwstate: *mut INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiQueryComponentStateW(szproductcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: ::windows::core::PCWSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    }
    MsiQueryComponentStateW(szproductcode.into(), szusersid.into(), dwcontext, szcomponentcode.into(), ::core::mem::transmute(pdwstate))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryFeatureStateA<'a, P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiQueryFeatureStateA(szproduct: ::windows::core::PCSTR, szfeature: ::windows::core::PCSTR) -> INSTALLSTATE;
    }
    MsiQueryFeatureStateA(szproduct.into(), szfeature.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryFeatureStateExA<'a, P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szfeature: P2, pdwstate: *mut INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiQueryFeatureStateExA(szproductcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, szfeature: ::windows::core::PCSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    }
    MsiQueryFeatureStateExA(szproductcode.into(), szusersid.into(), dwcontext, szfeature.into(), ::core::mem::transmute(pdwstate))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryFeatureStateExW<'a, P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szfeature: P2, pdwstate: *mut INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiQueryFeatureStateExW(szproductcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, szfeature: ::windows::core::PCWSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    }
    MsiQueryFeatureStateExW(szproductcode.into(), szusersid.into(), dwcontext, szfeature.into(), ::core::mem::transmute(pdwstate))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryFeatureStateW<'a, P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiQueryFeatureStateW(szproduct: ::windows::core::PCWSTR, szfeature: ::windows::core::PCWSTR) -> INSTALLSTATE;
    }
    MsiQueryFeatureStateW(szproduct.into(), szfeature.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryProductStateA<'a, P0>(szproduct: P0) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiQueryProductStateA(szproduct: ::windows::core::PCSTR) -> INSTALLSTATE;
    }
    MsiQueryProductStateA(szproduct.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryProductStateW<'a, P0>(szproduct: P0) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiQueryProductStateW(szproduct: ::windows::core::PCWSTR) -> INSTALLSTATE;
    }
    MsiQueryProductStateW(szproduct.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordClearData<'a, P0>(hrecord: P0) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordClearData(hrecord: MSIHANDLE) -> u32;
    }
    MsiRecordClearData(hrecord.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordDataSize<'a, P0>(hrecord: P0, ifield: u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordDataSize(hrecord: MSIHANDLE, ifield: u32) -> u32;
    }
    MsiRecordDataSize(hrecord.into(), ifield)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordGetFieldCount<'a, P0>(hrecord: P0) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordGetFieldCount(hrecord: MSIHANDLE) -> u32;
    }
    MsiRecordGetFieldCount(hrecord.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordGetInteger<'a, P0>(hrecord: P0, ifield: u32) -> i32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordGetInteger(hrecord: MSIHANDLE, ifield: u32) -> i32;
    }
    MsiRecordGetInteger(hrecord.into(), ifield)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordGetStringA<'a, P0>(hrecord: P0, ifield: u32, szvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordGetStringA(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiRecordGetStringA(hrecord.into(), ifield, ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordGetStringW<'a, P0>(hrecord: P0, ifield: u32, szvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordGetStringW(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiRecordGetStringW(hrecord.into(), ifield, ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRecordIsNull<'a, P0>(hrecord: P0, ifield: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordIsNull(hrecord: MSIHANDLE, ifield: u32) -> super::super::Foundation::BOOL;
    }
    MsiRecordIsNull(hrecord.into(), ifield)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordReadStream<'a, P0>(hrecord: P0, ifield: u32, szdatabuf: ::windows::core::PSTR, pcbdatabuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordReadStream(hrecord: MSIHANDLE, ifield: u32, szdatabuf: ::windows::core::PSTR, pcbdatabuf: *mut u32) -> u32;
    }
    MsiRecordReadStream(hrecord.into(), ifield, ::core::mem::transmute(szdatabuf), ::core::mem::transmute(pcbdatabuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordSetInteger<'a, P0>(hrecord: P0, ifield: u32, ivalue: i32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordSetInteger(hrecord: MSIHANDLE, ifield: u32, ivalue: i32) -> u32;
    }
    MsiRecordSetInteger(hrecord.into(), ifield, ivalue)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordSetStreamA<'a, P0, P1>(hrecord: P0, ifield: u32, szfilepath: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordSetStreamA(hrecord: MSIHANDLE, ifield: u32, szfilepath: ::windows::core::PCSTR) -> u32;
    }
    MsiRecordSetStreamA(hrecord.into(), ifield, szfilepath.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordSetStreamW<'a, P0, P1>(hrecord: P0, ifield: u32, szfilepath: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordSetStreamW(hrecord: MSIHANDLE, ifield: u32, szfilepath: ::windows::core::PCWSTR) -> u32;
    }
    MsiRecordSetStreamW(hrecord.into(), ifield, szfilepath.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordSetStringA<'a, P0, P1>(hrecord: P0, ifield: u32, szvalue: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordSetStringA(hrecord: MSIHANDLE, ifield: u32, szvalue: ::windows::core::PCSTR) -> u32;
    }
    MsiRecordSetStringA(hrecord.into(), ifield, szvalue.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordSetStringW<'a, P0, P1>(hrecord: P0, ifield: u32, szvalue: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRecordSetStringW(hrecord: MSIHANDLE, ifield: u32, szvalue: ::windows::core::PCWSTR) -> u32;
    }
    MsiRecordSetStringW(hrecord.into(), ifield, szvalue.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiReinstallFeatureA<'a, P0, P1>(szproduct: P0, szfeature: P1, dwreinstallmode: REINSTALLMODE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiReinstallFeatureA(szproduct: ::windows::core::PCSTR, szfeature: ::windows::core::PCSTR, dwreinstallmode: REINSTALLMODE) -> u32;
    }
    MsiReinstallFeatureA(szproduct.into(), szfeature.into(), dwreinstallmode)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiReinstallFeatureW<'a, P0, P1>(szproduct: P0, szfeature: P1, dwreinstallmode: REINSTALLMODE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiReinstallFeatureW(szproduct: ::windows::core::PCWSTR, szfeature: ::windows::core::PCWSTR, dwreinstallmode: REINSTALLMODE) -> u32;
    }
    MsiReinstallFeatureW(szproduct.into(), szfeature.into(), dwreinstallmode)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiReinstallProductA<'a, P0>(szproduct: P0, szreinstallmode: REINSTALLMODE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiReinstallProductA(szproduct: ::windows::core::PCSTR, szreinstallmode: REINSTALLMODE) -> u32;
    }
    MsiReinstallProductA(szproduct.into(), szreinstallmode)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiReinstallProductW<'a, P0>(szproduct: P0, szreinstallmode: REINSTALLMODE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiReinstallProductW(szproduct: ::windows::core::PCWSTR, szreinstallmode: REINSTALLMODE) -> u32;
    }
    MsiReinstallProductW(szproduct.into(), szreinstallmode)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRemovePatchesA<'a, P0, P1, P2>(szpatchlist: P0, szproductcode: P1, euninstalltype: INSTALLTYPE, szpropertylist: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRemovePatchesA(szpatchlist: ::windows::core::PCSTR, szproductcode: ::windows::core::PCSTR, euninstalltype: INSTALLTYPE, szpropertylist: ::windows::core::PCSTR) -> u32;
    }
    MsiRemovePatchesA(szpatchlist.into(), szproductcode.into(), euninstalltype, szpropertylist.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRemovePatchesW<'a, P0, P1, P2>(szpatchlist: P0, szproductcode: P1, euninstalltype: INSTALLTYPE, szpropertylist: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiRemovePatchesW(szpatchlist: ::windows::core::PCWSTR, szproductcode: ::windows::core::PCWSTR, euninstalltype: INSTALLTYPE, szpropertylist: ::windows::core::PCWSTR) -> u32;
    }
    MsiRemovePatchesW(szpatchlist.into(), szproductcode.into(), euninstalltype, szpropertylist.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSequenceA<'a, P0, P1>(hinstall: P0, sztable: P1, isequencemode: i32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSequenceA(hinstall: MSIHANDLE, sztable: ::windows::core::PCSTR, isequencemode: i32) -> u32;
    }
    MsiSequenceA(hinstall.into(), sztable.into(), isequencemode)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSequenceW<'a, P0, P1>(hinstall: P0, sztable: P1, isequencemode: i32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSequenceW(hinstall: MSIHANDLE, sztable: ::windows::core::PCWSTR, isequencemode: i32) -> u32;
    }
    MsiSequenceW(hinstall.into(), sztable.into(), isequencemode)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetComponentStateA<'a, P0, P1>(hinstall: P0, szcomponent: P1, istate: INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetComponentStateA(hinstall: MSIHANDLE, szcomponent: ::windows::core::PCSTR, istate: INSTALLSTATE) -> u32;
    }
    MsiSetComponentStateA(hinstall.into(), szcomponent.into(), istate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetComponentStateW<'a, P0, P1>(hinstall: P0, szcomponent: P1, istate: INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetComponentStateW(hinstall: MSIHANDLE, szcomponent: ::windows::core::PCWSTR, istate: INSTALLSTATE) -> u32;
    }
    MsiSetComponentStateW(hinstall.into(), szcomponent.into(), istate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetExternalUIA(puihandler: INSTALLUI_HANDLERA, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> INSTALLUI_HANDLERA {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetExternalUIA(puihandler: *mut ::core::ffi::c_void, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> INSTALLUI_HANDLERA;
    }
    MsiSetExternalUIA(::core::mem::transmute(puihandler), dwmessagefilter, ::core::mem::transmute(pvcontext))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetExternalUIRecord(puihandler: PINSTALLUI_HANDLER_RECORD, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void, ppuiprevhandler: PINSTALLUI_HANDLER_RECORD) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetExternalUIRecord(puihandler: *mut ::core::ffi::c_void, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void, ppuiprevhandler: *mut ::core::ffi::c_void) -> u32;
    }
    MsiSetExternalUIRecord(::core::mem::transmute(puihandler), dwmessagefilter, ::core::mem::transmute(pvcontext), ::core::mem::transmute(ppuiprevhandler))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetExternalUIW(puihandler: INSTALLUI_HANDLERW, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> INSTALLUI_HANDLERW {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetExternalUIW(puihandler: *mut ::core::ffi::c_void, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> INSTALLUI_HANDLERW;
    }
    MsiSetExternalUIW(::core::mem::transmute(puihandler), dwmessagefilter, ::core::mem::transmute(pvcontext))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetFeatureAttributesA<'a, P0, P1>(hinstall: P0, szfeature: P1, dwattributes: u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetFeatureAttributesA(hinstall: MSIHANDLE, szfeature: ::windows::core::PCSTR, dwattributes: u32) -> u32;
    }
    MsiSetFeatureAttributesA(hinstall.into(), szfeature.into(), dwattributes)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetFeatureAttributesW<'a, P0, P1>(hinstall: P0, szfeature: P1, dwattributes: u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetFeatureAttributesW(hinstall: MSIHANDLE, szfeature: ::windows::core::PCWSTR, dwattributes: u32) -> u32;
    }
    MsiSetFeatureAttributesW(hinstall.into(), szfeature.into(), dwattributes)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetFeatureStateA<'a, P0, P1>(hinstall: P0, szfeature: P1, istate: INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetFeatureStateA(hinstall: MSIHANDLE, szfeature: ::windows::core::PCSTR, istate: INSTALLSTATE) -> u32;
    }
    MsiSetFeatureStateA(hinstall.into(), szfeature.into(), istate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetFeatureStateW<'a, P0, P1>(hinstall: P0, szfeature: P1, istate: INSTALLSTATE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetFeatureStateW(hinstall: MSIHANDLE, szfeature: ::windows::core::PCWSTR, istate: INSTALLSTATE) -> u32;
    }
    MsiSetFeatureStateW(hinstall.into(), szfeature.into(), istate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetInstallLevel<'a, P0>(hinstall: P0, iinstalllevel: i32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetInstallLevel(hinstall: MSIHANDLE, iinstalllevel: i32) -> u32;
    }
    MsiSetInstallLevel(hinstall.into(), iinstalllevel)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetInternalUI(dwuilevel: INSTALLUILEVEL, phwnd: *mut super::super::Foundation::HWND) -> INSTALLUILEVEL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetInternalUI(dwuilevel: INSTALLUILEVEL, phwnd: *mut super::super::Foundation::HWND) -> INSTALLUILEVEL;
    }
    MsiSetInternalUI(dwuilevel, ::core::mem::transmute(phwnd))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetMode<'a, P0, P1>(hinstall: P0, erunmode: MSIRUNMODE, fstate: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE, fstate: super::super::Foundation::BOOL) -> u32;
    }
    MsiSetMode(hinstall.into(), erunmode, fstate.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetPropertyA<'a, P0, P1, P2>(hinstall: P0, szname: P1, szvalue: P2) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetPropertyA(hinstall: MSIHANDLE, szname: ::windows::core::PCSTR, szvalue: ::windows::core::PCSTR) -> u32;
    }
    MsiSetPropertyA(hinstall.into(), szname.into(), szvalue.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetPropertyW<'a, P0, P1, P2>(hinstall: P0, szname: P1, szvalue: P2) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetPropertyW(hinstall: MSIHANDLE, szname: ::windows::core::PCWSTR, szvalue: ::windows::core::PCWSTR) -> u32;
    }
    MsiSetPropertyW(hinstall.into(), szname.into(), szvalue.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetTargetPathA<'a, P0, P1, P2>(hinstall: P0, szfolder: P1, szfolderpath: P2) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetTargetPathA(hinstall: MSIHANDLE, szfolder: ::windows::core::PCSTR, szfolderpath: ::windows::core::PCSTR) -> u32;
    }
    MsiSetTargetPathA(hinstall.into(), szfolder.into(), szfolderpath.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetTargetPathW<'a, P0, P1, P2>(hinstall: P0, szfolder: P1, szfolderpath: P2) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSetTargetPathW(hinstall: MSIHANDLE, szfolder: ::windows::core::PCWSTR, szfolderpath: ::windows::core::PCWSTR) -> u32;
    }
    MsiSetTargetPathW(hinstall.into(), szfolder.into(), szfolderpath.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskA<'a, P0, P1, P2, P3>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: P2, szdiskprompt: P3) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListAddMediaDiskA(szproductcodeorpatchcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: ::windows::core::PCSTR, szdiskprompt: ::windows::core::PCSTR) -> u32;
    }
    MsiSourceListAddMediaDiskA(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, dwdiskid, szvolumelabel.into(), szdiskprompt.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskW<'a, P0, P1, P2, P3>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: P2, szdiskprompt: P3) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListAddMediaDiskW(szproductcodeorpatchcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: ::windows::core::PCWSTR, szdiskprompt: ::windows::core::PCWSTR) -> u32;
    }
    MsiSourceListAddMediaDiskW(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, dwdiskid, szvolumelabel.into(), szdiskprompt.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddSourceA<'a, P0, P1, P2>(szproduct: P0, szusername: P1, dwreserved: u32, szsource: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListAddSourceA(szproduct: ::windows::core::PCSTR, szusername: ::windows::core::PCSTR, dwreserved: u32, szsource: ::windows::core::PCSTR) -> u32;
    }
    MsiSourceListAddSourceA(szproduct.into(), szusername.into(), dwreserved, szsource.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddSourceExA<'a, P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P2, dwindex: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListAddSourceExA(szproductcodeorpatchcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: ::windows::core::PCSTR, dwindex: u32) -> u32;
    }
    MsiSourceListAddSourceExA(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, szsource.into(), dwindex)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddSourceExW<'a, P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P2, dwindex: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListAddSourceExW(szproductcodeorpatchcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: ::windows::core::PCWSTR, dwindex: u32) -> u32;
    }
    MsiSourceListAddSourceExW(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, szsource.into(), dwindex)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddSourceW<'a, P0, P1, P2>(szproduct: P0, szusername: P1, dwreserved: u32, szsource: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListAddSourceW(szproduct: ::windows::core::PCWSTR, szusername: ::windows::core::PCWSTR, dwreserved: u32, szsource: ::windows::core::PCWSTR) -> u32;
    }
    MsiSourceListAddSourceW(szproduct.into(), szusername.into(), dwreserved, szsource.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearAllA<'a, P0, P1>(szproduct: P0, szusername: P1, dwreserved: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListClearAllA(szproduct: ::windows::core::PCSTR, szusername: ::windows::core::PCSTR, dwreserved: u32) -> u32;
    }
    MsiSourceListClearAllA(szproduct.into(), szusername.into(), dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearAllExA<'a, P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListClearAllExA(szproductcodeorpatchcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    }
    MsiSourceListClearAllExA(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearAllExW<'a, P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListClearAllExW(szproductcodeorpatchcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    }
    MsiSourceListClearAllExW(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearAllW<'a, P0, P1>(szproduct: P0, szusername: P1, dwreserved: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListClearAllW(szproduct: ::windows::core::PCWSTR, szusername: ::windows::core::PCWSTR, dwreserved: u32) -> u32;
    }
    MsiSourceListClearAllW(szproduct.into(), szusername.into(), dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskA<'a, P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListClearMediaDiskA(szproductcodeorpatchcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32;
    }
    MsiSourceListClearMediaDiskA(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, dwdiskid)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskW<'a, P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListClearMediaDiskW(szproductcodeorpatchcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32;
    }
    MsiSourceListClearMediaDiskW(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, dwdiskid)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearSourceA<'a, P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListClearSourceA(szproductcodeorpatchcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: ::windows::core::PCSTR) -> u32;
    }
    MsiSourceListClearSourceA(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, szsource.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearSourceW<'a, P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListClearSourceW(szproductcodeorpatchcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: ::windows::core::PCWSTR) -> u32;
    }
    MsiSourceListClearSourceW(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, szsource.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksA<'a, P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: ::windows::core::PSTR, pcchvolumelabel: *mut u32, szdiskprompt: ::windows::core::PSTR, pcchdiskprompt: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: ::windows::core::PSTR, pcchvolumelabel: *mut u32, szdiskprompt: ::windows::core::PSTR, pcchdiskprompt: *mut u32) -> u32;
    }
    MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, dwindex, ::core::mem::transmute(pdwdiskid), ::core::mem::transmute(szvolumelabel), ::core::mem::transmute(pcchvolumelabel), ::core::mem::transmute(szdiskprompt), ::core::mem::transmute(pcchdiskprompt))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksW<'a, P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: ::windows::core::PWSTR, pcchvolumelabel: *mut u32, szdiskprompt: ::windows::core::PWSTR, pcchdiskprompt: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: ::windows::core::PWSTR, pcchvolumelabel: *mut u32, szdiskprompt: ::windows::core::PWSTR, pcchdiskprompt: *mut u32) -> u32;
    }
    MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, dwindex, ::core::mem::transmute(pdwdiskid), ::core::mem::transmute(szvolumelabel), ::core::mem::transmute(pcchvolumelabel), ::core::mem::transmute(szdiskprompt), ::core::mem::transmute(pcchdiskprompt))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListEnumSourcesA<'a, P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: ::windows::core::PSTR, pcchsource: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListEnumSourcesA(szproductcodeorpatchcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: ::windows::core::PSTR, pcchsource: *mut u32) -> u32;
    }
    MsiSourceListEnumSourcesA(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, dwindex, ::core::mem::transmute(szsource), ::core::mem::transmute(pcchsource))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListEnumSourcesW<'a, P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: ::windows::core::PWSTR, pcchsource: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListEnumSourcesW(szproductcodeorpatchcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: ::windows::core::PWSTR, pcchsource: *mut u32) -> u32;
    }
    MsiSourceListEnumSourcesW(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, dwindex, ::core::mem::transmute(szsource), ::core::mem::transmute(pcchsource))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListForceResolutionA<'a, P0, P1>(szproduct: P0, szusername: P1, dwreserved: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListForceResolutionA(szproduct: ::windows::core::PCSTR, szusername: ::windows::core::PCSTR, dwreserved: u32) -> u32;
    }
    MsiSourceListForceResolutionA(szproduct.into(), szusername.into(), dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListForceResolutionExA<'a, P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListForceResolutionExA(szproductcodeorpatchcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    }
    MsiSourceListForceResolutionExA(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListForceResolutionExW<'a, P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListForceResolutionExW(szproductcodeorpatchcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    }
    MsiSourceListForceResolutionExW(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListForceResolutionW<'a, P0, P1>(szproduct: P0, szusername: P1, dwreserved: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListForceResolutionW(szproduct: ::windows::core::PCWSTR, szusername: ::windows::core::PCWSTR, dwreserved: u32) -> u32;
    }
    MsiSourceListForceResolutionW(szproduct.into(), szusername.into(), dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListGetInfoA<'a, P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P2, szvalue: ::windows::core::PSTR, pcchvalue: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListGetInfoA(szproductcodeorpatchcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: ::windows::core::PCSTR, szvalue: ::windows::core::PSTR, pcchvalue: *mut u32) -> u32;
    }
    MsiSourceListGetInfoA(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, szproperty.into(), ::core::mem::transmute(szvalue), ::core::mem::transmute(pcchvalue))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListGetInfoW<'a, P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P2, szvalue: ::windows::core::PWSTR, pcchvalue: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListGetInfoW(szproductcodeorpatchcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: ::windows::core::PCWSTR, szvalue: ::windows::core::PWSTR, pcchvalue: *mut u32) -> u32;
    }
    MsiSourceListGetInfoW(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, szproperty.into(), ::core::mem::transmute(szvalue), ::core::mem::transmute(pcchvalue))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListSetInfoA<'a, P0, P1, P2, P3>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P2, szvalue: P3) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListSetInfoA(szproductcodeorpatchcode: ::windows::core::PCSTR, szusersid: ::windows::core::PCSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: ::windows::core::PCSTR, szvalue: ::windows::core::PCSTR) -> u32;
    }
    MsiSourceListSetInfoA(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, szproperty.into(), szvalue.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListSetInfoW<'a, P0, P1, P2, P3>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P2, szvalue: P3) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSourceListSetInfoW(szproductcodeorpatchcode: ::windows::core::PCWSTR, szusersid: ::windows::core::PCWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: ::windows::core::PCWSTR, szvalue: ::windows::core::PCWSTR) -> u32;
    }
    MsiSourceListSetInfoW(szproductcodeorpatchcode.into(), szusersid.into(), dwcontext, dwoptions, szproperty.into(), szvalue.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyA<'a, P0>(hsummaryinfo: P0, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut super::super::Foundation::FILETIME, szvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSummaryInfoGetPropertyA(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut super::super::Foundation::FILETIME, szvaluebuf: ::windows::core::PSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiSummaryInfoGetPropertyA(hsummaryinfo.into(), uiproperty, ::core::mem::transmute(puidatatype), ::core::mem::transmute(pivalue), ::core::mem::transmute(pftvalue), ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyCount<'a, P0>(hsummaryinfo: P0, puipropertycount: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSummaryInfoGetPropertyCount(hsummaryinfo: MSIHANDLE, puipropertycount: *mut u32) -> u32;
    }
    MsiSummaryInfoGetPropertyCount(hsummaryinfo.into(), ::core::mem::transmute(puipropertycount))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyW<'a, P0>(hsummaryinfo: P0, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut super::super::Foundation::FILETIME, szvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSummaryInfoGetPropertyW(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut super::super::Foundation::FILETIME, szvaluebuf: ::windows::core::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    }
    MsiSummaryInfoGetPropertyW(hsummaryinfo.into(), uiproperty, ::core::mem::transmute(puidatatype), ::core::mem::transmute(pivalue), ::core::mem::transmute(pftvalue), ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSummaryInfoPersist<'a, P0>(hsummaryinfo: P0) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSummaryInfoPersist(hsummaryinfo: MSIHANDLE) -> u32;
    }
    MsiSummaryInfoPersist(hsummaryinfo.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyA<'a, P0, P1>(hsummaryinfo: P0, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSummaryInfoSetPropertyA(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: ::windows::core::PCSTR) -> u32;
    }
    MsiSummaryInfoSetPropertyA(hsummaryinfo.into(), uiproperty, uidatatype, ivalue, ::core::mem::transmute(pftvalue), szvalue.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyW<'a, P0, P1>(hsummaryinfo: P0, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiSummaryInfoSetPropertyW(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: ::windows::core::PCWSTR) -> u32;
    }
    MsiSummaryInfoSetPropertyW(hsummaryinfo.into(), uiproperty, uidatatype, ivalue, ::core::mem::transmute(pftvalue), szvalue.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiUseFeatureA<'a, P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiUseFeatureA(szproduct: ::windows::core::PCSTR, szfeature: ::windows::core::PCSTR) -> INSTALLSTATE;
    }
    MsiUseFeatureA(szproduct.into(), szfeature.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiUseFeatureExA<'a, P0, P1>(szproduct: P0, szfeature: P1, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiUseFeatureExA(szproduct: ::windows::core::PCSTR, szfeature: ::windows::core::PCSTR, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE;
    }
    MsiUseFeatureExA(szproduct.into(), szfeature.into(), dwinstallmode, dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiUseFeatureExW<'a, P0, P1>(szproduct: P0, szfeature: P1, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiUseFeatureExW(szproduct: ::windows::core::PCWSTR, szfeature: ::windows::core::PCWSTR, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE;
    }
    MsiUseFeatureExW(szproduct.into(), szfeature.into(), dwinstallmode, dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiUseFeatureW<'a, P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiUseFeatureW(szproduct: ::windows::core::PCWSTR, szfeature: ::windows::core::PCWSTR) -> INSTALLSTATE;
    }
    MsiUseFeatureW(szproduct.into(), szfeature.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiVerifyDiskSpace<'a, P0>(hinstall: P0) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiVerifyDiskSpace(hinstall: MSIHANDLE) -> u32;
    }
    MsiVerifyDiskSpace(hinstall.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiVerifyPackageA<'a, P0>(szpackagepath: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiVerifyPackageA(szpackagepath: ::windows::core::PCSTR) -> u32;
    }
    MsiVerifyPackageA(szpackagepath.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiVerifyPackageW<'a, P0>(szpackagepath: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiVerifyPackageW(szpackagepath: ::windows::core::PCWSTR) -> u32;
    }
    MsiVerifyPackageW(szpackagepath.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewClose<'a, P0>(hview: P0) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiViewClose(hview: MSIHANDLE) -> u32;
    }
    MsiViewClose(hview.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewExecute<'a, P0, P1>(hview: P0, hrecord: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiViewExecute(hview: MSIHANDLE, hrecord: MSIHANDLE) -> u32;
    }
    MsiViewExecute(hview.into(), hrecord.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewFetch<'a, P0>(hview: P0, phrecord: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiViewFetch(hview: MSIHANDLE, phrecord: *mut MSIHANDLE) -> u32;
    }
    MsiViewFetch(hview.into(), ::core::mem::transmute(phrecord))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewGetColumnInfo<'a, P0>(hview: P0, ecolumninfo: MSICOLINFO, phrecord: *mut MSIHANDLE) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiViewGetColumnInfo(hview: MSIHANDLE, ecolumninfo: MSICOLINFO, phrecord: *mut MSIHANDLE) -> u32;
    }
    MsiViewGetColumnInfo(hview.into(), ecolumninfo, ::core::mem::transmute(phrecord))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewGetErrorA<'a, P0>(hview: P0, szcolumnnamebuffer: ::windows::core::PSTR, pcchbuf: *mut u32) -> MSIDBERROR
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiViewGetErrorA(hview: MSIHANDLE, szcolumnnamebuffer: ::windows::core::PSTR, pcchbuf: *mut u32) -> MSIDBERROR;
    }
    MsiViewGetErrorA(hview.into(), ::core::mem::transmute(szcolumnnamebuffer), ::core::mem::transmute(pcchbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewGetErrorW<'a, P0>(hview: P0, szcolumnnamebuffer: ::windows::core::PWSTR, pcchbuf: *mut u32) -> MSIDBERROR
where
    P0: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiViewGetErrorW(hview: MSIHANDLE, szcolumnnamebuffer: ::windows::core::PWSTR, pcchbuf: *mut u32) -> MSIDBERROR;
    }
    MsiViewGetErrorW(hview.into(), ::core::mem::transmute(szcolumnnamebuffer), ::core::mem::transmute(pcchbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewModify<'a, P0, P1>(hview: P0, emodifymode: MSIMODIFY, hrecord: P1) -> u32
where
    P0: ::std::convert::Into<MSIHANDLE>,
    P1: ::std::convert::Into<MSIHANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MsiViewModify(hview: MSIHANDLE, emodifymode: MSIMODIFY, hrecord: MSIHANDLE) -> u32;
    }
    MsiViewModify(hview.into(), emodifymode, hrecord.into())
}
pub const MsmMerge: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0adda830_2c26_11d2_ad65_00a0c9af11a6);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NormalizeFileForPatchSignature(filebuffer: *mut ::core::ffi::c_void, filesize: u32, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, newfilecoffbase: u32, newfilecofftime: u32, ignorerangearray: &[PATCH_IGNORE_RANGE], retainrangearray: &[PATCH_RETAIN_RANGE]) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NormalizeFileForPatchSignature(filebuffer: *mut ::core::ffi::c_void, filesize: u32, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, newfilecoffbase: u32, newfilecofftime: u32, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE) -> i32;
    }
    NormalizeFileForPatchSignature(::core::mem::transmute(filebuffer), filesize, optionflags, ::core::mem::transmute(optiondata), newfilecoffbase, newfilecofftime, ignorerangearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ignorerangearray)), retainrangearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(retainrangearray)))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PACKMAN_RUNTIME(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_NATIVE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_SILVERLIGHTMOBILE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_XNA: PACKMAN_RUNTIME = PACKMAN_RUNTIME(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_MODERN_NATIVE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_JUPITER: PACKMAN_RUNTIME = PACKMAN_RUNTIME(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_INVALID: PACKMAN_RUNTIME = PACKMAN_RUNTIME(6i32);
impl ::core::marker::Copy for PACKMAN_RUNTIME {}
impl ::core::clone::Clone for PACKMAN_RUNTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PACKMAN_RUNTIME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PACKMAN_RUNTIME {
    type Abi = Self;
}
impl ::core::fmt::Debug for PACKMAN_RUNTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PACKMAN_RUNTIME").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_IGNORE_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
}
impl ::core::marker::Copy for PATCH_IGNORE_RANGE {}
impl ::core::clone::Clone for PATCH_IGNORE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_IGNORE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_IGNORE_RANGE").field("OffsetInOldFile", &self.OffsetInOldFile).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for PATCH_IGNORE_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PATCH_IGNORE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PATCH_IGNORE_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PATCH_IGNORE_RANGE {}
impl ::core::default::Default for PATCH_IGNORE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_INTERLEAVE_MAP {
    pub CountRanges: u32,
    pub Range: [PATCH_INTERLEAVE_MAP_0; 1],
}
impl ::core::marker::Copy for PATCH_INTERLEAVE_MAP {}
impl ::core::clone::Clone for PATCH_INTERLEAVE_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_INTERLEAVE_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_INTERLEAVE_MAP").field("CountRanges", &self.CountRanges).field("Range", &self.Range).finish()
    }
}
unsafe impl ::windows::core::Abi for PATCH_INTERLEAVE_MAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PATCH_INTERLEAVE_MAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PATCH_INTERLEAVE_MAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for PATCH_INTERLEAVE_MAP {}
impl ::core::default::Default for PATCH_INTERLEAVE_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_INTERLEAVE_MAP_0 {
    pub OldOffset: u32,
    pub OldLength: u32,
    pub NewLength: u32,
}
impl ::core::marker::Copy for PATCH_INTERLEAVE_MAP_0 {}
impl ::core::clone::Clone for PATCH_INTERLEAVE_MAP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_INTERLEAVE_MAP_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_INTERLEAVE_MAP_0").field("OldOffset", &self.OldOffset).field("OldLength", &self.OldLength).field("NewLength", &self.NewLength).finish()
    }
}
unsafe impl ::windows::core::Abi for PATCH_INTERLEAVE_MAP_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PATCH_INTERLEAVE_MAP_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PATCH_INTERLEAVE_MAP_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PATCH_INTERLEAVE_MAP_0 {}
impl ::core::default::Default for PATCH_INTERLEAVE_MAP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO {
    pub SizeOfThisStruct: u32,
    pub Anonymous: PATCH_OLD_FILE_INFO_0,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PATCH_OLD_FILE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PATCH_OLD_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PATCH_OLD_FILE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PATCH_OLD_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PATCH_OLD_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PATCH_OLD_FILE_INFO_0 {
    pub OldFileNameA: ::windows::core::PCSTR,
    pub OldFileNameW: ::windows::core::PCWSTR,
    pub OldFileHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PATCH_OLD_FILE_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PATCH_OLD_FILE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PATCH_OLD_FILE_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PATCH_OLD_FILE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PATCH_OLD_FILE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_OLD_FILE_INFO_A {
    pub SizeOfThisStruct: u32,
    pub OldFileName: ::windows::core::PCSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_A {}
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_OLD_FILE_INFO_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_OLD_FILE_INFO_A").field("SizeOfThisStruct", &self.SizeOfThisStruct).field("OldFileName", &self.OldFileName).field("IgnoreRangeCount", &self.IgnoreRangeCount).field("IgnoreRangeArray", &self.IgnoreRangeArray).field("RetainRangeCount", &self.RetainRangeCount).field("RetainRangeArray", &self.RetainRangeArray).finish()
    }
}
unsafe impl ::windows::core::Abi for PATCH_OLD_FILE_INFO_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PATCH_OLD_FILE_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PATCH_OLD_FILE_INFO_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for PATCH_OLD_FILE_INFO_A {}
impl ::core::default::Default for PATCH_OLD_FILE_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO_H {
    pub SizeOfThisStruct: u32,
    pub OldFileHandle: super::super::Foundation::HANDLE,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_H {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_H {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PATCH_OLD_FILE_INFO_H {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_OLD_FILE_INFO_H").field("SizeOfThisStruct", &self.SizeOfThisStruct).field("OldFileHandle", &self.OldFileHandle).field("IgnoreRangeCount", &self.IgnoreRangeCount).field("IgnoreRangeArray", &self.IgnoreRangeArray).field("RetainRangeCount", &self.RetainRangeCount).field("RetainRangeArray", &self.RetainRangeArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PATCH_OLD_FILE_INFO_H {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PATCH_OLD_FILE_INFO_H {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PATCH_OLD_FILE_INFO_H>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PATCH_OLD_FILE_INFO_H {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PATCH_OLD_FILE_INFO_H {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_OLD_FILE_INFO_W {
    pub SizeOfThisStruct: u32,
    pub OldFileName: ::windows::core::PCWSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_W {}
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_OLD_FILE_INFO_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_OLD_FILE_INFO_W").field("SizeOfThisStruct", &self.SizeOfThisStruct).field("OldFileName", &self.OldFileName).field("IgnoreRangeCount", &self.IgnoreRangeCount).field("IgnoreRangeArray", &self.IgnoreRangeArray).field("RetainRangeCount", &self.RetainRangeCount).field("RetainRangeArray", &self.RetainRangeArray).finish()
    }
}
unsafe impl ::windows::core::Abi for PATCH_OLD_FILE_INFO_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PATCH_OLD_FILE_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PATCH_OLD_FILE_INFO_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for PATCH_OLD_FILE_INFO_W {}
impl ::core::default::Default for PATCH_OLD_FILE_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OPTION_DATA {
    pub SizeOfThisStruct: u32,
    pub SymbolOptionFlags: u32,
    pub NewFileSymbolPath: ::windows::core::PCSTR,
    pub OldFileSymbolPathArray: *mut ::windows::core::PSTR,
    pub ExtendedOptionFlags: u32,
    pub SymLoadCallback: PPATCH_SYMLOAD_CALLBACK,
    pub SymLoadContext: *mut ::core::ffi::c_void,
    pub InterleaveMapArray: *mut *mut PATCH_INTERLEAVE_MAP,
    pub MaxLzxWindowSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OPTION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OPTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PATCH_OPTION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_OPTION_DATA")
            .field("SizeOfThisStruct", &self.SizeOfThisStruct)
            .field("SymbolOptionFlags", &self.SymbolOptionFlags)
            .field("NewFileSymbolPath", &self.NewFileSymbolPath)
            .field("OldFileSymbolPathArray", &self.OldFileSymbolPathArray)
            .field("ExtendedOptionFlags", &self.ExtendedOptionFlags)
            .field("SymLoadCallback", &self.SymLoadCallback.map(|f| f as usize))
            .field("SymLoadContext", &self.SymLoadContext)
            .field("InterleaveMapArray", &self.InterleaveMapArray)
            .field("MaxLzxWindowSize", &self.MaxLzxWindowSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PATCH_OPTION_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PATCH_OPTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PATCH_OPTION_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PATCH_OPTION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PATCH_OPTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_FAIL_IF_BIGGER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_FAIL_IF_SAME_FILE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_INTERLEAVE_FILES: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_BINDFIX: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_CHECKSUM: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_LOCKFIX: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_REBASE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_RESTIMEFIX: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_TIMESTAMP: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_RESERVED1: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_SIGNATURE_MD5: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_BEST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_A: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_B: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_BEST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_LARGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_VALID_FLAGS: u32 = 3237937159u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_RETAIN_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
    pub OffsetInNewFile: u32,
}
impl ::core::marker::Copy for PATCH_RETAIN_RANGE {}
impl ::core::clone::Clone for PATCH_RETAIN_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_RETAIN_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_RETAIN_RANGE").field("OffsetInOldFile", &self.OffsetInOldFile).field("LengthInBytes", &self.LengthInBytes).field("OffsetInNewFile", &self.OffsetInNewFile).finish()
    }
}
unsafe impl ::windows::core::Abi for PATCH_RETAIN_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PATCH_RETAIN_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PATCH_RETAIN_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PATCH_RETAIN_RANGE {}
impl ::core::default::Default for PATCH_RETAIN_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_NO_FAILURES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_NO_IMAGEHLP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_RESERVED1: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_UNDECORATED_TOO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_TRANSFORM_PE_IRELOC_2: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_TRANSFORM_PE_RESOURCE_2: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_APPNAME: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_AUTHOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_CHARCOUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_COMMENTS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_CREATE_DTM: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_EDITTIME: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_KEYWORDS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_LASTAUTHOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_LASTPRINTED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_LASTSAVE_DTM: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_MSIRESTRICT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_MSISOURCE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_MSIVERSION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_PAGECOUNT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_REVNUMBER: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_SUBJECT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_TEMPLATE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_THUMBNAIL: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_TITLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_WORDCOUNT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PINSTALLUI_HANDLER_RECORD = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, hrecord: MSIHANDLE) -> i32>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PMSIHANDLE {
    pub m_h: MSIHANDLE,
}
impl ::core::marker::Copy for PMSIHANDLE {}
impl ::core::clone::Clone for PMSIHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PMSIHANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PMSIHANDLE").field("m_h", &self.m_h).finish()
    }
}
unsafe impl ::windows::core::Abi for PMSIHANDLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PMSIHANDLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PMSIHANDLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PMSIHANDLE {}
impl ::core::default::Default for PMSIHANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PMSvc: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9e511fc_e364_497a_a121_b7b3612cedce);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ACTIVATION_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_RESUME: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_RESUMESAMEPARAMS: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_REPLACE: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_REPLACESAMEPARAMS: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_MULTISESSION: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_REPLACE_IGNOREFOREGROUND: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_UNKNOWN: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_INVALID: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(7i32);
impl ::core::marker::Copy for PM_ACTIVATION_POLICY {}
impl ::core::clone::Clone for PM_ACTIVATION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ACTIVATION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_ACTIVATION_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_ACTIVATION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ACTIVATION_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_APPLICATION_HUBTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_HUBTYPE_NONMUSIC: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_HUBTYPE_MUSIC: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_HUBTYPE_INVALID: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(2i32);
impl ::core::marker::Copy for PM_APPLICATION_HUBTYPE {}
impl ::core::clone::Clone for PM_APPLICATION_HUBTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_APPLICATION_HUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_APPLICATION_HUBTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_APPLICATION_HUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_APPLICATION_HUBTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_APPLICATION_INSTALL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_NORMAL: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_IN_ROM: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_PA: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_DEBUG: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_ENTERPRISE: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_INVALID: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(5i32);
impl ::core::marker::Copy for PM_APPLICATION_INSTALL_TYPE {}
impl ::core::clone::Clone for PM_APPLICATION_INSTALL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_APPLICATION_INSTALL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_APPLICATION_INSTALL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_APPLICATION_INSTALL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_APPLICATION_INSTALL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_APPLICATION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_MIN: PM_APPLICATION_STATE = PM_APPLICATION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_INSTALLED: PM_APPLICATION_STATE = PM_APPLICATION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_INSTALLING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_UPDATING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_UNINSTALLING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_LICENSE_UPDATING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_MOVING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_SD_CARD: PM_APPLICATION_STATE = PM_APPLICATION_STATE(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_ENTERPRISE: PM_APPLICATION_STATE = PM_APPLICATION_STATE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_BACKING_UP: PM_APPLICATION_STATE = PM_APPLICATION_STATE(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_MDIL_BINDING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_MAX: PM_APPLICATION_STATE = PM_APPLICATION_STATE(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_INVALID: PM_APPLICATION_STATE = PM_APPLICATION_STATE(11i32);
impl ::core::marker::Copy for PM_APPLICATION_STATE {}
impl ::core::clone::Clone for PM_APPLICATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_APPLICATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_APPLICATION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_APPLICATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_APPLICATION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_APP_GENRE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_GENRE_GAMES: PM_APP_GENRE = PM_APP_GENRE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_GENRE_OTHER: PM_APP_GENRE = PM_APP_GENRE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_GENRE_INVALID: PM_APP_GENRE = PM_APP_GENRE(2i32);
impl ::core::marker::Copy for PM_APP_GENRE {}
impl ::core::clone::Clone for PM_APP_GENRE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_APP_GENRE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_APP_GENRE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_APP_GENRE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_APP_GENRE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_BSATASKID {
    pub ProductID: ::windows::core::GUID,
    pub TaskID: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_BSATASKID {
    fn clone(&self) -> Self {
        Self { ProductID: self.ProductID, TaskID: self.TaskID.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_BSATASKID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_BSATASKID").field("ProductID", &self.ProductID).field("TaskID", &self.TaskID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PM_BSATASKID {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_BSATASKID {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.TaskID == other.TaskID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_BSATASKID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_BSATASKID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_BWTASKID {
    pub ProductID: ::windows::core::GUID,
    pub TaskID: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_BWTASKID {
    fn clone(&self) -> Self {
        Self { ProductID: self.ProductID, TaskID: self.TaskID.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_BWTASKID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_BWTASKID").field("ProductID", &self.ProductID).field("TaskID", &self.TaskID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PM_BWTASKID {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_BWTASKID {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.TaskID == other.TaskID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_BWTASKID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_BWTASKID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_APP_FILTER(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_ALL: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_VISIBLE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_GENRE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_NONGAMES: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_HUBTYPE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_PINABLEONKIDZONE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_ALL_INCLUDE_MODERN: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_FRAMEWORK: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_MAX: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(8i32);
impl ::core::marker::Copy for PM_ENUM_APP_FILTER {}
impl ::core::clone::Clone for PM_ENUM_APP_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_APP_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_ENUM_APP_FILTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_ENUM_APP_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_APP_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_BSA_FILTER(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_ALL: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(26i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_TASKID: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(27i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_PRODUCTID: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(28i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_PERIODIC: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(29i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_ALL_LAUNCHONBOOT: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(30i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_MAX: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(31i32);
impl ::core::marker::Copy for PM_ENUM_BSA_FILTER {}
impl ::core::clone::Clone for PM_ENUM_BSA_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_BSA_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_ENUM_BSA_FILTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_ENUM_BSA_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_BSA_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_BW_FILTER(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BW_FILTER_BOOTWORKER_ALL: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(31i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BW_FILTER_BY_TASKID: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BW_FILTER_MAX: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(33i32);
impl ::core::marker::Copy for PM_ENUM_BW_FILTER {}
impl ::core::clone::Clone for PM_ENUM_BW_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_BW_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_ENUM_BW_FILTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_ENUM_BW_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_BW_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_EXTENSION_FILTER(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_BY_CONSUMER: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_APPCONNECT: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_PROTOCOL_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(18i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_FILETYPE_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(19i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_CONTENTTYPE_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(20i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_APPLICATION_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(21i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_SHARETARGET_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(22i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FILEOPENPICKER_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(23i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FILESAVEPICKER_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(24i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_CACHEDFILEUPDATER_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(25i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_MAX: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(26i32);
impl ::core::marker::Copy for PM_ENUM_EXTENSION_FILTER {}
impl ::core::clone::Clone for PM_ENUM_EXTENSION_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_EXTENSION_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_ENUM_EXTENSION_FILTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_ENUM_EXTENSION_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_EXTENSION_FILTER").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_ENUM_FILTER {
    pub FilterType: i32,
    pub FilterParameter: PM_ENUM_FILTER_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_ENUM_FILTER {
    fn clone(&self) -> Self {
        Self { FilterType: self.FilterType, FilterParameter: self.FilterParameter.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PM_ENUM_FILTER {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_ENUM_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.FilterType == other.FilterType && self.FilterParameter == other.FilterParameter
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_ENUM_FILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_ENUM_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PM_ENUM_FILTER_0 {
    pub Dummy: i32,
    pub Genre: PM_APP_GENRE,
    pub AppHubType: PM_APPLICATION_HUBTYPE,
    pub HubType: PM_TILE_HUBTYPE,
    pub Tasktype: PM_TASK_TYPE,
    pub TaskProductID: ::windows::core::GUID,
    pub TileProductID: ::windows::core::GUID,
    pub AppTaskType: _tagAPPTASKTYPE,
    pub Consumer: ::core::mem::ManuallyDrop<PM_EXTENSIONCONSUMER>,
    pub BSATask: ::core::mem::ManuallyDrop<PM_BSATASKID>,
    pub BSAProductID: ::windows::core::GUID,
    pub BWTask: ::core::mem::ManuallyDrop<PM_BWTASKID>,
    pub ProtocolName: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub FileType: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub ContentType: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub AppSupportedFileExtPID: ::windows::core::GUID,
    pub ShareTargetFileType: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_ENUM_FILTER_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PM_ENUM_FILTER_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_ENUM_FILTER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PM_ENUM_FILTER_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_ENUM_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_ENUM_FILTER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_TASK_FILTER(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_APP_ALL: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(12i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_TASK_TYPE: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(13i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_DEHYD_SUPRESSING: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(14i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_APP_TASK_TYPE: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(15i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_BGEXECUTION: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_MAX: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(17i32);
impl ::core::marker::Copy for PM_ENUM_TASK_FILTER {}
impl ::core::clone::Clone for PM_ENUM_TASK_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_TASK_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_ENUM_TASK_FILTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_ENUM_TASK_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_TASK_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_TILE_FILTER(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_APPLIST: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_PINNED: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_HUBTYPE: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_APP_ALL: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_MAX: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(12i32);
impl ::core::marker::Copy for PM_ENUM_TILE_FILTER {}
impl ::core::clone::Clone for PM_ENUM_TILE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_TILE_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_ENUM_TILE_FILTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_ENUM_TILE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_TILE_FILTER").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_EXTENSIONCONSUMER {
    pub ConsumerPID: ::windows::core::GUID,
    pub ExtensionID: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_EXTENSIONCONSUMER {
    fn clone(&self) -> Self {
        Self { ConsumerPID: self.ConsumerPID, ExtensionID: self.ExtensionID.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_EXTENSIONCONSUMER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_EXTENSIONCONSUMER").field("ConsumerPID", &self.ConsumerPID).field("ExtensionID", &self.ExtensionID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PM_EXTENSIONCONSUMER {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_EXTENSIONCONSUMER {
    fn eq(&self, other: &Self) -> bool {
        self.ConsumerPID == other.ConsumerPID && self.ExtensionID == other.ExtensionID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_EXTENSIONCONSUMER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_EXTENSIONCONSUMER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_INSTALLINFO {
    pub ProductID: ::windows::core::GUID,
    pub PackagePath: super::super::Foundation::BSTR,
    pub InstanceID: ::windows::core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub IsUninstallDisabled: super::super::Foundation::BOOL,
    pub DeploymentOptions: u32,
    pub OfferID: ::windows::core::GUID,
    pub MarketplaceAppVersion: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_INSTALLINFO {
    fn clone(&self) -> Self {
        Self {
            ProductID: self.ProductID,
            PackagePath: self.PackagePath.clone(),
            InstanceID: self.InstanceID,
            pbLicense: self.pbLicense,
            cbLicense: self.cbLicense,
            IsUninstallDisabled: self.IsUninstallDisabled,
            DeploymentOptions: self.DeploymentOptions,
            OfferID: self.OfferID,
            MarketplaceAppVersion: self.MarketplaceAppVersion.clone(),
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_INSTALLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_INSTALLINFO").field("ProductID", &self.ProductID).field("PackagePath", &self.PackagePath).field("InstanceID", &self.InstanceID).field("pbLicense", &self.pbLicense).field("cbLicense", &self.cbLicense).field("IsUninstallDisabled", &self.IsUninstallDisabled).field("DeploymentOptions", &self.DeploymentOptions).field("OfferID", &self.OfferID).field("MarketplaceAppVersion", &self.MarketplaceAppVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PM_INSTALLINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_INSTALLINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.PackagePath == other.PackagePath && self.InstanceID == other.InstanceID && self.pbLicense == other.pbLicense && self.cbLicense == other.cbLicense && self.IsUninstallDisabled == other.IsUninstallDisabled && self.DeploymentOptions == other.DeploymentOptions && self.OfferID == other.OfferID && self.MarketplaceAppVersion == other.MarketplaceAppVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_INSTALLINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_INSTALLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_INVOCATIONINFO {
    pub URIBaseOrAUMID: super::super::Foundation::BSTR,
    pub URIFragmentOrArgs: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_INVOCATIONINFO {
    fn clone(&self) -> Self {
        Self { URIBaseOrAUMID: self.URIBaseOrAUMID.clone(), URIFragmentOrArgs: self.URIFragmentOrArgs.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_INVOCATIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_INVOCATIONINFO").field("URIBaseOrAUMID", &self.URIBaseOrAUMID).field("URIFragmentOrArgs", &self.URIFragmentOrArgs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PM_INVOCATIONINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_INVOCATIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.URIBaseOrAUMID == other.URIBaseOrAUMID && self.URIFragmentOrArgs == other.URIFragmentOrArgs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_INVOCATIONINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_INVOCATIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_LIVETILE_RECURRENCE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_INSTANT: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_ONETIME: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_INTERVAL: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_MAX: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(2i32);
impl ::core::marker::Copy for PM_LIVETILE_RECURRENCE_TYPE {}
impl ::core::clone::Clone for PM_LIVETILE_RECURRENCE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_LIVETILE_RECURRENCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_LIVETILE_RECURRENCE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_LIVETILE_RECURRENCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_LIVETILE_RECURRENCE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_LOGO_SIZE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_SMALL: PM_LOGO_SIZE = PM_LOGO_SIZE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_MEDIUM: PM_LOGO_SIZE = PM_LOGO_SIZE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_LARGE: PM_LOGO_SIZE = PM_LOGO_SIZE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_INVALID: PM_LOGO_SIZE = PM_LOGO_SIZE(3i32);
impl ::core::marker::Copy for PM_LOGO_SIZE {}
impl ::core::clone::Clone for PM_LOGO_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_LOGO_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_LOGO_SIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_LOGO_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_LOGO_SIZE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_STARTAPPBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows::core::GUID,
    pub AppTitle: super::super::Foundation::BSTR,
    pub IconPath: super::super::Foundation::BSTR,
    pub IsUninstallable: super::super::Foundation::BOOL,
    pub AppInstallType: PM_APPLICATION_INSTALL_TYPE,
    pub InstanceID: ::windows::core::GUID,
    pub State: PM_APPLICATION_STATE,
    pub IsModern: super::super::Foundation::BOOL,
    pub IsModernLightUp: super::super::Foundation::BOOL,
    pub LightUpSupportMask: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_STARTAPPBLOB {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            ProductID: self.ProductID,
            AppTitle: self.AppTitle.clone(),
            IconPath: self.IconPath.clone(),
            IsUninstallable: self.IsUninstallable,
            AppInstallType: self.AppInstallType,
            InstanceID: self.InstanceID,
            State: self.State,
            IsModern: self.IsModern,
            IsModernLightUp: self.IsModernLightUp,
            LightUpSupportMask: self.LightUpSupportMask,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_STARTAPPBLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_STARTAPPBLOB").field("cbSize", &self.cbSize).field("ProductID", &self.ProductID).field("AppTitle", &self.AppTitle).field("IconPath", &self.IconPath).field("IsUninstallable", &self.IsUninstallable).field("AppInstallType", &self.AppInstallType).field("InstanceID", &self.InstanceID).field("State", &self.State).field("IsModern", &self.IsModern).field("IsModernLightUp", &self.IsModernLightUp).field("LightUpSupportMask", &self.LightUpSupportMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PM_STARTAPPBLOB {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_STARTAPPBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ProductID == other.ProductID && self.AppTitle == other.AppTitle && self.IconPath == other.IconPath && self.IsUninstallable == other.IsUninstallable && self.AppInstallType == other.AppInstallType && self.InstanceID == other.InstanceID && self.State == other.State && self.IsModern == other.IsModern && self.IsModernLightUp == other.IsModernLightUp && self.LightUpSupportMask == other.LightUpSupportMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_STARTAPPBLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_STARTAPPBLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_STARTTILEBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows::core::GUID,
    pub TileID: super::super::Foundation::BSTR,
    pub TemplateType: TILE_TEMPLATE_TYPE,
    pub HubPosition: [u32; 32],
    pub HubVisibilityBitmask: u32,
    pub IsDefault: super::super::Foundation::BOOL,
    pub TileType: PM_STARTTILE_TYPE,
    pub pbPropBlob: *mut u8,
    pub cbPropBlob: u32,
    pub IsRestoring: super::super::Foundation::BOOL,
    pub IsModern: super::super::Foundation::BOOL,
    pub InvocationInfo: PM_INVOCATIONINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_STARTTILEBLOB {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            ProductID: self.ProductID,
            TileID: self.TileID.clone(),
            TemplateType: self.TemplateType,
            HubPosition: self.HubPosition,
            HubVisibilityBitmask: self.HubVisibilityBitmask,
            IsDefault: self.IsDefault,
            TileType: self.TileType,
            pbPropBlob: self.pbPropBlob,
            cbPropBlob: self.cbPropBlob,
            IsRestoring: self.IsRestoring,
            IsModern: self.IsModern,
            InvocationInfo: self.InvocationInfo.clone(),
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_STARTTILEBLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_STARTTILEBLOB")
            .field("cbSize", &self.cbSize)
            .field("ProductID", &self.ProductID)
            .field("TileID", &self.TileID)
            .field("TemplateType", &self.TemplateType)
            .field("HubPosition", &self.HubPosition)
            .field("HubVisibilityBitmask", &self.HubVisibilityBitmask)
            .field("IsDefault", &self.IsDefault)
            .field("TileType", &self.TileType)
            .field("pbPropBlob", &self.pbPropBlob)
            .field("cbPropBlob", &self.cbPropBlob)
            .field("IsRestoring", &self.IsRestoring)
            .field("IsModern", &self.IsModern)
            .field("InvocationInfo", &self.InvocationInfo)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PM_STARTTILEBLOB {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_STARTTILEBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ProductID == other.ProductID && self.TileID == other.TileID && self.TemplateType == other.TemplateType && self.HubPosition == other.HubPosition && self.HubVisibilityBitmask == other.HubVisibilityBitmask && self.IsDefault == other.IsDefault && self.TileType == other.TileType && self.pbPropBlob == other.pbPropBlob && self.cbPropBlob == other.cbPropBlob && self.IsRestoring == other.IsRestoring && self.IsModern == other.IsModern && self.InvocationInfo == other.InvocationInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_STARTTILEBLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_STARTTILEBLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_STARTTILE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_PRIMARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_SECONDARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_APPLIST: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_APPLISTPRIMARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_INVALID: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(5i32);
impl ::core::marker::Copy for PM_STARTTILE_TYPE {}
impl ::core::clone::Clone for PM_STARTTILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_STARTTILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_STARTTILE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_STARTTILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_STARTTILE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_TASK_TRANSITION(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_DEFAULT: PM_TASK_TRANSITION = PM_TASK_TRANSITION(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_NONE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_TURNSTILE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_SLIDE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_SWIVEL: PM_TASK_TRANSITION = PM_TASK_TRANSITION(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_READERBOARD: PM_TASK_TRANSITION = PM_TASK_TRANSITION(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_CUSTOM: PM_TASK_TRANSITION = PM_TASK_TRANSITION(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_INVALID: PM_TASK_TRANSITION = PM_TASK_TRANSITION(7i32);
impl ::core::marker::Copy for PM_TASK_TRANSITION {}
impl ::core::clone::Clone for PM_TASK_TRANSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_TASK_TRANSITION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_TASK_TRANSITION {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_TASK_TRANSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_TASK_TRANSITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_TASK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_NORMAL: PM_TASK_TYPE = PM_TASK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_DEFAULT: PM_TASK_TYPE = PM_TASK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_SETTINGS: PM_TASK_TYPE = PM_TASK_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_BACKGROUNDSERVICEAGENT: PM_TASK_TYPE = PM_TASK_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_BACKGROUNDWORKER: PM_TASK_TYPE = PM_TASK_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_INVALID: PM_TASK_TYPE = PM_TASK_TYPE(5i32);
impl ::core::marker::Copy for PM_TASK_TYPE {}
impl ::core::clone::Clone for PM_TASK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_TASK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_TASK_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_TASK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_TASK_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_TILE_HUBTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_MUSIC: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_MOSETTINGS: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(268435456i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_GAMES: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(536870912i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_APPLIST: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(1073741824i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_STARTMENU: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(-2147483648i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_LOCKSCREEN: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(16777216i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_KIDZONE: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(33554432i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_CACHED: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(67108864i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_INVALID: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(67108865i32);
impl ::core::marker::Copy for PM_TILE_HUBTYPE {}
impl ::core::clone::Clone for PM_TILE_HUBTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_TILE_HUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_TILE_HUBTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_TILE_HUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_TILE_HUBTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_TILE_SIZE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_SMALL: PM_TILE_SIZE = PM_TILE_SIZE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_MEDIUM: PM_TILE_SIZE = PM_TILE_SIZE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_LARGE: PM_TILE_SIZE = PM_TILE_SIZE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_SQUARE310X310: PM_TILE_SIZE = PM_TILE_SIZE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_TALL150X310: PM_TILE_SIZE = PM_TILE_SIZE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_INVALID: PM_TILE_SIZE = PM_TILE_SIZE(5i32);
impl ::core::marker::Copy for PM_TILE_SIZE {}
impl ::core::clone::Clone for PM_TILE_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_TILE_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PM_TILE_SIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PM_TILE_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_TILE_SIZE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_UPDATEINFO {
    pub ProductID: ::windows::core::GUID,
    pub PackagePath: super::super::Foundation::BSTR,
    pub InstanceID: ::windows::core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: super::super::Foundation::BSTR,
    pub DeploymentOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_UPDATEINFO {
    fn clone(&self) -> Self {
        Self {
            ProductID: self.ProductID,
            PackagePath: self.PackagePath.clone(),
            InstanceID: self.InstanceID,
            pbLicense: self.pbLicense,
            cbLicense: self.cbLicense,
            MarketplaceAppVersion: self.MarketplaceAppVersion.clone(),
            DeploymentOptions: self.DeploymentOptions,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_UPDATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_UPDATEINFO").field("ProductID", &self.ProductID).field("PackagePath", &self.PackagePath).field("InstanceID", &self.InstanceID).field("pbLicense", &self.pbLicense).field("cbLicense", &self.cbLicense).field("MarketplaceAppVersion", &self.MarketplaceAppVersion).field("DeploymentOptions", &self.DeploymentOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PM_UPDATEINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_UPDATEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.PackagePath == other.PackagePath && self.InstanceID == other.InstanceID && self.pbLicense == other.pbLicense && self.cbLicense == other.cbLicense && self.MarketplaceAppVersion == other.MarketplaceAppVersion && self.DeploymentOptions == other.DeploymentOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_UPDATEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_UPDATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_UPDATEINFO_LEGACY {
    pub ProductID: ::windows::core::GUID,
    pub PackagePath: super::super::Foundation::BSTR,
    pub InstanceID: ::windows::core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_UPDATEINFO_LEGACY {
    fn clone(&self) -> Self {
        Self {
            ProductID: self.ProductID,
            PackagePath: self.PackagePath.clone(),
            InstanceID: self.InstanceID,
            pbLicense: self.pbLicense,
            cbLicense: self.cbLicense,
            MarketplaceAppVersion: self.MarketplaceAppVersion.clone(),
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_UPDATEINFO_LEGACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_UPDATEINFO_LEGACY").field("ProductID", &self.ProductID).field("PackagePath", &self.PackagePath).field("InstanceID", &self.InstanceID).field("pbLicense", &self.pbLicense).field("cbLicense", &self.cbLicense).field("MarketplaceAppVersion", &self.MarketplaceAppVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PM_UPDATEINFO_LEGACY {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_UPDATEINFO_LEGACY {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.PackagePath == other.PackagePath && self.InstanceID == other.InstanceID && self.pbLicense == other.pbLicense && self.cbLicense == other.cbLicense && self.MarketplaceAppVersion == other.MarketplaceAppVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_UPDATEINFO_LEGACY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_UPDATEINFO_LEGACY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_PROGRESS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: *mut ::core::ffi::c_void, currentposition: u32, maximumposition: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_SYMLOAD_CALLBACK = ::core::option::Option<unsafe extern "system" fn(whichfile: u32, symbolfilename: ::windows::core::PCSTR, symtype: u32, symbolfilechecksum: u32, symbolfiletimedate: u32, imagefilechecksum: u32, imagefiletimedate: u32, callbackcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PROTECTED_FILE_DATA {
    pub FileName: [u16; 260],
    pub FileNumber: u32,
}
impl ::core::marker::Copy for PROTECTED_FILE_DATA {}
impl ::core::clone::Clone for PROTECTED_FILE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROTECTED_FILE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTECTED_FILE_DATA").field("FileName", &self.FileName).field("FileNumber", &self.FileNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for PROTECTED_FILE_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROTECTED_FILE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROTECTED_FILE_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROTECTED_FILE_DATA {}
impl ::core::default::Default for PROTECTED_FILE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QUERYASMINFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const QUERYASMINFO_FLAG_VALIDATE: QUERYASMINFO_FLAGS = QUERYASMINFO_FLAGS(1u32);
impl ::core::marker::Copy for QUERYASMINFO_FLAGS {}
impl ::core::clone::Clone for QUERYASMINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUERYASMINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QUERYASMINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for QUERYASMINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUERYASMINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUERYASMINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUERYASMINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryActCtxSettingsW<'a, P0, P1, P2>(dwflags: u32, hactctx: P0, settingsnamespace: P1, settingname: P2, pvbuffer: ::windows::core::PWSTR, dwbuffer: usize, pdwwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryActCtxSettingsW(dwflags: u32, hactctx: super::super::Foundation::HANDLE, settingsnamespace: ::windows::core::PCWSTR, settingname: ::windows::core::PCWSTR, pvbuffer: ::windows::core::PWSTR, dwbuffer: usize, pdwwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL;
    }
    QueryActCtxSettingsW(dwflags, hactctx.into(), settingsnamespace.into(), settingname.into(), ::core::mem::transmute(pvbuffer), dwbuffer, ::core::mem::transmute(pdwwrittenorrequired))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryActCtxW<'a, P0>(dwflags: u32, hactctx: P0, pvsubinstance: *const ::core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryActCtxW(dwflags: u32, hactctx: super::super::Foundation::HANDLE, pvsubinstance: *const ::core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL;
    }
    QueryActCtxW(dwflags, hactctx.into(), ::core::mem::transmute(pvsubinstance), ulinfoclass, ::core::mem::transmute(pvbuffer), cbbuffer, ::core::mem::transmute(pcbwrittenorrequired))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REINSTALLMODE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_REPAIR: REINSTALLMODE = REINSTALLMODE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEMISSING: REINSTALLMODE = REINSTALLMODE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEOLDERVERSION: REINSTALLMODE = REINSTALLMODE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEEQUALVERSION: REINSTALLMODE = REINSTALLMODE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEEXACT: REINSTALLMODE = REINSTALLMODE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEVERIFY: REINSTALLMODE = REINSTALLMODE(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEREPLACE: REINSTALLMODE = REINSTALLMODE(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_MACHINEDATA: REINSTALLMODE = REINSTALLMODE(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_USERDATA: REINSTALLMODE = REINSTALLMODE(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_SHORTCUT: REINSTALLMODE = REINSTALLMODE(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_PACKAGE: REINSTALLMODE = REINSTALLMODE(1024i32);
impl ::core::marker::Copy for REINSTALLMODE {}
impl ::core::clone::Clone for REINSTALLMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REINSTALLMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REINSTALLMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for REINSTALLMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REINSTALLMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RESULTTYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieUnknown: RESULTTYPES = RESULTTYPES(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieError: RESULTTYPES = RESULTTYPES(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieWarning: RESULTTYPES = RESULTTYPES(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieInfo: RESULTTYPES = RESULTTYPES(3i32);
impl ::core::marker::Copy for RESULTTYPES {}
impl ::core::clone::Clone for RESULTTYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RESULTTYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RESULTTYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RESULTTYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESULTTYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseActCtx<'a, P0>(hactctx: P0)
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReleaseActCtx(hactctx: super::super::Foundation::HANDLE);
    }
    ReleaseActCtx(hactctx.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPTFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_CACHEINFO: SCRIPTFLAGS = SCRIPTFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_SHORTCUTS: SCRIPTFLAGS = SCRIPTFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_MACHINEASSIGN: SCRIPTFLAGS = SCRIPTFLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_CNFGINFO: SCRIPTFLAGS = SCRIPTFLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_VALIDATE_TRANSFORMS_LIST: SCRIPTFLAGS = SCRIPTFLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_CLASSINFO: SCRIPTFLAGS = SCRIPTFLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_EXTENSIONINFO: SCRIPTFLAGS = SCRIPTFLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_APPINFO: SCRIPTFLAGS = SCRIPTFLAGS(384i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA: SCRIPTFLAGS = SCRIPTFLAGS(416i32);
impl ::core::marker::Copy for SCRIPTFLAGS {}
impl ::core::clone::Clone for SCRIPTFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPTFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCRIPTFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCRIPTFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_ASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_NOPOPUPS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_ONCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_SETUP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_IDLE_TRIGGER: &str = "WFP_IDLE_TRIGGER";
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_QUOTA_DEFAULT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_ALWAYS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_IMMEDIATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_ONCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STATUSTYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusGetCUB: STATUSTYPES = STATUSTYPES(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusICECount: STATUSTYPES = STATUSTYPES(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusMerge: STATUSTYPES = STATUSTYPES(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusSummaryInfo: STATUSTYPES = STATUSTYPES(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusCreateEngine: STATUSTYPES = STATUSTYPES(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusStarting: STATUSTYPES = STATUSTYPES(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusRunICE: STATUSTYPES = STATUSTYPES(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusShutdown: STATUSTYPES = STATUSTYPES(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusSuccess: STATUSTYPES = STATUSTYPES(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusFail: STATUSTYPES = STATUSTYPES(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusCancel: STATUSTYPES = STATUSTYPES(10i32);
impl ::core::marker::Copy for STATUSTYPES {}
impl ::core::clone::Clone for STATUSTYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STATUSTYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STATUSTYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for STATUSTYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STATUSTYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_COMPLIB_MANIFEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_COMPLIB_MODULE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_WIN32_MANIFEST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_WIN32_MODULE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SfcGetNextProtectedFile<'a, P0>(rpchandle: P0, protfiledata: *mut PROTECTED_FILE_DATA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SfcGetNextProtectedFile(rpchandle: super::super::Foundation::HANDLE, protfiledata: *mut PROTECTED_FILE_DATA) -> super::super::Foundation::BOOL;
    }
    SfcGetNextProtectedFile(rpchandle.into(), ::core::mem::transmute(protfiledata))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SfcIsFileProtected<'a, P0, P1>(rpchandle: P0, protfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SfcIsFileProtected(rpchandle: super::super::Foundation::HANDLE, protfilename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    SfcIsFileProtected(rpchandle.into(), protfilename.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn SfcIsKeyProtected<'a, P0, P1>(keyhandle: P0, subkeyname: P1, keysam: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::Registry::HKEY>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SfcIsKeyProtected(keyhandle: super::Registry::HKEY, subkeyname: ::windows::core::PCWSTR, keysam: u32) -> super::super::Foundation::BOOL;
    }
    SfcIsKeyProtected(keyhandle.into(), subkeyname.into(), keysam)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SfpVerifyFile<'a, P0>(pszfilename: P0, pszerror: &[u8]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SfpVerifyFile(pszfilename: ::windows::core::PCSTR, pszerror: ::windows::core::PCSTR, dwerrsize: u32) -> super::super::Foundation::BOOL;
    }
    SfpVerifyFile(pszfilename.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pszerror)), pszerror.len() as _)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TILE_TEMPLATE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_INVALID: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_FLIP: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_DEEPLINK: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_CYCLE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_METROCOUNT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_AGILESTORE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_GAMES: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_CALENDAR: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_MUSICVIDEO: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEOPLE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_CONTACT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_GROUP: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_DEFAULT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BADGE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BLOCK: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(21i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(22i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(23i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT07: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(24i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT08: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(25i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT09: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(26i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT10: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(27i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT11: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(28i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(29i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGECOLLECTION: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(30i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(31i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BLOCKANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(33i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BLOCKANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(34i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(35i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(36i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(37i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(38i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(39i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(40i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(41i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(42i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(43i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(44i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(45i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(46i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(47i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(48i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(49i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(50i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(51i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(52i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(53i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(54i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(55i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_METROCOUNTQUEUE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(56i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SEARCH: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(57i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TILEFLYOUT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(58i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_FOLDER: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(59i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_ALL: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(100i32);
impl ::core::marker::Copy for TILE_TEMPLATE_TYPE {}
impl ::core::clone::Clone for TILE_TEMPLATE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TILE_TEMPLATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TILE_TEMPLATE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TILE_TEMPLATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TILE_TEMPLATE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_BACKUP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_CMI: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_COPYFILES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEPTH_DECR: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEPTH_INCR: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DETAILS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEVINST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEVMGR: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DRIVER_STORE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DRVSETUP: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_FILEQ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_FLUSH_FILE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_INF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_INFDB: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_INSTALLER: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_NEWDEV: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_POLICY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_RESERVED_FLAGS: u32 = 65520u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUP: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUPAPI_BITS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUPAPI_CMDLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUPAPI_DEVLOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SIGVERIF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SUMMARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SYSTEM_STATE_CHANGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_TAB_1: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_TIMESTAMP: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_UI: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_UMPNPMGR: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_UTIL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_VENDOR: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_VERBOSE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_VERY_VERBOSE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_WARNING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileA<'a, P0, P1>(patchfilename: P0, oldfilename: P1, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TestApplyPatchToFileA(patchfilename: ::windows::core::PCSTR, oldfilename: ::windows::core::PCSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    }
    TestApplyPatchToFileA(patchfilename.into(), oldfilename.into(), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileByBuffers(patchfilebuffer: *const u8, patchfilesize: u32, oldfilebuffer: *const u8, oldfilesize: u32, newfilesize: *mut u32, applyoptionflags: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TestApplyPatchToFileByBuffers(patchfilebuffer: *const u8, patchfilesize: u32, oldfilebuffer: *const u8, oldfilesize: u32, newfilesize: *mut u32, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    }
    TestApplyPatchToFileByBuffers(::core::mem::transmute(patchfilebuffer), patchfilesize, ::core::mem::transmute(oldfilebuffer), oldfilesize, ::core::mem::transmute(newfilesize), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileByHandles<'a, P0, P1>(patchfilehandle: P0, oldfilehandle: P1, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TestApplyPatchToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    }
    TestApplyPatchToFileByHandles(patchfilehandle.into(), oldfilehandle.into(), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileW<'a, P0, P1>(patchfilename: P0, oldfilename: P1, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TestApplyPatchToFileW(patchfilename: ::windows::core::PCWSTR, oldfilename: ::windows::core::PCWSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    }
    TestApplyPatchToFileW(patchfilename.into(), oldfilename.into(), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const UIALL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const UILOGBITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const UINONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USERINFOSTATE(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_MOREDATA: USERINFOSTATE = USERINFOSTATE(-3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_INVALIDARG: USERINFOSTATE = USERINFOSTATE(-2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_UNKNOWN: USERINFOSTATE = USERINFOSTATE(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_ABSENT: USERINFOSTATE = USERINFOSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_PRESENT: USERINFOSTATE = USERINFOSTATE(1i32);
impl ::core::marker::Copy for USERINFOSTATE {}
impl ::core::clone::Clone for USERINFOSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USERINFOSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for USERINFOSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for USERINFOSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USERINFOSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_BAD_MAJOR_VERSION: u32 = 3222294792u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_BASE: u32 = 3222294785u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_EQUAL_FILE_VERSION: u32 = 3222294794u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_FILE_VERSION_DOWNREV: u32 = 3222294793u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_IMPROPER_TRANSFORM_VALIDATION: u32 = 3222294788u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_INVALID_TRANSFORM_VALIDATION: u32 = 3222294791u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_MAJOR_UPGRADE_PATCH: u32 = 3222294785u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_OBSOLETION_WITH_MSI30: u32 = 3222294801u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222294803u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222294802u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_PATCHPROPERTYNOTSET: u32 = 3222294795u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222294789u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222294790u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_SEQUENCE_DATA_GENERATION_DISABLED: u32 = 3222294786u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_SEQUENCE_DATA_SUPERSEDENCE_IGNORED: u32 = 3222294787u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ZombifyActCtx<'a, P0>(hactctx: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ZombifyActCtx(hactctx: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    ZombifyActCtx(hactctx.into())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const _WIN32_MSI: u32 = 500u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const _WIN32_MSM: u32 = 100u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct _tagAPPTASKTYPE {
    pub ProductID: ::windows::core::GUID,
    pub TaskType: PM_TASK_TYPE,
}
impl ::core::marker::Copy for _tagAPPTASKTYPE {}
impl ::core::clone::Clone for _tagAPPTASKTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _tagAPPTASKTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_tagAPPTASKTYPE").field("ProductID", &self.ProductID).field("TaskType", &self.TaskType).finish()
    }
}
unsafe impl ::windows::core::Abi for _tagAPPTASKTYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _tagAPPTASKTYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_tagAPPTASKTYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for _tagAPPTASKTYPE {}
impl ::core::default::Default for _tagAPPTASKTYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const cchMaxInteger: i32 = 12i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbAssemblyAttributes(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbAssemblyAttributesURT: msidbAssemblyAttributes = msidbAssemblyAttributes(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbAssemblyAttributesWin32: msidbAssemblyAttributes = msidbAssemblyAttributes(1i32);
impl ::core::marker::Copy for msidbAssemblyAttributes {}
impl ::core::clone::Clone for msidbAssemblyAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbAssemblyAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbAssemblyAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbAssemblyAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbAssemblyAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbClassAttributes(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbClassAttributesRelativePath: msidbClassAttributes = msidbClassAttributes(1i32);
impl ::core::marker::Copy for msidbClassAttributes {}
impl ::core::clone::Clone for msidbClassAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbClassAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbClassAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbClassAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbClassAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbComponentAttributes(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesLocalOnly: msidbComponentAttributes = msidbComponentAttributes(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesSourceOnly: msidbComponentAttributes = msidbComponentAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesOptional: msidbComponentAttributes = msidbComponentAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesRegistryKeyPath: msidbComponentAttributes = msidbComponentAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesSharedDllRefCount: msidbComponentAttributes = msidbComponentAttributes(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesPermanent: msidbComponentAttributes = msidbComponentAttributes(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesODBCDataSource: msidbComponentAttributes = msidbComponentAttributes(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesTransitive: msidbComponentAttributes = msidbComponentAttributes(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesNeverOverwrite: msidbComponentAttributes = msidbComponentAttributes(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributes64bit: msidbComponentAttributes = msidbComponentAttributes(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesDisableRegistryReflection: msidbComponentAttributes = msidbComponentAttributes(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesUninstallOnSupersedence: msidbComponentAttributes = msidbComponentAttributes(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesShared: msidbComponentAttributes = msidbComponentAttributes(2048i32);
impl ::core::marker::Copy for msidbComponentAttributes {}
impl ::core::clone::Clone for msidbComponentAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbComponentAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbComponentAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbComponentAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbComponentAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbControlAttributes(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesVisible: msidbControlAttributes = msidbControlAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesEnabled: msidbControlAttributes = msidbControlAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesSunken: msidbControlAttributes = msidbControlAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIndirect: msidbControlAttributes = msidbControlAttributes(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesInteger: msidbControlAttributes = msidbControlAttributes(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRTLRO: msidbControlAttributes = msidbControlAttributes(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRightAligned: msidbControlAttributes = msidbControlAttributes(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesLeftScroll: msidbControlAttributes = msidbControlAttributes(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesBiDi: msidbControlAttributes = msidbControlAttributes(224i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesTransparent: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesNoPrefix: msidbControlAttributes = msidbControlAttributes(131072i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesNoWrap: msidbControlAttributes = msidbControlAttributes(262144i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFormatSize: msidbControlAttributes = msidbControlAttributes(524288i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesUsersLanguage: msidbControlAttributes = msidbControlAttributes(1048576i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesMultiline: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesPasswordInput: msidbControlAttributes = msidbControlAttributes(2097152i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesProgress95: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRemovableVolume: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFixedVolume: msidbControlAttributes = msidbControlAttributes(131072i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRemoteVolume: msidbControlAttributes = msidbControlAttributes(262144i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesCDROMVolume: msidbControlAttributes = msidbControlAttributes(524288i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRAMDiskVolume: msidbControlAttributes = msidbControlAttributes(1048576i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFloppyVolume: msidbControlAttributes = msidbControlAttributes(2097152i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlShowRollbackCost: msidbControlAttributes = msidbControlAttributes(4194304i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesSorted: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesComboList: msidbControlAttributes = msidbControlAttributes(131072i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesImageHandle: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesPushLike: msidbControlAttributes = msidbControlAttributes(131072i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesBitmap: msidbControlAttributes = msidbControlAttributes(262144i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIcon: msidbControlAttributes = msidbControlAttributes(524288i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFixedSize: msidbControlAttributes = msidbControlAttributes(1048576i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIconSize16: msidbControlAttributes = msidbControlAttributes(2097152i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIconSize32: msidbControlAttributes = msidbControlAttributes(4194304i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIconSize48: msidbControlAttributes = msidbControlAttributes(6291456i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesElevationShield: msidbControlAttributes = msidbControlAttributes(8388608i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesHasBorder: msidbControlAttributes = msidbControlAttributes(16777216i32);
impl ::core::marker::Copy for msidbControlAttributes {}
impl ::core::clone::Clone for msidbControlAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbControlAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbControlAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbControlAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbControlAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbCustomActionType(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeDll: msidbCustomActionType = msidbCustomActionType(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeExe: msidbCustomActionType = msidbCustomActionType(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeTextData: msidbCustomActionType = msidbCustomActionType(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeJScript: msidbCustomActionType = msidbCustomActionType(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeVBScript: msidbCustomActionType = msidbCustomActionType(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeInstall: msidbCustomActionType = msidbCustomActionType(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeBinaryData: msidbCustomActionType = msidbCustomActionType(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeSourceFile: msidbCustomActionType = msidbCustomActionType(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeDirectory: msidbCustomActionType = msidbCustomActionType(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeProperty: msidbCustomActionType = msidbCustomActionType(48i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeContinue: msidbCustomActionType = msidbCustomActionType(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeAsync: msidbCustomActionType = msidbCustomActionType(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeFirstSequence: msidbCustomActionType = msidbCustomActionType(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeOncePerProcess: msidbCustomActionType = msidbCustomActionType(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeClientRepeat: msidbCustomActionType = msidbCustomActionType(768i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeInScript: msidbCustomActionType = msidbCustomActionType(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeRollback: msidbCustomActionType = msidbCustomActionType(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeCommit: msidbCustomActionType = msidbCustomActionType(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeNoImpersonate: msidbCustomActionType = msidbCustomActionType(2048i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeTSAware: msidbCustomActionType = msidbCustomActionType(16384i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionType64BitScript: msidbCustomActionType = msidbCustomActionType(4096i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeHideTarget: msidbCustomActionType = msidbCustomActionType(8192i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypePatchUninstall: msidbCustomActionType = msidbCustomActionType(32768i32);
impl ::core::marker::Copy for msidbCustomActionType {}
impl ::core::clone::Clone for msidbCustomActionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbCustomActionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbCustomActionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbCustomActionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbCustomActionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbDialogAttributes(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesVisible: msidbDialogAttributes = msidbDialogAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesModal: msidbDialogAttributes = msidbDialogAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesMinimize: msidbDialogAttributes = msidbDialogAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesSysModal: msidbDialogAttributes = msidbDialogAttributes(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesKeepModeless: msidbDialogAttributes = msidbDialogAttributes(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesTrackDiskSpace: msidbDialogAttributes = msidbDialogAttributes(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesUseCustomPalette: msidbDialogAttributes = msidbDialogAttributes(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesRTLRO: msidbDialogAttributes = msidbDialogAttributes(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesRightAligned: msidbDialogAttributes = msidbDialogAttributes(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesLeftScroll: msidbDialogAttributes = msidbDialogAttributes(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesBiDi: msidbDialogAttributes = msidbDialogAttributes(896i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesError: msidbDialogAttributes = msidbDialogAttributes(65536i32);
impl ::core::marker::Copy for msidbDialogAttributes {}
impl ::core::clone::Clone for msidbDialogAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbDialogAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbDialogAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbDialogAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbDialogAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbEmbeddedUIAttributes(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbEmbeddedUI: msidbEmbeddedUIAttributes = msidbEmbeddedUIAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbEmbeddedHandlesBasic: msidbEmbeddedUIAttributes = msidbEmbeddedUIAttributes(2i32);
impl ::core::marker::Copy for msidbEmbeddedUIAttributes {}
impl ::core::clone::Clone for msidbEmbeddedUIAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbEmbeddedUIAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbEmbeddedUIAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbEmbeddedUIAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbEmbeddedUIAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbFeatureAttributes(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFavorLocal: msidbFeatureAttributes = msidbFeatureAttributes(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFavorSource: msidbFeatureAttributes = msidbFeatureAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFollowParent: msidbFeatureAttributes = msidbFeatureAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFavorAdvertise: msidbFeatureAttributes = msidbFeatureAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesDisallowAdvertise: msidbFeatureAttributes = msidbFeatureAttributes(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesUIDisallowAbsent: msidbFeatureAttributes = msidbFeatureAttributes(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesNoUnsupportedAdvertise: msidbFeatureAttributes = msidbFeatureAttributes(32i32);
impl ::core::marker::Copy for msidbFeatureAttributes {}
impl ::core::clone::Clone for msidbFeatureAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbFeatureAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbFeatureAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbFeatureAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbFeatureAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbFileAttributes(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReadOnly: msidbFileAttributes = msidbFileAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesHidden: msidbFileAttributes = msidbFileAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesSystem: msidbFileAttributes = msidbFileAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved0: msidbFileAttributes = msidbFileAttributes(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesIsolatedComp: msidbFileAttributes = msidbFileAttributes(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved1: msidbFileAttributes = msidbFileAttributes(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved2: msidbFileAttributes = msidbFileAttributes(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved3: msidbFileAttributes = msidbFileAttributes(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesVital: msidbFileAttributes = msidbFileAttributes(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesChecksum: msidbFileAttributes = msidbFileAttributes(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesPatchAdded: msidbFileAttributes = msidbFileAttributes(4096i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesNoncompressed: msidbFileAttributes = msidbFileAttributes(8192i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesCompressed: msidbFileAttributes = msidbFileAttributes(16384i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved4: msidbFileAttributes = msidbFileAttributes(32768i32);
impl ::core::marker::Copy for msidbFileAttributes {}
impl ::core::clone::Clone for msidbFileAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbFileAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbFileAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbFileAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbFileAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbIniFileAction(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionAddLine: msidbIniFileAction = msidbIniFileAction(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionCreateLine: msidbIniFileAction = msidbIniFileAction(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionRemoveLine: msidbIniFileAction = msidbIniFileAction(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionAddTag: msidbIniFileAction = msidbIniFileAction(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionRemoveTag: msidbIniFileAction = msidbIniFileAction(4i32);
impl ::core::marker::Copy for msidbIniFileAction {}
impl ::core::clone::Clone for msidbIniFileAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbIniFileAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbIniFileAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbIniFileAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbIniFileAction").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbLocatorType(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorTypeDirectory: msidbLocatorType = msidbLocatorType(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorTypeFileName: msidbLocatorType = msidbLocatorType(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorTypeRawValue: msidbLocatorType = msidbLocatorType(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorType64bit: msidbLocatorType = msidbLocatorType(16i32);
impl ::core::marker::Copy for msidbLocatorType {}
impl ::core::clone::Clone for msidbLocatorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbLocatorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbLocatorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbLocatorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbLocatorType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbMoveFileOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbMoveFileOptionsMove: msidbMoveFileOptions = msidbMoveFileOptions(1i32);
impl ::core::marker::Copy for msidbMoveFileOptions {}
impl ::core::clone::Clone for msidbMoveFileOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbMoveFileOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbMoveFileOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbMoveFileOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbMoveFileOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbODBCDataSourceRegistration(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbODBCDataSourceRegistrationPerMachine: msidbODBCDataSourceRegistration = msidbODBCDataSourceRegistration(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbODBCDataSourceRegistrationPerUser: msidbODBCDataSourceRegistration = msidbODBCDataSourceRegistration(1i32);
impl ::core::marker::Copy for msidbODBCDataSourceRegistration {}
impl ::core::clone::Clone for msidbODBCDataSourceRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbODBCDataSourceRegistration {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbODBCDataSourceRegistration {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbODBCDataSourceRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbODBCDataSourceRegistration").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbPatchAttributes(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbPatchAttributesNonVital: msidbPatchAttributes = msidbPatchAttributes(1i32);
impl ::core::marker::Copy for msidbPatchAttributes {}
impl ::core::clone::Clone for msidbPatchAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbPatchAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbPatchAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbPatchAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbPatchAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbRegistryRoot(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootClassesRoot: msidbRegistryRoot = msidbRegistryRoot(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootCurrentUser: msidbRegistryRoot = msidbRegistryRoot(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootLocalMachine: msidbRegistryRoot = msidbRegistryRoot(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootUsers: msidbRegistryRoot = msidbRegistryRoot(3i32);
impl ::core::marker::Copy for msidbRegistryRoot {}
impl ::core::clone::Clone for msidbRegistryRoot {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbRegistryRoot {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbRegistryRoot {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbRegistryRoot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbRegistryRoot").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbRemoveFileInstallMode(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRemoveFileInstallModeOnInstall: msidbRemoveFileInstallMode = msidbRemoveFileInstallMode(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRemoveFileInstallModeOnRemove: msidbRemoveFileInstallMode = msidbRemoveFileInstallMode(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRemoveFileInstallModeOnBoth: msidbRemoveFileInstallMode = msidbRemoveFileInstallMode(3i32);
impl ::core::marker::Copy for msidbRemoveFileInstallMode {}
impl ::core::clone::Clone for msidbRemoveFileInstallMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbRemoveFileInstallMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbRemoveFileInstallMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbRemoveFileInstallMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbRemoveFileInstallMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbServiceConfigEvent(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceConfigEventInstall: msidbServiceConfigEvent = msidbServiceConfigEvent(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceConfigEventUninstall: msidbServiceConfigEvent = msidbServiceConfigEvent(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceConfigEventReinstall: msidbServiceConfigEvent = msidbServiceConfigEvent(4i32);
impl ::core::marker::Copy for msidbServiceConfigEvent {}
impl ::core::clone::Clone for msidbServiceConfigEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbServiceConfigEvent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbServiceConfigEvent {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbServiceConfigEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbServiceConfigEvent").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbServiceControlEvent(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventStart: msidbServiceControlEvent = msidbServiceControlEvent(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventStop: msidbServiceControlEvent = msidbServiceControlEvent(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventDelete: msidbServiceControlEvent = msidbServiceControlEvent(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventUninstallStart: msidbServiceControlEvent = msidbServiceControlEvent(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventUninstallStop: msidbServiceControlEvent = msidbServiceControlEvent(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventUninstallDelete: msidbServiceControlEvent = msidbServiceControlEvent(128i32);
impl ::core::marker::Copy for msidbServiceControlEvent {}
impl ::core::clone::Clone for msidbServiceControlEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbServiceControlEvent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbServiceControlEvent {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbServiceControlEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbServiceControlEvent").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbServiceInstallErrorControl(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceInstallErrorControlVital: msidbServiceInstallErrorControl = msidbServiceInstallErrorControl(32768i32);
impl ::core::marker::Copy for msidbServiceInstallErrorControl {}
impl ::core::clone::Clone for msidbServiceInstallErrorControl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbServiceInstallErrorControl {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbServiceInstallErrorControl {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbServiceInstallErrorControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbServiceInstallErrorControl").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbSumInfoSourceType(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeSFN: msidbSumInfoSourceType = msidbSumInfoSourceType(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeCompressed: msidbSumInfoSourceType = msidbSumInfoSourceType(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeAdminImage: msidbSumInfoSourceType = msidbSumInfoSourceType(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeLUAPackage: msidbSumInfoSourceType = msidbSumInfoSourceType(8i32);
impl ::core::marker::Copy for msidbSumInfoSourceType {}
impl ::core::clone::Clone for msidbSumInfoSourceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbSumInfoSourceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbSumInfoSourceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbSumInfoSourceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbSumInfoSourceType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbTextStyleStyleBits(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsBold: msidbTextStyleStyleBits = msidbTextStyleStyleBits(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsItalic: msidbTextStyleStyleBits = msidbTextStyleStyleBits(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsUnderline: msidbTextStyleStyleBits = msidbTextStyleStyleBits(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsStrike: msidbTextStyleStyleBits = msidbTextStyleStyleBits(8i32);
impl ::core::marker::Copy for msidbTextStyleStyleBits {}
impl ::core::clone::Clone for msidbTextStyleStyleBits {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbTextStyleStyleBits {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbTextStyleStyleBits {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbTextStyleStyleBits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbTextStyleStyleBits").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbUpgradeAttributes(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesMigrateFeatures: msidbUpgradeAttributes = msidbUpgradeAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesOnlyDetect: msidbUpgradeAttributes = msidbUpgradeAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesIgnoreRemoveFailure: msidbUpgradeAttributes = msidbUpgradeAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesVersionMinInclusive: msidbUpgradeAttributes = msidbUpgradeAttributes(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesVersionMaxInclusive: msidbUpgradeAttributes = msidbUpgradeAttributes(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesLanguagesExclusive: msidbUpgradeAttributes = msidbUpgradeAttributes(1024i32);
impl ::core::marker::Copy for msidbUpgradeAttributes {}
impl ::core::clone::Clone for msidbUpgradeAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbUpgradeAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msidbUpgradeAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for msidbUpgradeAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbUpgradeAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msifiFastInstallBits(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msifiFastInstallNoSR: msifiFastInstallBits = msifiFastInstallBits(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msifiFastInstallQuickCosting: msifiFastInstallBits = msifiFastInstallBits(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msifiFastInstallLessPrgMsg: msifiFastInstallBits = msifiFastInstallBits(4i32);
impl ::core::marker::Copy for msifiFastInstallBits {}
impl ::core::clone::Clone for msifiFastInstallBits {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msifiFastInstallBits {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msifiFastInstallBits {
    type Abi = Self;
}
impl ::core::fmt::Debug for msifiFastInstallBits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msifiFastInstallBits").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msirbRebootReason(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootUndeterminedReason: msirbRebootReason = msirbRebootReason(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootInUseFilesReason: msirbRebootReason = msirbRebootReason(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootScheduleRebootReason: msirbRebootReason = msirbRebootReason(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootForceRebootReason: msirbRebootReason = msirbRebootReason(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootCustomActionReason: msirbRebootReason = msirbRebootReason(4i32);
impl ::core::marker::Copy for msirbRebootReason {}
impl ::core::clone::Clone for msirbRebootReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msirbRebootReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msirbRebootReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for msirbRebootReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msirbRebootReason").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msirbRebootType(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootImmediate: msirbRebootType = msirbRebootType(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootDeferred: msirbRebootType = msirbRebootType(2i32);
impl ::core::marker::Copy for msirbRebootType {}
impl ::core::clone::Clone for msirbRebootType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msirbRebootType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msirbRebootType {
    type Abi = Self;
}
impl ::core::fmt::Debug for msirbRebootType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msirbRebootType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msmErrorType(pub i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorLanguageUnsupported: msmErrorType = msmErrorType(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorLanguageFailed: msmErrorType = msmErrorType(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorExclusion: msmErrorType = msmErrorType(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorTableMerge: msmErrorType = msmErrorType(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorResequenceMerge: msmErrorType = msmErrorType(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorFileCreate: msmErrorType = msmErrorType(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorDirCreate: msmErrorType = msmErrorType(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorFeatureRequired: msmErrorType = msmErrorType(8i32);
impl ::core::marker::Copy for msmErrorType {}
impl ::core::clone::Clone for msmErrorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msmErrorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for msmErrorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for msmErrorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msmErrorType").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
