#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTXA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: super::super::Foundation::PSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: super::super::Foundation::PSTR,
    pub lpResourceName: super::super::Foundation::PSTR,
    pub lpApplicationName: super::super::Foundation::PSTR,
    pub hModule: super::super::Foundation::HINSTANCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ACTCTXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACTCTXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ACTCTXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACTCTXA")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("lpSource", &self.lpSource)
            .field("wProcessorArchitecture", &self.wProcessorArchitecture)
            .field("wLangId", &self.wLangId)
            .field("lpAssemblyDirectory", &self.lpAssemblyDirectory)
            .field("lpResourceName", &self.lpResourceName)
            .field("lpApplicationName", &self.lpApplicationName)
            .field("hModule", &self.hModule)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACTCTXA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwFlags == other.dwFlags
            && self.lpSource == other.lpSource
            && self.wProcessorArchitecture == other.wProcessorArchitecture
            && self.wLangId == other.wLangId
            && self.lpAssemblyDirectory == other.lpAssemblyDirectory
            && self.lpResourceName == other.lpResourceName
            && self.lpApplicationName == other.lpApplicationName
            && self.hModule == other.hModule
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACTCTXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACTCTXA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTXW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: super::super::Foundation::PWSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: super::super::Foundation::PWSTR,
    pub lpResourceName: super::super::Foundation::PWSTR,
    pub lpApplicationName: super::super::Foundation::PWSTR,
    pub hModule: super::super::Foundation::HINSTANCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ACTCTXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACTCTXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ACTCTXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACTCTXW")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("lpSource", &self.lpSource)
            .field("wProcessorArchitecture", &self.wProcessorArchitecture)
            .field("wLangId", &self.wLangId)
            .field("lpAssemblyDirectory", &self.lpAssemblyDirectory)
            .field("lpResourceName", &self.lpResourceName)
            .field("lpApplicationName", &self.lpApplicationName)
            .field("hModule", &self.hModule)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACTCTXW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwFlags == other.dwFlags
            && self.lpSource == other.lpSource
            && self.wProcessorArchitecture == other.wProcessorArchitecture
            && self.wLangId == other.wLangId
            && self.lpAssemblyDirectory == other.lpAssemblyDirectory
            && self.lpResourceName == other.lpResourceName
            && self.lpApplicationName == other.lpApplicationName
            && self.hModule == other.hModule
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACTCTXW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACTCTXW {
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
pub struct ACTCTX_COMPATIBILITY_ELEMENT_TYPE(pub i32);
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN: ACTCTX_COMPATIBILITY_ELEMENT_TYPE =
    ACTCTX_COMPATIBILITY_ELEMENT_TYPE(0i32);
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS: ACTCTX_COMPATIBILITY_ELEMENT_TYPE =
    ACTCTX_COMPATIBILITY_ELEMENT_TYPE(1i32);
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION: ACTCTX_COMPATIBILITY_ELEMENT_TYPE =
    ACTCTX_COMPATIBILITY_ELEMENT_TYPE(2i32);
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MAXVERSIONTESTED: ACTCTX_COMPATIBILITY_ELEMENT_TYPE =
    ACTCTX_COMPATIBILITY_ELEMENT_TYPE(3i32);
impl ::std::convert::From<i32> for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {
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
pub struct ACTCTX_REQUESTED_RUN_LEVEL(pub i32);
pub const ACTCTX_RUN_LEVEL_UNSPECIFIED: ACTCTX_REQUESTED_RUN_LEVEL =
    ACTCTX_REQUESTED_RUN_LEVEL(0i32);
pub const ACTCTX_RUN_LEVEL_AS_INVOKER: ACTCTX_REQUESTED_RUN_LEVEL =
    ACTCTX_REQUESTED_RUN_LEVEL(1i32);
pub const ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE: ACTCTX_REQUESTED_RUN_LEVEL =
    ACTCTX_REQUESTED_RUN_LEVEL(2i32);
pub const ACTCTX_RUN_LEVEL_REQUIRE_ADMIN: ACTCTX_REQUESTED_RUN_LEVEL =
    ACTCTX_REQUESTED_RUN_LEVEL(3i32);
pub const ACTCTX_RUN_LEVEL_NUMBERS: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(4i32);
impl ::std::convert::From<i32> for ACTCTX_REQUESTED_RUN_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACTCTX_REQUESTED_RUN_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_WindowsProgramming"
))]
pub struct ACTCTX_SECTION_KEYED_DATA {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut ::std::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut ::std::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut ::std::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
    pub ulFlags: u32,
    pub AssemblyMetadata: super::WindowsProgramming::ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA,
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_WindowsProgramming"
))]
impl ACTCTX_SECTION_KEYED_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::default::Default for ACTCTX_SECTION_KEYED_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::fmt::Debug for ACTCTX_SECTION_KEYED_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACTCTX_SECTION_KEYED_DATA")
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
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.ulDataFormatVersion == other.ulDataFormatVersion
            && self.lpData == other.lpData
            && self.ulLength == other.ulLength
            && self.lpSectionGlobalData == other.lpSectionGlobalData
            && self.ulSectionGlobalDataLength == other.ulSectionGlobalDataLength
            && self.lpSectionBase == other.lpSectionBase
            && self.ulSectionTotalLength == other.ulSectionTotalLength
            && self.hActCtx == other.hActCtx
            && self.ulAssemblyRosterIndex == other.ulAssemblyRosterIndex
            && self.ulFlags == other.ulFlags
            && self.AssemblyMetadata == other.AssemblyMetadata
    }
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_WindowsProgramming"
))]
impl ::std::cmp::Eq for ACTCTX_SECTION_KEYED_DATA {}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_WindowsProgramming"
))]
unsafe impl ::windows::runtime::Abi for ACTCTX_SECTION_KEYED_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    pub lpAssemblyEncodedAssemblyIdentity: super::super::Foundation::PWSTR,
    pub lpAssemblyManifestPath: super::super::Foundation::PWSTR,
    pub lpAssemblyPolicyPath: super::super::Foundation::PWSTR,
    pub lpAssemblyDirectoryName: super::super::Foundation::PWSTR,
    pub ulFileCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION")
            .field("ulFlags", &self.ulFlags)
            .field(
                "ulEncodedAssemblyIdentityLength",
                &self.ulEncodedAssemblyIdentityLength,
            )
            .field("ulManifestPathType", &self.ulManifestPathType)
            .field("ulManifestPathLength", &self.ulManifestPathLength)
            .field("liManifestLastWriteTime", &self.liManifestLastWriteTime)
            .field("ulPolicyPathType", &self.ulPolicyPathType)
            .field("ulPolicyPathLength", &self.ulPolicyPathLength)
            .field("liPolicyLastWriteTime", &self.liPolicyLastWriteTime)
            .field(
                "ulMetadataSatelliteRosterIndex",
                &self.ulMetadataSatelliteRosterIndex,
            )
            .field("ulManifestVersionMajor", &self.ulManifestVersionMajor)
            .field("ulManifestVersionMinor", &self.ulManifestVersionMinor)
            .field("ulPolicyVersionMajor", &self.ulPolicyVersionMajor)
            .field("ulPolicyVersionMinor", &self.ulPolicyVersionMinor)
            .field(
                "ulAssemblyDirectoryNameLength",
                &self.ulAssemblyDirectoryNameLength,
            )
            .field(
                "lpAssemblyEncodedAssemblyIdentity",
                &self.lpAssemblyEncodedAssemblyIdentity,
            )
            .field("lpAssemblyManifestPath", &self.lpAssemblyManifestPath)
            .field("lpAssemblyPolicyPath", &self.lpAssemblyPolicyPath)
            .field("lpAssemblyDirectoryName", &self.lpAssemblyDirectoryName)
            .field("ulFileCount", &self.ulFileCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags
            && self.ulEncodedAssemblyIdentityLength == other.ulEncodedAssemblyIdentityLength
            && self.ulManifestPathType == other.ulManifestPathType
            && self.ulManifestPathLength == other.ulManifestPathLength
            && self.liManifestLastWriteTime == other.liManifestLastWriteTime
            && self.ulPolicyPathType == other.ulPolicyPathType
            && self.ulPolicyPathLength == other.ulPolicyPathLength
            && self.liPolicyLastWriteTime == other.liPolicyLastWriteTime
            && self.ulMetadataSatelliteRosterIndex == other.ulMetadataSatelliteRosterIndex
            && self.ulManifestVersionMajor == other.ulManifestVersionMajor
            && self.ulManifestVersionMinor == other.ulManifestVersionMinor
            && self.ulPolicyVersionMajor == other.ulPolicyVersionMajor
            && self.ulPolicyVersionMinor == other.ulPolicyVersionMinor
            && self.ulAssemblyDirectoryNameLength == other.ulAssemblyDirectoryNameLength
            && self.lpAssemblyEncodedAssemblyIdentity == other.lpAssemblyEncodedAssemblyIdentity
            && self.lpAssemblyManifestPath == other.lpAssemblyManifestPath
            && self.lpAssemblyPolicyPath == other.lpAssemblyPolicyPath
            && self.lpAssemblyDirectoryName == other.lpAssemblyDirectoryName
            && self.ulFileCount == other.ulFileCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    pub ElementCount: u32,
    pub Elements: [COMPATIBILITY_CONTEXT_ELEMENT; 1],
}
impl ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {}
impl ::std::default::Default for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION")
            .field("ElementCount", &self.ElementCount)
            .field("Elements", &self.Elements)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ElementCount == other.ElementCount && self.Elements == other.Elements
    }
}
impl ::std::cmp::Eq for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {}
unsafe impl ::windows::runtime::Abi for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    pub lpRootManifestPath: super::super::Foundation::PWSTR,
    pub lpRootConfigurationPath: super::super::Foundation::PWSTR,
    pub lpAppDirPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ACTIVATION_CONTEXT_DETAILED_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACTIVATION_CONTEXT_DETAILED_INFORMATION")
            .field("dwFlags", &self.dwFlags)
            .field("ulFormatVersion", &self.ulFormatVersion)
            .field("ulAssemblyCount", &self.ulAssemblyCount)
            .field("ulRootManifestPathType", &self.ulRootManifestPathType)
            .field("ulRootManifestPathChars", &self.ulRootManifestPathChars)
            .field(
                "ulRootConfigurationPathType",
                &self.ulRootConfigurationPathType,
            )
            .field(
                "ulRootConfigurationPathChars",
                &self.ulRootConfigurationPathChars,
            )
            .field("ulAppDirPathType", &self.ulAppDirPathType)
            .field("ulAppDirPathChars", &self.ulAppDirPathChars)
            .field("lpRootManifestPath", &self.lpRootManifestPath)
            .field("lpRootConfigurationPath", &self.lpRootConfigurationPath)
            .field("lpAppDirPath", &self.lpAppDirPath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.ulFormatVersion == other.ulFormatVersion
            && self.ulAssemblyCount == other.ulAssemblyCount
            && self.ulRootManifestPathType == other.ulRootManifestPathType
            && self.ulRootManifestPathChars == other.ulRootManifestPathChars
            && self.ulRootConfigurationPathType == other.ulRootConfigurationPathType
            && self.ulRootConfigurationPathChars == other.ulRootConfigurationPathChars
            && self.ulAppDirPathType == other.ulAppDirPathType
            && self.ulAppDirPathChars == other.ulAppDirPathChars
            && self.lpRootManifestPath == other.lpRootManifestPath
            && self.lpRootConfigurationPath == other.lpRootConfigurationPath
            && self.lpAppDirPath == other.lpAppDirPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACTIVATION_CONTEXT_DETAILED_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ACTIVATION_CONTEXT_QUERY_INDEX {
    pub ulAssemblyIndex: u32,
    pub ulFileIndexInAssembly: u32,
}
impl ACTIVATION_CONTEXT_QUERY_INDEX {}
impl ::std::default::Default for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACTIVATION_CONTEXT_QUERY_INDEX")
            .field("ulAssemblyIndex", &self.ulAssemblyIndex)
            .field("ulFileIndexInAssembly", &self.ulFileIndexInAssembly)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn eq(&self, other: &Self) -> bool {
        self.ulAssemblyIndex == other.ulAssemblyIndex
            && self.ulFileIndexInAssembly == other.ulFileIndexInAssembly
    }
}
impl ::std::cmp::Eq for ACTIVATION_CONTEXT_QUERY_INDEX {}
unsafe impl ::windows::runtime::Abi for ACTIVATION_CONTEXT_QUERY_INDEX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    pub ulFlags: u32,
    pub RunLevel: ACTCTX_REQUESTED_RUN_LEVEL,
    pub UiAccess: u32,
}
impl ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {}
impl ::std::default::Default for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION")
            .field("ulFlags", &self.ulFlags)
            .field("RunLevel", &self.RunLevel)
            .field("UiAccess", &self.UiAccess)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags
            && self.RunLevel == other.RunLevel
            && self.UiAccess == other.UiAccess
    }
}
impl ::std::cmp::Eq for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {}
unsafe impl ::windows::runtime::Abi for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
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
pub struct ADVERTISEFLAGS(pub i32);
pub const ADVERTISEFLAGS_MACHINEASSIGN: ADVERTISEFLAGS = ADVERTISEFLAGS(0i32);
pub const ADVERTISEFLAGS_USERASSIGN: ADVERTISEFLAGS = ADVERTISEFLAGS(1i32);
impl ::std::convert::From<i32> for ADVERTISEFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ADVERTISEFLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const APPLY_OPTION_FAIL_IF_CLOSE: u32 = 2u32;
pub const APPLY_OPTION_FAIL_IF_EXACT: u32 = 1u32;
pub const APPLY_OPTION_TEST_ONLY: u32 = 4u32;
pub const APPLY_OPTION_VALID_FLAGS: u32 = 7u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ASM_BIND_FLAGS(pub u32);
pub const ASM_BINDF_FORCE_CACHE_INSTALL: ASM_BIND_FLAGS = ASM_BIND_FLAGS(1u32);
pub const ASM_BINDF_RFS_INTEGRITY_CHECK: ASM_BIND_FLAGS = ASM_BIND_FLAGS(2u32);
pub const ASM_BINDF_RFS_MODULE_CHECK: ASM_BIND_FLAGS = ASM_BIND_FLAGS(4u32);
pub const ASM_BINDF_BINPATH_PROBE_ONLY: ASM_BIND_FLAGS = ASM_BIND_FLAGS(8u32);
pub const ASM_BINDF_SHARED_BINPATH_HINT: ASM_BIND_FLAGS = ASM_BIND_FLAGS(16u32);
pub const ASM_BINDF_PARENT_ASM_HINT: ASM_BIND_FLAGS = ASM_BIND_FLAGS(32u32);
impl ::std::convert::From<u32> for ASM_BIND_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ASM_BIND_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for ASM_BIND_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ASM_BIND_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ASM_BIND_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ASM_BIND_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ASM_BIND_FLAGS {
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
pub struct ASM_CMP_FLAGS(pub i32);
pub const ASM_CMPF_NAME: ASM_CMP_FLAGS = ASM_CMP_FLAGS(1i32);
pub const ASM_CMPF_MAJOR_VERSION: ASM_CMP_FLAGS = ASM_CMP_FLAGS(2i32);
pub const ASM_CMPF_MINOR_VERSION: ASM_CMP_FLAGS = ASM_CMP_FLAGS(4i32);
pub const ASM_CMPF_BUILD_NUMBER: ASM_CMP_FLAGS = ASM_CMP_FLAGS(8i32);
pub const ASM_CMPF_REVISION_NUMBER: ASM_CMP_FLAGS = ASM_CMP_FLAGS(16i32);
pub const ASM_CMPF_PUBLIC_KEY_TOKEN: ASM_CMP_FLAGS = ASM_CMP_FLAGS(32i32);
pub const ASM_CMPF_CULTURE: ASM_CMP_FLAGS = ASM_CMP_FLAGS(64i32);
pub const ASM_CMPF_CUSTOM: ASM_CMP_FLAGS = ASM_CMP_FLAGS(128i32);
pub const ASM_CMPF_ALL: ASM_CMP_FLAGS = ASM_CMP_FLAGS(255i32);
pub const ASM_CMPF_DEFAULT: ASM_CMP_FLAGS = ASM_CMP_FLAGS(256i32);
impl ::std::convert::From<i32> for ASM_CMP_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ASM_CMP_FLAGS {
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
pub struct ASM_DISPLAY_FLAGS(pub i32);
pub const ASM_DISPLAYF_VERSION: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(1i32);
pub const ASM_DISPLAYF_CULTURE: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(2i32);
pub const ASM_DISPLAYF_PUBLIC_KEY_TOKEN: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(4i32);
pub const ASM_DISPLAYF_PUBLIC_KEY: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(8i32);
pub const ASM_DISPLAYF_CUSTOM: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(16i32);
pub const ASM_DISPLAYF_PROCESSORARCHITECTURE: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(32i32);
pub const ASM_DISPLAYF_LANGUAGEID: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(64i32);
impl ::std::convert::From<i32> for ASM_DISPLAY_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ASM_DISPLAY_FLAGS {
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
pub struct ASM_NAME(pub i32);
pub const ASM_NAME_PUBLIC_KEY: ASM_NAME = ASM_NAME(0i32);
pub const ASM_NAME_PUBLIC_KEY_TOKEN: ASM_NAME = ASM_NAME(1i32);
pub const ASM_NAME_HASH_VALUE: ASM_NAME = ASM_NAME(2i32);
pub const ASM_NAME_NAME: ASM_NAME = ASM_NAME(3i32);
pub const ASM_NAME_MAJOR_VERSION: ASM_NAME = ASM_NAME(4i32);
pub const ASM_NAME_MINOR_VERSION: ASM_NAME = ASM_NAME(5i32);
pub const ASM_NAME_BUILD_NUMBER: ASM_NAME = ASM_NAME(6i32);
pub const ASM_NAME_REVISION_NUMBER: ASM_NAME = ASM_NAME(7i32);
pub const ASM_NAME_CULTURE: ASM_NAME = ASM_NAME(8i32);
pub const ASM_NAME_PROCESSOR_ID_ARRAY: ASM_NAME = ASM_NAME(9i32);
pub const ASM_NAME_OSINFO_ARRAY: ASM_NAME = ASM_NAME(10i32);
pub const ASM_NAME_HASH_ALGID: ASM_NAME = ASM_NAME(11i32);
pub const ASM_NAME_ALIAS: ASM_NAME = ASM_NAME(12i32);
pub const ASM_NAME_CODEBASE_URL: ASM_NAME = ASM_NAME(13i32);
pub const ASM_NAME_CODEBASE_LASTMOD: ASM_NAME = ASM_NAME(14i32);
pub const ASM_NAME_NULL_PUBLIC_KEY: ASM_NAME = ASM_NAME(15i32);
pub const ASM_NAME_NULL_PUBLIC_KEY_TOKEN: ASM_NAME = ASM_NAME(16i32);
pub const ASM_NAME_CUSTOM: ASM_NAME = ASM_NAME(17i32);
pub const ASM_NAME_NULL_CUSTOM: ASM_NAME = ASM_NAME(18i32);
pub const ASM_NAME_MVID: ASM_NAME = ASM_NAME(19i32);
pub const ASM_NAME_MAX_PARAMS: ASM_NAME = ASM_NAME(20i32);
impl ::std::convert::From<i32> for ASM_NAME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ASM_NAME {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ASSEMBLYINFO_FLAG_INSTALLED: u32 = 1u32;
pub const ASSEMBLYINFO_FLAG_PAYLOADRESIDENT: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ASSEMBLY_FILE_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulFilenameLength: u32,
    pub ulPathLength: u32,
    pub lpFileName: super::super::Foundation::PWSTR,
    pub lpFilePath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ASSEMBLY_FILE_DETAILED_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ASSEMBLY_FILE_DETAILED_INFORMATION")
            .field("ulFlags", &self.ulFlags)
            .field("ulFilenameLength", &self.ulFilenameLength)
            .field("ulPathLength", &self.ulPathLength)
            .field("lpFileName", &self.lpFileName)
            .field("lpFilePath", &self.lpFilePath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags
            && self.ulFilenameLength == other.ulFilenameLength
            && self.ulPathLength == other.ulPathLength
            && self.lpFileName == other.lpFileName
            && self.lpFilePath == other.lpFilePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ASSEMBLY_FILE_DETAILED_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ASSEMBLY_FILE_DETAILED_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ASSEMBLY_INFO {
    pub cbAssemblyInfo: u32,
    pub dwAssemblyFlags: u32,
    pub uliAssemblySizeInKB: u64,
    pub pszCurrentAssemblyPathBuf: super::super::Foundation::PWSTR,
    pub cchBuf: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ASSEMBLY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ASSEMBLY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ASSEMBLY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ASSEMBLY_INFO")
            .field("cbAssemblyInfo", &self.cbAssemblyInfo)
            .field("dwAssemblyFlags", &self.dwAssemblyFlags)
            .field("uliAssemblySizeInKB", &self.uliAssemblySizeInKB)
            .field("pszCurrentAssemblyPathBuf", &self.pszCurrentAssemblyPathBuf)
            .field("cchBuf", &self.cchBuf)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ASSEMBLY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbAssemblyInfo == other.cbAssemblyInfo
            && self.dwAssemblyFlags == other.dwAssemblyFlags
            && self.uliAssemblySizeInKB == other.uliAssemblySizeInKB
            && self.pszCurrentAssemblyPathBuf == other.pszCurrentAssemblyPathBuf
            && self.cchBuf == other.cchBuf
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ASSEMBLY_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ASSEMBLY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ActivateActCtx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hactctx: Param0,
    lpcookie: *mut usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ActivateActCtx(
                hactctx: super::super::Foundation::HANDLE,
                lpcookie: *mut usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ActivateActCtx(
            hactctx.into_param().abi(),
            ::std::mem::transmute(lpcookie),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddRefActCtx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hactctx: Param0,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddRefActCtx(hactctx: super::super::Foundation::HANDLE);
        }
        ::std::mem::transmute(AddRefActCtx(hactctx.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    applyflags: i64,
    lpsourcename: Param1,
    lpdeltaname: Param2,
    lptargetname: Param3,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyDeltaA(
                applyflags: i64,
                lpsourcename: super::super::Foundation::PSTR,
                lpdeltaname: super::super::Foundation::PSTR,
                lptargetname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyDeltaA(
            ::std::mem::transmute(applyflags),
            lpsourcename.into_param().abi(),
            lpdeltaname.into_param().abi(),
            lptargetname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaB<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
    Param2: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
>(
    applyflags: i64,
    source: Param1,
    delta: Param2,
    lptarget: *mut DELTA_OUTPUT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyDeltaB(
                applyflags: i64,
                source: DELTA_INPUT,
                delta: DELTA_INPUT,
                lptarget: *mut DELTA_OUTPUT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyDeltaB(
            ::std::mem::transmute(applyflags),
            source.into_param().abi(),
            delta.into_param().abi(),
            ::std::mem::transmute(lptarget),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaGetReverseB<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
    Param2: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
>(
    applyflags: i64,
    source: Param1,
    delta: Param2,
    lpreversefiletime: *const super::super::Foundation::FILETIME,
    lptarget: *mut DELTA_OUTPUT,
    lptargetreverse: *mut DELTA_OUTPUT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyDeltaGetReverseB(
                applyflags: i64,
                source: DELTA_INPUT,
                delta: DELTA_INPUT,
                lpreversefiletime: *const super::super::Foundation::FILETIME,
                lptarget: *mut DELTA_OUTPUT,
                lptargetreverse: *mut DELTA_OUTPUT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyDeltaGetReverseB(
            ::std::mem::transmute(applyflags),
            source.into_param().abi(),
            delta.into_param().abi(),
            ::std::mem::transmute(lpreversefiletime),
            ::std::mem::transmute(lptarget),
            ::std::mem::transmute(lptargetreverse),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaProvidedB<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
    Param2: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
>(
    applyflags: i64,
    source: Param1,
    delta: Param2,
    lptarget: *mut ::std::ffi::c_void,
    utargetsize: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyDeltaProvidedB(
                applyflags: i64,
                source: DELTA_INPUT,
                delta: DELTA_INPUT,
                lptarget: *mut ::std::ffi::c_void,
                utargetsize: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyDeltaProvidedB(
            ::std::mem::transmute(applyflags),
            source.into_param().abi(),
            delta.into_param().abi(),
            ::std::mem::transmute(lptarget),
            ::std::mem::transmute(utargetsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    applyflags: i64,
    lpsourcename: Param1,
    lpdeltaname: Param2,
    lptargetname: Param3,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyDeltaW(
                applyflags: i64,
                lpsourcename: super::super::Foundation::PWSTR,
                lpdeltaname: super::super::Foundation::PWSTR,
                lptargetname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyDeltaW(
            ::std::mem::transmute(applyflags),
            lpsourcename.into_param().abi(),
            lpdeltaname.into_param().abi(),
            lptargetname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    patchfilename: Param0,
    oldfilename: Param1,
    newfilename: Param2,
    applyoptionflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyPatchToFileA(
                patchfilename: super::super::Foundation::PSTR,
                oldfilename: super::super::Foundation::PSTR,
                newfilename: super::super::Foundation::PSTR,
                applyoptionflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyPatchToFileA(
            patchfilename.into_param().abi(),
            oldfilename.into_param().abi(),
            newfilename.into_param().abi(),
            ::std::mem::transmute(applyoptionflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileByBuffers(
    patchfilemapped: *const u8,
    patchfilesize: u32,
    oldfilemapped: *const u8,
    oldfilesize: u32,
    newfilebuffer: *mut *mut u8,
    newfilebuffersize: u32,
    newfileactualsize: *mut u32,
    newfiletime: *mut super::super::Foundation::FILETIME,
    applyoptionflags: u32,
    progresscallback: ::std::option::Option<PPATCH_PROGRESS_CALLBACK>,
    callbackcontext: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyPatchToFileByBuffers(
                patchfilemapped: *const u8,
                patchfilesize: u32,
                oldfilemapped: *const u8,
                oldfilesize: u32,
                newfilebuffer: *mut *mut u8,
                newfilebuffersize: u32,
                newfileactualsize: *mut u32,
                newfiletime: *mut super::super::Foundation::FILETIME,
                applyoptionflags: u32,
                progresscallback: ::windows::runtime::RawPtr,
                callbackcontext: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyPatchToFileByBuffers(
            ::std::mem::transmute(patchfilemapped),
            ::std::mem::transmute(patchfilesize),
            ::std::mem::transmute(oldfilemapped),
            ::std::mem::transmute(oldfilesize),
            ::std::mem::transmute(newfilebuffer),
            ::std::mem::transmute(newfilebuffersize),
            ::std::mem::transmute(newfileactualsize),
            ::std::mem::transmute(newfiletime),
            ::std::mem::transmute(applyoptionflags),
            ::std::mem::transmute(progresscallback),
            ::std::mem::transmute(callbackcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileByHandles<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    patchfilehandle: Param0,
    oldfilehandle: Param1,
    newfilehandle: Param2,
    applyoptionflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyPatchToFileByHandles(
                patchfilehandle: super::super::Foundation::HANDLE,
                oldfilehandle: super::super::Foundation::HANDLE,
                newfilehandle: super::super::Foundation::HANDLE,
                applyoptionflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyPatchToFileByHandles(
            patchfilehandle.into_param().abi(),
            oldfilehandle.into_param().abi(),
            newfilehandle.into_param().abi(),
            ::std::mem::transmute(applyoptionflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileByHandlesEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    patchfilehandle: Param0,
    oldfilehandle: Param1,
    newfilehandle: Param2,
    applyoptionflags: u32,
    progresscallback: ::std::option::Option<PPATCH_PROGRESS_CALLBACK>,
    callbackcontext: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyPatchToFileByHandlesEx(
                patchfilehandle: super::super::Foundation::HANDLE,
                oldfilehandle: super::super::Foundation::HANDLE,
                newfilehandle: super::super::Foundation::HANDLE,
                applyoptionflags: u32,
                progresscallback: ::windows::runtime::RawPtr,
                callbackcontext: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyPatchToFileByHandlesEx(
            patchfilehandle.into_param().abi(),
            oldfilehandle.into_param().abi(),
            newfilehandle.into_param().abi(),
            ::std::mem::transmute(applyoptionflags),
            ::std::mem::transmute(progresscallback),
            ::std::mem::transmute(callbackcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    patchfilename: Param0,
    oldfilename: Param1,
    newfilename: Param2,
    applyoptionflags: u32,
    progresscallback: ::std::option::Option<PPATCH_PROGRESS_CALLBACK>,
    callbackcontext: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyPatchToFileExA(
                patchfilename: super::super::Foundation::PSTR,
                oldfilename: super::super::Foundation::PSTR,
                newfilename: super::super::Foundation::PSTR,
                applyoptionflags: u32,
                progresscallback: ::windows::runtime::RawPtr,
                callbackcontext: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyPatchToFileExA(
            patchfilename.into_param().abi(),
            oldfilename.into_param().abi(),
            newfilename.into_param().abi(),
            ::std::mem::transmute(applyoptionflags),
            ::std::mem::transmute(progresscallback),
            ::std::mem::transmute(callbackcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    patchfilename: Param0,
    oldfilename: Param1,
    newfilename: Param2,
    applyoptionflags: u32,
    progresscallback: ::std::option::Option<PPATCH_PROGRESS_CALLBACK>,
    callbackcontext: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyPatchToFileExW(
                patchfilename: super::super::Foundation::PWSTR,
                oldfilename: super::super::Foundation::PWSTR,
                newfilename: super::super::Foundation::PWSTR,
                applyoptionflags: u32,
                progresscallback: ::windows::runtime::RawPtr,
                callbackcontext: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyPatchToFileExW(
            patchfilename.into_param().abi(),
            oldfilename.into_param().abi(),
            newfilename.into_param().abi(),
            ::std::mem::transmute(applyoptionflags),
            ::std::mem::transmute(progresscallback),
            ::std::mem::transmute(callbackcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    patchfilename: Param0,
    oldfilename: Param1,
    newfilename: Param2,
    applyoptionflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyPatchToFileW(
                patchfilename: super::super::Foundation::PWSTR,
                oldfilename: super::super::Foundation::PWSTR,
                newfilename: super::super::Foundation::PWSTR,
                applyoptionflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApplyPatchToFileW(
            patchfilename.into_param().abi(),
            oldfilename.into_param().abi(),
            newfilename.into_param().abi(),
            ::std::mem::transmute(applyoptionflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CLSID_EvalCom2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1851660560,
    32851,
    18016,
    [183, 149, 107, 97, 46, 41, 188, 88],
);
pub const CLSID_MsmMerge2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4182345173,
    10745,
    18243,
    [152, 5, 153, 188, 63, 53, 182, 120],
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COMPATIBILITY_CONTEXT_ELEMENT {
    pub Id: ::windows::runtime::GUID,
    pub Type: ACTCTX_COMPATIBILITY_ELEMENT_TYPE,
    pub MaxVersionTested: u64,
}
impl COMPATIBILITY_CONTEXT_ELEMENT {}
impl ::std::default::Default for COMPATIBILITY_CONTEXT_ELEMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COMPATIBILITY_CONTEXT_ELEMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMPATIBILITY_CONTEXT_ELEMENT")
            .field("Id", &self.Id)
            .field("Type", &self.Type)
            .field("MaxVersionTested", &self.MaxVersionTested)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COMPATIBILITY_CONTEXT_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id
            && self.Type == other.Type
            && self.MaxVersionTested == other.MaxVersionTested
    }
}
impl ::std::cmp::Eq for COMPATIBILITY_CONTEXT_ELEMENT {}
unsafe impl ::windows::runtime::Abi for COMPATIBILITY_CONTEXT_ELEMENT {
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
pub struct CREATE_ASM_NAME_OBJ_FLAGS(pub i32);
pub const CANOF_PARSE_DISPLAY_NAME: CREATE_ASM_NAME_OBJ_FLAGS = CREATE_ASM_NAME_OBJ_FLAGS(1i32);
pub const CANOF_SET_DEFAULT_VALUES: CREATE_ASM_NAME_OBJ_FLAGS = CREATE_ASM_NAME_OBJ_FLAGS(2i32);
impl ::std::convert::From<i32> for CREATE_ASM_NAME_OBJ_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CREATE_ASM_NAME_OBJ_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateActCtxA(pactctx: *const ACTCTXA) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateActCtxA(pactctx: *const ACTCTXA) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateActCtxA(::std::mem::transmute(pactctx)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateActCtxW(pactctx: *const ACTCTXW) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateActCtxW(pactctx: *const ACTCTXW) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateActCtxW(::std::mem::transmute(pactctx)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeltaA<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param7: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
    Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    filetypeset: i64,
    setflags: i64,
    resetflags: i64,
    lpsourcename: Param3,
    lptargetname: Param4,
    lpsourceoptionsname: Param5,
    lptargetoptionsname: Param6,
    globaloptions: Param7,
    lptargetfiletime: *const super::super::Foundation::FILETIME,
    hashalgid: u32,
    lpdeltaname: Param10,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDeltaA(
                filetypeset: i64,
                setflags: i64,
                resetflags: i64,
                lpsourcename: super::super::Foundation::PSTR,
                lptargetname: super::super::Foundation::PSTR,
                lpsourceoptionsname: super::super::Foundation::PSTR,
                lptargetoptionsname: super::super::Foundation::PSTR,
                globaloptions: DELTA_INPUT,
                lptargetfiletime: *const super::super::Foundation::FILETIME,
                hashalgid: u32,
                lpdeltaname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDeltaA(
            ::std::mem::transmute(filetypeset),
            ::std::mem::transmute(setflags),
            ::std::mem::transmute(resetflags),
            lpsourcename.into_param().abi(),
            lptargetname.into_param().abi(),
            lpsourceoptionsname.into_param().abi(),
            lptargetoptionsname.into_param().abi(),
            globaloptions.into_param().abi(),
            ::std::mem::transmute(lptargetfiletime),
            ::std::mem::transmute(hashalgid),
            lpdeltaname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeltaB<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
    Param4: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
    Param5: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
    Param6: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
    Param7: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
>(
    filetypeset: i64,
    setflags: i64,
    resetflags: i64,
    source: Param3,
    target: Param4,
    sourceoptions: Param5,
    targetoptions: Param6,
    globaloptions: Param7,
    lptargetfiletime: *const super::super::Foundation::FILETIME,
    hashalgid: u32,
    lpdelta: *mut DELTA_OUTPUT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDeltaB(
                filetypeset: i64,
                setflags: i64,
                resetflags: i64,
                source: DELTA_INPUT,
                target: DELTA_INPUT,
                sourceoptions: DELTA_INPUT,
                targetoptions: DELTA_INPUT,
                globaloptions: DELTA_INPUT,
                lptargetfiletime: *const super::super::Foundation::FILETIME,
                hashalgid: u32,
                lpdelta: *mut DELTA_OUTPUT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDeltaB(
            ::std::mem::transmute(filetypeset),
            ::std::mem::transmute(setflags),
            ::std::mem::transmute(resetflags),
            source.into_param().abi(),
            target.into_param().abi(),
            sourceoptions.into_param().abi(),
            targetoptions.into_param().abi(),
            globaloptions.into_param().abi(),
            ::std::mem::transmute(lptargetfiletime),
            ::std::mem::transmute(hashalgid),
            ::std::mem::transmute(lpdelta),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeltaW<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param7: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
    Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    filetypeset: i64,
    setflags: i64,
    resetflags: i64,
    lpsourcename: Param3,
    lptargetname: Param4,
    lpsourceoptionsname: Param5,
    lptargetoptionsname: Param6,
    globaloptions: Param7,
    lptargetfiletime: *const super::super::Foundation::FILETIME,
    hashalgid: u32,
    lpdeltaname: Param10,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDeltaW(
                filetypeset: i64,
                setflags: i64,
                resetflags: i64,
                lpsourcename: super::super::Foundation::PWSTR,
                lptargetname: super::super::Foundation::PWSTR,
                lpsourceoptionsname: super::super::Foundation::PWSTR,
                lptargetoptionsname: super::super::Foundation::PWSTR,
                globaloptions: DELTA_INPUT,
                lptargetfiletime: *const super::super::Foundation::FILETIME,
                hashalgid: u32,
                lpdeltaname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDeltaW(
            ::std::mem::transmute(filetypeset),
            ::std::mem::transmute(setflags),
            ::std::mem::transmute(resetflags),
            lpsourcename.into_param().abi(),
            lptargetname.into_param().abi(),
            lpsourceoptionsname.into_param().abi(),
            lptargetoptionsname.into_param().abi(),
            globaloptions.into_param().abi(),
            ::std::mem::transmute(lptargetfiletime),
            ::std::mem::transmute(hashalgid),
            lpdeltaname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    oldfilename: Param0,
    newfilename: Param1,
    patchfilename: Param2,
    optionflags: u32,
    optiondata: *const PATCH_OPTION_DATA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePatchFileA(
                oldfilename: super::super::Foundation::PSTR,
                newfilename: super::super::Foundation::PSTR,
                patchfilename: super::super::Foundation::PSTR,
                optionflags: u32,
                optiondata: *const ::std::mem::ManuallyDrop<PATCH_OPTION_DATA>,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreatePatchFileA(
            oldfilename.into_param().abi(),
            newfilename.into_param().abi(),
            patchfilename.into_param().abi(),
            ::std::mem::transmute(optionflags),
            ::std::mem::transmute(optiondata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileByHandles<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    oldfilehandle: Param0,
    newfilehandle: Param1,
    patchfilehandle: Param2,
    optionflags: u32,
    optiondata: *const PATCH_OPTION_DATA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePatchFileByHandles(
                oldfilehandle: super::super::Foundation::HANDLE,
                newfilehandle: super::super::Foundation::HANDLE,
                patchfilehandle: super::super::Foundation::HANDLE,
                optionflags: u32,
                optiondata: *const ::std::mem::ManuallyDrop<PATCH_OPTION_DATA>,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreatePatchFileByHandles(
            oldfilehandle.into_param().abi(),
            newfilehandle.into_param().abi(),
            patchfilehandle.into_param().abi(),
            ::std::mem::transmute(optionflags),
            ::std::mem::transmute(optiondata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileByHandlesEx<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    oldfilecount: u32,
    oldfileinfoarray: *const PATCH_OLD_FILE_INFO_H,
    newfilehandle: Param2,
    patchfilehandle: Param3,
    optionflags: u32,
    optiondata: *const PATCH_OPTION_DATA,
    progresscallback: ::std::option::Option<PPATCH_PROGRESS_CALLBACK>,
    callbackcontext: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePatchFileByHandlesEx(
                oldfilecount: u32,
                oldfileinfoarray: *const PATCH_OLD_FILE_INFO_H,
                newfilehandle: super::super::Foundation::HANDLE,
                patchfilehandle: super::super::Foundation::HANDLE,
                optionflags: u32,
                optiondata: *const ::std::mem::ManuallyDrop<PATCH_OPTION_DATA>,
                progresscallback: ::windows::runtime::RawPtr,
                callbackcontext: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreatePatchFileByHandlesEx(
            ::std::mem::transmute(oldfilecount),
            ::std::mem::transmute(oldfileinfoarray),
            newfilehandle.into_param().abi(),
            patchfilehandle.into_param().abi(),
            ::std::mem::transmute(optionflags),
            ::std::mem::transmute(optiondata),
            ::std::mem::transmute(progresscallback),
            ::std::mem::transmute(callbackcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileExA<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    oldfilecount: u32,
    oldfileinfoarray: *const PATCH_OLD_FILE_INFO_A,
    newfilename: Param2,
    patchfilename: Param3,
    optionflags: u32,
    optiondata: *const PATCH_OPTION_DATA,
    progresscallback: ::std::option::Option<PPATCH_PROGRESS_CALLBACK>,
    callbackcontext: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePatchFileExA(
                oldfilecount: u32,
                oldfileinfoarray: *const PATCH_OLD_FILE_INFO_A,
                newfilename: super::super::Foundation::PSTR,
                patchfilename: super::super::Foundation::PSTR,
                optionflags: u32,
                optiondata: *const ::std::mem::ManuallyDrop<PATCH_OPTION_DATA>,
                progresscallback: ::windows::runtime::RawPtr,
                callbackcontext: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreatePatchFileExA(
            ::std::mem::transmute(oldfilecount),
            ::std::mem::transmute(oldfileinfoarray),
            newfilename.into_param().abi(),
            patchfilename.into_param().abi(),
            ::std::mem::transmute(optionflags),
            ::std::mem::transmute(optiondata),
            ::std::mem::transmute(progresscallback),
            ::std::mem::transmute(callbackcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileExW<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    oldfilecount: u32,
    oldfileinfoarray: *const PATCH_OLD_FILE_INFO_W,
    newfilename: Param2,
    patchfilename: Param3,
    optionflags: u32,
    optiondata: *const PATCH_OPTION_DATA,
    progresscallback: ::std::option::Option<PPATCH_PROGRESS_CALLBACK>,
    callbackcontext: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePatchFileExW(
                oldfilecount: u32,
                oldfileinfoarray: *const PATCH_OLD_FILE_INFO_W,
                newfilename: super::super::Foundation::PWSTR,
                patchfilename: super::super::Foundation::PWSTR,
                optionflags: u32,
                optiondata: *const ::std::mem::ManuallyDrop<PATCH_OPTION_DATA>,
                progresscallback: ::windows::runtime::RawPtr,
                callbackcontext: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreatePatchFileExW(
            ::std::mem::transmute(oldfilecount),
            ::std::mem::transmute(oldfileinfoarray),
            newfilename.into_param().abi(),
            patchfilename.into_param().abi(),
            ::std::mem::transmute(optionflags),
            ::std::mem::transmute(optiondata),
            ::std::mem::transmute(progresscallback),
            ::std::mem::transmute(callbackcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    oldfilename: Param0,
    newfilename: Param1,
    patchfilename: Param2,
    optionflags: u32,
    optiondata: *const PATCH_OPTION_DATA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePatchFileW(
                oldfilename: super::super::Foundation::PWSTR,
                newfilename: super::super::Foundation::PWSTR,
                patchfilename: super::super::Foundation::PWSTR,
                optionflags: u32,
                optiondata: *const ::std::mem::ManuallyDrop<PATCH_OPTION_DATA>,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreatePatchFileW(
            oldfilename.into_param().abi(),
            newfilename.into_param().abi(),
            patchfilename.into_param().abi(),
            ::std::mem::transmute(optionflags),
            ::std::mem::transmute(optiondata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DEFAULT_DISK_ID: u32 = 2u32;
pub const DEFAULT_FILE_SEQUENCE_START: u32 = 2u32;
pub const DEFAULT_MINIMUM_REQUIRED_MSI_VERSION: u32 = 100u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DELTA_HASH {
    pub HashSize: u32,
    pub HashValue: [u8; 32],
}
impl DELTA_HASH {}
impl ::std::default::Default for DELTA_HASH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DELTA_HASH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DELTA_HASH")
            .field("HashSize", &self.HashSize)
            .field("HashValue", &self.HashValue)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DELTA_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.HashSize == other.HashSize && self.HashValue == other.HashValue
    }
}
impl ::std::cmp::Eq for DELTA_HASH {}
unsafe impl ::windows::runtime::Abi for DELTA_HASH {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl DELTA_HEADER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DELTA_HEADER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DELTA_HEADER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DELTA_HEADER_INFO")
            .field("FileTypeSet", &self.FileTypeSet)
            .field("FileType", &self.FileType)
            .field("Flags", &self.Flags)
            .field("TargetSize", &self.TargetSize)
            .field("TargetFileTime", &self.TargetFileTime)
            .field("TargetHashAlgId", &self.TargetHashAlgId)
            .field("TargetHash", &self.TargetHash)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DELTA_HEADER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileTypeSet == other.FileTypeSet
            && self.FileType == other.FileType
            && self.Flags == other.Flags
            && self.TargetSize == other.TargetSize
            && self.TargetFileTime == other.TargetFileTime
            && self.TargetHashAlgId == other.TargetHashAlgId
            && self.TargetHash == other.TargetHash
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DELTA_HEADER_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DELTA_HEADER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DELTA_INPUT {
    pub Anonymous: DELTA_INPUT_0,
    pub uSize: usize,
    pub Editable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DELTA_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DELTA_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DELTA_INPUT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DELTA_INPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DELTA_INPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union DELTA_INPUT_0 {
    pub lpcStart: *mut ::std::ffi::c_void,
    pub lpStart: *mut ::std::ffi::c_void,
}
impl DELTA_INPUT_0 {}
impl ::std::default::Default for DELTA_INPUT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DELTA_INPUT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DELTA_INPUT_0 {}
unsafe impl ::windows::runtime::Abi for DELTA_INPUT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DELTA_MAX_HASH_SIZE: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DELTA_OUTPUT {
    pub lpStart: *mut ::std::ffi::c_void,
    pub uSize: usize,
}
impl DELTA_OUTPUT {}
impl ::std::default::Default for DELTA_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DELTA_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DELTA_OUTPUT")
            .field("lpStart", &self.lpStart)
            .field("uSize", &self.uSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DELTA_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.lpStart == other.lpStart && self.uSize == other.uSize
    }
}
impl ::std::cmp::Eq for DELTA_OUTPUT {}
unsafe impl ::windows::runtime::Abi for DELTA_OUTPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeactivateActCtx(dwflags: u32, ulcookie: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeactivateActCtx(dwflags: u32, ulcookie: usize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeactivateActCtx(
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(ulcookie),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeltaFree(lpmemory: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeltaFree(lpmemory: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeltaFree(::std::mem::transmute(lpmemory)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeltaNormalizeProvidedB<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, DELTA_INPUT>,
>(
    filetypeset: i64,
    normalizeflags: i64,
    normalizeoptions: Param2,
    lpsource: *mut ::std::ffi::c_void,
    usourcesize: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeltaNormalizeProvidedB(
                filetypeset: i64,
                normalizeflags: i64,
                normalizeoptions: DELTA_INPUT,
                lpsource: *mut ::std::ffi::c_void,
                usourcesize: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeltaNormalizeProvidedB(
            ::std::mem::transmute(filetypeset),
            ::std::mem::transmute(normalizeflags),
            normalizeoptions.into_param().abi(),
            ::std::mem::transmute(lpsource),
            ::std::mem::transmute(usourcesize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ERROR_PATCH_BIGGER_THAN_COMPRESSED: u32 = 3222155525u32;
pub const ERROR_PATCH_CORRUPT: u32 = 3222159618u32;
pub const ERROR_PATCH_DECODE_FAILURE: u32 = 3222159617u32;
pub const ERROR_PATCH_ENCODE_FAILURE: u32 = 3222155521u32;
pub const ERROR_PATCH_IMAGEHLP_FAILURE: u32 = 3222155526u32;
pub const ERROR_PATCH_INVALID_OPTIONS: u32 = 3222155522u32;
pub const ERROR_PATCH_NEWER_FORMAT: u32 = 3222159619u32;
pub const ERROR_PATCH_NOT_AVAILABLE: u32 = 3222159622u32;
pub const ERROR_PATCH_NOT_NECESSARY: u32 = 3222159621u32;
pub const ERROR_PATCH_RETAIN_RANGES_DIFFER: u32 = 3222155524u32;
pub const ERROR_PATCH_SAME_FILE: u32 = 3222155523u32;
pub const ERROR_PATCH_WRONG_FILE: u32 = 3222159620u32;
pub const ERROR_PCW_BAD_API_PATCHING_SYMBOL_FLAGS: u32 = 3222163725u32;
pub const ERROR_PCW_BAD_FAMILY_RANGE_NAME: u32 = 3222163801u32;
pub const ERROR_PCW_BAD_FILE_SEQUENCE_START: u32 = 3222163770u32;
pub const ERROR_PCW_BAD_GUIDS_TO_REPLACE: u32 = 3222163721u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_DISKID: u32 = 3222163773u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_FILESEQSTART: u32 = 3222163774u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_NAME: u32 = 3222163748u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_SRC_PROP: u32 = 3222163750u32;
pub const ERROR_PCW_BAD_MAJOR_VERSION: u32 = 3222163853u32;
pub const ERROR_PCW_BAD_PATCH_GUID: u32 = 3222163720u32;
pub const ERROR_PCW_BAD_PRODUCTVERSION_VALIDATION: u32 = 3222163844u32;
pub const ERROR_PCW_BAD_SEQUENCE: u32 = 3222163848u32;
pub const ERROR_PCW_BAD_SUPERCEDENCE: u32 = 3222163847u32;
pub const ERROR_PCW_BAD_TARGET: u32 = 3222163849u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_NAME: u32 = 3222163736u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_CODE: u32 = 3222163834u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_VERSION: u32 = 3222163835u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADED: u32 = 3222163776u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADE_CODE: u32 = 3222163836u32;
pub const ERROR_PCW_BAD_TARGET_PRODUCT_CODE_LIST: u32 = 3222163722u32;
pub const ERROR_PCW_BAD_TGT_UPD_IMAGES: u32 = 3222163846u32;
pub const ERROR_PCW_BAD_TRANSFORMSET: u32 = 3222163845u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_FAMILY: u32 = 3222163775u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_NAME: u32 = 3222163728u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_CODE: u32 = 3222163831u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_VERSION: u32 = 3222163832u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_UPGRADE_CODE: u32 = 3222163833u32;
pub const ERROR_PCW_BAD_VERSION_STRING: u32 = 3222163852u32;
pub const ERROR_PCW_BASE: u32 = 3222163713u32;
pub const ERROR_PCW_CANNOT_CREATE_TABLE: u32 = 3222163841u32;
pub const ERROR_PCW_CANNOT_RUN_MAKECAB: u32 = 3222163782u32;
pub const ERROR_PCW_CANNOT_WRITE_DDF: u32 = 3222163781u32;
pub const ERROR_PCW_CANT_COPY_FILE_TO_TEMP_FOLDER: u32 = 3222163771u32;
pub const ERROR_PCW_CANT_CREATE_ONE_PATCH_FILE: u32 = 3222163772u32;
pub const ERROR_PCW_CANT_CREATE_PATCH_FILE: u32 = 3222163718u32;
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO: u32 = 3222163828u32;
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO_POUND: u32 = 3222163830u32;
pub const ERROR_PCW_CANT_CREATE_TEMP_FOLDER: u32 = 3222163715u32;
pub const ERROR_PCW_CANT_DELETE_TEMP_FOLDER: u32 = 3222163974u32;
pub const ERROR_PCW_CANT_GENERATE_SEQUENCEINFO_MAJORUPGD: u32 = 3222163842u32;
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM: u32 = 3222163827u32;
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM_POUND: u32 = 3222163829u32;
pub const ERROR_PCW_CANT_OVERWRITE_PATCH: u32 = 3222163717u32;
pub const ERROR_PCW_CANT_READ_FILE: u32 = 3222163978u32;
pub const ERROR_PCW_CREATEFILE_LOG_FAILED: u32 = 3222163861u32;
pub const ERROR_PCW_DUPLICATE_SEQUENCE_RECORD: u32 = 3222163858u32;
pub const ERROR_PCW_DUP_IMAGE_FAMILY_NAME: u32 = 3222163749u32;
pub const ERROR_PCW_DUP_TARGET_IMAGE_NAME: u32 = 3222163737u32;
pub const ERROR_PCW_DUP_TARGET_IMAGE_PACKCODE: u32 = 3222163777u32;
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_NAME: u32 = 3222163729u32;
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_PACKCODE: u32 = 3222163795u32;
pub const ERROR_PCW_ERROR_WRITING_TO_LOG: u32 = 3222163864u32;
pub const ERROR_PCW_EXECUTE_VIEW: u32 = 3222163870u32;
pub const ERROR_PCW_EXTFILE_BAD_FAMILY_FIELD: u32 = 3222163756u32;
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_LENGTHS: u32 = 3222163814u32;
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_OFFSETS: u32 = 3222163812u32;
pub const ERROR_PCW_EXTFILE_BAD_RETAIN_OFFSETS: u32 = 3222163817u32;
pub const ERROR_PCW_EXTFILE_BLANK_FILE_TABLE_KEY: u32 = 3222163755u32;
pub const ERROR_PCW_EXTFILE_BLANK_PATH_TO_FILE: u32 = 3222163758u32;
pub const ERROR_PCW_EXTFILE_IGNORE_COUNT_MISMATCH: u32 = 3222163815u32;
pub const ERROR_PCW_EXTFILE_LONG_FILE_TABLE_KEY: u32 = 3222163754u32;
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_LENGTHS: u32 = 3222163813u32;
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_OFFSETS: u32 = 3222163811u32;
pub const ERROR_PCW_EXTFILE_LONG_PATH_TO_FILE: u32 = 3222163757u32;
pub const ERROR_PCW_EXTFILE_LONG_RETAIN_OFFSETS: u32 = 3222163816u32;
pub const ERROR_PCW_EXTFILE_MISSING_FILE: u32 = 3222163759u32;
pub const ERROR_PCW_FAILED_CREATE_TRANSFORM: u32 = 3222163973u32;
pub const ERROR_PCW_FAILED_EXPAND_PATH: u32 = 3222163872u32;
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_LENGTHS: u32 = 3222163809u32;
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_OFFSETS: u32 = 3222163806u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_FILE_TABLE_KEY: u32 = 3222163803u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_LENGTHS: u32 = 3222163808u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_OFFSETS: u32 = 3222163805u32;
pub const ERROR_PCW_FAMILY_RANGE_COUNT_MISMATCH: u32 = 3222163810u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_FILE_TABLE_KEY: u32 = 3222163802u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_LENGTHS: u32 = 3222163807u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_OFFSETS: u32 = 3222163804u32;
pub const ERROR_PCW_FAMILY_RANGE_NAME_TOO_LONG: u32 = 3222163800u32;
pub const ERROR_PCW_IMAGE_FAMILY_NAME_TOO_LONG: u32 = 3222163747u32;
pub const ERROR_PCW_IMAGE_PATH_NOT_EXIST: u32 = 3222163988u32;
pub const ERROR_PCW_INTERNAL_ERROR: u32 = 3222163969u32;
pub const ERROR_PCW_INVALID_LOG_LEVEL: u32 = 3222163862u32;
pub const ERROR_PCW_INVALID_MAJOR_VERSION: u32 = 3222163990u32;
pub const ERROR_PCW_INVALID_PARAMETER: u32 = 3222163860u32;
pub const ERROR_PCW_INVALID_PATCHMETADATA_PROP: u32 = 3222163856u32;
pub const ERROR_PCW_INVALID_PATCH_TYPE_SEQUENCING: u32 = 3222163977u32;
pub const ERROR_PCW_INVALID_PCP_EXTERNALFILES: u32 = 3222163982u32;
pub const ERROR_PCW_INVALID_PCP_FAMILYFILERANGES: u32 = 3222163992u32;
pub const ERROR_PCW_INVALID_PCP_IMAGEFAMILIES: u32 = 3222163983u32;
pub const ERROR_PCW_INVALID_PCP_PATCHSEQUENCE: u32 = 3222163984u32;
pub const ERROR_PCW_INVALID_PCP_PROPERTIES: u32 = 3222163991u32;
pub const ERROR_PCW_INVALID_PCP_PROPERTY: u32 = 3222163970u32;
pub const ERROR_PCW_INVALID_PCP_TARGETFILES_OPTIONALDATA: u32 = 3222163985u32;
pub const ERROR_PCW_INVALID_PCP_TARGETIMAGES: u32 = 3222163971u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILESTOIGNORE: u32 = 3222163980u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILES_OPTIONALDATA: u32 = 3222163986u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDIMAGES: u32 = 3222163981u32;
pub const ERROR_PCW_INVALID_RANGE_ELEMENT: u32 = 3222163989u32;
pub const ERROR_PCW_INVALID_SUPERCEDENCE: u32 = 3222163857u32;
pub const ERROR_PCW_INVALID_SUPERSEDENCE_VALUE: u32 = 3222163976u32;
pub const ERROR_PCW_INVALID_UI_LEVEL: u32 = 3222163863u32;
pub const ERROR_PCW_LAX_VALIDATION_FLAGS: u32 = 3222163972u32;
pub const ERROR_PCW_MAJOR_UPGD_WITHOUT_SEQUENCING: u32 = 3222163843u32;
pub const ERROR_PCW_MATCHED_PRODUCT_VERSIONS: u32 = 3222163837u32;
pub const ERROR_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222163779u32;
pub const ERROR_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222163780u32;
pub const ERROR_PCW_MISSING_DIRECTORY_TABLE: u32 = 3222163975u32;
pub const ERROR_PCW_MISSING_PATCHMETADATA: u32 = 3222163987u32;
pub const ERROR_PCW_MISSING_PATCH_GUID: u32 = 3222163719u32;
pub const ERROR_PCW_MISSING_PATCH_PATH: u32 = 3222163716u32;
pub const ERROR_PCW_NO_UPGRADED_IMAGES_TO_PATCH: u32 = 3222163723u32;
pub const ERROR_PCW_NULL_PATCHFAMILY: u32 = 3222163850u32;
pub const ERROR_PCW_NULL_SEQUENCE_NUMBER: u32 = 3222163851u32;
pub const ERROR_PCW_OBSOLETION_WITH_MSI30: u32 = 3222163839u32;
pub const ERROR_PCW_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222163840u32;
pub const ERROR_PCW_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222163838u32;
pub const ERROR_PCW_OODS_COPYING_MSI: u32 = 3222163726u32;
pub const ERROR_PCW_OPEN_VIEW: u32 = 3222163869u32;
pub const ERROR_PCW_OUT_OF_MEMORY: u32 = 3222163865u32;
pub const ERROR_PCW_PATCHMETADATA_PROP_NOT_SET: u32 = 3222163855u32;
pub const ERROR_PCW_PCP_BAD_FORMAT: u32 = 3222163714u32;
pub const ERROR_PCW_PCP_DOESNT_EXIST: u32 = 3222163713u32;
pub const ERROR_PCW_SEQUENCING_BAD_TARGET: u32 = 3222163854u32;
pub const ERROR_PCW_TARGET_BAD_PROD_CODE_VAL: u32 = 3222163744u32;
pub const ERROR_PCW_TARGET_BAD_PROD_VALIDATE: u32 = 3222163743u32;
pub const ERROR_PCW_TARGET_IMAGE_COMPRESSED: u32 = 3222163742u32;
pub const ERROR_PCW_TARGET_IMAGE_NAME_TOO_LONG: u32 = 3222163735u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_EMPTY: u32 = 3222163739u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_EXIST: u32 = 3222163740u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_MSI: u32 = 3222163741u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_TOO_LONG: u32 = 3222163738u32;
pub const ERROR_PCW_TARGET_MISSING_SRC_FILES: u32 = 3222163746u32;
pub const ERROR_PCW_TARGET_WRONG_PRODUCT_VERSION_COMP: u32 = 3222163979u32;
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_LENGTHS: u32 = 3222163822u32;
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_OFFSETS: u32 = 3222163820u32;
pub const ERROR_PCW_TFILEDATA_BAD_RETAIN_OFFSETS: u32 = 3222163825u32;
pub const ERROR_PCW_TFILEDATA_BAD_TARGET_FIELD: u32 = 3222163791u32;
pub const ERROR_PCW_TFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163789u32;
pub const ERROR_PCW_TFILEDATA_IGNORE_COUNT_MISMATCH: u32 = 3222163823u32;
pub const ERROR_PCW_TFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163788u32;
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_LENGTHS: u32 = 3222163821u32;
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_OFFSETS: u32 = 3222163819u32;
pub const ERROR_PCW_TFILEDATA_LONG_RETAIN_OFFSETS: u32 = 3222163824u32;
pub const ERROR_PCW_TFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163790u32;
pub const ERROR_PCW_UFILEDATA_BAD_UPGRADED_FIELD: u32 = 3222163778u32;
pub const ERROR_PCW_UFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163752u32;
pub const ERROR_PCW_UFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163751u32;
pub const ERROR_PCW_UFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163753u32;
pub const ERROR_PCW_UFILEIGNORE_BAD_FILE_TABLE_KEY: u32 = 3222163799u32;
pub const ERROR_PCW_UFILEIGNORE_BAD_UPGRADED_FIELD: u32 = 3222163796u32;
pub const ERROR_PCW_UFILEIGNORE_BLANK_FILE_TABLE_KEY: u32 = 3222163798u32;
pub const ERROR_PCW_UFILEIGNORE_LONG_FILE_TABLE_KEY: u32 = 3222163797u32;
pub const ERROR_PCW_UNKNOWN_ERROR: u32 = 3222163866u32;
pub const ERROR_PCW_UNKNOWN_INFO: u32 = 3222163867u32;
pub const ERROR_PCW_UNKNOWN_WARN: u32 = 3222163868u32;
pub const ERROR_PCW_UPGRADED_IMAGE_COMPRESSED: u32 = 3222163734u32;
pub const ERROR_PCW_UPGRADED_IMAGE_NAME_TOO_LONG: u32 = 3222163727u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_EXIST: u32 = 3222163793u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_MSI: u32 = 3222163794u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_TOO_LONG: u32 = 3222163792u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_EMPTY: u32 = 3222163731u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_EXIST: u32 = 3222163732u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_MSI: u32 = 3222163733u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_TOO_LONG: u32 = 3222163730u32;
pub const ERROR_PCW_UPGRADED_MISSING_SRC_FILES: u32 = 3222163745u32;
pub const ERROR_PCW_VIEW_FETCH: u32 = 3222163871u32;
pub const ERROR_PCW_WRITE_SUMMARY_PROPERTIES: u32 = 3222163787u32;
pub const ERROR_PCW_WRONG_PATCHMETADATA_STRD_PROP: u32 = 3222163859u32;
pub const ERROR_ROLLBACK_DISABLED: u32 = 1653u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtractPatchHeaderToFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    patchfilename: Param0,
    patchheaderfilename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExtractPatchHeaderToFileA(
                patchfilename: super::super::Foundation::PSTR,
                patchheaderfilename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ExtractPatchHeaderToFileA(
            patchfilename.into_param().abi(),
            patchheaderfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtractPatchHeaderToFileByHandles<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    patchfilehandle: Param0,
    patchheaderfilehandle: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExtractPatchHeaderToFileByHandles(
                patchfilehandle: super::super::Foundation::HANDLE,
                patchheaderfilehandle: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ExtractPatchHeaderToFileByHandles(
            patchfilehandle.into_param().abi(),
            patchheaderfilehandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtractPatchHeaderToFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    patchfilename: Param0,
    patchheaderfilename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExtractPatchHeaderToFileW(
                patchfilename: super::super::Foundation::PWSTR,
                patchheaderfilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ExtractPatchHeaderToFileW(
            patchfilename.into_param().abi(),
            patchheaderfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FUSION_INSTALL_REFERENCE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub guidScheme: ::windows::runtime::GUID,
    pub szIdentifier: super::super::Foundation::PWSTR,
    pub szNonCannonicalData: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FUSION_INSTALL_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FUSION_INSTALL_REFERENCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FUSION_INSTALL_REFERENCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FUSION_INSTALL_REFERENCE")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("guidScheme", &self.guidScheme)
            .field("szIdentifier", &self.szIdentifier)
            .field("szNonCannonicalData", &self.szNonCannonicalData)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FUSION_INSTALL_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwFlags == other.dwFlags
            && self.guidScheme == other.guidScheme
            && self.szIdentifier == other.szIdentifier
            && self.szNonCannonicalData == other.szNonCannonicalData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FUSION_INSTALL_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FUSION_INSTALL_REFERENCE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FUSION_REFCOUNT_FILEPATH_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2955910501,
        64375,
        20346,
        [175, 165, 179, 145, 48, 159, 17, 201],
    );
pub const FUSION_REFCOUNT_OPAQUE_STRING_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        784938083,
        45251,
        17889,
        [131, 100, 50, 126, 150, 174, 168, 86],
    );
pub const FUSION_REFCOUNT_UNINSTALL_SUBKEY_GUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2364391957,
        44107,
        18571,
        [147, 192, 165, 10, 73, 203, 47, 184],
    );
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_WindowsProgramming"
))]
#[inline]
pub unsafe fn FindActCtxSectionGuid(
    dwflags: u32,
    lpextensionguid: *const ::windows::runtime::GUID,
    ulsectionid: u32,
    lpguidtofind: *const ::windows::runtime::GUID,
    returneddata: *mut ACTCTX_SECTION_KEYED_DATA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindActCtxSectionGuid(
                dwflags: u32,
                lpextensionguid: *const ::windows::runtime::GUID,
                ulsectionid: u32,
                lpguidtofind: *const ::windows::runtime::GUID,
                returneddata: *mut ACTCTX_SECTION_KEYED_DATA,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindActCtxSectionGuid(
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpextensionguid),
            ::std::mem::transmute(ulsectionid),
            ::std::mem::transmute(lpguidtofind),
            ::std::mem::transmute(returneddata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_WindowsProgramming"
))]
#[inline]
pub unsafe fn FindActCtxSectionStringA<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    dwflags: u32,
    lpextensionguid: *const ::windows::runtime::GUID,
    ulsectionid: u32,
    lpstringtofind: Param3,
    returneddata: *mut ACTCTX_SECTION_KEYED_DATA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindActCtxSectionStringA(
                dwflags: u32,
                lpextensionguid: *const ::windows::runtime::GUID,
                ulsectionid: u32,
                lpstringtofind: super::super::Foundation::PSTR,
                returneddata: *mut ACTCTX_SECTION_KEYED_DATA,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindActCtxSectionStringA(
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpextensionguid),
            ::std::mem::transmute(ulsectionid),
            lpstringtofind.into_param().abi(),
            ::std::mem::transmute(returneddata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_System_WindowsProgramming"
))]
#[inline]
pub unsafe fn FindActCtxSectionStringW<
    'a,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwflags: u32,
    lpextensionguid: *const ::windows::runtime::GUID,
    ulsectionid: u32,
    lpstringtofind: Param3,
    returneddata: *mut ACTCTX_SECTION_KEYED_DATA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindActCtxSectionStringW(
                dwflags: u32,
                lpextensionguid: *const ::windows::runtime::GUID,
                ulsectionid: u32,
                lpstringtofind: super::super::Foundation::PWSTR,
                returneddata: *mut ACTCTX_SECTION_KEYED_DATA,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FindActCtxSectionStringW(
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpextensionguid),
            ::std::mem::transmute(ulsectionid),
            lpstringtofind.into_param().abi(),
            ::std::mem::transmute(returneddata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentActCtx(
    lphactctx: *mut super::super::Foundation::HANDLE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentActCtx(
                lphactctx: *mut super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCurrentActCtx(::std::mem::transmute(lphactctx)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpdeltaname: Param0,
    lpheaderinfo: *mut DELTA_HEADER_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeltaInfoA(
                lpdeltaname: super::super::Foundation::PSTR,
                lpheaderinfo: *mut DELTA_HEADER_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDeltaInfoA(
            lpdeltaname.into_param().abi(),
            ::std::mem::transmute(lpheaderinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaInfoB<'a, Param0: ::windows::runtime::IntoParam<'a, DELTA_INPUT>>(
    delta: Param0,
    lpheaderinfo: *mut DELTA_HEADER_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeltaInfoB(
                delta: DELTA_INPUT,
                lpheaderinfo: *mut DELTA_HEADER_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDeltaInfoB(
            delta.into_param().abi(),
            ::std::mem::transmute(lpheaderinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpdeltaname: Param0,
    lpheaderinfo: *mut DELTA_HEADER_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeltaInfoW(
                lpdeltaname: super::super::Foundation::PWSTR,
                lpheaderinfo: *mut DELTA_HEADER_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDeltaInfoW(
            lpdeltaname.into_param().abi(),
            ::std::mem::transmute(lpheaderinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaSignatureA<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    filetypeset: i64,
    hashalgid: u32,
    lpsourcename: Param2,
    lphash: *mut DELTA_HASH,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeltaSignatureA(
                filetypeset: i64,
                hashalgid: u32,
                lpsourcename: super::super::Foundation::PSTR,
                lphash: *mut DELTA_HASH,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDeltaSignatureA(
            ::std::mem::transmute(filetypeset),
            ::std::mem::transmute(hashalgid),
            lpsourcename.into_param().abi(),
            ::std::mem::transmute(lphash),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaSignatureB<'a, Param2: ::windows::runtime::IntoParam<'a, DELTA_INPUT>>(
    filetypeset: i64,
    hashalgid: u32,
    source: Param2,
    lphash: *mut DELTA_HASH,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeltaSignatureB(
                filetypeset: i64,
                hashalgid: u32,
                source: DELTA_INPUT,
                lphash: *mut DELTA_HASH,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDeltaSignatureB(
            ::std::mem::transmute(filetypeset),
            ::std::mem::transmute(hashalgid),
            source.into_param().abi(),
            ::std::mem::transmute(lphash),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaSignatureW<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    filetypeset: i64,
    hashalgid: u32,
    lpsourcename: Param2,
    lphash: *mut DELTA_HASH,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeltaSignatureW(
                filetypeset: i64,
                hashalgid: u32,
                lpsourcename: super::super::Foundation::PWSTR,
                lphash: *mut DELTA_HASH,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDeltaSignatureW(
            ::std::mem::transmute(filetypeset),
            ::std::mem::transmute(hashalgid),
            lpsourcename.into_param().abi(),
            ::std::mem::transmute(lphash),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    filename: Param0,
    optionflags: u32,
    optiondata: *const ::std::ffi::c_void,
    ignorerangecount: u32,
    ignorerangearray: *const PATCH_IGNORE_RANGE,
    retainrangecount: u32,
    retainrangearray: *const PATCH_RETAIN_RANGE,
    signaturebuffersize: u32,
    signaturebuffer: super::super::Foundation::PSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFilePatchSignatureA(
                filename: super::super::Foundation::PSTR,
                optionflags: u32,
                optiondata: *const ::std::ffi::c_void,
                ignorerangecount: u32,
                ignorerangearray: *const PATCH_IGNORE_RANGE,
                retainrangecount: u32,
                retainrangearray: *const PATCH_RETAIN_RANGE,
                signaturebuffersize: u32,
                signaturebuffer: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFilePatchSignatureA(
            filename.into_param().abi(),
            ::std::mem::transmute(optionflags),
            ::std::mem::transmute(optiondata),
            ::std::mem::transmute(ignorerangecount),
            ::std::mem::transmute(ignorerangearray),
            ::std::mem::transmute(retainrangecount),
            ::std::mem::transmute(retainrangearray),
            ::std::mem::transmute(signaturebuffersize),
            ::std::mem::transmute(signaturebuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureByBuffer(
    filebufferwritable: *mut u8,
    filesize: u32,
    optionflags: u32,
    optiondata: *const ::std::ffi::c_void,
    ignorerangecount: u32,
    ignorerangearray: *const PATCH_IGNORE_RANGE,
    retainrangecount: u32,
    retainrangearray: *const PATCH_RETAIN_RANGE,
    signaturebuffersize: u32,
    signaturebuffer: super::super::Foundation::PSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFilePatchSignatureByBuffer(
                filebufferwritable: *mut u8,
                filesize: u32,
                optionflags: u32,
                optiondata: *const ::std::ffi::c_void,
                ignorerangecount: u32,
                ignorerangearray: *const PATCH_IGNORE_RANGE,
                retainrangecount: u32,
                retainrangearray: *const PATCH_RETAIN_RANGE,
                signaturebuffersize: u32,
                signaturebuffer: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFilePatchSignatureByBuffer(
            ::std::mem::transmute(filebufferwritable),
            ::std::mem::transmute(filesize),
            ::std::mem::transmute(optionflags),
            ::std::mem::transmute(optiondata),
            ::std::mem::transmute(ignorerangecount),
            ::std::mem::transmute(ignorerangearray),
            ::std::mem::transmute(retainrangecount),
            ::std::mem::transmute(retainrangearray),
            ::std::mem::transmute(signaturebuffersize),
            ::std::mem::transmute(signaturebuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureByHandle<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    filehandle: Param0,
    optionflags: u32,
    optiondata: *const ::std::ffi::c_void,
    ignorerangecount: u32,
    ignorerangearray: *const PATCH_IGNORE_RANGE,
    retainrangecount: u32,
    retainrangearray: *const PATCH_RETAIN_RANGE,
    signaturebuffersize: u32,
    signaturebuffer: super::super::Foundation::PSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFilePatchSignatureByHandle(
                filehandle: super::super::Foundation::HANDLE,
                optionflags: u32,
                optiondata: *const ::std::ffi::c_void,
                ignorerangecount: u32,
                ignorerangearray: *const PATCH_IGNORE_RANGE,
                retainrangecount: u32,
                retainrangearray: *const PATCH_RETAIN_RANGE,
                signaturebuffersize: u32,
                signaturebuffer: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFilePatchSignatureByHandle(
            filehandle.into_param().abi(),
            ::std::mem::transmute(optionflags),
            ::std::mem::transmute(optiondata),
            ::std::mem::transmute(ignorerangecount),
            ::std::mem::transmute(ignorerangearray),
            ::std::mem::transmute(retainrangecount),
            ::std::mem::transmute(retainrangearray),
            ::std::mem::transmute(signaturebuffersize),
            ::std::mem::transmute(signaturebuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    filename: Param0,
    optionflags: u32,
    optiondata: *const ::std::ffi::c_void,
    ignorerangecount: u32,
    ignorerangearray: *const PATCH_IGNORE_RANGE,
    retainrangecount: u32,
    retainrangearray: *const PATCH_RETAIN_RANGE,
    signaturebuffersize: u32,
    signaturebuffer: super::super::Foundation::PWSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFilePatchSignatureW(
                filename: super::super::Foundation::PWSTR,
                optionflags: u32,
                optiondata: *const ::std::ffi::c_void,
                ignorerangecount: u32,
                ignorerangearray: *const PATCH_IGNORE_RANGE,
                retainrangecount: u32,
                retainrangearray: *const PATCH_RETAIN_RANGE,
                signaturebuffersize: u32,
                signaturebuffer: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetFilePatchSignatureW(
            filename.into_param().abi(),
            ::std::mem::transmute(optionflags),
            ::std::mem::transmute(optiondata),
            ::std::mem::transmute(ignorerangecount),
            ::std::mem::transmute(ignorerangearray),
            ::std::mem::transmute(retainrangecount),
            ::std::mem::transmute(retainrangearray),
            ::std::mem::transmute(signaturebuffersize),
            ::std::mem::transmute(signaturebuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_ALREADY_INSTALLED: u32 = 3u32;
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_INSTALLED: u32 = 1u32;
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_REFRESHED: u32 = 2u32;
pub const IASSEMBLYCACHEITEM_COMMIT_FLAG_REFRESH: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IASSEMBLYCACHE_UNINSTALL_DISPOSITION(pub u32);
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION =
    IASSEMBLYCACHE_UNINSTALL_DISPOSITION(1u32);
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_STILL_IN_USE: IASSEMBLYCACHE_UNINSTALL_DISPOSITION =
    IASSEMBLYCACHE_UNINSTALL_DISPOSITION(2u32);
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_ALREADY_UNINSTALLED:
    IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(3u32);
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_DELETE_PENDING:
    IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(4u32);
impl ::std::convert::From<u32> for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAssemblyCache(::windows::runtime::IUnknown);
impl IAssemblyCache {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallAssembly<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        dwflags: u32,
        pszassemblyname: Param1,
        prefdata: *mut FUSION_INSTALL_REFERENCE,
        puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
            pszassemblyname.into_param().abi(),
            ::std::mem::transmute(prefdata),
            ::std::mem::transmute(puldisposition),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryAssemblyInfo<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        dwflags: QUERYASMINFO_FLAGS,
        pszassemblyname: Param1,
        pasminfo: *mut ASSEMBLY_INFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
            pszassemblyname.into_param().abi(),
            ::std::mem::transmute(pasminfo),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAssemblyCacheItem<
        'a,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        dwflags: u32,
        pvreserved: *mut ::std::ffi::c_void,
        ppasmitem: *mut ::std::option::Option<IAssemblyCacheItem>,
        pszassemblyname: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pvreserved),
            ::std::mem::transmute(ppasmitem),
            pszassemblyname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Reserved(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallAssembly<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        dwflags: u32,
        pszmanifestfilepath: Param1,
        prefdata: *mut FUSION_INSTALL_REFERENCE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
            pszmanifestfilepath.into_param().abi(),
            ::std::mem::transmute(prefdata),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAssemblyCache {
    type Vtable = IAssemblyCache_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3876052190,
        53709,
        4562,
        [186, 185, 0, 192, 79, 142, 206, 174],
    );
}
impl ::std::convert::From<IAssemblyCache> for ::windows::runtime::IUnknown {
    fn from(value: IAssemblyCache) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAssemblyCache> for ::windows::runtime::IUnknown {
    fn from(value: &IAssemblyCache) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAssemblyCache {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAssemblyCache {
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
pub struct IAssemblyCache_abi(
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
        dwflags: u32,
        pszassemblyname: super::super::Foundation::PWSTR,
        prefdata: *mut FUSION_INSTALL_REFERENCE,
        puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwflags: QUERYASMINFO_FLAGS,
        pszassemblyname: super::super::Foundation::PWSTR,
        pasminfo: *mut ASSEMBLY_INFO,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwflags: u32,
        pvreserved: *mut ::std::ffi::c_void,
        ppasmitem: *mut ::windows::runtime::RawPtr,
        pszassemblyname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppunk: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwflags: u32,
        pszmanifestfilepath: super::super::Foundation::PWSTR,
        prefdata: *mut FUSION_INSTALL_REFERENCE,
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
pub struct IAssemblyCacheItem(::windows::runtime::IUnknown);
impl IAssemblyCacheItem {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        dwflags: u32,
        pszstreamname: Param1,
        dwformat: u32,
        dwformatflags: u32,
        ppistream: *mut ::std::option::Option<super::Com::IStream>,
        pulimaxsize: *mut u64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
            pszstreamname.into_param().abi(),
            ::std::mem::transmute(dwformat),
            ::std::mem::transmute(dwformatflags),
            ::std::mem::transmute(ppistream),
            ::std::mem::transmute(pulimaxsize),
        )
        .ok()
    }
    pub unsafe fn Commit(
        &self,
        dwflags: u32,
        puldisposition: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(puldisposition),
        )
        .ok()
    }
    pub unsafe fn AbortItem(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAssemblyCacheItem {
    type Vtable = IAssemblyCacheItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2654645940,
        53709,
        4562,
        [186, 185, 0, 192, 79, 142, 206, 174],
    );
}
impl ::std::convert::From<IAssemblyCacheItem> for ::windows::runtime::IUnknown {
    fn from(value: IAssemblyCacheItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAssemblyCacheItem> for ::windows::runtime::IUnknown {
    fn from(value: &IAssemblyCacheItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAssemblyCacheItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAssemblyCacheItem {
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
pub struct IAssemblyCacheItem_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwflags: u32,
        pszstreamname: super::super::Foundation::PWSTR,
        dwformat: u32,
        dwformatflags: u32,
        ppistream: *mut ::windows::runtime::RawPtr,
        pulimaxsize: *mut u64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwflags: u32,
        puldisposition: *mut u32,
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
pub struct IAssemblyName(::windows::runtime::IUnknown);
impl IAssemblyName {
    pub unsafe fn SetProperty(
        &self,
        propertyid: u32,
        pvproperty: *mut ::std::ffi::c_void,
        cbproperty: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propertyid),
            ::std::mem::transmute(pvproperty),
            ::std::mem::transmute(cbproperty),
        )
        .ok()
    }
    pub unsafe fn GetProperty(
        &self,
        propertyid: u32,
        pvproperty: *mut ::std::ffi::c_void,
        pcbproperty: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propertyid),
            ::std::mem::transmute(pvproperty),
            ::std::mem::transmute(pcbproperty),
        )
        .ok()
    }
    pub unsafe fn Finalize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(
        &self,
        szdisplayname: super::super::Foundation::PWSTR,
        pccdisplayname: *mut u32,
        dwdisplayflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(szdisplayname),
            ::std::mem::transmute(pccdisplayname),
            ::std::mem::transmute(dwdisplayflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reserved<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        refiid: *const ::windows::runtime::GUID,
        punkreserved1: Param1,
        punkreserved2: Param2,
        szreserved: Param3,
        llreserved: i64,
        pvreserved: *mut ::std::ffi::c_void,
        cbreserved: u32,
        ppreserved: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(refiid),
            punkreserved1.into_param().abi(),
            punkreserved2.into_param().abi(),
            szreserved.into_param().abi(),
            ::std::mem::transmute(llreserved),
            ::std::mem::transmute(pvreserved),
            ::std::mem::transmute(cbreserved),
            ::std::mem::transmute(ppreserved),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(
        &self,
        lpcwbuffer: *mut u32,
        pwzname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpcwbuffer),
            ::std::mem::transmute(pwzname),
        )
        .ok()
    }
    pub unsafe fn GetVersion(
        &self,
        pdwversionhi: *mut u32,
        pdwversionlow: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdwversionhi),
            ::std::mem::transmute(pdwversionlow),
        )
        .ok()
    }
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, IAssemblyName>>(
        &self,
        pname: Param0,
        dwcmpflags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            pname.into_param().abi(),
            ::std::mem::transmute(dwcmpflags),
        )
        .ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IAssemblyName> {
        let mut result__: <IAssemblyName as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IAssemblyName>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAssemblyName {
    type Vtable = IAssemblyName_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3440982976,
        46268,
        4562,
        [152, 51, 0, 192, 79, 195, 29, 46],
    );
}
impl ::std::convert::From<IAssemblyName> for ::windows::runtime::IUnknown {
    fn from(value: IAssemblyName) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAssemblyName> for ::windows::runtime::IUnknown {
    fn from(value: &IAssemblyName) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAssemblyName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAssemblyName {
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
pub struct IAssemblyName_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyid: u32,
        pvproperty: *mut ::std::ffi::c_void,
        cbproperty: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propertyid: u32,
        pvproperty: *mut ::std::ffi::c_void,
        pcbproperty: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szdisplayname: super::super::Foundation::PWSTR,
        pccdisplayname: *mut u32,
        dwdisplayflags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        refiid: *const ::windows::runtime::GUID,
        punkreserved1: ::windows::runtime::RawPtr,
        punkreserved2: ::windows::runtime::RawPtr,
        szreserved: super::super::Foundation::PWSTR,
        llreserved: i64,
        pvreserved: *mut ::std::ffi::c_void,
        cbreserved: u32,
        ppreserved: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpcwbuffer: *mut u32,
        pwzname: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdwversionhi: *mut u32,
        pdwversionlow: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pname: ::windows::runtime::RawPtr,
        dwcmpflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pname: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumMsmDependency(::windows::runtime::IUnknown);
impl IEnumMsmDependency {
    pub unsafe fn Next(
        &self,
        cfetch: u32,
        rgmsmdependencies: *mut ::std::option::Option<IMsmDependency>,
        pcfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cfetch),
            ::std::mem::transmute(rgmsmdependencies),
            ::std::mem::transmute(pcfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, cskip: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cskip),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumMsmDependency> {
        let mut result__: <IEnumMsmDependency as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumMsmDependency>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumMsmDependency {
    type Vtable = IEnumMsmDependency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        182298668,
        11302,
        4562,
        [173, 101, 0, 160, 201, 175, 17, 166],
    );
}
impl ::std::convert::From<IEnumMsmDependency> for ::windows::runtime::IUnknown {
    fn from(value: IEnumMsmDependency) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumMsmDependency> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumMsmDependency) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumMsmDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumMsmDependency {
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
pub struct IEnumMsmDependency_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cfetch: u32,
        rgmsmdependencies: *mut ::windows::runtime::RawPtr,
        pcfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cskip: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pemsmdependencies: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumMsmError(::windows::runtime::IUnknown);
impl IEnumMsmError {
    pub unsafe fn Next(
        &self,
        cfetch: u32,
        rgmsmerrors: *mut ::std::option::Option<IMsmError>,
        pcfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cfetch),
            ::std::mem::transmute(rgmsmerrors),
            ::std::mem::transmute(pcfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, cskip: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cskip),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumMsmError> {
        let mut result__: <IEnumMsmError as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumMsmError>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumMsmError {
    type Vtable = IEnumMsmError_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        182298665,
        11302,
        4562,
        [173, 101, 0, 160, 201, 175, 17, 166],
    );
}
impl ::std::convert::From<IEnumMsmError> for ::windows::runtime::IUnknown {
    fn from(value: IEnumMsmError) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumMsmError> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumMsmError) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumMsmError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumMsmError {
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
pub struct IEnumMsmError_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cfetch: u32,
        rgmsmerrors: *mut ::windows::runtime::RawPtr,
        pcfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cskip: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pemsmerrors: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumMsmString(::windows::runtime::IUnknown);
impl IEnumMsmString {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(
        &self,
        cfetch: u32,
        rgbstrstrings: *mut super::super::Foundation::BSTR,
        pcfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cfetch),
            ::std::mem::transmute(rgbstrstrings),
            ::std::mem::transmute(pcfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, cskip: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cskip),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumMsmString> {
        let mut result__: <IEnumMsmString as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumMsmString>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumMsmString {
    type Vtable = IEnumMsmString_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        182298662,
        11302,
        4562,
        [173, 101, 0, 160, 201, 175, 17, 166],
    );
}
impl ::std::convert::From<IEnumMsmString> for ::windows::runtime::IUnknown {
    fn from(value: IEnumMsmString) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumMsmString> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumMsmString) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumMsmString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumMsmString {
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
pub struct IEnumMsmString_abi(
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
        cfetch: u32,
        rgbstrstrings: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pcfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cskip: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pemsmstrings: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMsmDependencies(::windows::runtime::IUnknown);
impl IMsmDependencies {
    pub unsafe fn Item(&self, item: i32) -> ::windows::runtime::Result<IMsmDependency> {
        let mut result__: <IMsmDependency as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
            &mut result__,
        )
        .from_abi::<IMsmDependency>(result__)
    }
    pub unsafe fn Count(&self, count: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(count),
        )
        .ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMsmDependencies {
    type Vtable = IMsmDependencies_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        182298669,
        11302,
        4562,
        [173, 101, 0, 160, 201, 175, 17, 166],
    );
}
impl ::std::convert::From<IMsmDependencies> for ::windows::runtime::IUnknown {
    fn from(value: IMsmDependencies) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMsmDependencies> for ::windows::runtime::IUnknown {
    fn from(value: &IMsmDependencies) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMsmDependencies {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMsmDependencies {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMsmDependencies> for super::Ole::Automation::IDispatch {
    fn from(value: IMsmDependencies) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMsmDependencies> for super::Ole::Automation::IDispatch {
    fn from(value: &IMsmDependencies) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMsmDependencies {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>
    for &IMsmDependencies
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMsmDependencies_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: i32,
        r#return: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        newenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMsmDependency(::windows::runtime::IUnknown);
impl IMsmDependency {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Module(
        &self,
        module: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(module),
        )
        .ok()
    }
    pub unsafe fn Language(&self, language: *mut i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(language),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(
        &self,
        version: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(version),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMsmDependency {
    type Vtable = IMsmDependency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        182298667,
        11302,
        4562,
        [173, 101, 0, 160, 201, 175, 17, 166],
    );
}
impl ::std::convert::From<IMsmDependency> for ::windows::runtime::IUnknown {
    fn from(value: IMsmDependency) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMsmDependency> for ::windows::runtime::IUnknown {
    fn from(value: &IMsmDependency) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMsmDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMsmDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMsmDependency> for super::Ole::Automation::IDispatch {
    fn from(value: IMsmDependency) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMsmDependency> for super::Ole::Automation::IDispatch {
    fn from(value: &IMsmDependency) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMsmDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMsmDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMsmDependency_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        module: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        language: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        version: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IMsmError(::windows::runtime::IUnknown);
impl IMsmError {
    pub unsafe fn Type(&self, errortype: *mut msmErrorType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(errortype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(
        &self,
        errorpath: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(errorpath),
        )
        .ok()
    }
    pub unsafe fn Language(&self, errorlanguage: *mut i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(errorlanguage),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DatabaseTable(
        &self,
        errortable: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(errortable),
        )
        .ok()
    }
    pub unsafe fn DatabaseKeys(&self) -> ::windows::runtime::Result<IMsmStrings> {
        let mut result__: <IMsmStrings as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMsmStrings>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleTable(
        &self,
        errortable: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(errortable),
        )
        .ok()
    }
    pub unsafe fn ModuleKeys(&self) -> ::windows::runtime::Result<IMsmStrings> {
        let mut result__: <IMsmStrings as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMsmStrings>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMsmError {
    type Vtable = IMsmError_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        182298664,
        11302,
        4562,
        [173, 101, 0, 160, 201, 175, 17, 166],
    );
}
impl ::std::convert::From<IMsmError> for ::windows::runtime::IUnknown {
    fn from(value: IMsmError) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMsmError> for ::windows::runtime::IUnknown {
    fn from(value: &IMsmError) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMsmError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMsmError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMsmError> for super::Ole::Automation::IDispatch {
    fn from(value: IMsmError) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMsmError> for super::Ole::Automation::IDispatch {
    fn from(value: &IMsmError) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMsmError {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMsmError {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMsmError_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        errortype: *mut msmErrorType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        errorpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        errorlanguage: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        errortable: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        errorkeys: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        errortable: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        errorkeys: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMsmErrors(::windows::runtime::IUnknown);
impl IMsmErrors {
    pub unsafe fn Item(&self, item: i32) -> ::windows::runtime::Result<IMsmError> {
        let mut result__: <IMsmError as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
            &mut result__,
        )
        .from_abi::<IMsmError>(result__)
    }
    pub unsafe fn Count(&self, count: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(count),
        )
        .ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMsmErrors {
    type Vtable = IMsmErrors_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        182298666,
        11302,
        4562,
        [173, 101, 0, 160, 201, 175, 17, 166],
    );
}
impl ::std::convert::From<IMsmErrors> for ::windows::runtime::IUnknown {
    fn from(value: IMsmErrors) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMsmErrors> for ::windows::runtime::IUnknown {
    fn from(value: &IMsmErrors) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMsmErrors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMsmErrors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMsmErrors> for super::Ole::Automation::IDispatch {
    fn from(value: IMsmErrors) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMsmErrors> for super::Ole::Automation::IDispatch {
    fn from(value: &IMsmErrors) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMsmErrors {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMsmErrors {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMsmErrors_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: i32,
        r#return: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        newenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMsmGetFiles(::windows::runtime::IUnknown);
impl IMsmGetFiles {
    pub unsafe fn ModuleFiles(&self) -> ::windows::runtime::Result<IMsmStrings> {
        let mut result__: <IMsmStrings as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMsmStrings>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMsmGetFiles {
    type Vtable = IMsmGetFiles_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1883352614,
        11640,
        4562,
        [136, 138, 0, 160, 201, 129, 176, 21],
    );
}
impl ::std::convert::From<IMsmGetFiles> for ::windows::runtime::IUnknown {
    fn from(value: IMsmGetFiles) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMsmGetFiles> for ::windows::runtime::IUnknown {
    fn from(value: &IMsmGetFiles) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMsmGetFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMsmGetFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMsmGetFiles> for super::Ole::Automation::IDispatch {
    fn from(value: IMsmGetFiles) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMsmGetFiles> for super::Ole::Automation::IDispatch {
    fn from(value: &IMsmGetFiles) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMsmGetFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMsmGetFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMsmGetFiles_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        files: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IMsmMerge(::windows::runtime::IUnknown);
impl IMsmMerge {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenDatabase<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        path: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            path.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenModule<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        path: Param0,
        language: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            path.into_param().abi(),
            ::std::mem::transmute(language),
        )
        .ok()
    }
    pub unsafe fn CloseDatabase(&self, commit: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(commit),
        )
        .ok()
    }
    pub unsafe fn CloseModule(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenLog<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        path: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            path.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CloseLog(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Log<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        message: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            message.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Errors(&self) -> ::windows::runtime::Result<IMsmErrors> {
        let mut result__: <IMsmErrors as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMsmErrors>(result__)
    }
    pub unsafe fn Dependencies(&self) -> ::windows::runtime::Result<IMsmDependencies> {
        let mut result__: <IMsmDependencies as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IMsmDependencies>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Merge<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        feature: Param0,
        redirectdir: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            feature.into_param().abi(),
            redirectdir.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        feature: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            feature.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtractCAB<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        filename: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            filename.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtractFiles<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        path: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            path.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMsmMerge {
    type Vtable = IMsmMerge_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        182298670,
        11302,
        4562,
        [173, 101, 0, 160, 201, 175, 17, 166],
    );
}
impl ::std::convert::From<IMsmMerge> for ::windows::runtime::IUnknown {
    fn from(value: IMsmMerge) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMsmMerge> for ::windows::runtime::IUnknown {
    fn from(value: &IMsmMerge) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMsmMerge {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMsmMerge {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMsmMerge> for super::Ole::Automation::IDispatch {
    fn from(value: IMsmMerge) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMsmMerge> for super::Ole::Automation::IDispatch {
    fn from(value: &IMsmMerge) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMsmMerge {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMsmMerge {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMsmMerge_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        language: i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        commit: i16,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        message: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        errors: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dependencies: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        feature: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        redirectdir: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        feature: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IMsmStrings(::windows::runtime::IUnknown);
impl IMsmStrings {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item(
        &self,
        item: i32,
        r#return: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(item),
            ::std::mem::transmute(r#return),
        )
        .ok()
    }
    pub unsafe fn Count(&self, count: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(count),
        )
        .ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMsmStrings {
    type Vtable = IMsmStrings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        182298663,
        11302,
        4562,
        [173, 101, 0, 160, 201, 175, 17, 166],
    );
}
impl ::std::convert::From<IMsmStrings> for ::windows::runtime::IUnknown {
    fn from(value: IMsmStrings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMsmStrings> for ::windows::runtime::IUnknown {
    fn from(value: &IMsmStrings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMsmStrings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMsmStrings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMsmStrings> for super::Ole::Automation::IDispatch {
    fn from(value: IMsmStrings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMsmStrings> for super::Ole::Automation::IDispatch {
    fn from(value: &IMsmStrings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IMsmStrings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IMsmStrings {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::Ole::Automation::IDispatch>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMsmStrings_abi(
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
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        item: i32,
        r#return: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        newenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
pub const INFO_BASE: u32 = 3222229249u32;
pub const INFO_ENTERING_PHASE_I: u32 = 3222229251u32;
pub const INFO_ENTERING_PHASE_II: u32 = 3222229256u32;
pub const INFO_ENTERING_PHASE_III: u32 = 3222229257u32;
pub const INFO_ENTERING_PHASE_IV: u32 = 3222229258u32;
pub const INFO_ENTERING_PHASE_I_VALIDATION: u32 = 3222229250u32;
pub const INFO_ENTERING_PHASE_V: u32 = 3222229259u32;
pub const INFO_GENERATING_METADATA: u32 = 3222229265u32;
pub const INFO_PASSED_MAIN_CONTROL: u32 = 3222229249u32;
pub const INFO_PATCHCACHE_FILEINFO_FAILURE: u32 = 3222229267u32;
pub const INFO_PATCHCACHE_PCI_READFAILURE: u32 = 3222229268u32;
pub const INFO_PATCHCACHE_PCI_WRITEFAILURE: u32 = 3222229269u32;
pub const INFO_PCP_PATH: u32 = 3222229252u32;
pub const INFO_PROPERTY: u32 = 3222229255u32;
pub const INFO_SET_OPTIONS: u32 = 3222229254u32;
pub const INFO_SUCCESSFUL_PATCH_CREATION: u32 = 3222229271u32;
pub const INFO_TEMP_DIR: u32 = 3222229253u32;
pub const INFO_TEMP_DIR_CLEANUP: u32 = 3222229266u32;
pub const INFO_USING_USER_MSI_FOR_PATCH_TABLES: u32 = 3222229270u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct INSTALLFEATUREATTRIBUTE(pub i32);
pub const INSTALLFEATUREATTRIBUTE_FAVORLOCAL: INSTALLFEATUREATTRIBUTE =
    INSTALLFEATUREATTRIBUTE(1i32);
pub const INSTALLFEATUREATTRIBUTE_FAVORSOURCE: INSTALLFEATUREATTRIBUTE =
    INSTALLFEATUREATTRIBUTE(2i32);
pub const INSTALLFEATUREATTRIBUTE_FOLLOWPARENT: INSTALLFEATUREATTRIBUTE =
    INSTALLFEATUREATTRIBUTE(4i32);
pub const INSTALLFEATUREATTRIBUTE_FAVORADVERTISE: INSTALLFEATUREATTRIBUTE =
    INSTALLFEATUREATTRIBUTE(8i32);
pub const INSTALLFEATUREATTRIBUTE_DISALLOWADVERTISE: INSTALLFEATUREATTRIBUTE =
    INSTALLFEATUREATTRIBUTE(16i32);
pub const INSTALLFEATUREATTRIBUTE_NOUNSUPPORTEDADVERTISE: INSTALLFEATUREATTRIBUTE =
    INSTALLFEATUREATTRIBUTE(32i32);
impl ::std::convert::From<i32> for INSTALLFEATUREATTRIBUTE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSTALLFEATUREATTRIBUTE {
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
pub struct INSTALLLEVEL(pub i32);
pub const INSTALLLEVEL_DEFAULT: INSTALLLEVEL = INSTALLLEVEL(0i32);
pub const INSTALLLEVEL_MINIMUM: INSTALLLEVEL = INSTALLLEVEL(1i32);
pub const INSTALLLEVEL_MAXIMUM: INSTALLLEVEL = INSTALLLEVEL(65535i32);
impl ::std::convert::From<i32> for INSTALLLEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSTALLLEVEL {
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
pub struct INSTALLLOGATTRIBUTES(pub i32);
pub const INSTALLLOGATTRIBUTES_APPEND: INSTALLLOGATTRIBUTES = INSTALLLOGATTRIBUTES(1i32);
pub const INSTALLLOGATTRIBUTES_FLUSHEACHLINE: INSTALLLOGATTRIBUTES = INSTALLLOGATTRIBUTES(2i32);
impl ::std::convert::From<i32> for INSTALLLOGATTRIBUTES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSTALLLOGATTRIBUTES {
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
pub struct INSTALLMESSAGE(pub i32);
pub const INSTALLMESSAGE_FATALEXIT: INSTALLMESSAGE = INSTALLMESSAGE(0i32);
pub const INSTALLMESSAGE_ERROR: INSTALLMESSAGE = INSTALLMESSAGE(16777216i32);
pub const INSTALLMESSAGE_WARNING: INSTALLMESSAGE = INSTALLMESSAGE(33554432i32);
pub const INSTALLMESSAGE_USER: INSTALLMESSAGE = INSTALLMESSAGE(50331648i32);
pub const INSTALLMESSAGE_INFO: INSTALLMESSAGE = INSTALLMESSAGE(67108864i32);
pub const INSTALLMESSAGE_FILESINUSE: INSTALLMESSAGE = INSTALLMESSAGE(83886080i32);
pub const INSTALLMESSAGE_RESOLVESOURCE: INSTALLMESSAGE = INSTALLMESSAGE(100663296i32);
pub const INSTALLMESSAGE_OUTOFDISKSPACE: INSTALLMESSAGE = INSTALLMESSAGE(117440512i32);
pub const INSTALLMESSAGE_ACTIONSTART: INSTALLMESSAGE = INSTALLMESSAGE(134217728i32);
pub const INSTALLMESSAGE_ACTIONDATA: INSTALLMESSAGE = INSTALLMESSAGE(150994944i32);
pub const INSTALLMESSAGE_PROGRESS: INSTALLMESSAGE = INSTALLMESSAGE(167772160i32);
pub const INSTALLMESSAGE_COMMONDATA: INSTALLMESSAGE = INSTALLMESSAGE(184549376i32);
pub const INSTALLMESSAGE_INITIALIZE: INSTALLMESSAGE = INSTALLMESSAGE(201326592i32);
pub const INSTALLMESSAGE_TERMINATE: INSTALLMESSAGE = INSTALLMESSAGE(218103808i32);
pub const INSTALLMESSAGE_SHOWDIALOG: INSTALLMESSAGE = INSTALLMESSAGE(234881024i32);
pub const INSTALLMESSAGE_PERFORMANCE: INSTALLMESSAGE = INSTALLMESSAGE(251658240i32);
pub const INSTALLMESSAGE_RMFILESINUSE: INSTALLMESSAGE = INSTALLMESSAGE(419430400i32);
pub const INSTALLMESSAGE_INSTALLSTART: INSTALLMESSAGE = INSTALLMESSAGE(436207616i32);
pub const INSTALLMESSAGE_INSTALLEND: INSTALLMESSAGE = INSTALLMESSAGE(452984832i32);
impl ::std::convert::From<i32> for INSTALLMESSAGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSTALLMESSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const INSTALLMESSAGE_TYPEMASK: i32 = -16777216i32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct INSTALLMODE(pub i32);
pub const INSTALLMODE_NODETECTION_ANY: INSTALLMODE = INSTALLMODE(-4i32);
pub const INSTALLMODE_NOSOURCERESOLUTION: INSTALLMODE = INSTALLMODE(-3i32);
pub const INSTALLMODE_NODETECTION: INSTALLMODE = INSTALLMODE(-2i32);
pub const INSTALLMODE_EXISTING: INSTALLMODE = INSTALLMODE(-1i32);
pub const INSTALLMODE_DEFAULT: INSTALLMODE = INSTALLMODE(0i32);
impl ::std::convert::From<i32> for INSTALLMODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSTALLMODE {
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
pub struct INSTALLOGMODE(pub i32);
pub const INSTALLLOGMODE_FATALEXIT: INSTALLOGMODE = INSTALLOGMODE(1i32);
pub const INSTALLLOGMODE_ERROR: INSTALLOGMODE = INSTALLOGMODE(2i32);
pub const INSTALLLOGMODE_WARNING: INSTALLOGMODE = INSTALLOGMODE(4i32);
pub const INSTALLLOGMODE_USER: INSTALLOGMODE = INSTALLOGMODE(8i32);
pub const INSTALLLOGMODE_INFO: INSTALLOGMODE = INSTALLOGMODE(16i32);
pub const INSTALLLOGMODE_RESOLVESOURCE: INSTALLOGMODE = INSTALLOGMODE(64i32);
pub const INSTALLLOGMODE_OUTOFDISKSPACE: INSTALLOGMODE = INSTALLOGMODE(128i32);
pub const INSTALLLOGMODE_ACTIONSTART: INSTALLOGMODE = INSTALLOGMODE(256i32);
pub const INSTALLLOGMODE_ACTIONDATA: INSTALLOGMODE = INSTALLOGMODE(512i32);
pub const INSTALLLOGMODE_COMMONDATA: INSTALLOGMODE = INSTALLOGMODE(2048i32);
pub const INSTALLLOGMODE_PROPERTYDUMP: INSTALLOGMODE = INSTALLOGMODE(1024i32);
pub const INSTALLLOGMODE_VERBOSE: INSTALLOGMODE = INSTALLOGMODE(4096i32);
pub const INSTALLLOGMODE_EXTRADEBUG: INSTALLOGMODE = INSTALLOGMODE(8192i32);
pub const INSTALLLOGMODE_LOGONLYONERROR: INSTALLOGMODE = INSTALLOGMODE(16384i32);
pub const INSTALLLOGMODE_LOGPERFORMANCE: INSTALLOGMODE = INSTALLOGMODE(32768i32);
pub const INSTALLLOGMODE_PROGRESS: INSTALLOGMODE = INSTALLOGMODE(1024i32);
pub const INSTALLLOGMODE_INITIALIZE: INSTALLOGMODE = INSTALLOGMODE(4096i32);
pub const INSTALLLOGMODE_TERMINATE: INSTALLOGMODE = INSTALLOGMODE(8192i32);
pub const INSTALLLOGMODE_SHOWDIALOG: INSTALLOGMODE = INSTALLOGMODE(16384i32);
pub const INSTALLLOGMODE_FILESINUSE: INSTALLOGMODE = INSTALLOGMODE(32i32);
pub const INSTALLLOGMODE_RMFILESINUSE: INSTALLOGMODE = INSTALLOGMODE(33554432i32);
pub const INSTALLLOGMODE_INSTALLSTART: INSTALLOGMODE = INSTALLOGMODE(67108864i32);
pub const INSTALLLOGMODE_INSTALLEND: INSTALLOGMODE = INSTALLOGMODE(134217728i32);
impl ::std::convert::From<i32> for INSTALLOGMODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSTALLOGMODE {
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
pub struct INSTALLSTATE(pub i32);
pub const INSTALLSTATE_NOTUSED: INSTALLSTATE = INSTALLSTATE(-7i32);
pub const INSTALLSTATE_BADCONFIG: INSTALLSTATE = INSTALLSTATE(-6i32);
pub const INSTALLSTATE_INCOMPLETE: INSTALLSTATE = INSTALLSTATE(-5i32);
pub const INSTALLSTATE_SOURCEABSENT: INSTALLSTATE = INSTALLSTATE(-4i32);
pub const INSTALLSTATE_MOREDATA: INSTALLSTATE = INSTALLSTATE(-3i32);
pub const INSTALLSTATE_INVALIDARG: INSTALLSTATE = INSTALLSTATE(-2i32);
pub const INSTALLSTATE_UNKNOWN: INSTALLSTATE = INSTALLSTATE(-1i32);
pub const INSTALLSTATE_BROKEN: INSTALLSTATE = INSTALLSTATE(0i32);
pub const INSTALLSTATE_ADVERTISED: INSTALLSTATE = INSTALLSTATE(1i32);
pub const INSTALLSTATE_REMOVED: INSTALLSTATE = INSTALLSTATE(1i32);
pub const INSTALLSTATE_ABSENT: INSTALLSTATE = INSTALLSTATE(2i32);
pub const INSTALLSTATE_LOCAL: INSTALLSTATE = INSTALLSTATE(3i32);
pub const INSTALLSTATE_SOURCE: INSTALLSTATE = INSTALLSTATE(4i32);
pub const INSTALLSTATE_DEFAULT: INSTALLSTATE = INSTALLSTATE(5i32);
impl ::std::convert::From<i32> for INSTALLSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSTALLSTATE {
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
pub struct INSTALLTYPE(pub i32);
pub const INSTALLTYPE_DEFAULT: INSTALLTYPE = INSTALLTYPE(0i32);
pub const INSTALLTYPE_NETWORK_IMAGE: INSTALLTYPE = INSTALLTYPE(1i32);
pub const INSTALLTYPE_SINGLE_INSTANCE: INSTALLTYPE = INSTALLTYPE(2i32);
impl ::std::convert::From<i32> for INSTALLTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSTALLTYPE {
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
pub struct INSTALLUILEVEL(pub i32);
pub const INSTALLUILEVEL_NOCHANGE: INSTALLUILEVEL = INSTALLUILEVEL(0i32);
pub const INSTALLUILEVEL_DEFAULT: INSTALLUILEVEL = INSTALLUILEVEL(1i32);
pub const INSTALLUILEVEL_NONE: INSTALLUILEVEL = INSTALLUILEVEL(2i32);
pub const INSTALLUILEVEL_BASIC: INSTALLUILEVEL = INSTALLUILEVEL(3i32);
pub const INSTALLUILEVEL_REDUCED: INSTALLUILEVEL = INSTALLUILEVEL(4i32);
pub const INSTALLUILEVEL_FULL: INSTALLUILEVEL = INSTALLUILEVEL(5i32);
pub const INSTALLUILEVEL_ENDDIALOG: INSTALLUILEVEL = INSTALLUILEVEL(128i32);
pub const INSTALLUILEVEL_PROGRESSONLY: INSTALLUILEVEL = INSTALLUILEVEL(64i32);
pub const INSTALLUILEVEL_HIDECANCEL: INSTALLUILEVEL = INSTALLUILEVEL(32i32);
pub const INSTALLUILEVEL_SOURCERESONLY: INSTALLUILEVEL = INSTALLUILEVEL(256i32);
pub const INSTALLUILEVEL_UACONLY: INSTALLUILEVEL = INSTALLUILEVEL(512i32);
impl ::std::convert::From<i32> for INSTALLUILEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INSTALLUILEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type INSTALLUI_HANDLERA = unsafe extern "system" fn(
    pvcontext: *mut ::std::ffi::c_void,
    imessagetype: u32,
    szmessage: super::super::Foundation::PSTR,
) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type INSTALLUI_HANDLERW = unsafe extern "system" fn(
    pvcontext: *mut ::std::ffi::c_void,
    imessagetype: u32,
    szmessage: super::super::Foundation::PWSTR,
) -> i32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPMApplicationInfo(::windows::runtime::IUnknown);
impl IPMApplicationInfo {
    pub unsafe fn ProductID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn InstanceID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn OfferID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefaultTask(
        &self,
        pdefaulttask: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdefaulttask),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AppTitle(
        &self,
        papptitle: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(papptitle),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IconPath(
        &self,
        pappiconpath: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pappiconpath),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotificationState(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn AppInstallType(&self) -> ::windows::runtime::Result<PM_APPLICATION_INSTALL_TYPE> {
        let mut result__: <PM_APPLICATION_INSTALL_TYPE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PM_APPLICATION_INSTALL_TYPE>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::runtime::Result<PM_APPLICATION_STATE> {
        let mut result__: <PM_APPLICATION_STATE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PM_APPLICATION_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRevoked(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateAvailable(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallDate(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUninstallable(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThemable(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTrial(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallPath(
        &self,
        pinstallpath: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pinstallpath),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DataRoot(
        &self,
        pdataroot: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdataroot),
        )
        .ok()
    }
    pub unsafe fn Genre(&self) -> ::windows::runtime::Result<PM_APP_GENRE> {
        let mut result__: <PM_APP_GENRE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PM_APP_GENRE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Publisher(
        &self,
        ppublisher: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppublisher),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Author(
        &self,
        pauthor: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pauthor),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(
        &self,
        pdescription: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdescription),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(
        &self,
        pversion: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pversion),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvocationInfo(
        &self,
        pimageurn: *mut super::super::Foundation::BSTR,
        pparameters: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pimageurn),
            ::std::mem::transmute(pparameters),
        )
        .ok()
    }
    pub unsafe fn AppPlatMajorVersion(&self) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn AppPlatMinorVersion(&self) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn PublisherID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMultiCore(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SID(
        &self,
        psid: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(psid),
        )
        .ok()
    }
    pub unsafe fn AppPlatMajorVersionLightUp(&self) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u8>(result__)
    }
    pub unsafe fn AppPlatMinorVersionLightUp(&self) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u8>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_UpdateAvailable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        isupdateavailable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            isupdateavailable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_NotificationState<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        isnotified: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            isnotified.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IconPath<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        appiconpath: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            appiconpath.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_UninstallableState<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        isuninstallable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            isuninstallable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinableOnKidZone(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOriginallyPreInstalled(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInstallOnSD(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOptoutOnSD(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOptoutBackupRestore(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_EnterpriseDisabled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        isdisabled: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(
            ::std::mem::transmute_copy(self),
            isdisabled.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_EnterpriseUninstallable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        isuninstallable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(
            ::std::mem::transmute_copy(self),
            isuninstallable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnterpriseDisabled(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnterpriseUninstallable(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVisibleOnAppList(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInboxApp(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn StorageID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartAppBlob(
        &self,
        pblob: *mut PM_STARTAPPBLOB,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pblob),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMovable(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn DeploymentAppEnumerationHubFilter(
        &self,
    ) -> ::windows::runtime::Result<PM_TILE_HUBTYPE> {
        let mut result__: <PM_TILE_HUBTYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PM_TILE_HUBTYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModifiedDate(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOriginallyRestored(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShouldDeferMdilBind(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).54)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFullyPreInstall(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsMdilMaintenanceNeeded<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        fismdilmaintenanceneeded: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(
            ::std::mem::transmute_copy(self),
            fismdilmaintenanceneeded.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_Title<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        apptitle: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(
            ::std::mem::transmute_copy(self),
            apptitle.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPMApplicationInfo {
    type Vtable = IPMApplicationInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1353692554,
        17292,
        16520,
        [151, 137, 248, 196, 137, 152, 41, 199],
    );
}
impl ::std::convert::From<IPMApplicationInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMApplicationInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMApplicationInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMApplicationInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPMApplicationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPMApplicationInfo {
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
pub struct IPMApplicationInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pproductid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pinstanceid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pofferid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdefaulttask: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        papptitle: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pappiconpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisnotified: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pappinstalltype: *mut PM_APPLICATION_INSTALL_TYPE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstate: *mut PM_APPLICATION_STATE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisrevoked: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisupdateavailable: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pinstalldate: *mut super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisuninstallable: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisthemable: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pistrial: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pinstallpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdataroot: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pgenre: *mut PM_APP_GENRE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppublisher: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pauthor: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pversion: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pimageurn: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pparameters: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmajorver: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pminorver: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppublisherid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pismulticore: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmajorver: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pminorver: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isupdateavailable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isnotified: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appiconpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isuninstallable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pispinable: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pispreinstalled: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisinstallonsd: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisoptoutonsd: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisoptoutbackuprestore: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isdisabled: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isuninstallable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isdisabled: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isuninstallable: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisvisible: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisinboxapp: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstorageid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pblob: *mut ::std::mem::ManuallyDrop<PM_STARTAPPBLOB>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pismovable: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hubtype: *mut PM_TILE_HUBTYPE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmodifieddate: *mut super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisrestored: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfdefermdilbind: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfisfullypreinstall: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fismdilmaintenanceneeded: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        apptitle: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IPMApplicationInfoEnumerator(::windows::runtime::IUnknown);
impl IPMApplicationInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IPMApplicationInfo> {
        let mut result__: <IPMApplicationInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IPMApplicationInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMApplicationInfoEnumerator {
    type Vtable = IPMApplicationInfoEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        247736982,
        19782,
        19910,
        [163, 217, 167, 172, 170, 192, 245, 250],
    );
}
impl ::std::convert::From<IPMApplicationInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IPMApplicationInfoEnumerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMApplicationInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IPMApplicationInfoEnumerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMApplicationInfoEnumerator
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMApplicationInfoEnumerator
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
pub struct IPMApplicationInfoEnumerator_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppappinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPMBackgroundServiceAgentInfo(::windows::runtime::IUnknown);
impl IPMBackgroundServiceAgentInfo {
    pub unsafe fn ProductID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskID(
        &self,
        ptaskid: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptaskid),
        )
        .ok()
    }
    pub unsafe fn BSAID(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BGSpecifier(
        &self,
        pbgspecifier: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbgspecifier),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BGName(
        &self,
        pbgname: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbgname),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BGSource(
        &self,
        pbgsource: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbgsource),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BGType(
        &self,
        pbgtype: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbgtype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPeriodic(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsScheduled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsScheduleAllowed(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(
        &self,
        pdescription: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdescription),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLaunchOnBoot(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsScheduled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        isscheduled: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            isscheduled.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsScheduleAllowed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        isscheduleallowed: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            isscheduleallowed.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPMBackgroundServiceAgentInfo {
    type Vtable = IPMBackgroundServiceAgentInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        982206170,
        37516,
        18553,
        [153, 140, 9, 220, 150, 243, 212, 144],
    );
}
impl ::std::convert::From<IPMBackgroundServiceAgentInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMBackgroundServiceAgentInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMBackgroundServiceAgentInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMBackgroundServiceAgentInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMBackgroundServiceAgentInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMBackgroundServiceAgentInfo
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
pub struct IPMBackgroundServiceAgentInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pproductid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptaskid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbsaid: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbgspecifier: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbgname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbgsource: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbgtype: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisperiodic: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisscheduled: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisscheduleallowed: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plaunchonboot: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isscheduled: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isscheduleallowed: super::super::Foundation::BOOL,
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
pub struct IPMBackgroundServiceAgentInfoEnumerator(::windows::runtime::IUnknown);
impl IPMBackgroundServiceAgentInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IPMBackgroundServiceAgentInfo> {
        let mut result__: <IPMBackgroundServiceAgentInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IPMBackgroundServiceAgentInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMBackgroundServiceAgentInfoEnumerator {
    type Vtable = IPMBackgroundServiceAgentInfoEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        418062450,
        43862,
        17331,
        [135, 44, 190, 175, 183, 166, 179, 145],
    );
}
impl ::std::convert::From<IPMBackgroundServiceAgentInfoEnumerator>
    for ::windows::runtime::IUnknown
{
    fn from(value: IPMBackgroundServiceAgentInfoEnumerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMBackgroundServiceAgentInfoEnumerator>
    for ::windows::runtime::IUnknown
{
    fn from(value: &IPMBackgroundServiceAgentInfoEnumerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMBackgroundServiceAgentInfoEnumerator
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMBackgroundServiceAgentInfoEnumerator
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
pub struct IPMBackgroundServiceAgentInfoEnumerator_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppbsainfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPMBackgroundWorkerInfo(::windows::runtime::IUnknown);
impl IPMBackgroundWorkerInfo {
    pub unsafe fn ProductID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskID(
        &self,
        ptaskid: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptaskid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BGName(
        &self,
        pbgname: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbgname),
        )
        .ok()
    }
    pub unsafe fn MaxStartupLatency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn ExpectedRuntime(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBootWorker(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMBackgroundWorkerInfo {
    type Vtable = IPMBackgroundWorkerInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2111066907,
        54207,
        19307,
        [148, 243, 105, 192, 152, 177, 73, 125],
    );
}
impl ::std::convert::From<IPMBackgroundWorkerInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMBackgroundWorkerInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMBackgroundWorkerInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMBackgroundWorkerInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMBackgroundWorkerInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMBackgroundWorkerInfo
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
pub struct IPMBackgroundWorkerInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pproductid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptaskid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbgname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmaxstartuplatency: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pexpectedruntime: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisbootworker: *mut super::super::Foundation::BOOL,
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
pub struct IPMBackgroundWorkerInfoEnumerator(::windows::runtime::IUnknown);
impl IPMBackgroundWorkerInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IPMBackgroundWorkerInfo> {
        let mut result__: <IPMBackgroundWorkerInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IPMBackgroundWorkerInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMBackgroundWorkerInfoEnumerator {
    type Vtable = IPMBackgroundWorkerInfoEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2280946168,
        37080,
        20167,
        [146, 185, 114, 120, 126, 47, 99, 107],
    );
}
impl ::std::convert::From<IPMBackgroundWorkerInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IPMBackgroundWorkerInfoEnumerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMBackgroundWorkerInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IPMBackgroundWorkerInfoEnumerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMBackgroundWorkerInfoEnumerator
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMBackgroundWorkerInfoEnumerator
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
pub struct IPMBackgroundWorkerInfoEnumerator_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppbwinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPMDeploymentManager(::windows::runtime::IUnknown);
impl IPMDeploymentManager {
    pub unsafe fn ReportDownloadBegin<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReportDownloadProgress<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
        usprogress: u16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            ::std::mem::transmute(usprogress),
        )
        .ok()
    }
    pub unsafe fn ReportDownloadComplete<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
        hrresult: ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            ::std::mem::transmute(hrresult),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginInstall(
        &self,
        pinstallinfo: *const PM_INSTALLINFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pinstallinfo),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginUpdate(
        &self,
        pupdateinfo: *const PM_UPDATEINFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pupdateinfo),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDeployPackage(
        &self,
        pinstallinfo: *const PM_INSTALLINFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pinstallinfo),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginUpdateDeployedPackageLegacy(
        &self,
        pupdateinfo: *const PM_UPDATEINFO_LEGACY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pupdateinfo),
        )
        .ok()
    }
    pub unsafe fn BeginUninstall<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginEnterpriseAppInstall(
        &self,
        pinstallinfo: *const PM_INSTALLINFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pinstallinfo),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginEnterpriseAppUpdate(
        &self,
        pupdateinfo: *const PM_UPDATEINFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pupdateinfo),
        )
        .ok()
    }
    pub unsafe fn BeginUpdateLicense<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
        offerid: Param1,
        pblicense: *const u8,
        cblicense: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            offerid.into_param().abi(),
            ::std::mem::transmute(pblicense),
            ::std::mem::transmute(cblicense),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLicenseChallenge<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        packagepath: Param0,
        ppbchallenge: *mut *mut u8,
        pcbchallenge: *mut u32,
        ppbkid: *mut *mut u8,
        pcbkid: *mut u32,
        ppbdeviceid: *mut *mut u8,
        pcbdeviceid: *mut u32,
        ppbsaltvalue: *mut *mut u8,
        pcbsaltvalue: *mut u32,
        ppbkgvvalue: *mut *mut u8,
        pcbkgvvalue: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            packagepath.into_param().abi(),
            ::std::mem::transmute(ppbchallenge),
            ::std::mem::transmute(pcbchallenge),
            ::std::mem::transmute(ppbkid),
            ::std::mem::transmute(pcbkid),
            ::std::mem::transmute(ppbdeviceid),
            ::std::mem::transmute(pcbdeviceid),
            ::std::mem::transmute(ppbsaltvalue),
            ::std::mem::transmute(pcbsaltvalue),
            ::std::mem::transmute(ppbkgvvalue),
            ::std::mem::transmute(pcbkgvvalue),
        )
        .ok()
    }
    pub unsafe fn GetLicenseChallengeByProductID<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
        ppbchallenge: *mut *mut u8,
        pcblicense: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            ::std::mem::transmute(ppbchallenge),
            ::std::mem::transmute(pcblicense),
        )
        .ok()
    }
    pub unsafe fn GetLicenseChallengeByProductID2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
        ppbchallenge: *mut *mut u8,
        pcblicense: *mut u32,
        ppbkid: *mut *mut u8,
        pcbkid: *mut u32,
        ppbdeviceid: *mut *mut u8,
        pcbdeviceid: *mut u32,
        ppbsaltvalue: *mut *mut u8,
        pcbsaltvalue: *mut u32,
        ppbkgvvalue: *mut *mut u8,
        pcbkgvvalue: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            ::std::mem::transmute(ppbchallenge),
            ::std::mem::transmute(pcblicense),
            ::std::mem::transmute(ppbkid),
            ::std::mem::transmute(pcbkid),
            ::std::mem::transmute(ppbdeviceid),
            ::std::mem::transmute(pcbdeviceid),
            ::std::mem::transmute(ppbsaltvalue),
            ::std::mem::transmute(pcbsaltvalue),
            ::std::mem::transmute(ppbkgvvalue),
            ::std::mem::transmute(pcbkgvvalue),
        )
        .ok()
    }
    pub unsafe fn RevokeLicense<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RebindMdilBinaries<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
        filenames: *const super::Com::SAFEARRAY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            ::std::mem::transmute(filenames),
        )
        .ok()
    }
    pub unsafe fn RebindAllMdilBinaries<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
        instanceid: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            instanceid.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegenerateXbf<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
        assemblypaths: *const super::Com::SAFEARRAY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            ::std::mem::transmute(assemblypaths),
        )
        .ok()
    }
    pub unsafe fn GenerateXbfForCurrentLocale<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginProvision<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        productid: Param0,
        xmlpath: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            xmlpath.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginDeprovision<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ReindexSQLCEDatabases<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetApplicationsNeedMaintenance(
        &self,
        requiredmaintenanceoperations: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(requiredmaintenanceoperations),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn UpdateChamberProfile<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnterprisePolicyIsApplicationAllowed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        productid: Param0,
        publishername: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            publishername.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginUpdateDeployedPackage(
        &self,
        pupdateinfo: *const PM_UPDATEINFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pupdateinfo),
        )
        .ok()
    }
    pub unsafe fn ReportRestoreCancelled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResolveResourceString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        resourcestring: Param0,
        presolvedresourcestring: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            resourcestring.into_param().abi(),
            ::std::mem::transmute(presolvedresourcestring),
        )
        .ok()
    }
    pub unsafe fn UpdateCapabilitiesForModernApps(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ReportDownloadStatusUpdate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn BeginUninstallWithOptions<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
        removaloptions: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            ::std::mem::transmute(removaloptions),
        )
        .ok()
    }
    pub unsafe fn BindDeferredMdilBinaries(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateXamlLightupXbfForCurrentLocale<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        packagefamilyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            packagefamilyname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn AddLicenseForAppx<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
        pblicense: *const u8,
        cblicense: u32,
        pbplayreadyheader: *const u8,
        cbplayreadyheader: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            ::std::mem::transmute(pblicense),
            ::std::mem::transmute(cblicense),
            ::std::mem::transmute(pbplayreadyheader),
            ::std::mem::transmute(cbplayreadyheader),
        )
        .ok()
    }
    pub unsafe fn FixJunctionsForAppsOnSDCard(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPMDeploymentManager {
    type Vtable = IPMDeploymentManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        905414138,
        6521,
        19083,
        [188, 143, 253, 112, 235, 13, 21, 68],
    );
}
impl ::std::convert::From<IPMDeploymentManager> for ::windows::runtime::IUnknown {
    fn from(value: IPMDeploymentManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMDeploymentManager> for ::windows::runtime::IUnknown {
    fn from(value: &IPMDeploymentManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPMDeploymentManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPMDeploymentManager {
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
pub struct IPMDeploymentManager_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        usprogress: u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        hrresult: ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pinstallinfo: *const ::std::mem::ManuallyDrop<PM_INSTALLINFO>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pupdateinfo: *const ::std::mem::ManuallyDrop<PM_UPDATEINFO>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pinstallinfo: *const ::std::mem::ManuallyDrop<PM_INSTALLINFO>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pupdateinfo: *const ::std::mem::ManuallyDrop<PM_UPDATEINFO_LEGACY>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pinstallinfo: *const ::std::mem::ManuallyDrop<PM_INSTALLINFO>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pupdateinfo: *const ::std::mem::ManuallyDrop<PM_UPDATEINFO>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        offerid: ::windows::runtime::GUID,
        pblicense: *const u8,
        cblicense: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        packagepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppbchallenge: *mut *mut u8,
        pcbchallenge: *mut u32,
        ppbkid: *mut *mut u8,
        pcbkid: *mut u32,
        ppbdeviceid: *mut *mut u8,
        pcbdeviceid: *mut u32,
        ppbsaltvalue: *mut *mut u8,
        pcbsaltvalue: *mut u32,
        ppbkgvvalue: *mut *mut u8,
        pcbkgvvalue: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        ppbchallenge: *mut *mut u8,
        pcblicense: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        ppbchallenge: *mut *mut u8,
        pcblicense: *mut u32,
        ppbkid: *mut *mut u8,
        pcbkid: *mut u32,
        ppbdeviceid: *mut *mut u8,
        pcbdeviceid: *mut u32,
        ppbsaltvalue: *mut *mut u8,
        pcbsaltvalue: *mut u32,
        ppbkgvvalue: *mut *mut u8,
        pcbkgvvalue: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        filenames: *const super::Com::SAFEARRAY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        instanceid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        assemblypaths: *const super::Com::SAFEARRAY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        xmlpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        requiredmaintenanceoperations: u32,
        pcapplications: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        publishername: super::super::Foundation::PWSTR,
        pisallowed: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pupdateinfo: *const ::std::mem::ManuallyDrop<PM_UPDATEINFO>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        resourcestring: super::super::Foundation::PWSTR,
        presolvedresourcestring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        removaloptions: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        packagefamilyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        pblicense: *const u8,
        cblicense: u32,
        pbplayreadyheader: *const u8,
        cbplayreadyheader: u32,
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
pub struct IPMEnumerationManager(::windows::runtime::IUnknown);
impl IPMEnumerationManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllApplications<'a, Param1: ::windows::runtime::IntoParam<'a, PM_ENUM_FILTER>>(
        &self,
        ppappenum: *mut ::std::option::Option<IPMApplicationInfoEnumerator>,
        filter: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppappenum),
            filter.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllTiles<'a, Param1: ::windows::runtime::IntoParam<'a, PM_ENUM_FILTER>>(
        &self,
        pptileenum: *mut ::std::option::Option<IPMTileInfoEnumerator>,
        filter: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pptileenum),
            filter.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllTasks<'a, Param1: ::windows::runtime::IntoParam<'a, PM_ENUM_FILTER>>(
        &self,
        pptaskenum: *mut ::std::option::Option<IPMTaskInfoEnumerator>,
        filter: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pptaskenum),
            filter.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllExtensions<'a, Param1: ::windows::runtime::IntoParam<'a, PM_ENUM_FILTER>>(
        &self,
        ppextensionenum: *mut ::std::option::Option<IPMExtensionInfoEnumerator>,
        filter: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppextensionenum),
            filter.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllBackgroundServiceAgents<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, PM_ENUM_FILTER>,
    >(
        &self,
        ppbsaenum: *mut ::std::option::Option<IPMBackgroundServiceAgentInfoEnumerator>,
        filter: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppbsaenum),
            filter.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllBackgroundWorkers<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, PM_ENUM_FILTER>,
    >(
        &self,
        ppbswenum: *mut ::std::option::Option<IPMBackgroundWorkerInfoEnumerator>,
        filter: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppbswenum),
            filter.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ApplicationInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
    ) -> ::windows::runtime::Result<IPMApplicationInfo> {
        let mut result__: <IPMApplicationInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IPMApplicationInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TileInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        productid: Param0,
        tileid: Param1,
    ) -> ::windows::runtime::Result<IPMTileInfo> {
        let mut result__: <IPMTileInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            tileid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IPMTileInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        productid: Param0,
        taskid: Param1,
    ) -> ::windows::runtime::Result<IPMTaskInfo> {
        let mut result__: <IPMTaskInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            taskid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IPMTaskInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskInfoEx<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        productid: Param0,
        taskid: Param1,
    ) -> ::windows::runtime::Result<IPMTaskInfo> {
        let mut result__: <IPMTaskInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            taskid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IPMTaskInfo>(result__)
    }
    pub unsafe fn BackgroundServiceAgentInfo(
        &self,
        bsaid: u32,
    ) -> ::windows::runtime::Result<IPMBackgroundServiceAgentInfo> {
        let mut result__: <IPMBackgroundServiceAgentInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bsaid),
            &mut result__,
        )
        .from_abi::<IPMBackgroundServiceAgentInfo>(result__)
    }
    pub unsafe fn AllLiveTileJobs(
        &self,
    ) -> ::windows::runtime::Result<IPMLiveTileJobInfoEnumerator> {
        let mut result__: <IPMLiveTileJobInfoEnumerator as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IPMLiveTileJobInfoEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LiveTileJob<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        productid: Param0,
        tileid: Param1,
        recurrencetype: PM_LIVETILE_RECURRENCE_TYPE,
    ) -> ::windows::runtime::Result<IPMLiveTileJobInfo> {
        let mut result__: <IPMLiveTileJobInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            tileid.into_param().abi(),
            ::std::mem::transmute(recurrencetype),
            &mut result__,
        )
        .from_abi::<IPMLiveTileJobInfo>(result__)
    }
    pub unsafe fn ApplicationInfoExternal<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        &self,
        productid: Param0,
    ) -> ::windows::runtime::Result<IPMApplicationInfo> {
        let mut result__: <IPMApplicationInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            productid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IPMApplicationInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileHandlerGenericLogo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        filetype: Param0,
        logosize: PM_LOGO_SIZE,
        plogo: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            filetype.into_param().abi(),
            ::std::mem::transmute(logosize),
            ::std::mem::transmute(plogo),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplicationInfoFromAccessClaims<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        sysappid0: Param0,
        sysappid1: Param1,
    ) -> ::windows::runtime::Result<IPMApplicationInfo> {
        let mut result__: <IPMApplicationInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            sysappid0.into_param().abi(),
            sysappid1.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IPMApplicationInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartTileEnumeratorBlob<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, PM_ENUM_FILTER>,
    >(
        &self,
        filter: Param0,
        pctiles: *mut u32,
        pptileblobs: *mut *mut PM_STARTTILEBLOB,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            filter.into_param().abi(),
            ::std::mem::transmute(pctiles),
            ::std::mem::transmute(pptileblobs),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartAppEnumeratorBlob<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, PM_ENUM_FILTER>,
    >(
        &self,
        filter: Param0,
        pcapps: *mut u32,
        ppappblobs: *mut *mut PM_STARTAPPBLOB,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            filter.into_param().abi(),
            ::std::mem::transmute(pcapps),
            ::std::mem::transmute(ppappblobs),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPMEnumerationManager {
    type Vtable = IPMEnumerationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1770870722,
        10541,
        19699,
        [183, 60, 217, 90, 105, 34, 237, 154],
    );
}
impl ::std::convert::From<IPMEnumerationManager> for ::windows::runtime::IUnknown {
    fn from(value: IPMEnumerationManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMEnumerationManager> for ::windows::runtime::IUnknown {
    fn from(value: &IPMEnumerationManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPMEnumerationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMEnumerationManager
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
pub struct IPMEnumerationManager_abi(
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
        ppappenum: *mut ::windows::runtime::RawPtr,
        filter: ::std::mem::ManuallyDrop<PM_ENUM_FILTER>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptileenum: *mut ::windows::runtime::RawPtr,
        filter: ::std::mem::ManuallyDrop<PM_ENUM_FILTER>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptaskenum: *mut ::windows::runtime::RawPtr,
        filter: ::std::mem::ManuallyDrop<PM_ENUM_FILTER>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppextensionenum: *mut ::windows::runtime::RawPtr,
        filter: ::std::mem::ManuallyDrop<PM_ENUM_FILTER>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppbsaenum: *mut ::windows::runtime::RawPtr,
        filter: ::std::mem::ManuallyDrop<PM_ENUM_FILTER>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppbswenum: *mut ::windows::runtime::RawPtr,
        filter: ::std::mem::ManuallyDrop<PM_ENUM_FILTER>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        ppappinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        tileid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pptileinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        taskid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pptaskinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        taskid: super::super::Foundation::PWSTR,
        pptaskinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bsaid: u32,
        pptaskinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pplivetilejobenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        tileid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        recurrencetype: PM_LIVETILE_RECURRENCE_TYPE,
        pplivetilejobinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        productid: ::windows::runtime::GUID,
        ppappinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filetype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        logosize: PM_LOGO_SIZE,
        plogo: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sysappid0: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        sysappid1: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppappinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filter: ::std::mem::ManuallyDrop<PM_ENUM_FILTER>,
        pctiles: *mut u32,
        pptileblobs: *mut *mut PM_STARTTILEBLOB,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filter: ::std::mem::ManuallyDrop<PM_ENUM_FILTER>,
        pcapps: *mut u32,
        ppappblobs: *mut *mut PM_STARTAPPBLOB,
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
pub struct IPMExtensionCachedFileUpdaterInfo(::windows::runtime::IUnknown);
impl IPMExtensionCachedFileUpdaterInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsUpdates(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMExtensionCachedFileUpdaterInfo {
    type Vtable = IPMExtensionCachedFileUpdaterInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3805771017,
        20056,
        19369,
        [175, 126, 182, 66, 227, 112, 225, 176],
    );
}
impl ::std::convert::From<IPMExtensionCachedFileUpdaterInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMExtensionCachedFileUpdaterInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMExtensionCachedFileUpdaterInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMExtensionCachedFileUpdaterInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMExtensionCachedFileUpdaterInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMExtensionCachedFileUpdaterInfo
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
pub struct IPMExtensionCachedFileUpdaterInfo_abi(
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
        psupportsupdates: *mut super::super::Foundation::BOOL,
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
pub struct IPMExtensionContractInfo(::windows::runtime::IUnknown);
impl IPMExtensionContractInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvocationInfo(
        &self,
        paumid: *mut super::super::Foundation::BSTR,
        pargs: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(paumid),
            ::std::mem::transmute(pargs),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPMExtensionContractInfo {
    type Vtable = IPMExtensionContractInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3848692595,
        31649,
        18044,
        [184, 25, 177, 117, 219, 28, 41, 91],
    );
}
impl ::std::convert::From<IPMExtensionContractInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMExtensionContractInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMExtensionContractInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMExtensionContractInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMExtensionContractInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMExtensionContractInfo
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
pub struct IPMExtensionContractInfo_abi(
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
        paumid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pargs: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IPMExtensionFileExtensionInfo(::windows::runtime::IUnknown);
impl IPMExtensionFileExtensionInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(
        &self,
        pname: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pname),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(
        &self,
        pdisplayname: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdisplayname),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Logo(
        &self,
        logosize: PM_LOGO_SIZE,
        plogo: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(logosize),
            ::std::mem::transmute(plogo),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContentType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        filetype: Param0,
        pcontenttype: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            filetype.into_param().abi(),
            ::std::mem::transmute(pcontenttype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        contenttype: Param0,
        pfiletype: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            contenttype.into_param().abi(),
            ::std::mem::transmute(pfiletype),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvocationInfo(
        &self,
        pimageurn: *mut super::super::Foundation::BSTR,
        pparameters: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pimageurn),
            ::std::mem::transmute(pparameters),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllFileTypes(
        &self,
        pcbtypes: *mut u32,
        pptypes: *mut *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcbtypes),
            ::std::mem::transmute(pptypes),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPMExtensionFileExtensionInfo {
    type Vtable = IPMExtensionFileExtensionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1804061548,
        2952,
        18825,
        [164, 236, 3, 55, 20, 247, 16, 212],
    );
}
impl ::std::convert::From<IPMExtensionFileExtensionInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMExtensionFileExtensionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMExtensionFileExtensionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMExtensionFileExtensionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMExtensionFileExtensionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMExtensionFileExtensionInfo
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
pub struct IPMExtensionFileExtensionInfo_abi(
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
        pname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdisplayname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        logosize: PM_LOGO_SIZE,
        plogo: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filetype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pcontenttype: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        contenttype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pfiletype: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pimageurn: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pparameters: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcbtypes: *mut u32,
        pptypes: *mut *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IPMExtensionFileOpenPickerInfo(::windows::runtime::IUnknown);
impl IPMExtensionFileOpenPickerInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllFileTypes(
        &self,
        pctypes: *mut u32,
        pptypes: *mut *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pctypes),
            ::std::mem::transmute(pptypes),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsAllFileTypes(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMExtensionFileOpenPickerInfo {
    type Vtable = IPMExtensionFileOpenPickerInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1841896741,
        38406,
        16908,
        [154, 120, 224, 52, 163, 65, 131, 69],
    );
}
impl ::std::convert::From<IPMExtensionFileOpenPickerInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMExtensionFileOpenPickerInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMExtensionFileOpenPickerInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMExtensionFileOpenPickerInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMExtensionFileOpenPickerInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMExtensionFileOpenPickerInfo
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
pub struct IPMExtensionFileOpenPickerInfo_abi(
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
        pctypes: *mut u32,
        pptypes: *mut *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psupportsalltypes: *mut super::super::Foundation::BOOL,
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
pub struct IPMExtensionFileSavePickerInfo(::windows::runtime::IUnknown);
impl IPMExtensionFileSavePickerInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllFileTypes(
        &self,
        pctypes: *mut u32,
        pptypes: *mut *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pctypes),
            ::std::mem::transmute(pptypes),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsAllFileTypes(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMExtensionFileSavePickerInfo {
    type Vtable = IPMExtensionFileSavePickerInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        939547834,
        63514,
        18750,
        [160, 248, 146, 44, 134, 128, 218, 67],
    );
}
impl ::std::convert::From<IPMExtensionFileSavePickerInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMExtensionFileSavePickerInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMExtensionFileSavePickerInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMExtensionFileSavePickerInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMExtensionFileSavePickerInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMExtensionFileSavePickerInfo
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
pub struct IPMExtensionFileSavePickerInfo_abi(
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
        pctypes: *mut u32,
        pptypes: *mut *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psupportsalltypes: *mut super::super::Foundation::BOOL,
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
pub struct IPMExtensionInfo(::windows::runtime::IUnknown);
impl IPMExtensionInfo {
    pub unsafe fn SupplierPID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupplierTaskID(
        &self,
        psuppliertid: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(psuppliertid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(
        &self,
        ptitle: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptitle),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IconPath(
        &self,
        piconpath: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(piconpath),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtraFile(
        &self,
        pfilepath: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pfilepath),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvocationInfo(
        &self,
        pimageurn: *mut super::super::Foundation::BSTR,
        pparameters: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pimageurn),
            ::std::mem::transmute(pparameters),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPMExtensionInfo {
    type Vtable = IPMExtensionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1236065913,
        38792,
        19722,
        [138, 160, 23, 70, 175, 219, 158, 157],
    );
}
impl ::std::convert::From<IPMExtensionInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMExtensionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMExtensionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMExtensionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPMExtensionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPMExtensionInfo {
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
pub struct IPMExtensionInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psupplierpid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psuppliertid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptitle: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        piconpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfilepath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pimageurn: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pparameters: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IPMExtensionInfoEnumerator(::windows::runtime::IUnknown);
impl IPMExtensionInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IPMExtensionInfo> {
        let mut result__: <IPMExtensionInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IPMExtensionInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMExtensionInfoEnumerator {
    type Vtable = IPMExtensionInfoEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1077649026,
        4465,
        17779,
        [142, 111, 111, 51, 243, 155, 131, 221],
    );
}
impl ::std::convert::From<IPMExtensionInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IPMExtensionInfoEnumerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMExtensionInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IPMExtensionInfoEnumerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMExtensionInfoEnumerator
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMExtensionInfoEnumerator
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
pub struct IPMExtensionInfoEnumerator_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppextensioninfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPMExtensionProtocolInfo(::windows::runtime::IUnknown);
impl IPMExtensionProtocolInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Protocol(
        &self,
        pprotocol: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pprotocol),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvocationInfo(
        &self,
        pimageurn: *mut super::super::Foundation::BSTR,
        pparameters: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pimageurn),
            ::std::mem::transmute(pparameters),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPMExtensionProtocolInfo {
    type Vtable = IPMExtensionProtocolInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        507486262,
        20971,
        17491,
        [186, 255, 184, 216, 228, 180, 108, 142],
    );
}
impl ::std::convert::From<IPMExtensionProtocolInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMExtensionProtocolInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMExtensionProtocolInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMExtensionProtocolInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMExtensionProtocolInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMExtensionProtocolInfo
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
pub struct IPMExtensionProtocolInfo_abi(
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
        pprotocol: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pimageurn: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pparameters: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IPMExtensionShareTargetInfo(::windows::runtime::IUnknown);
impl IPMExtensionShareTargetInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllFileTypes(
        &self,
        pctypes: *mut u32,
        pptypes: *mut *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pctypes),
            ::std::mem::transmute(pptypes),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllDataFormats(
        &self,
        pcdataformats: *mut u32,
        ppdataformats: *mut *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcdataformats),
            ::std::mem::transmute(ppdataformats),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsAllFileTypes(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMExtensionShareTargetInfo {
    type Vtable = IPMExtensionShareTargetInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1416754315,
        50780,
        18006,
        [140, 112, 36, 46, 49, 25, 95, 234],
    );
}
impl ::std::convert::From<IPMExtensionShareTargetInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMExtensionShareTargetInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMExtensionShareTargetInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMExtensionShareTargetInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMExtensionShareTargetInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMExtensionShareTargetInfo
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
pub struct IPMExtensionShareTargetInfo_abi(
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
        pctypes: *mut u32,
        pptypes: *mut *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcdataformats: *mut u32,
        ppdataformats: *mut *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psupportsalltypes: *mut super::super::Foundation::BOOL,
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
pub struct IPMLiveTileJobInfo(::windows::runtime::IUnknown);
impl IPMLiveTileJobInfo {
    pub unsafe fn ProductID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TileID(
        &self,
        ptileid: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptileid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NextSchedule(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_NextSchedule<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::FILETIME>,
    >(
        &self,
        ftnextschedule: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ftnextschedule.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartSchedule(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_StartSchedule<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::FILETIME>,
    >(
        &self,
        ftstartschedule: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ftstartschedule.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn IntervalDuration(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn set_IntervalDuration(
        &self,
        ulintervalduration: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ulintervalduration),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunForever(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_RunForever<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        frunforever: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            frunforever.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn MaxRunCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn set_MaxRunCount(&self, ulmaxruncount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ulmaxruncount),
        )
        .ok()
    }
    pub unsafe fn RunCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn set_RunCount(&self, ulruncount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ulruncount),
        )
        .ok()
    }
    pub unsafe fn RecurrenceType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn set_RecurrenceType(
        &self,
        ulrecurrencetype: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ulrecurrencetype),
        )
        .ok()
    }
    pub unsafe fn TileXML(
        &self,
        ptilexml: *mut *mut u8,
        pcbtilexml: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptilexml),
            ::std::mem::transmute(pcbtilexml),
        )
        .ok()
    }
    pub unsafe fn set_TileXML(
        &self,
        ptilexml: *const u8,
        cbtilexml: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptilexml),
            ::std::mem::transmute(cbtilexml),
        )
        .ok()
    }
    pub unsafe fn UrlXML(
        &self,
        purlxml: *mut *mut u8,
        pcburlxml: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(purlxml),
            ::std::mem::transmute(pcburlxml),
        )
        .ok()
    }
    pub unsafe fn set_UrlXML(
        &self,
        purlxml: *const u8,
        cburlxml: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(purlxml),
            ::std::mem::transmute(cburlxml),
        )
        .ok()
    }
    pub unsafe fn AttemptCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn set_AttemptCount(&self, ulattemptcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ulattemptcount),
        )
        .ok()
    }
    pub unsafe fn DownloadState(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn set_DownloadState(&self, uldownloadstate: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uldownloadstate),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPMLiveTileJobInfo {
    type Vtable = IPMLiveTileJobInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1611245599,
        18192,
        18071,
        [181, 246, 34, 8, 246, 5, 123, 142],
    );
}
impl ::std::convert::From<IPMLiveTileJobInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMLiveTileJobInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMLiveTileJobInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMLiveTileJobInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPMLiveTileJobInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPMLiveTileJobInfo {
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
pub struct IPMLiveTileJobInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pproductid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptileid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pnextschedule: *mut super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ftnextschedule: super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstartschedule: *mut super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ftstartschedule: super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pintervalduration: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ulintervalduration: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        isrunforever: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        frunforever: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmaxruncount: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ulmaxruncount: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pruncount: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ulruncount: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        precurrencetype: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ulrecurrencetype: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptilexml: *mut *mut u8,
        pcbtilexml: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptilexml: *const u8,
        cbtilexml: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        purlxml: *mut *mut u8,
        pcburlxml: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        purlxml: *const u8,
        cburlxml: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pattemptcount: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ulattemptcount: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdownloadstate: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uldownloadstate: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPMLiveTileJobInfoEnumerator(::windows::runtime::IUnknown);
impl IPMLiveTileJobInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IPMLiveTileJobInfo> {
        let mut result__: <IPMLiveTileJobInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IPMLiveTileJobInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMLiveTileJobInfoEnumerator {
    type Vtable = IPMLiveTileJobInfoEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3154388354,
        37909,
        20278,
        [159, 153, 6, 241, 4, 192, 124, 3],
    );
}
impl ::std::convert::From<IPMLiveTileJobInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IPMLiveTileJobInfoEnumerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMLiveTileJobInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IPMLiveTileJobInfoEnumerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMLiveTileJobInfoEnumerator
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMLiveTileJobInfoEnumerator
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
pub struct IPMLiveTileJobInfoEnumerator_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pplivetilejobinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPMTaskInfo(::windows::runtime::IUnknown);
impl IPMTaskInfo {
    pub unsafe fn ProductID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskID(
        &self,
        ptaskid: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptaskid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NavigationPage(
        &self,
        pnavigationpage: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pnavigationpage),
        )
        .ok()
    }
    pub unsafe fn TaskTransition(&self) -> ::windows::runtime::Result<PM_TASK_TRANSITION> {
        let mut result__: <PM_TASK_TRANSITION as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PM_TASK_TRANSITION>(result__)
    }
    pub unsafe fn RuntimeType(&self) -> ::windows::runtime::Result<PACKMAN_RUNTIME> {
        let mut result__: <PACKMAN_RUNTIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PACKMAN_RUNTIME>(result__)
    }
    pub unsafe fn ActivationPolicy(&self) -> ::windows::runtime::Result<PM_ACTIVATION_POLICY> {
        let mut result__: <PM_ACTIVATION_POLICY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PM_ACTIVATION_POLICY>(result__)
    }
    pub unsafe fn TaskType(&self) -> ::windows::runtime::Result<PM_TASK_TYPE> {
        let mut result__: <PM_TASK_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PM_TASK_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvocationInfo(
        &self,
        pimageurn: *mut super::super::Foundation::BSTR,
        pparameters: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pimageurn),
            ::std::mem::transmute(pparameters),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImagePath(
        &self,
        pimagepath: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pimagepath),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImageParams(
        &self,
        pimageparams: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pimageparams),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallRootFolder(
        &self,
        pinstallrootfolder: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pinstallrootfolder),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DataRootFolder(
        &self,
        pdatarootfolder: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdatarootfolder),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSingleInstanceHost(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInteropEnabled(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn ApplicationState(&self) -> ::windows::runtime::Result<PM_APPLICATION_STATE> {
        let mut result__: <PM_APPLICATION_STATE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PM_APPLICATION_STATE>(result__)
    }
    pub unsafe fn InstallType(&self) -> ::windows::runtime::Result<PM_APPLICATION_INSTALL_TYPE> {
        let mut result__: <PM_APPLICATION_INSTALL_TYPE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PM_APPLICATION_INSTALL_TYPE>(result__)
    }
    pub unsafe fn Version(
        &self,
        ptargetmajorversion: *mut u8,
        ptargetminorversion: *mut u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptargetmajorversion),
            ::std::mem::transmute(ptargetminorversion),
        )
        .ok()
    }
    pub unsafe fn BitsPerPixel(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressesDehydration(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BackgroundExecutionAbilities(
        &self,
        pbackgroundexecutionabilities: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbackgroundexecutionabilities),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOptedForExtendedMem(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMTaskInfo {
    type Vtable = IPMTaskInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3206384691,
        7157,
        20192,
        [181, 73, 107, 157, 211, 131, 73, 66],
    );
}
impl ::std::convert::From<IPMTaskInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMTaskInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMTaskInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMTaskInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPMTaskInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPMTaskInfo {
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
pub struct IPMTaskInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pproductid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptaskid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pnavigationpage: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptasktransition: *mut PM_TASK_TRANSITION,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pruntimetype: *mut PACKMAN_RUNTIME,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pactivationpolicy: *mut PM_ACTIVATION_POLICY,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptasktype: *mut PM_TASK_TYPE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pimageurn: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pparameters: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pimagepath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pimageparams: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pinstallrootfolder: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdatarootfolder: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pissingleinstancehost: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisinteropenabled: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        papplicationstate: *mut PM_APPLICATION_STATE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pinstalltype: *mut PM_APPLICATION_INSTALL_TYPE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptargetmajorversion: *mut u8,
        ptargetminorversion: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbitsperpixel: *mut u16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psuppressesdehydration: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbackgroundexecutionabilities: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisoptedin: *mut super::super::Foundation::BOOL,
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
pub struct IPMTaskInfoEnumerator(::windows::runtime::IUnknown);
impl IPMTaskInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IPMTaskInfo> {
        let mut result__: <IPMTaskInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IPMTaskInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMTaskInfoEnumerator {
    type Vtable = IPMTaskInfoEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        103854328,
        3004,
        18465,
        [190, 116, 199, 153, 81, 102, 237, 42],
    );
}
impl ::std::convert::From<IPMTaskInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IPMTaskInfoEnumerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMTaskInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IPMTaskInfoEnumerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPMTaskInfoEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMTaskInfoEnumerator
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
pub struct IPMTaskInfoEnumerator_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptaskinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPMTileInfo(::windows::runtime::IUnknown);
impl IPMTileInfo {
    pub unsafe fn ProductID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TileID(
        &self,
        ptileid: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptileid),
        )
        .ok()
    }
    pub unsafe fn TemplateType(&self) -> ::windows::runtime::Result<TILE_TEMPLATE_TYPE> {
        let mut result__: <TILE_TEMPLATE_TYPE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<TILE_TEMPLATE_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HubPinnedState(
        &self,
        hubtype: PM_TILE_HUBTYPE,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hubtype),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn HubPosition(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hubtype),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsNotified(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDefault(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskID(
        &self,
        ptaskid: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ptaskid),
        )
        .ok()
    }
    pub unsafe fn TileType(&self) -> ::windows::runtime::Result<PM_STARTTILE_TYPE> {
        let mut result__: <PM_STARTTILE_TYPE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PM_STARTTILE_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThemable(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn PropertyById(
        &self,
        propid: u32,
    ) -> ::windows::runtime::Result<IPMTilePropertyInfo> {
        let mut result__: <IPMTilePropertyInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(propid),
            &mut result__,
        )
        .from_abi::<IPMTilePropertyInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvocationInfo(
        &self,
        pimageurn: *mut super::super::Foundation::BSTR,
        pparameters: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pimageurn),
            ::std::mem::transmute(pparameters),
        )
        .ok()
    }
    pub unsafe fn PropertyEnum(&self) -> ::windows::runtime::Result<IPMTilePropertyEnumerator> {
        let mut result__: <IPMTilePropertyEnumerator as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IPMTilePropertyEnumerator>(result__)
    }
    pub unsafe fn HubTileSize(
        &self,
        hubtype: PM_TILE_HUBTYPE,
    ) -> ::windows::runtime::Result<PM_TILE_SIZE> {
        let mut result__: <PM_TILE_SIZE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hubtype),
            &mut result__,
        )
        .from_abi::<PM_TILE_SIZE>(result__)
    }
    pub unsafe fn set_HubPosition(
        &self,
        hubtype: PM_TILE_HUBTYPE,
        position: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hubtype),
            ::std::mem::transmute(position),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_NotifiedState<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        notified: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            notified.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_HubPinnedState<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hubtype: PM_TILE_HUBTYPE,
        pinned: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hubtype),
            pinned.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn set_HubTileSize(
        &self,
        hubtype: PM_TILE_HUBTYPE,
        size: PM_TILE_SIZE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hubtype),
            ::std::mem::transmute(size),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_InvocationInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        taskname: Param0,
        taskparameters: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            taskname.into_param().abi(),
            taskparameters.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartTileBlob(
        &self,
        pblob: *mut PM_STARTTILEBLOB,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pblob),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRestoring(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAutoRestoreDisabled(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsRestoring<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        restoring: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            restoring.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsAutoRestoreDisabled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        autorestoredisabled: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            autorestoredisabled.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPMTileInfo {
    type Vtable = IPMTileInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3512748083,
        11016,
        16385,
        [130, 205, 24, 58, 215, 52, 247, 82],
    );
}
impl ::std::convert::From<IPMTileInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMTileInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMTileInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMTileInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPMTileInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPMTileInfo {
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
pub struct IPMTileInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pproductid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptileid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptemplatetype: *mut TILE_TEMPLATE_TYPE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hubtype: PM_TILE_HUBTYPE,
        ppinned: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hubtype: PM_TILE_HUBTYPE,
        pposition: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisnotified: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisdefault: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ptaskid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstarttiletype: *mut PM_STARTTILE_TYPE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisthemable: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propid: u32,
        pppropinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pimageurn: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pparameters: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptilepropenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hubtype: PM_TILE_HUBTYPE,
        psize: *mut PM_TILE_SIZE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hubtype: PM_TILE_HUBTYPE,
        position: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        notified: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hubtype: PM_TILE_HUBTYPE,
        pinned: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hubtype: PM_TILE_HUBTYPE,
        size: PM_TILE_SIZE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        taskname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        taskparameters: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pblob: *mut ::std::mem::ManuallyDrop<PM_STARTTILEBLOB>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisrestoring: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pisautorestoredisabled: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        restoring: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        autorestoredisabled: super::super::Foundation::BOOL,
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
pub struct IPMTileInfoEnumerator(::windows::runtime::IUnknown);
impl IPMTileInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IPMTileInfo> {
        let mut result__: <IPMTileInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IPMTileInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMTileInfoEnumerator {
    type Vtable = IPMTileInfoEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3738710117,
        58466,
        19244,
        [172, 181, 227, 156, 234, 97, 200, 116],
    );
}
impl ::std::convert::From<IPMTileInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IPMTileInfoEnumerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMTileInfoEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IPMTileInfoEnumerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPMTileInfoEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMTileInfoEnumerator
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
pub struct IPMTileInfoEnumerator_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pptileinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPMTilePropertyEnumerator(::windows::runtime::IUnknown);
impl IPMTilePropertyEnumerator {
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<IPMTilePropertyInfo> {
        let mut result__: <IPMTilePropertyInfo as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IPMTilePropertyInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPMTilePropertyEnumerator {
    type Vtable = IPMTilePropertyEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3427587625,
        36935,
        16976,
        [170, 200, 147, 14, 71, 129, 36, 33],
    );
}
impl ::std::convert::From<IPMTilePropertyEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IPMTilePropertyEnumerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMTilePropertyEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IPMTilePropertyEnumerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IPMTilePropertyEnumerator
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IPMTilePropertyEnumerator
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
pub struct IPMTilePropertyEnumerator_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pppropinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IPMTilePropertyInfo(::windows::runtime::IUnknown);
impl IPMTilePropertyInfo {
    pub unsafe fn PropertyID(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyValue(
        &self,
        ppropvalue: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppropvalue),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_Property<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        propvalue: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            propvalue.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPMTilePropertyInfo {
    type Vtable = IPMTilePropertyInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1814790167,
        7930,
        17063,
        [134, 192, 109, 75, 100, 11, 245, 40],
    );
}
impl ::std::convert::From<IPMTilePropertyInfo> for ::windows::runtime::IUnknown {
    fn from(value: IPMTilePropertyInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPMTilePropertyInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IPMTilePropertyInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPMTilePropertyInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPMTilePropertyInfo {
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
pub struct IPMTilePropertyInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppropid: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppropvalue: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        propvalue: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
pub struct IValidate(::windows::runtime::IUnknown);
impl IValidate {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenDatabase<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szdatabase: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            szdatabase.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenCUB<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        szcubfile: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            szcubfile.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn CloseDatabase(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CloseCUB(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplay(
        &self,
        pdisplayfunction: ::std::option::Option<LPDISPLAYVAL>,
        pcontext: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdisplayfunction),
            ::std::mem::transmute(pcontext),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatus(
        &self,
        pstatusfunction: ::std::option::Option<LPEVALCOMCALLBACK>,
        pcontext: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pstatusfunction),
            ::std::mem::transmute(pcontext),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Validate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        wzices: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            wzices.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IValidate {
    type Vtable = IValidate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3833783750,
        58142,
        16707,
        [162, 230, 219, 195, 216, 228, 184, 211],
    );
}
impl ::std::convert::From<IValidate> for ::windows::runtime::IUnknown {
    fn from(value: IValidate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IValidate> for ::windows::runtime::IUnknown {
    fn from(value: &IValidate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IValidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IValidate {
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
pub struct IValidate_abi(
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
        szdatabase: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        szcubfile: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdisplayfunction: ::windows::runtime::RawPtr,
        pcontext: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstatusfunction: ::windows::runtime::RawPtr,
        pcontext: *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        wzices: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const LIBID_MsmMergeTypeLib: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    182298671,
    11302,
    4562,
    [173, 101, 0, 160, 201, 175, 17, 166],
);
pub const LOGALL: u32 = 15u32;
pub const LOGERR: u32 = 4u32;
pub const LOGINFO: u32 = 1u32;
pub const LOGNONE: u32 = 0u32;
pub const LOGPERFMESSAGES: u32 = 8u32;
pub const LOGTOKEN_NO_LOG: u32 = 1u32;
pub const LOGTOKEN_SETUPAPI_APPLOG: u32 = 2u32;
pub const LOGTOKEN_SETUPAPI_DEVLOG: u32 = 3u32;
pub const LOGTOKEN_TYPE_MASK: u32 = 3u32;
pub const LOGTOKEN_UNSPECIFIED: u32 = 0u32;
pub const LOGWARN: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPDISPLAYVAL = unsafe extern "system" fn(
    pcontext: *mut ::std::ffi::c_void,
    uitype: RESULTTYPES,
    szwval: super::super::Foundation::PWSTR,
    szwdescription: super::super::Foundation::PWSTR,
    szwlocation: super::super::Foundation::PWSTR,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPEVALCOMCALLBACK = unsafe extern "system" fn(
    istatus: STATUSTYPES,
    szdata: super::super::Foundation::PWSTR,
    pcontext: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL;
pub const MAX_FEATURE_CHARS: u32 = 38u32;
pub const MAX_GUID_CHARS: u32 = 38u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MSIADVERTISEOPTIONFLAGS(pub i32);
pub const MSIADVERTISEOPTIONFLAGS_INSTANCE: MSIADVERTISEOPTIONFLAGS = MSIADVERTISEOPTIONFLAGS(1i32);
impl ::std::convert::From<i32> for MSIADVERTISEOPTIONFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSIADVERTISEOPTIONFLAGS {
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
pub struct MSIARCHITECTUREFLAGS(pub i32);
pub const MSIARCHITECTUREFLAGS_X86: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(1i32);
pub const MSIARCHITECTUREFLAGS_IA64: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(2i32);
pub const MSIARCHITECTUREFLAGS_AMD64: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(4i32);
pub const MSIARCHITECTUREFLAGS_ARM: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(8i32);
impl ::std::convert::From<i32> for MSIARCHITECTUREFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSIARCHITECTUREFLAGS {
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
pub struct MSIASSEMBLYINFO(pub u32);
pub const MSIASSEMBLYINFO_NETASSEMBLY: MSIASSEMBLYINFO = MSIASSEMBLYINFO(0u32);
pub const MSIASSEMBLYINFO_WIN32ASSEMBLY: MSIASSEMBLYINFO = MSIASSEMBLYINFO(1u32);
impl ::std::convert::From<u32> for MSIASSEMBLYINFO {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSIASSEMBLYINFO {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MSIASSEMBLYINFO {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MSIASSEMBLYINFO {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MSIASSEMBLYINFO {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MSIASSEMBLYINFO {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MSIASSEMBLYINFO {
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
pub struct MSICODE(pub i32);
pub const MSICODE_PRODUCT: MSICODE = MSICODE(0i32);
pub const MSICODE_PATCH: MSICODE = MSICODE(1073741824i32);
impl ::std::convert::From<i32> for MSICODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSICODE {
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
pub struct MSICOLINFO(pub i32);
pub const MSICOLINFO_NAMES: MSICOLINFO = MSICOLINFO(0i32);
pub const MSICOLINFO_TYPES: MSICOLINFO = MSICOLINFO(1i32);
impl ::std::convert::From<i32> for MSICOLINFO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSICOLINFO {
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
pub struct MSICONDITION(pub i32);
pub const MSICONDITION_FALSE: MSICONDITION = MSICONDITION(0i32);
pub const MSICONDITION_TRUE: MSICONDITION = MSICONDITION(1i32);
pub const MSICONDITION_NONE: MSICONDITION = MSICONDITION(2i32);
pub const MSICONDITION_ERROR: MSICONDITION = MSICONDITION(3i32);
impl ::std::convert::From<i32> for MSICONDITION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSICONDITION {
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
pub struct MSICOSTTREE(pub i32);
pub const MSICOSTTREE_SELFONLY: MSICOSTTREE = MSICOSTTREE(0i32);
pub const MSICOSTTREE_CHILDREN: MSICOSTTREE = MSICOSTTREE(1i32);
pub const MSICOSTTREE_PARENTS: MSICOSTTREE = MSICOSTTREE(2i32);
pub const MSICOSTTREE_RESERVED: MSICOSTTREE = MSICOSTTREE(3i32);
impl ::std::convert::From<i32> for MSICOSTTREE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSICOSTTREE {
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
pub struct MSIDBERROR(pub i32);
pub const MSIDBERROR_INVALIDARG: MSIDBERROR = MSIDBERROR(-3i32);
pub const MSIDBERROR_MOREDATA: MSIDBERROR = MSIDBERROR(-2i32);
pub const MSIDBERROR_FUNCTIONERROR: MSIDBERROR = MSIDBERROR(-1i32);
pub const MSIDBERROR_NOERROR: MSIDBERROR = MSIDBERROR(0i32);
pub const MSIDBERROR_DUPLICATEKEY: MSIDBERROR = MSIDBERROR(1i32);
pub const MSIDBERROR_REQUIRED: MSIDBERROR = MSIDBERROR(2i32);
pub const MSIDBERROR_BADLINK: MSIDBERROR = MSIDBERROR(3i32);
pub const MSIDBERROR_OVERFLOW: MSIDBERROR = MSIDBERROR(4i32);
pub const MSIDBERROR_UNDERFLOW: MSIDBERROR = MSIDBERROR(5i32);
pub const MSIDBERROR_NOTINSET: MSIDBERROR = MSIDBERROR(6i32);
pub const MSIDBERROR_BADVERSION: MSIDBERROR = MSIDBERROR(7i32);
pub const MSIDBERROR_BADCASE: MSIDBERROR = MSIDBERROR(8i32);
pub const MSIDBERROR_BADGUID: MSIDBERROR = MSIDBERROR(9i32);
pub const MSIDBERROR_BADWILDCARD: MSIDBERROR = MSIDBERROR(10i32);
pub const MSIDBERROR_BADIDENTIFIER: MSIDBERROR = MSIDBERROR(11i32);
pub const MSIDBERROR_BADLANGUAGE: MSIDBERROR = MSIDBERROR(12i32);
pub const MSIDBERROR_BADFILENAME: MSIDBERROR = MSIDBERROR(13i32);
pub const MSIDBERROR_BADPATH: MSIDBERROR = MSIDBERROR(14i32);
pub const MSIDBERROR_BADCONDITION: MSIDBERROR = MSIDBERROR(15i32);
pub const MSIDBERROR_BADFORMATTED: MSIDBERROR = MSIDBERROR(16i32);
pub const MSIDBERROR_BADTEMPLATE: MSIDBERROR = MSIDBERROR(17i32);
pub const MSIDBERROR_BADDEFAULTDIR: MSIDBERROR = MSIDBERROR(18i32);
pub const MSIDBERROR_BADREGPATH: MSIDBERROR = MSIDBERROR(19i32);
pub const MSIDBERROR_BADCUSTOMSOURCE: MSIDBERROR = MSIDBERROR(20i32);
pub const MSIDBERROR_BADPROPERTY: MSIDBERROR = MSIDBERROR(21i32);
pub const MSIDBERROR_MISSINGDATA: MSIDBERROR = MSIDBERROR(22i32);
pub const MSIDBERROR_BADCATEGORY: MSIDBERROR = MSIDBERROR(23i32);
pub const MSIDBERROR_BADKEYTABLE: MSIDBERROR = MSIDBERROR(24i32);
pub const MSIDBERROR_BADMAXMINVALUES: MSIDBERROR = MSIDBERROR(25i32);
pub const MSIDBERROR_BADCABINET: MSIDBERROR = MSIDBERROR(26i32);
pub const MSIDBERROR_BADSHORTCUT: MSIDBERROR = MSIDBERROR(27i32);
pub const MSIDBERROR_STRINGOVERFLOW: MSIDBERROR = MSIDBERROR(28i32);
pub const MSIDBERROR_BADLOCALIZEATTRIB: MSIDBERROR = MSIDBERROR(29i32);
impl ::std::convert::From<i32> for MSIDBERROR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSIDBERROR {
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
pub struct MSIDBSTATE(pub i32);
pub const MSIDBSTATE_ERROR: MSIDBSTATE = MSIDBSTATE(-1i32);
pub const MSIDBSTATE_READ: MSIDBSTATE = MSIDBSTATE(0i32);
pub const MSIDBSTATE_WRITE: MSIDBSTATE = MSIDBSTATE(1i32);
impl ::std::convert::From<i32> for MSIDBSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSIDBSTATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MSIFILEHASHINFO {
    pub dwFileHashInfoSize: u32,
    pub dwData: [u32; 4],
}
impl MSIFILEHASHINFO {}
impl ::std::default::Default for MSIFILEHASHINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MSIFILEHASHINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MSIFILEHASHINFO")
            .field("dwFileHashInfoSize", &self.dwFileHashInfoSize)
            .field("dwData", &self.dwData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MSIFILEHASHINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileHashInfoSize == other.dwFileHashInfoSize && self.dwData == other.dwData
    }
}
impl ::std::cmp::Eq for MSIFILEHASHINFO {}
unsafe impl ::windows::runtime::Abi for MSIFILEHASHINFO {
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
pub struct MSIHANDLE(pub u32);
impl ::std::default::Default for MSIHANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for MSIHANDLE {}
unsafe impl ::windows::runtime::Abi for MSIHANDLE {
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
pub struct MSIINSTALLCONTEXT(pub i32);
pub const MSIINSTALLCONTEXT_FIRSTVISIBLE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(0i32);
pub const MSIINSTALLCONTEXT_NONE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(0i32);
pub const MSIINSTALLCONTEXT_USERMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(1i32);
pub const MSIINSTALLCONTEXT_USERUNMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(2i32);
pub const MSIINSTALLCONTEXT_MACHINE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(4i32);
pub const MSIINSTALLCONTEXT_ALL: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(7i32);
pub const MSIINSTALLCONTEXT_ALLUSERMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(8i32);
impl ::std::convert::From<i32> for MSIINSTALLCONTEXT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSIINSTALLCONTEXT {
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
pub struct MSIMODIFY(pub i32);
pub const MSIMODIFY_SEEK: MSIMODIFY = MSIMODIFY(-1i32);
pub const MSIMODIFY_REFRESH: MSIMODIFY = MSIMODIFY(0i32);
pub const MSIMODIFY_INSERT: MSIMODIFY = MSIMODIFY(1i32);
pub const MSIMODIFY_UPDATE: MSIMODIFY = MSIMODIFY(2i32);
pub const MSIMODIFY_ASSIGN: MSIMODIFY = MSIMODIFY(3i32);
pub const MSIMODIFY_REPLACE: MSIMODIFY = MSIMODIFY(4i32);
pub const MSIMODIFY_MERGE: MSIMODIFY = MSIMODIFY(5i32);
pub const MSIMODIFY_DELETE: MSIMODIFY = MSIMODIFY(6i32);
pub const MSIMODIFY_INSERT_TEMPORARY: MSIMODIFY = MSIMODIFY(7i32);
pub const MSIMODIFY_VALIDATE: MSIMODIFY = MSIMODIFY(8i32);
pub const MSIMODIFY_VALIDATE_NEW: MSIMODIFY = MSIMODIFY(9i32);
pub const MSIMODIFY_VALIDATE_FIELD: MSIMODIFY = MSIMODIFY(10i32);
pub const MSIMODIFY_VALIDATE_DELETE: MSIMODIFY = MSIMODIFY(11i32);
impl ::std::convert::From<i32> for MSIMODIFY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSIMODIFY {
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
pub struct MSIOPENPACKAGEFLAGS(pub i32);
pub const MSIOPENPACKAGEFLAGS_IGNOREMACHINESTATE: MSIOPENPACKAGEFLAGS = MSIOPENPACKAGEFLAGS(1i32);
impl ::std::convert::From<i32> for MSIOPENPACKAGEFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSIOPENPACKAGEFLAGS {
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
pub struct MSIPATCHDATATYPE(pub i32);
pub const MSIPATCH_DATATYPE_PATCHFILE: MSIPATCHDATATYPE = MSIPATCHDATATYPE(0i32);
pub const MSIPATCH_DATATYPE_XMLPATH: MSIPATCHDATATYPE = MSIPATCHDATATYPE(1i32);
pub const MSIPATCH_DATATYPE_XMLBLOB: MSIPATCHDATATYPE = MSIPATCHDATATYPE(2i32);
impl ::std::convert::From<i32> for MSIPATCHDATATYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSIPATCHDATATYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MSIPATCHSEQUENCEINFOA {
    pub szPatchData: super::super::Foundation::PSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MSIPATCHSEQUENCEINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MSIPATCHSEQUENCEINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MSIPATCHSEQUENCEINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MSIPATCHSEQUENCEINFOA")
            .field("szPatchData", &self.szPatchData)
            .field("ePatchDataType", &self.ePatchDataType)
            .field("dwOrder", &self.dwOrder)
            .field("uStatus", &self.uStatus)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MSIPATCHSEQUENCEINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.szPatchData == other.szPatchData
            && self.ePatchDataType == other.ePatchDataType
            && self.dwOrder == other.dwOrder
            && self.uStatus == other.uStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MSIPATCHSEQUENCEINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MSIPATCHSEQUENCEINFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MSIPATCHSEQUENCEINFOW {
    pub szPatchData: super::super::Foundation::PWSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MSIPATCHSEQUENCEINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MSIPATCHSEQUENCEINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MSIPATCHSEQUENCEINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MSIPATCHSEQUENCEINFOW")
            .field("szPatchData", &self.szPatchData)
            .field("ePatchDataType", &self.ePatchDataType)
            .field("dwOrder", &self.dwOrder)
            .field("uStatus", &self.uStatus)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MSIPATCHSEQUENCEINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.szPatchData == other.szPatchData
            && self.ePatchDataType == other.ePatchDataType
            && self.dwOrder == other.dwOrder
            && self.uStatus == other.uStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MSIPATCHSEQUENCEINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MSIPATCHSEQUENCEINFOW {
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
pub struct MSIPATCHSTATE(pub i32);
pub const MSIPATCHSTATE_INVALID: MSIPATCHSTATE = MSIPATCHSTATE(0i32);
pub const MSIPATCHSTATE_APPLIED: MSIPATCHSTATE = MSIPATCHSTATE(1i32);
pub const MSIPATCHSTATE_SUPERSEDED: MSIPATCHSTATE = MSIPATCHSTATE(2i32);
pub const MSIPATCHSTATE_OBSOLETED: MSIPATCHSTATE = MSIPATCHSTATE(4i32);
pub const MSIPATCHSTATE_REGISTERED: MSIPATCHSTATE = MSIPATCHSTATE(8i32);
pub const MSIPATCHSTATE_ALL: MSIPATCHSTATE = MSIPATCHSTATE(15i32);
impl ::std::convert::From<i32> for MSIPATCHSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSIPATCHSTATE {
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
pub struct MSIRUNMODE(pub i32);
pub const MSIRUNMODE_ADMIN: MSIRUNMODE = MSIRUNMODE(0i32);
pub const MSIRUNMODE_ADVERTISE: MSIRUNMODE = MSIRUNMODE(1i32);
pub const MSIRUNMODE_MAINTENANCE: MSIRUNMODE = MSIRUNMODE(2i32);
pub const MSIRUNMODE_ROLLBACKENABLED: MSIRUNMODE = MSIRUNMODE(3i32);
pub const MSIRUNMODE_LOGENABLED: MSIRUNMODE = MSIRUNMODE(4i32);
pub const MSIRUNMODE_OPERATIONS: MSIRUNMODE = MSIRUNMODE(5i32);
pub const MSIRUNMODE_REBOOTATEND: MSIRUNMODE = MSIRUNMODE(6i32);
pub const MSIRUNMODE_REBOOTNOW: MSIRUNMODE = MSIRUNMODE(7i32);
pub const MSIRUNMODE_CABINET: MSIRUNMODE = MSIRUNMODE(8i32);
pub const MSIRUNMODE_SOURCESHORTNAMES: MSIRUNMODE = MSIRUNMODE(9i32);
pub const MSIRUNMODE_TARGETSHORTNAMES: MSIRUNMODE = MSIRUNMODE(10i32);
pub const MSIRUNMODE_RESERVED11: MSIRUNMODE = MSIRUNMODE(11i32);
pub const MSIRUNMODE_WINDOWS9X: MSIRUNMODE = MSIRUNMODE(12i32);
pub const MSIRUNMODE_ZAWENABLED: MSIRUNMODE = MSIRUNMODE(13i32);
pub const MSIRUNMODE_RESERVED14: MSIRUNMODE = MSIRUNMODE(14i32);
pub const MSIRUNMODE_RESERVED15: MSIRUNMODE = MSIRUNMODE(15i32);
pub const MSIRUNMODE_SCHEDULED: MSIRUNMODE = MSIRUNMODE(16i32);
pub const MSIRUNMODE_ROLLBACK: MSIRUNMODE = MSIRUNMODE(17i32);
pub const MSIRUNMODE_COMMIT: MSIRUNMODE = MSIRUNMODE(18i32);
impl ::std::convert::From<i32> for MSIRUNMODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSIRUNMODE {
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
pub struct MSISOURCETYPE(pub i32);
pub const MSISOURCETYPE_UNKNOWN: MSISOURCETYPE = MSISOURCETYPE(0i32);
pub const MSISOURCETYPE_NETWORK: MSISOURCETYPE = MSISOURCETYPE(1i32);
pub const MSISOURCETYPE_URL: MSISOURCETYPE = MSISOURCETYPE(2i32);
pub const MSISOURCETYPE_MEDIA: MSISOURCETYPE = MSISOURCETYPE(4i32);
impl ::std::convert::From<i32> for MSISOURCETYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSISOURCETYPE {
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
pub struct MSITRANSACTION(pub i32);
pub const MSITRANSACTION_CHAIN_EMBEDDEDUI: MSITRANSACTION = MSITRANSACTION(1i32);
pub const MSITRANSACTION_JOIN_EXISTING_EMBEDDEDUI: MSITRANSACTION = MSITRANSACTION(2i32);
impl ::std::convert::From<i32> for MSITRANSACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSITRANSACTION {
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
pub struct MSITRANSACTIONSTATE(pub u32);
pub const MSITRANSACTIONSTATE_ROLLBACK: MSITRANSACTIONSTATE = MSITRANSACTIONSTATE(0u32);
pub const MSITRANSACTIONSTATE_COMMIT: MSITRANSACTIONSTATE = MSITRANSACTIONSTATE(1u32);
impl ::std::convert::From<u32> for MSITRANSACTIONSTATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSITRANSACTIONSTATE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MSITRANSACTIONSTATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MSITRANSACTIONSTATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MSITRANSACTIONSTATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MSITRANSACTIONSTATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MSITRANSACTIONSTATE {
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
pub struct MSITRANSFORM_ERROR(pub i32);
pub const MSITRANSFORM_ERROR_ADDEXISTINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(1i32);
pub const MSITRANSFORM_ERROR_DELMISSINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(2i32);
pub const MSITRANSFORM_ERROR_ADDEXISTINGTABLE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(4i32);
pub const MSITRANSFORM_ERROR_DELMISSINGTABLE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(8i32);
pub const MSITRANSFORM_ERROR_UPDATEMISSINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(16i32);
pub const MSITRANSFORM_ERROR_CHANGECODEPAGE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(32i32);
pub const MSITRANSFORM_ERROR_VIEWTRANSFORM: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(256i32);
pub const MSITRANSFORM_ERROR_NONE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(0i32);
impl ::std::convert::From<i32> for MSITRANSFORM_ERROR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSITRANSFORM_ERROR {
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
pub struct MSITRANSFORM_VALIDATE(pub i32);
pub const MSITRANSFORM_VALIDATE_LANGUAGE: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(1i32);
pub const MSITRANSFORM_VALIDATE_PRODUCT: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(2i32);
pub const MSITRANSFORM_VALIDATE_PLATFORM: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(4i32);
pub const MSITRANSFORM_VALIDATE_MAJORVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(8i32);
pub const MSITRANSFORM_VALIDATE_MINORVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(16i32);
pub const MSITRANSFORM_VALIDATE_UPDATEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(32i32);
pub const MSITRANSFORM_VALIDATE_NEWLESSBASEVERSION: MSITRANSFORM_VALIDATE =
    MSITRANSFORM_VALIDATE(64i32);
pub const MSITRANSFORM_VALIDATE_NEWLESSEQUALBASEVERSION: MSITRANSFORM_VALIDATE =
    MSITRANSFORM_VALIDATE(128i32);
pub const MSITRANSFORM_VALIDATE_NEWEQUALBASEVERSION: MSITRANSFORM_VALIDATE =
    MSITRANSFORM_VALIDATE(256i32);
pub const MSITRANSFORM_VALIDATE_NEWGREATEREQUALBASEVERSION: MSITRANSFORM_VALIDATE =
    MSITRANSFORM_VALIDATE(512i32);
pub const MSITRANSFORM_VALIDATE_NEWGREATERBASEVERSION: MSITRANSFORM_VALIDATE =
    MSITRANSFORM_VALIDATE(1024i32);
pub const MSITRANSFORM_VALIDATE_UPGRADECODE: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(2048i32);
impl ::std::convert::From<i32> for MSITRANSFORM_VALIDATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSITRANSFORM_VALIDATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MSI_INVALID_HASH_IS_FATAL: u32 = 1u32;
pub const MSI_NULL_INTEGER: u32 = 2147483648u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiAdvertiseProductA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpackagepath: Param0,
    szscriptfilepath: Param1,
    sztransforms: Param2,
    lgidlanguage: u16,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiAdvertiseProductA(
                szpackagepath: super::super::Foundation::PSTR,
                szscriptfilepath: super::super::Foundation::PSTR,
                sztransforms: super::super::Foundation::PSTR,
                lgidlanguage: u16,
            ) -> u32;
        }
        ::std::mem::transmute(MsiAdvertiseProductA(
            szpackagepath.into_param().abi(),
            szscriptfilepath.into_param().abi(),
            sztransforms.into_param().abi(),
            ::std::mem::transmute(lgidlanguage),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiAdvertiseProductExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpackagepath: Param0,
    szscriptfilepath: Param1,
    sztransforms: Param2,
    lgidlanguage: u16,
    dwplatform: u32,
    dwoptions: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiAdvertiseProductExA(
                szpackagepath: super::super::Foundation::PSTR,
                szscriptfilepath: super::super::Foundation::PSTR,
                sztransforms: super::super::Foundation::PSTR,
                lgidlanguage: u16,
                dwplatform: u32,
                dwoptions: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiAdvertiseProductExA(
            szpackagepath.into_param().abi(),
            szscriptfilepath.into_param().abi(),
            sztransforms.into_param().abi(),
            ::std::mem::transmute(lgidlanguage),
            ::std::mem::transmute(dwplatform),
            ::std::mem::transmute(dwoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiAdvertiseProductExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpackagepath: Param0,
    szscriptfilepath: Param1,
    sztransforms: Param2,
    lgidlanguage: u16,
    dwplatform: u32,
    dwoptions: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiAdvertiseProductExW(
                szpackagepath: super::super::Foundation::PWSTR,
                szscriptfilepath: super::super::Foundation::PWSTR,
                sztransforms: super::super::Foundation::PWSTR,
                lgidlanguage: u16,
                dwplatform: u32,
                dwoptions: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiAdvertiseProductExW(
            szpackagepath.into_param().abi(),
            szscriptfilepath.into_param().abi(),
            sztransforms.into_param().abi(),
            ::std::mem::transmute(lgidlanguage),
            ::std::mem::transmute(dwplatform),
            ::std::mem::transmute(dwoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiAdvertiseProductW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpackagepath: Param0,
    szscriptfilepath: Param1,
    sztransforms: Param2,
    lgidlanguage: u16,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiAdvertiseProductW(
                szpackagepath: super::super::Foundation::PWSTR,
                szscriptfilepath: super::super::Foundation::PWSTR,
                sztransforms: super::super::Foundation::PWSTR,
                lgidlanguage: u16,
            ) -> u32;
        }
        ::std::mem::transmute(MsiAdvertiseProductW(
            szpackagepath.into_param().abi(),
            szscriptfilepath.into_param().abi(),
            sztransforms.into_param().abi(),
            ::std::mem::transmute(lgidlanguage),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiAdvertiseScriptA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    szscriptfile: Param0,
    dwflags: u32,
    phregdata: *const super::Registry::HKEY,
    fremoveitems: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiAdvertiseScriptA(
                szscriptfile: super::super::Foundation::PSTR,
                dwflags: u32,
                phregdata: *const super::Registry::HKEY,
                fremoveitems: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(MsiAdvertiseScriptA(
            szscriptfile.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(phregdata),
            fremoveitems.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiAdvertiseScriptW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    szscriptfile: Param0,
    dwflags: u32,
    phregdata: *const super::Registry::HKEY,
    fremoveitems: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiAdvertiseScriptW(
                szscriptfile: super::super::Foundation::PWSTR,
                dwflags: u32,
                phregdata: *const super::Registry::HKEY,
                fremoveitems: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(MsiAdvertiseScriptW(
            szscriptfile.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(phregdata),
            fremoveitems.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiApplyMultiplePatchesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpatchpackages: Param0,
    szproductcode: Param1,
    szpropertieslist: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiApplyMultiplePatchesA(
                szpatchpackages: super::super::Foundation::PSTR,
                szproductcode: super::super::Foundation::PSTR,
                szpropertieslist: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiApplyMultiplePatchesA(
            szpatchpackages.into_param().abi(),
            szproductcode.into_param().abi(),
            szpropertieslist.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiApplyMultiplePatchesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpatchpackages: Param0,
    szproductcode: Param1,
    szpropertieslist: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiApplyMultiplePatchesW(
                szpatchpackages: super::super::Foundation::PWSTR,
                szproductcode: super::super::Foundation::PWSTR,
                szpropertieslist: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiApplyMultiplePatchesW(
            szpatchpackages.into_param().abi(),
            szproductcode.into_param().abi(),
            szpropertieslist.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiApplyPatchA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpatchpackage: Param0,
    szinstallpackage: Param1,
    einstalltype: INSTALLTYPE,
    szcommandline: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiApplyPatchA(
                szpatchpackage: super::super::Foundation::PSTR,
                szinstallpackage: super::super::Foundation::PSTR,
                einstalltype: INSTALLTYPE,
                szcommandline: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiApplyPatchA(
            szpatchpackage.into_param().abi(),
            szinstallpackage.into_param().abi(),
            ::std::mem::transmute(einstalltype),
            szcommandline.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiApplyPatchW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpatchpackage: Param0,
    szinstallpackage: Param1,
    einstalltype: INSTALLTYPE,
    szcommandline: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiApplyPatchW(
                szpatchpackage: super::super::Foundation::PWSTR,
                szinstallpackage: super::super::Foundation::PWSTR,
                einstalltype: INSTALLTYPE,
                szcommandline: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiApplyPatchW(
            szpatchpackage.into_param().abi(),
            szinstallpackage.into_param().abi(),
            ::std::mem::transmute(einstalltype),
            szcommandline.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiBeginTransactionA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szname: Param0,
    dwtransactionattributes: u32,
    phtransactionhandle: *mut MSIHANDLE,
    phchangeofownerevent: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiBeginTransactionA(
                szname: super::super::Foundation::PSTR,
                dwtransactionattributes: u32,
                phtransactionhandle: *mut MSIHANDLE,
                phchangeofownerevent: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiBeginTransactionA(
            szname.into_param().abi(),
            ::std::mem::transmute(dwtransactionattributes),
            ::std::mem::transmute(phtransactionhandle),
            ::std::mem::transmute(phchangeofownerevent),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiBeginTransactionW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szname: Param0,
    dwtransactionattributes: u32,
    phtransactionhandle: *mut MSIHANDLE,
    phchangeofownerevent: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiBeginTransactionW(
                szname: super::super::Foundation::PWSTR,
                dwtransactionattributes: u32,
                phtransactionhandle: *mut MSIHANDLE,
                phchangeofownerevent: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiBeginTransactionW(
            szname.into_param().abi(),
            ::std::mem::transmute(dwtransactionattributes),
            ::std::mem::transmute(phtransactionhandle),
            ::std::mem::transmute(phchangeofownerevent),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiCloseAllHandles() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiCloseAllHandles() -> u32;
        }
        ::std::mem::transmute(MsiCloseAllHandles())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiCloseHandle<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hany: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiCloseHandle(hany: MSIHANDLE) -> u32;
        }
        ::std::mem::transmute(MsiCloseHandle(hany.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiCollectUserInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiCollectUserInfoA(szproduct: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(MsiCollectUserInfoA(szproduct.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiCollectUserInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiCollectUserInfoW(szproduct: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MsiCollectUserInfoW(szproduct.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiConfigureFeatureA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
    einstallstate: INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiConfigureFeatureA(
                szproduct: super::super::Foundation::PSTR,
                szfeature: super::super::Foundation::PSTR,
                einstallstate: INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiConfigureFeatureA(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(einstallstate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiConfigureFeatureW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
    einstallstate: INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiConfigureFeatureW(
                szproduct: super::super::Foundation::PWSTR,
                szfeature: super::super::Foundation::PWSTR,
                einstallstate: INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiConfigureFeatureW(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(einstallstate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiConfigureProductA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    iinstalllevel: INSTALLLEVEL,
    einstallstate: INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiConfigureProductA(
                szproduct: super::super::Foundation::PSTR,
                iinstalllevel: INSTALLLEVEL,
                einstallstate: INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiConfigureProductA(
            szproduct.into_param().abi(),
            ::std::mem::transmute(iinstalllevel),
            ::std::mem::transmute(einstallstate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiConfigureProductExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    iinstalllevel: INSTALLLEVEL,
    einstallstate: INSTALLSTATE,
    szcommandline: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiConfigureProductExA(
                szproduct: super::super::Foundation::PSTR,
                iinstalllevel: INSTALLLEVEL,
                einstallstate: INSTALLSTATE,
                szcommandline: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiConfigureProductExA(
            szproduct.into_param().abi(),
            ::std::mem::transmute(iinstalllevel),
            ::std::mem::transmute(einstallstate),
            szcommandline.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiConfigureProductExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    iinstalllevel: INSTALLLEVEL,
    einstallstate: INSTALLSTATE,
    szcommandline: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiConfigureProductExW(
                szproduct: super::super::Foundation::PWSTR,
                iinstalllevel: INSTALLLEVEL,
                einstallstate: INSTALLSTATE,
                szcommandline: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiConfigureProductExW(
            szproduct.into_param().abi(),
            ::std::mem::transmute(iinstalllevel),
            ::std::mem::transmute(einstallstate),
            szcommandline.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiConfigureProductW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    iinstalllevel: INSTALLLEVEL,
    einstallstate: INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiConfigureProductW(
                szproduct: super::super::Foundation::PWSTR,
                iinstalllevel: INSTALLLEVEL,
                einstallstate: INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiConfigureProductW(
            szproduct.into_param().abi(),
            ::std::mem::transmute(iinstalllevel),
            ::std::mem::transmute(einstallstate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiCreateRecord(cparams: u32) -> MSIHANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiCreateRecord(cparams: u32) -> MSIHANDLE;
        }
        ::std::mem::transmute(MsiCreateRecord(::std::mem::transmute(cparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hdatabase: Param0,
    hdatabasereference: Param1,
    sztransformfile: Param2,
    ierrorconditions: MSITRANSFORM_ERROR,
    ivalidation: MSITRANSFORM_VALIDATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiCreateTransformSummaryInfoA(
                hdatabase: MSIHANDLE,
                hdatabasereference: MSIHANDLE,
                sztransformfile: super::super::Foundation::PSTR,
                ierrorconditions: MSITRANSFORM_ERROR,
                ivalidation: MSITRANSFORM_VALIDATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiCreateTransformSummaryInfoA(
            hdatabase.into_param().abi(),
            hdatabasereference.into_param().abi(),
            sztransformfile.into_param().abi(),
            ::std::mem::transmute(ierrorconditions),
            ::std::mem::transmute(ivalidation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdatabase: Param0,
    hdatabasereference: Param1,
    sztransformfile: Param2,
    ierrorconditions: MSITRANSFORM_ERROR,
    ivalidation: MSITRANSFORM_VALIDATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiCreateTransformSummaryInfoW(
                hdatabase: MSIHANDLE,
                hdatabasereference: MSIHANDLE,
                sztransformfile: super::super::Foundation::PWSTR,
                ierrorconditions: MSITRANSFORM_ERROR,
                ivalidation: MSITRANSFORM_VALIDATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiCreateTransformSummaryInfoW(
            hdatabase.into_param().abi(),
            hdatabasereference.into_param().abi(),
            sztransformfile.into_param().abi(),
            ::std::mem::transmute(ierrorconditions),
            ::std::mem::transmute(ivalidation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseApplyTransformA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hdatabase: Param0,
    sztransformfile: Param1,
    ierrorconditions: MSITRANSFORM_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseApplyTransformA(
                hdatabase: MSIHANDLE,
                sztransformfile: super::super::Foundation::PSTR,
                ierrorconditions: MSITRANSFORM_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseApplyTransformA(
            hdatabase.into_param().abi(),
            sztransformfile.into_param().abi(),
            ::std::mem::transmute(ierrorconditions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseApplyTransformW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdatabase: Param0,
    sztransformfile: Param1,
    ierrorconditions: MSITRANSFORM_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseApplyTransformW(
                hdatabase: MSIHANDLE,
                sztransformfile: super::super::Foundation::PWSTR,
                ierrorconditions: MSITRANSFORM_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseApplyTransformW(
            hdatabase.into_param().abi(),
            sztransformfile.into_param().abi(),
            ::std::mem::transmute(ierrorconditions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiDatabaseCommit<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hdatabase: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseCommit(hdatabase: MSIHANDLE) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseCommit(hdatabase.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseExportA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hdatabase: Param0,
    sztablename: Param1,
    szfolderpath: Param2,
    szfilename: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseExportA(
                hdatabase: MSIHANDLE,
                sztablename: super::super::Foundation::PSTR,
                szfolderpath: super::super::Foundation::PSTR,
                szfilename: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseExportA(
            hdatabase.into_param().abi(),
            sztablename.into_param().abi(),
            szfolderpath.into_param().abi(),
            szfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseExportW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdatabase: Param0,
    sztablename: Param1,
    szfolderpath: Param2,
    szfilename: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseExportW(
                hdatabase: MSIHANDLE,
                sztablename: super::super::Foundation::PWSTR,
                szfolderpath: super::super::Foundation::PWSTR,
                szfilename: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseExportW(
            hdatabase.into_param().abi(),
            sztablename.into_param().abi(),
            szfolderpath.into_param().abi(),
            szfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hdatabase: Param0,
    hdatabasereference: Param1,
    sztransformfile: Param2,
    ireserved1: i32,
    ireserved2: i32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseGenerateTransformA(
                hdatabase: MSIHANDLE,
                hdatabasereference: MSIHANDLE,
                sztransformfile: super::super::Foundation::PSTR,
                ireserved1: i32,
                ireserved2: i32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseGenerateTransformA(
            hdatabase.into_param().abi(),
            hdatabasereference.into_param().abi(),
            sztransformfile.into_param().abi(),
            ::std::mem::transmute(ireserved1),
            ::std::mem::transmute(ireserved2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdatabase: Param0,
    hdatabasereference: Param1,
    sztransformfile: Param2,
    ireserved1: i32,
    ireserved2: i32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseGenerateTransformW(
                hdatabase: MSIHANDLE,
                hdatabasereference: MSIHANDLE,
                sztransformfile: super::super::Foundation::PWSTR,
                ireserved1: i32,
                ireserved2: i32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseGenerateTransformW(
            hdatabase.into_param().abi(),
            hdatabasereference.into_param().abi(),
            sztransformfile.into_param().abi(),
            ::std::mem::transmute(ireserved1),
            ::std::mem::transmute(ireserved2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hdatabase: Param0,
    sztablename: Param1,
    phrecord: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseGetPrimaryKeysA(
                hdatabase: MSIHANDLE,
                sztablename: super::super::Foundation::PSTR,
                phrecord: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseGetPrimaryKeysA(
            hdatabase.into_param().abi(),
            sztablename.into_param().abi(),
            ::std::mem::transmute(phrecord),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdatabase: Param0,
    sztablename: Param1,
    phrecord: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseGetPrimaryKeysW(
                hdatabase: MSIHANDLE,
                sztablename: super::super::Foundation::PWSTR,
                phrecord: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseGetPrimaryKeysW(
            hdatabase.into_param().abi(),
            sztablename.into_param().abi(),
            ::std::mem::transmute(phrecord),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseImportA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hdatabase: Param0,
    szfolderpath: Param1,
    szfilename: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseImportA(
                hdatabase: MSIHANDLE,
                szfolderpath: super::super::Foundation::PSTR,
                szfilename: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseImportA(
            hdatabase.into_param().abi(),
            szfolderpath.into_param().abi(),
            szfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseImportW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdatabase: Param0,
    szfolderpath: Param1,
    szfilename: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseImportW(
                hdatabase: MSIHANDLE,
                szfolderpath: super::super::Foundation::PWSTR,
                szfilename: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseImportW(
            hdatabase.into_param().abi(),
            szfolderpath.into_param().abi(),
            szfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hdatabase: Param0,
    sztablename: Param1,
) -> MSICONDITION {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseIsTablePersistentA(
                hdatabase: MSIHANDLE,
                sztablename: super::super::Foundation::PSTR,
            ) -> MSICONDITION;
        }
        ::std::mem::transmute(MsiDatabaseIsTablePersistentA(
            hdatabase.into_param().abi(),
            sztablename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdatabase: Param0,
    sztablename: Param1,
) -> MSICONDITION {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseIsTablePersistentW(
                hdatabase: MSIHANDLE,
                sztablename: super::super::Foundation::PWSTR,
            ) -> MSICONDITION;
        }
        ::std::mem::transmute(MsiDatabaseIsTablePersistentW(
            hdatabase.into_param().abi(),
            sztablename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseMergeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hdatabase: Param0,
    hdatabasemerge: Param1,
    sztablename: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseMergeA(
                hdatabase: MSIHANDLE,
                hdatabasemerge: MSIHANDLE,
                sztablename: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseMergeA(
            hdatabase.into_param().abi(),
            hdatabasemerge.into_param().abi(),
            sztablename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseMergeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdatabase: Param0,
    hdatabasemerge: Param1,
    sztablename: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseMergeW(
                hdatabase: MSIHANDLE,
                hdatabasemerge: MSIHANDLE,
                sztablename: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseMergeW(
            hdatabase.into_param().abi(),
            hdatabasemerge.into_param().abi(),
            sztablename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseOpenViewA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hdatabase: Param0,
    szquery: Param1,
    phview: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseOpenViewA(
                hdatabase: MSIHANDLE,
                szquery: super::super::Foundation::PSTR,
                phview: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseOpenViewA(
            hdatabase.into_param().abi(),
            szquery.into_param().abi(),
            ::std::mem::transmute(phview),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDatabaseOpenViewW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdatabase: Param0,
    szquery: Param1,
    phview: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDatabaseOpenViewW(
                hdatabase: MSIHANDLE,
                szquery: super::super::Foundation::PWSTR,
                phview: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDatabaseOpenViewW(
            hdatabase.into_param().abi(),
            szquery.into_param().abi(),
            ::std::mem::transmute(phview),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductpackagepath: Param0,
    cpatchinfo: u32,
    ppatchinfo: *mut MSIPATCHSEQUENCEINFOA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDetermineApplicablePatchesA(
                szproductpackagepath: super::super::Foundation::PSTR,
                cpatchinfo: u32,
                ppatchinfo: *mut MSIPATCHSEQUENCEINFOA,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDetermineApplicablePatchesA(
            szproductpackagepath.into_param().abi(),
            ::std::mem::transmute(cpatchinfo),
            ::std::mem::transmute(ppatchinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductpackagepath: Param0,
    cpatchinfo: u32,
    ppatchinfo: *mut MSIPATCHSEQUENCEINFOW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDetermineApplicablePatchesW(
                szproductpackagepath: super::super::Foundation::PWSTR,
                cpatchinfo: u32,
                ppatchinfo: *mut MSIPATCHSEQUENCEINFOW,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDetermineApplicablePatchesW(
            szproductpackagepath.into_param().abi(),
            ::std::mem::transmute(cpatchinfo),
            ::std::mem::transmute(ppatchinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDeterminePatchSequenceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    cpatchinfo: u32,
    ppatchinfo: *mut MSIPATCHSEQUENCEINFOA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDeterminePatchSequenceA(
                szproductcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                cpatchinfo: u32,
                ppatchinfo: *mut MSIPATCHSEQUENCEINFOA,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDeterminePatchSequenceA(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(cpatchinfo),
            ::std::mem::transmute(ppatchinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDeterminePatchSequenceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    cpatchinfo: u32,
    ppatchinfo: *mut MSIPATCHSEQUENCEINFOW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDeterminePatchSequenceW(
                szproductcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                cpatchinfo: u32,
                ppatchinfo: *mut MSIPATCHSEQUENCEINFOW,
            ) -> u32;
        }
        ::std::mem::transmute(MsiDeterminePatchSequenceW(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(cpatchinfo),
            ::std::mem::transmute(ppatchinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDoActionA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szaction: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDoActionA(hinstall: MSIHANDLE, szaction: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(MsiDoActionA(
            hinstall.into_param().abi(),
            szaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiDoActionW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szaction: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiDoActionW(hinstall: MSIHANDLE, szaction: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MsiDoActionW(
            hinstall.into_param().abi(),
            szaction.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnableLogA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    dwlogmode: INSTALLOGMODE,
    szlogfile: Param1,
    dwlogattributes: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnableLogA(
                dwlogmode: INSTALLOGMODE,
                szlogfile: super::super::Foundation::PSTR,
                dwlogattributes: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnableLogA(
            ::std::mem::transmute(dwlogmode),
            szlogfile.into_param().abi(),
            ::std::mem::transmute(dwlogattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnableLogW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwlogmode: INSTALLOGMODE,
    szlogfile: Param1,
    dwlogattributes: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnableLogW(
                dwlogmode: INSTALLOGMODE,
                szlogfile: super::super::Foundation::PWSTR,
                dwlogattributes: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnableLogW(
            ::std::mem::transmute(dwlogmode),
            szlogfile.into_param().abi(),
            ::std::mem::transmute(dwlogattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiEnableUIPreview<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hdatabase: Param0,
    phpreview: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnableUIPreview(hdatabase: MSIHANDLE, phpreview: *mut MSIHANDLE) -> u32;
        }
        ::std::mem::transmute(MsiEnableUIPreview(
            hdatabase.into_param().abi(),
            ::std::mem::transmute(phpreview),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiEndTransaction(dwtransactionstate: MSITRANSACTIONSTATE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEndTransaction(dwtransactionstate: MSITRANSACTIONSTATE) -> u32;
        }
        ::std::mem::transmute(MsiEndTransaction(::std::mem::transmute(dwtransactionstate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumClientsA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szcomponent: Param0,
    iproductindex: u32,
    lpproductbuf: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumClientsA(
                szcomponent: super::super::Foundation::PSTR,
                iproductindex: u32,
                lpproductbuf: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumClientsA(
            szcomponent.into_param().abi(),
            ::std::mem::transmute(iproductindex),
            ::std::mem::transmute(lpproductbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumClientsExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szcomponent: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwproductindex: u32,
    szproductbuf: super::super::Foundation::PSTR,
    pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
    szsid: super::super::Foundation::PSTR,
    pcchsid: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumClientsExA(
                szcomponent: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwproductindex: u32,
                szproductbuf: super::super::Foundation::PSTR,
                pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
                szsid: super::super::Foundation::PSTR,
                pcchsid: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumClientsExA(
            szcomponent.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwproductindex),
            ::std::mem::transmute(szproductbuf),
            ::std::mem::transmute(pdwinstalledcontext),
            ::std::mem::transmute(szsid),
            ::std::mem::transmute(pcchsid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumClientsExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szcomponent: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwproductindex: u32,
    szproductbuf: super::super::Foundation::PWSTR,
    pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
    szsid: super::super::Foundation::PWSTR,
    pcchsid: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumClientsExW(
                szcomponent: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwproductindex: u32,
                szproductbuf: super::super::Foundation::PWSTR,
                pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
                szsid: super::super::Foundation::PWSTR,
                pcchsid: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumClientsExW(
            szcomponent.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwproductindex),
            ::std::mem::transmute(szproductbuf),
            ::std::mem::transmute(pdwinstalledcontext),
            ::std::mem::transmute(szsid),
            ::std::mem::transmute(pcchsid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumClientsW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szcomponent: Param0,
    iproductindex: u32,
    lpproductbuf: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumClientsW(
                szcomponent: super::super::Foundation::PWSTR,
                iproductindex: u32,
                lpproductbuf: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumClientsW(
            szcomponent.into_param().abi(),
            ::std::mem::transmute(iproductindex),
            ::std::mem::transmute(lpproductbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumComponentCostsA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szcomponent: Param1,
    dwindex: u32,
    istate: INSTALLSTATE,
    szdrivebuf: super::super::Foundation::PSTR,
    pcchdrivebuf: *mut u32,
    picost: *mut i32,
    pitempcost: *mut i32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumComponentCostsA(
                hinstall: MSIHANDLE,
                szcomponent: super::super::Foundation::PSTR,
                dwindex: u32,
                istate: INSTALLSTATE,
                szdrivebuf: super::super::Foundation::PSTR,
                pcchdrivebuf: *mut u32,
                picost: *mut i32,
                pitempcost: *mut i32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumComponentCostsA(
            hinstall.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(istate),
            ::std::mem::transmute(szdrivebuf),
            ::std::mem::transmute(pcchdrivebuf),
            ::std::mem::transmute(picost),
            ::std::mem::transmute(pitempcost),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumComponentCostsW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szcomponent: Param1,
    dwindex: u32,
    istate: INSTALLSTATE,
    szdrivebuf: super::super::Foundation::PWSTR,
    pcchdrivebuf: *mut u32,
    picost: *mut i32,
    pitempcost: *mut i32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumComponentCostsW(
                hinstall: MSIHANDLE,
                szcomponent: super::super::Foundation::PWSTR,
                dwindex: u32,
                istate: INSTALLSTATE,
                szdrivebuf: super::super::Foundation::PWSTR,
                pcchdrivebuf: *mut u32,
                picost: *mut i32,
                pitempcost: *mut i32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumComponentCostsW(
            hinstall.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(istate),
            ::std::mem::transmute(szdrivebuf),
            ::std::mem::transmute(pcchdrivebuf),
            ::std::mem::transmute(picost),
            ::std::mem::transmute(pitempcost),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumComponentQualifiersA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szcomponent: Param0,
    iindex: u32,
    lpqualifierbuf: super::super::Foundation::PSTR,
    pcchqualifierbuf: *mut u32,
    lpapplicationdatabuf: super::super::Foundation::PSTR,
    pcchapplicationdatabuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumComponentQualifiersA(
                szcomponent: super::super::Foundation::PSTR,
                iindex: u32,
                lpqualifierbuf: super::super::Foundation::PSTR,
                pcchqualifierbuf: *mut u32,
                lpapplicationdatabuf: super::super::Foundation::PSTR,
                pcchapplicationdatabuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumComponentQualifiersA(
            szcomponent.into_param().abi(),
            ::std::mem::transmute(iindex),
            ::std::mem::transmute(lpqualifierbuf),
            ::std::mem::transmute(pcchqualifierbuf),
            ::std::mem::transmute(lpapplicationdatabuf),
            ::std::mem::transmute(pcchapplicationdatabuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumComponentQualifiersW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szcomponent: Param0,
    iindex: u32,
    lpqualifierbuf: super::super::Foundation::PWSTR,
    pcchqualifierbuf: *mut u32,
    lpapplicationdatabuf: super::super::Foundation::PWSTR,
    pcchapplicationdatabuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumComponentQualifiersW(
                szcomponent: super::super::Foundation::PWSTR,
                iindex: u32,
                lpqualifierbuf: super::super::Foundation::PWSTR,
                pcchqualifierbuf: *mut u32,
                lpapplicationdatabuf: super::super::Foundation::PWSTR,
                pcchapplicationdatabuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumComponentQualifiersW(
            szcomponent.into_param().abi(),
            ::std::mem::transmute(iindex),
            ::std::mem::transmute(lpqualifierbuf),
            ::std::mem::transmute(pcchqualifierbuf),
            ::std::mem::transmute(lpapplicationdatabuf),
            ::std::mem::transmute(pcchapplicationdatabuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumComponentsA(
    icomponentindex: u32,
    lpcomponentbuf: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumComponentsA(
                icomponentindex: u32,
                lpcomponentbuf: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumComponentsA(
            ::std::mem::transmute(icomponentindex),
            ::std::mem::transmute(lpcomponentbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumComponentsExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szusersid: Param0,
    dwcontext: u32,
    dwindex: u32,
    szinstalledcomponentcode: super::super::Foundation::PSTR,
    pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
    szsid: super::super::Foundation::PSTR,
    pcchsid: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumComponentsExA(
                szusersid: super::super::Foundation::PSTR,
                dwcontext: u32,
                dwindex: u32,
                szinstalledcomponentcode: super::super::Foundation::PSTR,
                pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
                szsid: super::super::Foundation::PSTR,
                pcchsid: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumComponentsExA(
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(szinstalledcomponentcode),
            ::std::mem::transmute(pdwinstalledcontext),
            ::std::mem::transmute(szsid),
            ::std::mem::transmute(pcchsid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumComponentsExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szusersid: Param0,
    dwcontext: u32,
    dwindex: u32,
    szinstalledcomponentcode: super::super::Foundation::PWSTR,
    pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
    szsid: super::super::Foundation::PWSTR,
    pcchsid: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumComponentsExW(
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: u32,
                dwindex: u32,
                szinstalledcomponentcode: super::super::Foundation::PWSTR,
                pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
                szsid: super::super::Foundation::PWSTR,
                pcchsid: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumComponentsExW(
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(szinstalledcomponentcode),
            ::std::mem::transmute(pdwinstalledcontext),
            ::std::mem::transmute(szsid),
            ::std::mem::transmute(pcchsid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumComponentsW(
    icomponentindex: u32,
    lpcomponentbuf: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumComponentsW(
                icomponentindex: u32,
                lpcomponentbuf: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumComponentsW(
            ::std::mem::transmute(icomponentindex),
            ::std::mem::transmute(lpcomponentbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumFeaturesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    ifeatureindex: u32,
    lpfeaturebuf: super::super::Foundation::PSTR,
    lpparentbuf: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumFeaturesA(
                szproduct: super::super::Foundation::PSTR,
                ifeatureindex: u32,
                lpfeaturebuf: super::super::Foundation::PSTR,
                lpparentbuf: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumFeaturesA(
            szproduct.into_param().abi(),
            ::std::mem::transmute(ifeatureindex),
            ::std::mem::transmute(lpfeaturebuf),
            ::std::mem::transmute(lpparentbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumFeaturesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    ifeatureindex: u32,
    lpfeaturebuf: super::super::Foundation::PWSTR,
    lpparentbuf: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumFeaturesW(
                szproduct: super::super::Foundation::PWSTR,
                ifeatureindex: u32,
                lpfeaturebuf: super::super::Foundation::PWSTR,
                lpparentbuf: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumFeaturesW(
            szproduct.into_param().abi(),
            ::std::mem::transmute(ifeatureindex),
            ::std::mem::transmute(lpfeaturebuf),
            ::std::mem::transmute(lpparentbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumPatchesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    ipatchindex: u32,
    lppatchbuf: super::super::Foundation::PSTR,
    lptransformsbuf: super::super::Foundation::PSTR,
    pcchtransformsbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumPatchesA(
                szproduct: super::super::Foundation::PSTR,
                ipatchindex: u32,
                lppatchbuf: super::super::Foundation::PSTR,
                lptransformsbuf: super::super::Foundation::PSTR,
                pcchtransformsbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumPatchesA(
            szproduct.into_param().abi(),
            ::std::mem::transmute(ipatchindex),
            ::std::mem::transmute(lppatchbuf),
            ::std::mem::transmute(lptransformsbuf),
            ::std::mem::transmute(pcchtransformsbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumPatchesExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: u32,
    dwfilter: u32,
    dwindex: u32,
    szpatchcode: super::super::Foundation::PSTR,
    sztargetproductcode: super::super::Foundation::PSTR,
    pdwtargetproductcontext: *mut MSIINSTALLCONTEXT,
    sztargetusersid: super::super::Foundation::PSTR,
    pcchtargetusersid: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumPatchesExA(
                szproductcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: u32,
                dwfilter: u32,
                dwindex: u32,
                szpatchcode: super::super::Foundation::PSTR,
                sztargetproductcode: super::super::Foundation::PSTR,
                pdwtargetproductcontext: *mut MSIINSTALLCONTEXT,
                sztargetusersid: super::super::Foundation::PSTR,
                pcchtargetusersid: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumPatchesExA(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwfilter),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(szpatchcode),
            ::std::mem::transmute(sztargetproductcode),
            ::std::mem::transmute(pdwtargetproductcontext),
            ::std::mem::transmute(sztargetusersid),
            ::std::mem::transmute(pcchtargetusersid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumPatchesExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: u32,
    dwfilter: u32,
    dwindex: u32,
    szpatchcode: super::super::Foundation::PWSTR,
    sztargetproductcode: super::super::Foundation::PWSTR,
    pdwtargetproductcontext: *mut MSIINSTALLCONTEXT,
    sztargetusersid: super::super::Foundation::PWSTR,
    pcchtargetusersid: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumPatchesExW(
                szproductcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: u32,
                dwfilter: u32,
                dwindex: u32,
                szpatchcode: super::super::Foundation::PWSTR,
                sztargetproductcode: super::super::Foundation::PWSTR,
                pdwtargetproductcontext: *mut MSIINSTALLCONTEXT,
                sztargetusersid: super::super::Foundation::PWSTR,
                pcchtargetusersid: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumPatchesExW(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwfilter),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(szpatchcode),
            ::std::mem::transmute(sztargetproductcode),
            ::std::mem::transmute(pdwtargetproductcontext),
            ::std::mem::transmute(sztargetusersid),
            ::std::mem::transmute(pcchtargetusersid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumPatchesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    ipatchindex: u32,
    lppatchbuf: super::super::Foundation::PWSTR,
    lptransformsbuf: super::super::Foundation::PWSTR,
    pcchtransformsbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumPatchesW(
                szproduct: super::super::Foundation::PWSTR,
                ipatchindex: u32,
                lppatchbuf: super::super::Foundation::PWSTR,
                lptransformsbuf: super::super::Foundation::PWSTR,
                pcchtransformsbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumPatchesW(
            szproduct.into_param().abi(),
            ::std::mem::transmute(ipatchindex),
            ::std::mem::transmute(lppatchbuf),
            ::std::mem::transmute(lptransformsbuf),
            ::std::mem::transmute(pcchtransformsbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumProductsA(
    iproductindex: u32,
    lpproductbuf: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumProductsA(
                iproductindex: u32,
                lpproductbuf: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumProductsA(
            ::std::mem::transmute(iproductindex),
            ::std::mem::transmute(lpproductbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumProductsExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: u32,
    dwindex: u32,
    szinstalledproductcode: super::super::Foundation::PSTR,
    pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
    szsid: super::super::Foundation::PSTR,
    pcchsid: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumProductsExA(
                szproductcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: u32,
                dwindex: u32,
                szinstalledproductcode: super::super::Foundation::PSTR,
                pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
                szsid: super::super::Foundation::PSTR,
                pcchsid: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumProductsExA(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(szinstalledproductcode),
            ::std::mem::transmute(pdwinstalledcontext),
            ::std::mem::transmute(szsid),
            ::std::mem::transmute(pcchsid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumProductsExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: u32,
    dwindex: u32,
    szinstalledproductcode: super::super::Foundation::PWSTR,
    pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
    szsid: super::super::Foundation::PWSTR,
    pcchsid: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumProductsExW(
                szproductcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: u32,
                dwindex: u32,
                szinstalledproductcode: super::super::Foundation::PWSTR,
                pdwinstalledcontext: *mut MSIINSTALLCONTEXT,
                szsid: super::super::Foundation::PWSTR,
                pcchsid: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumProductsExW(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(szinstalledproductcode),
            ::std::mem::transmute(pdwinstalledcontext),
            ::std::mem::transmute(szsid),
            ::std::mem::transmute(pcchsid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumProductsW(
    iproductindex: u32,
    lpproductbuf: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumProductsW(
                iproductindex: u32,
                lpproductbuf: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumProductsW(
            ::std::mem::transmute(iproductindex),
            ::std::mem::transmute(lpproductbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumRelatedProductsA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpupgradecode: Param0,
    dwreserved: u32,
    iproductindex: u32,
    lpproductbuf: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumRelatedProductsA(
                lpupgradecode: super::super::Foundation::PSTR,
                dwreserved: u32,
                iproductindex: u32,
                lpproductbuf: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumRelatedProductsA(
            lpupgradecode.into_param().abi(),
            ::std::mem::transmute(dwreserved),
            ::std::mem::transmute(iproductindex),
            ::std::mem::transmute(lpproductbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEnumRelatedProductsW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpupgradecode: Param0,
    dwreserved: u32,
    iproductindex: u32,
    lpproductbuf: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEnumRelatedProductsW(
                lpupgradecode: super::super::Foundation::PWSTR,
                dwreserved: u32,
                iproductindex: u32,
                lpproductbuf: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiEnumRelatedProductsW(
            lpupgradecode.into_param().abi(),
            ::std::mem::transmute(dwreserved),
            ::std::mem::transmute(iproductindex),
            ::std::mem::transmute(lpproductbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEvaluateConditionA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szcondition: Param1,
) -> MSICONDITION {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEvaluateConditionA(
                hinstall: MSIHANDLE,
                szcondition: super::super::Foundation::PSTR,
            ) -> MSICONDITION;
        }
        ::std::mem::transmute(MsiEvaluateConditionA(
            hinstall.into_param().abi(),
            szcondition.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiEvaluateConditionW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szcondition: Param1,
) -> MSICONDITION {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiEvaluateConditionW(
                hinstall: MSIHANDLE,
                szcondition: super::super::Foundation::PWSTR,
            ) -> MSICONDITION;
        }
        ::std::mem::transmute(MsiEvaluateConditionW(
            hinstall.into_param().abi(),
            szcondition.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiExtractPatchXMLDataA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpatchpath: Param0,
    dwreserved: u32,
    szxmldata: super::super::Foundation::PSTR,
    pcchxmldata: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiExtractPatchXMLDataA(
                szpatchpath: super::super::Foundation::PSTR,
                dwreserved: u32,
                szxmldata: super::super::Foundation::PSTR,
                pcchxmldata: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiExtractPatchXMLDataA(
            szpatchpath.into_param().abi(),
            ::std::mem::transmute(dwreserved),
            ::std::mem::transmute(szxmldata),
            ::std::mem::transmute(pcchxmldata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiExtractPatchXMLDataW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpatchpath: Param0,
    dwreserved: u32,
    szxmldata: super::super::Foundation::PWSTR,
    pcchxmldata: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiExtractPatchXMLDataW(
                szpatchpath: super::super::Foundation::PWSTR,
                dwreserved: u32,
                szxmldata: super::super::Foundation::PWSTR,
                pcchxmldata: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiExtractPatchXMLDataW(
            szpatchpath.into_param().abi(),
            ::std::mem::transmute(dwreserved),
            ::std::mem::transmute(szxmldata),
            ::std::mem::transmute(pcchxmldata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiFormatRecordA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
>(
    hinstall: Param0,
    hrecord: Param1,
    szresultbuf: super::super::Foundation::PSTR,
    pcchresultbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiFormatRecordA(
                hinstall: MSIHANDLE,
                hrecord: MSIHANDLE,
                szresultbuf: super::super::Foundation::PSTR,
                pcchresultbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiFormatRecordA(
            hinstall.into_param().abi(),
            hrecord.into_param().abi(),
            ::std::mem::transmute(szresultbuf),
            ::std::mem::transmute(pcchresultbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiFormatRecordW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
>(
    hinstall: Param0,
    hrecord: Param1,
    szresultbuf: super::super::Foundation::PWSTR,
    pcchresultbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiFormatRecordW(
                hinstall: MSIHANDLE,
                hrecord: MSIHANDLE,
                szresultbuf: super::super::Foundation::PWSTR,
                pcchresultbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiFormatRecordW(
            hinstall.into_param().abi(),
            hrecord.into_param().abi(),
            ::std::mem::transmute(szresultbuf),
            ::std::mem::transmute(pcchresultbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiGetActiveDatabase<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hinstall: Param0,
) -> MSIHANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetActiveDatabase(hinstall: MSIHANDLE) -> MSIHANDLE;
        }
        ::std::mem::transmute(MsiGetActiveDatabase(hinstall.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetComponentPathA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szcomponent: Param1,
    lppathbuf: super::super::Foundation::PSTR,
    pcchbuf: *mut u32,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetComponentPathA(
                szproduct: super::super::Foundation::PSTR,
                szcomponent: super::super::Foundation::PSTR,
                lppathbuf: super::super::Foundation::PSTR,
                pcchbuf: *mut u32,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiGetComponentPathA(
            szproduct.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetComponentPathExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcode: Param0,
    szcomponentcode: Param1,
    szusersid: Param2,
    dwcontext: MSIINSTALLCONTEXT,
    lpoutpathbuffer: super::super::Foundation::PSTR,
    pcchoutpathbuffer: *mut u32,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetComponentPathExA(
                szproductcode: super::super::Foundation::PSTR,
                szcomponentcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                lpoutpathbuffer: super::super::Foundation::PSTR,
                pcchoutpathbuffer: *mut u32,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiGetComponentPathExA(
            szproductcode.into_param().abi(),
            szcomponentcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(lpoutpathbuffer),
            ::std::mem::transmute(pcchoutpathbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetComponentPathExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcode: Param0,
    szcomponentcode: Param1,
    szusersid: Param2,
    dwcontext: MSIINSTALLCONTEXT,
    lpoutpathbuffer: super::super::Foundation::PWSTR,
    pcchoutpathbuffer: *mut u32,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetComponentPathExW(
                szproductcode: super::super::Foundation::PWSTR,
                szcomponentcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                lpoutpathbuffer: super::super::Foundation::PWSTR,
                pcchoutpathbuffer: *mut u32,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiGetComponentPathExW(
            szproductcode.into_param().abi(),
            szcomponentcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(lpoutpathbuffer),
            ::std::mem::transmute(pcchoutpathbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetComponentPathW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szcomponent: Param1,
    lppathbuf: super::super::Foundation::PWSTR,
    pcchbuf: *mut u32,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetComponentPathW(
                szproduct: super::super::Foundation::PWSTR,
                szcomponent: super::super::Foundation::PWSTR,
                lppathbuf: super::super::Foundation::PWSTR,
                pcchbuf: *mut u32,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiGetComponentPathW(
            szproduct.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetComponentStateA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szcomponent: Param1,
    piinstalled: *mut INSTALLSTATE,
    piaction: *mut INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetComponentStateA(
                hinstall: MSIHANDLE,
                szcomponent: super::super::Foundation::PSTR,
                piinstalled: *mut INSTALLSTATE,
                piaction: *mut INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetComponentStateA(
            hinstall.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(piinstalled),
            ::std::mem::transmute(piaction),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetComponentStateW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szcomponent: Param1,
    piinstalled: *mut INSTALLSTATE,
    piaction: *mut INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetComponentStateW(
                hinstall: MSIHANDLE,
                szcomponent: super::super::Foundation::PWSTR,
                piinstalled: *mut INSTALLSTATE,
                piaction: *mut INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetComponentStateW(
            hinstall.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(piinstalled),
            ::std::mem::transmute(piaction),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiGetDatabaseState<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hdatabase: Param0,
) -> MSIDBSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetDatabaseState(hdatabase: MSIHANDLE) -> MSIDBSTATE;
        }
        ::std::mem::transmute(MsiGetDatabaseState(hdatabase.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFeatureCostA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szfeature: Param1,
    icosttree: MSICOSTTREE,
    istate: INSTALLSTATE,
    picost: *mut i32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFeatureCostA(
                hinstall: MSIHANDLE,
                szfeature: super::super::Foundation::PSTR,
                icosttree: MSICOSTTREE,
                istate: INSTALLSTATE,
                picost: *mut i32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFeatureCostA(
            hinstall.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(icosttree),
            ::std::mem::transmute(istate),
            ::std::mem::transmute(picost),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFeatureCostW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szfeature: Param1,
    icosttree: MSICOSTTREE,
    istate: INSTALLSTATE,
    picost: *mut i32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFeatureCostW(
                hinstall: MSIHANDLE,
                szfeature: super::super::Foundation::PWSTR,
                icosttree: MSICOSTTREE,
                istate: INSTALLSTATE,
                picost: *mut i32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFeatureCostW(
            hinstall.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(icosttree),
            ::std::mem::transmute(istate),
            ::std::mem::transmute(picost),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFeatureInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hproduct: Param0,
    szfeature: Param1,
    lpattributes: *mut u32,
    lptitlebuf: super::super::Foundation::PSTR,
    pcchtitlebuf: *mut u32,
    lphelpbuf: super::super::Foundation::PSTR,
    pcchhelpbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFeatureInfoA(
                hproduct: MSIHANDLE,
                szfeature: super::super::Foundation::PSTR,
                lpattributes: *mut u32,
                lptitlebuf: super::super::Foundation::PSTR,
                pcchtitlebuf: *mut u32,
                lphelpbuf: super::super::Foundation::PSTR,
                pcchhelpbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFeatureInfoA(
            hproduct.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(lpattributes),
            ::std::mem::transmute(lptitlebuf),
            ::std::mem::transmute(pcchtitlebuf),
            ::std::mem::transmute(lphelpbuf),
            ::std::mem::transmute(pcchhelpbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFeatureInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hproduct: Param0,
    szfeature: Param1,
    lpattributes: *mut u32,
    lptitlebuf: super::super::Foundation::PWSTR,
    pcchtitlebuf: *mut u32,
    lphelpbuf: super::super::Foundation::PWSTR,
    pcchhelpbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFeatureInfoW(
                hproduct: MSIHANDLE,
                szfeature: super::super::Foundation::PWSTR,
                lpattributes: *mut u32,
                lptitlebuf: super::super::Foundation::PWSTR,
                pcchtitlebuf: *mut u32,
                lphelpbuf: super::super::Foundation::PWSTR,
                pcchhelpbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFeatureInfoW(
            hproduct.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(lpattributes),
            ::std::mem::transmute(lptitlebuf),
            ::std::mem::transmute(pcchtitlebuf),
            ::std::mem::transmute(lphelpbuf),
            ::std::mem::transmute(pcchhelpbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFeatureStateA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szfeature: Param1,
    piinstalled: *mut INSTALLSTATE,
    piaction: *mut INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFeatureStateA(
                hinstall: MSIHANDLE,
                szfeature: super::super::Foundation::PSTR,
                piinstalled: *mut INSTALLSTATE,
                piaction: *mut INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFeatureStateA(
            hinstall.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(piinstalled),
            ::std::mem::transmute(piaction),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFeatureStateW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szfeature: Param1,
    piinstalled: *mut INSTALLSTATE,
    piaction: *mut INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFeatureStateW(
                hinstall: MSIHANDLE,
                szfeature: super::super::Foundation::PWSTR,
                piinstalled: *mut INSTALLSTATE,
                piaction: *mut INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFeatureStateW(
            hinstall.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(piinstalled),
            ::std::mem::transmute(piaction),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFeatureUsageA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
    pdwusecount: *mut u32,
    pwdateused: *mut u16,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFeatureUsageA(
                szproduct: super::super::Foundation::PSTR,
                szfeature: super::super::Foundation::PSTR,
                pdwusecount: *mut u32,
                pwdateused: *mut u16,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFeatureUsageA(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(pdwusecount),
            ::std::mem::transmute(pwdateused),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFeatureUsageW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
    pdwusecount: *mut u32,
    pwdateused: *mut u16,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFeatureUsageW(
                szproduct: super::super::Foundation::PWSTR,
                szfeature: super::super::Foundation::PWSTR,
                pdwusecount: *mut u32,
                pwdateused: *mut u16,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFeatureUsageW(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(pdwusecount),
            ::std::mem::transmute(pwdateused),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFeatureValidStatesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szfeature: Param1,
    lpinstallstates: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFeatureValidStatesA(
                hinstall: MSIHANDLE,
                szfeature: super::super::Foundation::PSTR,
                lpinstallstates: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFeatureValidStatesA(
            hinstall.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(lpinstallstates),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFeatureValidStatesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szfeature: Param1,
    lpinstallstates: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFeatureValidStatesW(
                hinstall: MSIHANDLE,
                szfeature: super::super::Foundation::PWSTR,
                lpinstallstates: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFeatureValidStatesW(
            hinstall.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(lpinstallstates),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFileHashA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szfilepath: Param0,
    dwoptions: u32,
    phash: *mut MSIFILEHASHINFO,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFileHashA(
                szfilepath: super::super::Foundation::PSTR,
                dwoptions: u32,
                phash: *mut MSIFILEHASHINFO,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFileHashA(
            szfilepath.into_param().abi(),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(phash),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFileHashW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szfilepath: Param0,
    dwoptions: u32,
    phash: *mut MSIFILEHASHINFO,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFileHashW(
                szfilepath: super::super::Foundation::PWSTR,
                dwoptions: u32,
                phash: *mut MSIFILEHASHINFO,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFileHashW(
            szfilepath.into_param().abi(),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(phash),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szsignedobjectpath: Param0,
    dwflags: u32,
    ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pbhashdata: *mut u8,
    pcbhashdata: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFileSignatureInformationA(
                szsignedobjectpath: super::super::Foundation::PSTR,
                dwflags: u32,
                ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
                pbhashdata: *mut u8,
                pcbhashdata: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        MsiGetFileSignatureInformationA(
            szsignedobjectpath.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(ppccertcontext),
            ::std::mem::transmute(pbhashdata),
            ::std::mem::transmute(pcbhashdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szsignedobjectpath: Param0,
    dwflags: u32,
    ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pbhashdata: *mut u8,
    pcbhashdata: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFileSignatureInformationW(
                szsignedobjectpath: super::super::Foundation::PWSTR,
                dwflags: u32,
                ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT,
                pbhashdata: *mut u8,
                pcbhashdata: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        MsiGetFileSignatureInformationW(
            szsignedobjectpath.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(ppccertcontext),
            ::std::mem::transmute(pbhashdata),
            ::std::mem::transmute(pcbhashdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFileVersionA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szfilepath: Param0,
    lpversionbuf: super::super::Foundation::PSTR,
    pcchversionbuf: *mut u32,
    lplangbuf: super::super::Foundation::PSTR,
    pcchlangbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFileVersionA(
                szfilepath: super::super::Foundation::PSTR,
                lpversionbuf: super::super::Foundation::PSTR,
                pcchversionbuf: *mut u32,
                lplangbuf: super::super::Foundation::PSTR,
                pcchlangbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFileVersionA(
            szfilepath.into_param().abi(),
            ::std::mem::transmute(lpversionbuf),
            ::std::mem::transmute(pcchversionbuf),
            ::std::mem::transmute(lplangbuf),
            ::std::mem::transmute(pcchlangbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetFileVersionW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szfilepath: Param0,
    lpversionbuf: super::super::Foundation::PWSTR,
    pcchversionbuf: *mut u32,
    lplangbuf: super::super::Foundation::PWSTR,
    pcchlangbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetFileVersionW(
                szfilepath: super::super::Foundation::PWSTR,
                lpversionbuf: super::super::Foundation::PWSTR,
                pcchversionbuf: *mut u32,
                lplangbuf: super::super::Foundation::PWSTR,
                pcchlangbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetFileVersionW(
            szfilepath.into_param().abi(),
            ::std::mem::transmute(lpversionbuf),
            ::std::mem::transmute(pcchversionbuf),
            ::std::mem::transmute(lplangbuf),
            ::std::mem::transmute(pcchlangbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiGetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hinstall: Param0,
) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetLanguage(hinstall: MSIHANDLE) -> u16;
        }
        ::std::mem::transmute(MsiGetLanguage(hinstall.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiGetLastErrorRecord() -> MSIHANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetLastErrorRecord() -> MSIHANDLE;
        }
        ::std::mem::transmute(MsiGetLastErrorRecord())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetMode<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hinstall: Param0,
    erunmode: MSIRUNMODE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetMode(
                hinstall: MSIHANDLE,
                erunmode: MSIRUNMODE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MsiGetMode(
            hinstall.into_param().abi(),
            ::std::mem::transmute(erunmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetPatchFileListA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcode: Param0,
    szpatchpackages: Param1,
    pcfiles: *mut u32,
    pphfilerecords: *mut *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetPatchFileListA(
                szproductcode: super::super::Foundation::PSTR,
                szpatchpackages: super::super::Foundation::PSTR,
                pcfiles: *mut u32,
                pphfilerecords: *mut *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetPatchFileListA(
            szproductcode.into_param().abi(),
            szpatchpackages.into_param().abi(),
            ::std::mem::transmute(pcfiles),
            ::std::mem::transmute(pphfilerecords),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetPatchFileListW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcode: Param0,
    szpatchpackages: Param1,
    pcfiles: *mut u32,
    pphfilerecords: *mut *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetPatchFileListW(
                szproductcode: super::super::Foundation::PWSTR,
                szpatchpackages: super::super::Foundation::PWSTR,
                pcfiles: *mut u32,
                pphfilerecords: *mut *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetPatchFileListW(
            szproductcode.into_param().abi(),
            szpatchpackages.into_param().abi(),
            ::std::mem::transmute(pcfiles),
            ::std::mem::transmute(pphfilerecords),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetPatchInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpatch: Param0,
    szattribute: Param1,
    lpvaluebuf: super::super::Foundation::PSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetPatchInfoA(
                szpatch: super::super::Foundation::PSTR,
                szattribute: super::super::Foundation::PSTR,
                lpvaluebuf: super::super::Foundation::PSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetPatchInfoA(
            szpatch.into_param().abi(),
            szattribute.into_param().abi(),
            ::std::mem::transmute(lpvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetPatchInfoExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpatchcode: Param0,
    szproductcode: Param1,
    szusersid: Param2,
    dwcontext: MSIINSTALLCONTEXT,
    szproperty: Param4,
    lpvalue: super::super::Foundation::PSTR,
    pcchvalue: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetPatchInfoExA(
                szpatchcode: super::super::Foundation::PSTR,
                szproductcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                szproperty: super::super::Foundation::PSTR,
                lpvalue: super::super::Foundation::PSTR,
                pcchvalue: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetPatchInfoExA(
            szpatchcode.into_param().abi(),
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            szproperty.into_param().abi(),
            ::std::mem::transmute(lpvalue),
            ::std::mem::transmute(pcchvalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetPatchInfoExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpatchcode: Param0,
    szproductcode: Param1,
    szusersid: Param2,
    dwcontext: MSIINSTALLCONTEXT,
    szproperty: Param4,
    lpvalue: super::super::Foundation::PWSTR,
    pcchvalue: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetPatchInfoExW(
                szpatchcode: super::super::Foundation::PWSTR,
                szproductcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                szproperty: super::super::Foundation::PWSTR,
                lpvalue: super::super::Foundation::PWSTR,
                pcchvalue: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetPatchInfoExW(
            szpatchcode.into_param().abi(),
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            szproperty.into_param().abi(),
            ::std::mem::transmute(lpvalue),
            ::std::mem::transmute(pcchvalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetPatchInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpatch: Param0,
    szattribute: Param1,
    lpvaluebuf: super::super::Foundation::PWSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetPatchInfoW(
                szpatch: super::super::Foundation::PWSTR,
                szattribute: super::super::Foundation::PWSTR,
                lpvaluebuf: super::super::Foundation::PWSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetPatchInfoW(
            szpatch.into_param().abi(),
            szattribute.into_param().abi(),
            ::std::mem::transmute(lpvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetProductCodeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szcomponent: Param0,
    lpbuf39: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetProductCodeA(
                szcomponent: super::super::Foundation::PSTR,
                lpbuf39: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetProductCodeA(
            szcomponent.into_param().abi(),
            ::std::mem::transmute(lpbuf39),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetProductCodeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szcomponent: Param0,
    lpbuf39: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetProductCodeW(
                szcomponent: super::super::Foundation::PWSTR,
                lpbuf39: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetProductCodeW(
            szcomponent.into_param().abi(),
            ::std::mem::transmute(lpbuf39),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetProductInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szattribute: Param1,
    lpvaluebuf: super::super::Foundation::PSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetProductInfoA(
                szproduct: super::super::Foundation::PSTR,
                szattribute: super::super::Foundation::PSTR,
                lpvaluebuf: super::super::Foundation::PSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetProductInfoA(
            szproduct.into_param().abi(),
            szattribute.into_param().abi(),
            ::std::mem::transmute(lpvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetProductInfoExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    szproperty: Param3,
    szvalue: super::super::Foundation::PSTR,
    pcchvalue: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetProductInfoExA(
                szproductcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                szproperty: super::super::Foundation::PSTR,
                szvalue: super::super::Foundation::PSTR,
                pcchvalue: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetProductInfoExA(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            szproperty.into_param().abi(),
            ::std::mem::transmute(szvalue),
            ::std::mem::transmute(pcchvalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetProductInfoExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    szproperty: Param3,
    szvalue: super::super::Foundation::PWSTR,
    pcchvalue: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetProductInfoExW(
                szproductcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                szproperty: super::super::Foundation::PWSTR,
                szvalue: super::super::Foundation::PWSTR,
                pcchvalue: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetProductInfoExW(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            szproperty.into_param().abi(),
            ::std::mem::transmute(szvalue),
            ::std::mem::transmute(pcchvalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szscriptfile: Param0,
    lpproductbuf39: super::super::Foundation::PSTR,
    plgidlanguage: *mut u16,
    pdwversion: *mut u32,
    lpnamebuf: super::super::Foundation::PSTR,
    pcchnamebuf: *mut u32,
    lppackagebuf: super::super::Foundation::PSTR,
    pcchpackagebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetProductInfoFromScriptA(
                szscriptfile: super::super::Foundation::PSTR,
                lpproductbuf39: super::super::Foundation::PSTR,
                plgidlanguage: *mut u16,
                pdwversion: *mut u32,
                lpnamebuf: super::super::Foundation::PSTR,
                pcchnamebuf: *mut u32,
                lppackagebuf: super::super::Foundation::PSTR,
                pcchpackagebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetProductInfoFromScriptA(
            szscriptfile.into_param().abi(),
            ::std::mem::transmute(lpproductbuf39),
            ::std::mem::transmute(plgidlanguage),
            ::std::mem::transmute(pdwversion),
            ::std::mem::transmute(lpnamebuf),
            ::std::mem::transmute(pcchnamebuf),
            ::std::mem::transmute(lppackagebuf),
            ::std::mem::transmute(pcchpackagebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szscriptfile: Param0,
    lpproductbuf39: super::super::Foundation::PWSTR,
    plgidlanguage: *mut u16,
    pdwversion: *mut u32,
    lpnamebuf: super::super::Foundation::PWSTR,
    pcchnamebuf: *mut u32,
    lppackagebuf: super::super::Foundation::PWSTR,
    pcchpackagebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetProductInfoFromScriptW(
                szscriptfile: super::super::Foundation::PWSTR,
                lpproductbuf39: super::super::Foundation::PWSTR,
                plgidlanguage: *mut u16,
                pdwversion: *mut u32,
                lpnamebuf: super::super::Foundation::PWSTR,
                pcchnamebuf: *mut u32,
                lppackagebuf: super::super::Foundation::PWSTR,
                pcchpackagebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetProductInfoFromScriptW(
            szscriptfile.into_param().abi(),
            ::std::mem::transmute(lpproductbuf39),
            ::std::mem::transmute(plgidlanguage),
            ::std::mem::transmute(pdwversion),
            ::std::mem::transmute(lpnamebuf),
            ::std::mem::transmute(pcchnamebuf),
            ::std::mem::transmute(lppackagebuf),
            ::std::mem::transmute(pcchpackagebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetProductInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szattribute: Param1,
    lpvaluebuf: super::super::Foundation::PWSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetProductInfoW(
                szproduct: super::super::Foundation::PWSTR,
                szattribute: super::super::Foundation::PWSTR,
                lpvaluebuf: super::super::Foundation::PWSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetProductInfoW(
            szproduct.into_param().abi(),
            szattribute.into_param().abi(),
            ::std::mem::transmute(lpvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetProductPropertyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hproduct: Param0,
    szproperty: Param1,
    lpvaluebuf: super::super::Foundation::PSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetProductPropertyA(
                hproduct: MSIHANDLE,
                szproperty: super::super::Foundation::PSTR,
                lpvaluebuf: super::super::Foundation::PSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetProductPropertyA(
            hproduct.into_param().abi(),
            szproperty.into_param().abi(),
            ::std::mem::transmute(lpvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetProductPropertyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hproduct: Param0,
    szproperty: Param1,
    lpvaluebuf: super::super::Foundation::PWSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetProductPropertyW(
                hproduct: MSIHANDLE,
                szproperty: super::super::Foundation::PWSTR,
                lpvaluebuf: super::super::Foundation::PWSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetProductPropertyW(
            hproduct.into_param().abi(),
            szproperty.into_param().abi(),
            ::std::mem::transmute(lpvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetPropertyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szname: Param1,
    szvaluebuf: super::super::Foundation::PSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetPropertyA(
                hinstall: MSIHANDLE,
                szname: super::super::Foundation::PSTR,
                szvaluebuf: super::super::Foundation::PSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetPropertyA(
            hinstall.into_param().abi(),
            szname.into_param().abi(),
            ::std::mem::transmute(szvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetPropertyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szname: Param1,
    szvaluebuf: super::super::Foundation::PWSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetPropertyW(
                hinstall: MSIHANDLE,
                szname: super::super::Foundation::PWSTR,
                szvaluebuf: super::super::Foundation::PWSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetPropertyW(
            hinstall.into_param().abi(),
            szname.into_param().abi(),
            ::std::mem::transmute(szvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetShortcutTargetA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szshortcutpath: Param0,
    szproductcode: super::super::Foundation::PSTR,
    szfeatureid: super::super::Foundation::PSTR,
    szcomponentcode: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetShortcutTargetA(
                szshortcutpath: super::super::Foundation::PSTR,
                szproductcode: super::super::Foundation::PSTR,
                szfeatureid: super::super::Foundation::PSTR,
                szcomponentcode: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetShortcutTargetA(
            szshortcutpath.into_param().abi(),
            ::std::mem::transmute(szproductcode),
            ::std::mem::transmute(szfeatureid),
            ::std::mem::transmute(szcomponentcode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetShortcutTargetW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szshortcutpath: Param0,
    szproductcode: super::super::Foundation::PWSTR,
    szfeatureid: super::super::Foundation::PWSTR,
    szcomponentcode: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetShortcutTargetW(
                szshortcutpath: super::super::Foundation::PWSTR,
                szproductcode: super::super::Foundation::PWSTR,
                szfeatureid: super::super::Foundation::PWSTR,
                szcomponentcode: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetShortcutTargetW(
            szshortcutpath.into_param().abi(),
            ::std::mem::transmute(szproductcode),
            ::std::mem::transmute(szfeatureid),
            ::std::mem::transmute(szcomponentcode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetSourcePathA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szfolder: Param1,
    szpathbuf: super::super::Foundation::PSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetSourcePathA(
                hinstall: MSIHANDLE,
                szfolder: super::super::Foundation::PSTR,
                szpathbuf: super::super::Foundation::PSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetSourcePathA(
            hinstall.into_param().abi(),
            szfolder.into_param().abi(),
            ::std::mem::transmute(szpathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetSourcePathW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szfolder: Param1,
    szpathbuf: super::super::Foundation::PWSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetSourcePathW(
                hinstall: MSIHANDLE,
                szfolder: super::super::Foundation::PWSTR,
                szpathbuf: super::super::Foundation::PWSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetSourcePathW(
            hinstall.into_param().abi(),
            szfolder.into_param().abi(),
            ::std::mem::transmute(szpathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetSummaryInformationA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hdatabase: Param0,
    szdatabasepath: Param1,
    uiupdatecount: u32,
    phsummaryinfo: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetSummaryInformationA(
                hdatabase: MSIHANDLE,
                szdatabasepath: super::super::Foundation::PSTR,
                uiupdatecount: u32,
                phsummaryinfo: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetSummaryInformationA(
            hdatabase.into_param().abi(),
            szdatabasepath.into_param().abi(),
            ::std::mem::transmute(uiupdatecount),
            ::std::mem::transmute(phsummaryinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetSummaryInformationW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdatabase: Param0,
    szdatabasepath: Param1,
    uiupdatecount: u32,
    phsummaryinfo: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetSummaryInformationW(
                hdatabase: MSIHANDLE,
                szdatabasepath: super::super::Foundation::PWSTR,
                uiupdatecount: u32,
                phsummaryinfo: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetSummaryInformationW(
            hdatabase.into_param().abi(),
            szdatabasepath.into_param().abi(),
            ::std::mem::transmute(uiupdatecount),
            ::std::mem::transmute(phsummaryinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetTargetPathA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szfolder: Param1,
    szpathbuf: super::super::Foundation::PSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetTargetPathA(
                hinstall: MSIHANDLE,
                szfolder: super::super::Foundation::PSTR,
                szpathbuf: super::super::Foundation::PSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetTargetPathA(
            hinstall.into_param().abi(),
            szfolder.into_param().abi(),
            ::std::mem::transmute(szpathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetTargetPathW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szfolder: Param1,
    szpathbuf: super::super::Foundation::PWSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetTargetPathW(
                hinstall: MSIHANDLE,
                szfolder: super::super::Foundation::PWSTR,
                szpathbuf: super::super::Foundation::PWSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiGetTargetPathW(
            hinstall.into_param().abi(),
            szfolder.into_param().abi(),
            ::std::mem::transmute(szpathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetUserInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    lpusernamebuf: super::super::Foundation::PSTR,
    pcchusernamebuf: *mut u32,
    lporgnamebuf: super::super::Foundation::PSTR,
    pcchorgnamebuf: *mut u32,
    lpserialbuf: super::super::Foundation::PSTR,
    pcchserialbuf: *mut u32,
) -> USERINFOSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetUserInfoA(
                szproduct: super::super::Foundation::PSTR,
                lpusernamebuf: super::super::Foundation::PSTR,
                pcchusernamebuf: *mut u32,
                lporgnamebuf: super::super::Foundation::PSTR,
                pcchorgnamebuf: *mut u32,
                lpserialbuf: super::super::Foundation::PSTR,
                pcchserialbuf: *mut u32,
            ) -> USERINFOSTATE;
        }
        ::std::mem::transmute(MsiGetUserInfoA(
            szproduct.into_param().abi(),
            ::std::mem::transmute(lpusernamebuf),
            ::std::mem::transmute(pcchusernamebuf),
            ::std::mem::transmute(lporgnamebuf),
            ::std::mem::transmute(pcchorgnamebuf),
            ::std::mem::transmute(lpserialbuf),
            ::std::mem::transmute(pcchserialbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetUserInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    lpusernamebuf: super::super::Foundation::PWSTR,
    pcchusernamebuf: *mut u32,
    lporgnamebuf: super::super::Foundation::PWSTR,
    pcchorgnamebuf: *mut u32,
    lpserialbuf: super::super::Foundation::PWSTR,
    pcchserialbuf: *mut u32,
) -> USERINFOSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiGetUserInfoW(
                szproduct: super::super::Foundation::PWSTR,
                lpusernamebuf: super::super::Foundation::PWSTR,
                pcchusernamebuf: *mut u32,
                lporgnamebuf: super::super::Foundation::PWSTR,
                pcchorgnamebuf: *mut u32,
                lpserialbuf: super::super::Foundation::PWSTR,
                pcchserialbuf: *mut u32,
            ) -> USERINFOSTATE;
        }
        ::std::mem::transmute(MsiGetUserInfoW(
            szproduct.into_param().abi(),
            ::std::mem::transmute(lpusernamebuf),
            ::std::mem::transmute(pcchusernamebuf),
            ::std::mem::transmute(lporgnamebuf),
            ::std::mem::transmute(pcchorgnamebuf),
            ::std::mem::transmute(lpserialbuf),
            ::std::mem::transmute(pcchserialbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiInstallMissingComponentA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szcomponent: Param1,
    einstallstate: INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiInstallMissingComponentA(
                szproduct: super::super::Foundation::PSTR,
                szcomponent: super::super::Foundation::PSTR,
                einstallstate: INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiInstallMissingComponentA(
            szproduct.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(einstallstate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiInstallMissingComponentW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szcomponent: Param1,
    einstallstate: INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiInstallMissingComponentW(
                szproduct: super::super::Foundation::PWSTR,
                szcomponent: super::super::Foundation::PWSTR,
                einstallstate: INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiInstallMissingComponentW(
            szproduct.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(einstallstate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiInstallMissingFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szfile: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiInstallMissingFileA(
                szproduct: super::super::Foundation::PSTR,
                szfile: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiInstallMissingFileA(
            szproduct.into_param().abi(),
            szfile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiInstallMissingFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szfile: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiInstallMissingFileW(
                szproduct: super::super::Foundation::PWSTR,
                szfile: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiInstallMissingFileW(
            szproduct.into_param().abi(),
            szfile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiInstallProductA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpackagepath: Param0,
    szcommandline: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiInstallProductA(
                szpackagepath: super::super::Foundation::PSTR,
                szcommandline: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiInstallProductA(
            szpackagepath.into_param().abi(),
            szcommandline.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiInstallProductW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpackagepath: Param0,
    szcommandline: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiInstallProductW(
                szpackagepath: super::super::Foundation::PWSTR,
                szcommandline: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiInstallProductW(
            szpackagepath.into_param().abi(),
            szcommandline.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiIsProductElevatedA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    pfelevated: *mut super::super::Foundation::BOOL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiIsProductElevatedA(
                szproduct: super::super::Foundation::PSTR,
                pfelevated: *mut super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(MsiIsProductElevatedA(
            szproduct.into_param().abi(),
            ::std::mem::transmute(pfelevated),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiIsProductElevatedW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    pfelevated: *mut super::super::Foundation::BOOL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiIsProductElevatedW(
                szproduct: super::super::Foundation::PWSTR,
                pfelevated: *mut super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(MsiIsProductElevatedW(
            szproduct.into_param().abi(),
            ::std::mem::transmute(pfelevated),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiJoinTransaction<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    htransactionhandle: Param0,
    dwtransactionattributes: u32,
    phchangeofownerevent: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiJoinTransaction(
                htransactionhandle: MSIHANDLE,
                dwtransactionattributes: u32,
                phchangeofownerevent: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiJoinTransaction(
            htransactionhandle.into_param().abi(),
            ::std::mem::transmute(dwtransactionattributes),
            ::std::mem::transmute(phchangeofownerevent),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiLocateComponentA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szcomponent: Param0,
    lppathbuf: super::super::Foundation::PSTR,
    pcchbuf: *mut u32,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiLocateComponentA(
                szcomponent: super::super::Foundation::PSTR,
                lppathbuf: super::super::Foundation::PSTR,
                pcchbuf: *mut u32,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiLocateComponentA(
            szcomponent.into_param().abi(),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiLocateComponentW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szcomponent: Param0,
    lppathbuf: super::super::Foundation::PWSTR,
    pcchbuf: *mut u32,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiLocateComponentW(
                szcomponent: super::super::Foundation::PWSTR,
                lppathbuf: super::super::Foundation::PWSTR,
                pcchbuf: *mut u32,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiLocateComponentW(
            szcomponent.into_param().abi(),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiNotifySidChangeA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    poldsid: Param0,
    pnewsid: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiNotifySidChangeA(
                poldsid: super::super::Foundation::PSTR,
                pnewsid: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiNotifySidChangeA(
            poldsid.into_param().abi(),
            pnewsid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiNotifySidChangeW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    poldsid: Param0,
    pnewsid: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiNotifySidChangeW(
                poldsid: super::super::Foundation::PWSTR,
                pnewsid: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiNotifySidChangeW(
            poldsid.into_param().abi(),
            pnewsid.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiOpenDatabaseA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szdatabasepath: Param0,
    szpersist: Param1,
    phdatabase: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiOpenDatabaseA(
                szdatabasepath: super::super::Foundation::PSTR,
                szpersist: super::super::Foundation::PSTR,
                phdatabase: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiOpenDatabaseA(
            szdatabasepath.into_param().abi(),
            szpersist.into_param().abi(),
            ::std::mem::transmute(phdatabase),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiOpenDatabaseW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szdatabasepath: Param0,
    szpersist: Param1,
    phdatabase: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiOpenDatabaseW(
                szdatabasepath: super::super::Foundation::PWSTR,
                szpersist: super::super::Foundation::PWSTR,
                phdatabase: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiOpenDatabaseW(
            szdatabasepath.into_param().abi(),
            szpersist.into_param().abi(),
            ::std::mem::transmute(phdatabase),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiOpenPackageA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpackagepath: Param0,
    hproduct: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiOpenPackageA(
                szpackagepath: super::super::Foundation::PSTR,
                hproduct: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiOpenPackageA(
            szpackagepath.into_param().abi(),
            ::std::mem::transmute(hproduct),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiOpenPackageExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpackagepath: Param0,
    dwoptions: u32,
    hproduct: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiOpenPackageExA(
                szpackagepath: super::super::Foundation::PSTR,
                dwoptions: u32,
                hproduct: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiOpenPackageExA(
            szpackagepath.into_param().abi(),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(hproduct),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiOpenPackageExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpackagepath: Param0,
    dwoptions: u32,
    hproduct: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiOpenPackageExW(
                szpackagepath: super::super::Foundation::PWSTR,
                dwoptions: u32,
                hproduct: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiOpenPackageExW(
            szpackagepath.into_param().abi(),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(hproduct),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiOpenPackageW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpackagepath: Param0,
    hproduct: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiOpenPackageW(
                szpackagepath: super::super::Foundation::PWSTR,
                hproduct: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiOpenPackageW(
            szpackagepath.into_param().abi(),
            ::std::mem::transmute(hproduct),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiOpenProductA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    hproduct: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiOpenProductA(
                szproduct: super::super::Foundation::PSTR,
                hproduct: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiOpenProductA(
            szproduct.into_param().abi(),
            ::std::mem::transmute(hproduct),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiOpenProductW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    hproduct: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiOpenProductW(
                szproduct: super::super::Foundation::PWSTR,
                hproduct: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiOpenProductW(
            szproduct.into_param().abi(),
            ::std::mem::transmute(hproduct),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiPreviewBillboardA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hpreview: Param0,
    szcontrolname: Param1,
    szbillboard: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiPreviewBillboardA(
                hpreview: MSIHANDLE,
                szcontrolname: super::super::Foundation::PSTR,
                szbillboard: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiPreviewBillboardA(
            hpreview.into_param().abi(),
            szcontrolname.into_param().abi(),
            szbillboard.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiPreviewBillboardW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hpreview: Param0,
    szcontrolname: Param1,
    szbillboard: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiPreviewBillboardW(
                hpreview: MSIHANDLE,
                szcontrolname: super::super::Foundation::PWSTR,
                szbillboard: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiPreviewBillboardW(
            hpreview.into_param().abi(),
            szcontrolname.into_param().abi(),
            szbillboard.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiPreviewDialogA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hpreview: Param0,
    szdialogname: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiPreviewDialogA(
                hpreview: MSIHANDLE,
                szdialogname: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiPreviewDialogA(
            hpreview.into_param().abi(),
            szdialogname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiPreviewDialogW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hpreview: Param0,
    szdialogname: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiPreviewDialogW(
                hpreview: MSIHANDLE,
                szdialogname: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiPreviewDialogW(
            hpreview.into_param().abi(),
            szdialogname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    szscriptfile: Param0,
    sziconfolder: Param1,
    hregdata: Param2,
    fshortcuts: Param3,
    fremoveitems: Param4,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiProcessAdvertiseScriptA(
                szscriptfile: super::super::Foundation::PSTR,
                sziconfolder: super::super::Foundation::PSTR,
                hregdata: super::Registry::HKEY,
                fshortcuts: super::super::Foundation::BOOL,
                fremoveitems: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(MsiProcessAdvertiseScriptA(
            szscriptfile.into_param().abi(),
            sziconfolder.into_param().abi(),
            hregdata.into_param().abi(),
            fshortcuts.into_param().abi(),
            fremoveitems.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    szscriptfile: Param0,
    sziconfolder: Param1,
    hregdata: Param2,
    fshortcuts: Param3,
    fremoveitems: Param4,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiProcessAdvertiseScriptW(
                szscriptfile: super::super::Foundation::PWSTR,
                sziconfolder: super::super::Foundation::PWSTR,
                hregdata: super::Registry::HKEY,
                fshortcuts: super::super::Foundation::BOOL,
                fremoveitems: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(MsiProcessAdvertiseScriptW(
            szscriptfile.into_param().abi(),
            sziconfolder.into_param().abi(),
            hregdata.into_param().abi(),
            fshortcuts.into_param().abi(),
            fremoveitems.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiProcessMessage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
>(
    hinstall: Param0,
    emessagetype: INSTALLMESSAGE,
    hrecord: Param2,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiProcessMessage(
                hinstall: MSIHANDLE,
                emessagetype: INSTALLMESSAGE,
                hrecord: MSIHANDLE,
            ) -> i32;
        }
        ::std::mem::transmute(MsiProcessMessage(
            hinstall.into_param().abi(),
            ::std::mem::transmute(emessagetype),
            hrecord.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiProvideAssemblyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szassemblyname: Param0,
    szappcontext: Param1,
    dwinstallmode: INSTALLMODE,
    dwassemblyinfo: MSIASSEMBLYINFO,
    lppathbuf: super::super::Foundation::PSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiProvideAssemblyA(
                szassemblyname: super::super::Foundation::PSTR,
                szappcontext: super::super::Foundation::PSTR,
                dwinstallmode: INSTALLMODE,
                dwassemblyinfo: MSIASSEMBLYINFO,
                lppathbuf: super::super::Foundation::PSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiProvideAssemblyA(
            szassemblyname.into_param().abi(),
            szappcontext.into_param().abi(),
            ::std::mem::transmute(dwinstallmode),
            ::std::mem::transmute(dwassemblyinfo),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiProvideAssemblyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szassemblyname: Param0,
    szappcontext: Param1,
    dwinstallmode: INSTALLMODE,
    dwassemblyinfo: MSIASSEMBLYINFO,
    lppathbuf: super::super::Foundation::PWSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiProvideAssemblyW(
                szassemblyname: super::super::Foundation::PWSTR,
                szappcontext: super::super::Foundation::PWSTR,
                dwinstallmode: INSTALLMODE,
                dwassemblyinfo: MSIASSEMBLYINFO,
                lppathbuf: super::super::Foundation::PWSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiProvideAssemblyW(
            szassemblyname.into_param().abi(),
            szappcontext.into_param().abi(),
            ::std::mem::transmute(dwinstallmode),
            ::std::mem::transmute(dwassemblyinfo),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiProvideComponentA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
    szcomponent: Param2,
    dwinstallmode: INSTALLMODE,
    lppathbuf: super::super::Foundation::PSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiProvideComponentA(
                szproduct: super::super::Foundation::PSTR,
                szfeature: super::super::Foundation::PSTR,
                szcomponent: super::super::Foundation::PSTR,
                dwinstallmode: INSTALLMODE,
                lppathbuf: super::super::Foundation::PSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiProvideComponentA(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(dwinstallmode),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiProvideComponentW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
    szcomponent: Param2,
    dwinstallmode: INSTALLMODE,
    lppathbuf: super::super::Foundation::PWSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiProvideComponentW(
                szproduct: super::super::Foundation::PWSTR,
                szfeature: super::super::Foundation::PWSTR,
                szcomponent: super::super::Foundation::PWSTR,
                dwinstallmode: INSTALLMODE,
                lppathbuf: super::super::Foundation::PWSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiProvideComponentW(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(dwinstallmode),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szcategory: Param0,
    szqualifier: Param1,
    dwinstallmode: INSTALLMODE,
    lppathbuf: super::super::Foundation::PSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiProvideQualifiedComponentA(
                szcategory: super::super::Foundation::PSTR,
                szqualifier: super::super::Foundation::PSTR,
                dwinstallmode: INSTALLMODE,
                lppathbuf: super::super::Foundation::PSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiProvideQualifiedComponentA(
            szcategory.into_param().abi(),
            szqualifier.into_param().abi(),
            ::std::mem::transmute(dwinstallmode),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szcategory: Param0,
    szqualifier: Param1,
    dwinstallmode: INSTALLMODE,
    szproduct: Param3,
    dwunused1: u32,
    dwunused2: u32,
    lppathbuf: super::super::Foundation::PSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiProvideQualifiedComponentExA(
                szcategory: super::super::Foundation::PSTR,
                szqualifier: super::super::Foundation::PSTR,
                dwinstallmode: INSTALLMODE,
                szproduct: super::super::Foundation::PSTR,
                dwunused1: u32,
                dwunused2: u32,
                lppathbuf: super::super::Foundation::PSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiProvideQualifiedComponentExA(
            szcategory.into_param().abi(),
            szqualifier.into_param().abi(),
            ::std::mem::transmute(dwinstallmode),
            szproduct.into_param().abi(),
            ::std::mem::transmute(dwunused1),
            ::std::mem::transmute(dwunused2),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szcategory: Param0,
    szqualifier: Param1,
    dwinstallmode: INSTALLMODE,
    szproduct: Param3,
    dwunused1: u32,
    dwunused2: u32,
    lppathbuf: super::super::Foundation::PWSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiProvideQualifiedComponentExW(
                szcategory: super::super::Foundation::PWSTR,
                szqualifier: super::super::Foundation::PWSTR,
                dwinstallmode: INSTALLMODE,
                szproduct: super::super::Foundation::PWSTR,
                dwunused1: u32,
                dwunused2: u32,
                lppathbuf: super::super::Foundation::PWSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiProvideQualifiedComponentExW(
            szcategory.into_param().abi(),
            szqualifier.into_param().abi(),
            ::std::mem::transmute(dwinstallmode),
            szproduct.into_param().abi(),
            ::std::mem::transmute(dwunused1),
            ::std::mem::transmute(dwunused2),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szcategory: Param0,
    szqualifier: Param1,
    dwinstallmode: INSTALLMODE,
    lppathbuf: super::super::Foundation::PWSTR,
    pcchpathbuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiProvideQualifiedComponentW(
                szcategory: super::super::Foundation::PWSTR,
                szqualifier: super::super::Foundation::PWSTR,
                dwinstallmode: INSTALLMODE,
                lppathbuf: super::super::Foundation::PWSTR,
                pcchpathbuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiProvideQualifiedComponentW(
            szcategory.into_param().abi(),
            szqualifier.into_param().abi(),
            ::std::mem::transmute(dwinstallmode),
            ::std::mem::transmute(lppathbuf),
            ::std::mem::transmute(pcchpathbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiQueryComponentStateA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    szcomponentcode: Param3,
    pdwstate: *mut INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiQueryComponentStateA(
                szproductcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                szcomponentcode: super::super::Foundation::PSTR,
                pdwstate: *mut INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiQueryComponentStateA(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            szcomponentcode.into_param().abi(),
            ::std::mem::transmute(pdwstate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiQueryComponentStateW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    szcomponentcode: Param3,
    pdwstate: *mut INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiQueryComponentStateW(
                szproductcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                szcomponentcode: super::super::Foundation::PWSTR,
                pdwstate: *mut INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiQueryComponentStateW(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            szcomponentcode.into_param().abi(),
            ::std::mem::transmute(pdwstate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiQueryFeatureStateA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiQueryFeatureStateA(
                szproduct: super::super::Foundation::PSTR,
                szfeature: super::super::Foundation::PSTR,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiQueryFeatureStateA(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiQueryFeatureStateExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    szfeature: Param3,
    pdwstate: *mut INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiQueryFeatureStateExA(
                szproductcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                szfeature: super::super::Foundation::PSTR,
                pdwstate: *mut INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiQueryFeatureStateExA(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            szfeature.into_param().abi(),
            ::std::mem::transmute(pdwstate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiQueryFeatureStateExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    szfeature: Param3,
    pdwstate: *mut INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiQueryFeatureStateExW(
                szproductcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                szfeature: super::super::Foundation::PWSTR,
                pdwstate: *mut INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiQueryFeatureStateExW(
            szproductcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            szfeature.into_param().abi(),
            ::std::mem::transmute(pdwstate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiQueryFeatureStateW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiQueryFeatureStateW(
                szproduct: super::super::Foundation::PWSTR,
                szfeature: super::super::Foundation::PWSTR,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiQueryFeatureStateW(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiQueryProductStateA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiQueryProductStateA(szproduct: super::super::Foundation::PSTR) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiQueryProductStateA(szproduct.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiQueryProductStateW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiQueryProductStateW(szproduct: super::super::Foundation::PWSTR) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiQueryProductStateW(szproduct.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiRecordClearData<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hrecord: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordClearData(hrecord: MSIHANDLE) -> u32;
        }
        ::std::mem::transmute(MsiRecordClearData(hrecord.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiRecordDataSize<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hrecord: Param0,
    ifield: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordDataSize(hrecord: MSIHANDLE, ifield: u32) -> u32;
        }
        ::std::mem::transmute(MsiRecordDataSize(
            hrecord.into_param().abi(),
            ::std::mem::transmute(ifield),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiRecordGetFieldCount<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hrecord: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordGetFieldCount(hrecord: MSIHANDLE) -> u32;
        }
        ::std::mem::transmute(MsiRecordGetFieldCount(hrecord.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiRecordGetInteger<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hrecord: Param0,
    ifield: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordGetInteger(hrecord: MSIHANDLE, ifield: u32) -> i32;
        }
        ::std::mem::transmute(MsiRecordGetInteger(
            hrecord.into_param().abi(),
            ::std::mem::transmute(ifield),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRecordGetStringA<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hrecord: Param0,
    ifield: u32,
    szvaluebuf: super::super::Foundation::PSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordGetStringA(
                hrecord: MSIHANDLE,
                ifield: u32,
                szvaluebuf: super::super::Foundation::PSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiRecordGetStringA(
            hrecord.into_param().abi(),
            ::std::mem::transmute(ifield),
            ::std::mem::transmute(szvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRecordGetStringW<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hrecord: Param0,
    ifield: u32,
    szvaluebuf: super::super::Foundation::PWSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordGetStringW(
                hrecord: MSIHANDLE,
                ifield: u32,
                szvaluebuf: super::super::Foundation::PWSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiRecordGetStringW(
            hrecord.into_param().abi(),
            ::std::mem::transmute(ifield),
            ::std::mem::transmute(szvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRecordIsNull<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hrecord: Param0,
    ifield: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordIsNull(hrecord: MSIHANDLE, ifield: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MsiRecordIsNull(
            hrecord.into_param().abi(),
            ::std::mem::transmute(ifield),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRecordReadStream<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hrecord: Param0,
    ifield: u32,
    szdatabuf: super::super::Foundation::PSTR,
    pcbdatabuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordReadStream(
                hrecord: MSIHANDLE,
                ifield: u32,
                szdatabuf: super::super::Foundation::PSTR,
                pcbdatabuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiRecordReadStream(
            hrecord.into_param().abi(),
            ::std::mem::transmute(ifield),
            ::std::mem::transmute(szdatabuf),
            ::std::mem::transmute(pcbdatabuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiRecordSetInteger<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hrecord: Param0,
    ifield: u32,
    ivalue: i32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordSetInteger(hrecord: MSIHANDLE, ifield: u32, ivalue: i32) -> u32;
        }
        ::std::mem::transmute(MsiRecordSetInteger(
            hrecord.into_param().abi(),
            ::std::mem::transmute(ifield),
            ::std::mem::transmute(ivalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRecordSetStreamA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hrecord: Param0,
    ifield: u32,
    szfilepath: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordSetStreamA(
                hrecord: MSIHANDLE,
                ifield: u32,
                szfilepath: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiRecordSetStreamA(
            hrecord.into_param().abi(),
            ::std::mem::transmute(ifield),
            szfilepath.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRecordSetStreamW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hrecord: Param0,
    ifield: u32,
    szfilepath: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordSetStreamW(
                hrecord: MSIHANDLE,
                ifield: u32,
                szfilepath: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiRecordSetStreamW(
            hrecord.into_param().abi(),
            ::std::mem::transmute(ifield),
            szfilepath.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRecordSetStringA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hrecord: Param0,
    ifield: u32,
    szvalue: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordSetStringA(
                hrecord: MSIHANDLE,
                ifield: u32,
                szvalue: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiRecordSetStringA(
            hrecord.into_param().abi(),
            ::std::mem::transmute(ifield),
            szvalue.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRecordSetStringW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hrecord: Param0,
    ifield: u32,
    szvalue: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRecordSetStringW(
                hrecord: MSIHANDLE,
                ifield: u32,
                szvalue: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiRecordSetStringW(
            hrecord.into_param().abi(),
            ::std::mem::transmute(ifield),
            szvalue.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiReinstallFeatureA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
    dwreinstallmode: REINSTALLMODE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiReinstallFeatureA(
                szproduct: super::super::Foundation::PSTR,
                szfeature: super::super::Foundation::PSTR,
                dwreinstallmode: REINSTALLMODE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiReinstallFeatureA(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(dwreinstallmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiReinstallFeatureW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
    dwreinstallmode: REINSTALLMODE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiReinstallFeatureW(
                szproduct: super::super::Foundation::PWSTR,
                szfeature: super::super::Foundation::PWSTR,
                dwreinstallmode: REINSTALLMODE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiReinstallFeatureW(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(dwreinstallmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiReinstallProductA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szreinstallmode: REINSTALLMODE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiReinstallProductA(
                szproduct: super::super::Foundation::PSTR,
                szreinstallmode: REINSTALLMODE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiReinstallProductA(
            szproduct.into_param().abi(),
            ::std::mem::transmute(szreinstallmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiReinstallProductW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szreinstallmode: REINSTALLMODE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiReinstallProductW(
                szproduct: super::super::Foundation::PWSTR,
                szreinstallmode: REINSTALLMODE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiReinstallProductW(
            szproduct.into_param().abi(),
            ::std::mem::transmute(szreinstallmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRemovePatchesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpatchlist: Param0,
    szproductcode: Param1,
    euninstalltype: INSTALLTYPE,
    szpropertylist: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRemovePatchesA(
                szpatchlist: super::super::Foundation::PSTR,
                szproductcode: super::super::Foundation::PSTR,
                euninstalltype: INSTALLTYPE,
                szpropertylist: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiRemovePatchesA(
            szpatchlist.into_param().abi(),
            szproductcode.into_param().abi(),
            ::std::mem::transmute(euninstalltype),
            szpropertylist.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRemovePatchesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpatchlist: Param0,
    szproductcode: Param1,
    euninstalltype: INSTALLTYPE,
    szpropertylist: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiRemovePatchesW(
                szpatchlist: super::super::Foundation::PWSTR,
                szproductcode: super::super::Foundation::PWSTR,
                euninstalltype: INSTALLTYPE,
                szpropertylist: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiRemovePatchesW(
            szpatchlist.into_param().abi(),
            szproductcode.into_param().abi(),
            ::std::mem::transmute(euninstalltype),
            szpropertylist.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSequenceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    sztable: Param1,
    isequencemode: i32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSequenceA(
                hinstall: MSIHANDLE,
                sztable: super::super::Foundation::PSTR,
                isequencemode: i32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSequenceA(
            hinstall.into_param().abi(),
            sztable.into_param().abi(),
            ::std::mem::transmute(isequencemode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSequenceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    sztable: Param1,
    isequencemode: i32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSequenceW(
                hinstall: MSIHANDLE,
                sztable: super::super::Foundation::PWSTR,
                isequencemode: i32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSequenceW(
            hinstall.into_param().abi(),
            sztable.into_param().abi(),
            ::std::mem::transmute(isequencemode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetComponentStateA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szcomponent: Param1,
    istate: INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetComponentStateA(
                hinstall: MSIHANDLE,
                szcomponent: super::super::Foundation::PSTR,
                istate: INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetComponentStateA(
            hinstall.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(istate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetComponentStateW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szcomponent: Param1,
    istate: INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetComponentStateW(
                hinstall: MSIHANDLE,
                szcomponent: super::super::Foundation::PWSTR,
                istate: INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetComponentStateW(
            hinstall.into_param().abi(),
            szcomponent.into_param().abi(),
            ::std::mem::transmute(istate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetExternalUIA(
    puihandler: ::std::option::Option<INSTALLUI_HANDLERA>,
    dwmessagefilter: u32,
    pvcontext: *const ::std::ffi::c_void,
) -> ::std::option::Option<INSTALLUI_HANDLERA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetExternalUIA(
                puihandler: ::windows::runtime::RawPtr,
                dwmessagefilter: u32,
                pvcontext: *const ::std::ffi::c_void,
            ) -> ::std::option::Option<INSTALLUI_HANDLERA>;
        }
        ::std::mem::transmute(MsiSetExternalUIA(
            ::std::mem::transmute(puihandler),
            ::std::mem::transmute(dwmessagefilter),
            ::std::mem::transmute(pvcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiSetExternalUIRecord(
    puihandler: ::std::option::Option<PINSTALLUI_HANDLER_RECORD>,
    dwmessagefilter: u32,
    pvcontext: *const ::std::ffi::c_void,
    ppuiprevhandler: ::std::option::Option<PINSTALLUI_HANDLER_RECORD>,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetExternalUIRecord(
                puihandler: ::windows::runtime::RawPtr,
                dwmessagefilter: u32,
                pvcontext: *const ::std::ffi::c_void,
                ppuiprevhandler: ::windows::runtime::RawPtr,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetExternalUIRecord(
            ::std::mem::transmute(puihandler),
            ::std::mem::transmute(dwmessagefilter),
            ::std::mem::transmute(pvcontext),
            ::std::mem::transmute(ppuiprevhandler),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetExternalUIW(
    puihandler: ::std::option::Option<INSTALLUI_HANDLERW>,
    dwmessagefilter: u32,
    pvcontext: *const ::std::ffi::c_void,
) -> ::std::option::Option<INSTALLUI_HANDLERW> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetExternalUIW(
                puihandler: ::windows::runtime::RawPtr,
                dwmessagefilter: u32,
                pvcontext: *const ::std::ffi::c_void,
            ) -> ::std::option::Option<INSTALLUI_HANDLERW>;
        }
        ::std::mem::transmute(MsiSetExternalUIW(
            ::std::mem::transmute(puihandler),
            ::std::mem::transmute(dwmessagefilter),
            ::std::mem::transmute(pvcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetFeatureAttributesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szfeature: Param1,
    dwattributes: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetFeatureAttributesA(
                hinstall: MSIHANDLE,
                szfeature: super::super::Foundation::PSTR,
                dwattributes: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetFeatureAttributesA(
            hinstall.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(dwattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetFeatureAttributesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szfeature: Param1,
    dwattributes: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetFeatureAttributesW(
                hinstall: MSIHANDLE,
                szfeature: super::super::Foundation::PWSTR,
                dwattributes: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetFeatureAttributesW(
            hinstall.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(dwattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetFeatureStateA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szfeature: Param1,
    istate: INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetFeatureStateA(
                hinstall: MSIHANDLE,
                szfeature: super::super::Foundation::PSTR,
                istate: INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetFeatureStateA(
            hinstall.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(istate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetFeatureStateW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szfeature: Param1,
    istate: INSTALLSTATE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetFeatureStateW(
                hinstall: MSIHANDLE,
                szfeature: super::super::Foundation::PWSTR,
                istate: INSTALLSTATE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetFeatureStateW(
            hinstall.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(istate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiSetInstallLevel<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hinstall: Param0,
    iinstalllevel: i32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetInstallLevel(hinstall: MSIHANDLE, iinstalllevel: i32) -> u32;
        }
        ::std::mem::transmute(MsiSetInstallLevel(
            hinstall.into_param().abi(),
            ::std::mem::transmute(iinstalllevel),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetInternalUI(
    dwuilevel: INSTALLUILEVEL,
    phwnd: *mut super::super::Foundation::HWND,
) -> INSTALLUILEVEL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetInternalUI(
                dwuilevel: INSTALLUILEVEL,
                phwnd: *mut super::super::Foundation::HWND,
            ) -> INSTALLUILEVEL;
        }
        ::std::mem::transmute(MsiSetInternalUI(
            ::std::mem::transmute(dwuilevel),
            ::std::mem::transmute(phwnd),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetMode<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hinstall: Param0,
    erunmode: MSIRUNMODE,
    fstate: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetMode(
                hinstall: MSIHANDLE,
                erunmode: MSIRUNMODE,
                fstate: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetMode(
            hinstall.into_param().abi(),
            ::std::mem::transmute(erunmode),
            fstate.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetPropertyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szname: Param1,
    szvalue: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetPropertyA(
                hinstall: MSIHANDLE,
                szname: super::super::Foundation::PSTR,
                szvalue: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetPropertyA(
            hinstall.into_param().abi(),
            szname.into_param().abi(),
            szvalue.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetPropertyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szname: Param1,
    szvalue: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetPropertyW(
                hinstall: MSIHANDLE,
                szname: super::super::Foundation::PWSTR,
                szvalue: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetPropertyW(
            hinstall.into_param().abi(),
            szname.into_param().abi(),
            szvalue.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetTargetPathA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hinstall: Param0,
    szfolder: Param1,
    szfolderpath: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetTargetPathA(
                hinstall: MSIHANDLE,
                szfolder: super::super::Foundation::PSTR,
                szfolderpath: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetTargetPathA(
            hinstall.into_param().abi(),
            szfolder.into_param().abi(),
            szfolderpath.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetTargetPathW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hinstall: Param0,
    szfolder: Param1,
    szfolderpath: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSetTargetPathW(
                hinstall: MSIHANDLE,
                szfolder: super::super::Foundation::PWSTR,
                szfolderpath: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSetTargetPathW(
            hinstall.into_param().abi(),
            szfolder.into_param().abi(),
            szfolderpath.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    dwdiskid: u32,
    szvolumelabel: Param5,
    szdiskprompt: Param6,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListAddMediaDiskA(
                szproductcodeorpatchcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                dwdiskid: u32,
                szvolumelabel: super::super::Foundation::PSTR,
                szdiskprompt: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListAddMediaDiskA(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(dwdiskid),
            szvolumelabel.into_param().abi(),
            szdiskprompt.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    dwdiskid: u32,
    szvolumelabel: Param5,
    szdiskprompt: Param6,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListAddMediaDiskW(
                szproductcodeorpatchcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                dwdiskid: u32,
                szvolumelabel: super::super::Foundation::PWSTR,
                szdiskprompt: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListAddMediaDiskW(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(dwdiskid),
            szvolumelabel.into_param().abi(),
            szdiskprompt.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListAddSourceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szusername: Param1,
    dwreserved: u32,
    szsource: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListAddSourceA(
                szproduct: super::super::Foundation::PSTR,
                szusername: super::super::Foundation::PSTR,
                dwreserved: u32,
                szsource: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListAddSourceA(
            szproduct.into_param().abi(),
            szusername.into_param().abi(),
            ::std::mem::transmute(dwreserved),
            szsource.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListAddSourceExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    szsource: Param4,
    dwindex: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListAddSourceExA(
                szproductcodeorpatchcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                szsource: super::super::Foundation::PSTR,
                dwindex: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListAddSourceExA(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            szsource.into_param().abi(),
            ::std::mem::transmute(dwindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListAddSourceExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    szsource: Param4,
    dwindex: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListAddSourceExW(
                szproductcodeorpatchcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                szsource: super::super::Foundation::PWSTR,
                dwindex: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListAddSourceExW(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            szsource.into_param().abi(),
            ::std::mem::transmute(dwindex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListAddSourceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szusername: Param1,
    dwreserved: u32,
    szsource: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListAddSourceW(
                szproduct: super::super::Foundation::PWSTR,
                szusername: super::super::Foundation::PWSTR,
                dwreserved: u32,
                szsource: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListAddSourceW(
            szproduct.into_param().abi(),
            szusername.into_param().abi(),
            ::std::mem::transmute(dwreserved),
            szsource.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListClearAllA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szusername: Param1,
    dwreserved: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListClearAllA(
                szproduct: super::super::Foundation::PSTR,
                szusername: super::super::Foundation::PSTR,
                dwreserved: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListClearAllA(
            szproduct.into_param().abi(),
            szusername.into_param().abi(),
            ::std::mem::transmute(dwreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListClearAllExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListClearAllExA(
                szproductcodeorpatchcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListClearAllExA(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListClearAllExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListClearAllExW(
                szproductcodeorpatchcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListClearAllExW(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListClearAllW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szusername: Param1,
    dwreserved: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListClearAllW(
                szproduct: super::super::Foundation::PWSTR,
                szusername: super::super::Foundation::PWSTR,
                dwreserved: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListClearAllW(
            szproduct.into_param().abi(),
            szusername.into_param().abi(),
            ::std::mem::transmute(dwreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    dwdiskid: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListClearMediaDiskA(
                szproductcodeorpatchcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                dwdiskid: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListClearMediaDiskA(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(dwdiskid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    dwdiskid: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListClearMediaDiskW(
                szproductcodeorpatchcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                dwdiskid: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListClearMediaDiskW(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(dwdiskid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListClearSourceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    szsource: Param4,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListClearSourceA(
                szproductcodeorpatchcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                szsource: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListClearSourceA(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            szsource.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListClearSourceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    szsource: Param4,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListClearSourceW(
                szproductcodeorpatchcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                szsource: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListClearSourceW(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            szsource.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    dwindex: u32,
    pdwdiskid: *mut u32,
    szvolumelabel: super::super::Foundation::PSTR,
    pcchvolumelabel: *mut u32,
    szdiskprompt: super::super::Foundation::PSTR,
    pcchdiskprompt: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListEnumMediaDisksA(
                szproductcodeorpatchcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                dwindex: u32,
                pdwdiskid: *mut u32,
                szvolumelabel: super::super::Foundation::PSTR,
                pcchvolumelabel: *mut u32,
                szdiskprompt: super::super::Foundation::PSTR,
                pcchdiskprompt: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListEnumMediaDisksA(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(pdwdiskid),
            ::std::mem::transmute(szvolumelabel),
            ::std::mem::transmute(pcchvolumelabel),
            ::std::mem::transmute(szdiskprompt),
            ::std::mem::transmute(pcchdiskprompt),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    dwindex: u32,
    pdwdiskid: *mut u32,
    szvolumelabel: super::super::Foundation::PWSTR,
    pcchvolumelabel: *mut u32,
    szdiskprompt: super::super::Foundation::PWSTR,
    pcchdiskprompt: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListEnumMediaDisksW(
                szproductcodeorpatchcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                dwindex: u32,
                pdwdiskid: *mut u32,
                szvolumelabel: super::super::Foundation::PWSTR,
                pcchvolumelabel: *mut u32,
                szdiskprompt: super::super::Foundation::PWSTR,
                pcchdiskprompt: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListEnumMediaDisksW(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(pdwdiskid),
            ::std::mem::transmute(szvolumelabel),
            ::std::mem::transmute(pcchvolumelabel),
            ::std::mem::transmute(szdiskprompt),
            ::std::mem::transmute(pcchdiskprompt),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListEnumSourcesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    dwindex: u32,
    szsource: super::super::Foundation::PSTR,
    pcchsource: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListEnumSourcesA(
                szproductcodeorpatchcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                dwindex: u32,
                szsource: super::super::Foundation::PSTR,
                pcchsource: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListEnumSourcesA(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(szsource),
            ::std::mem::transmute(pcchsource),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListEnumSourcesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    dwindex: u32,
    szsource: super::super::Foundation::PWSTR,
    pcchsource: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListEnumSourcesW(
                szproductcodeorpatchcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                dwindex: u32,
                szsource: super::super::Foundation::PWSTR,
                pcchsource: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListEnumSourcesW(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(szsource),
            ::std::mem::transmute(pcchsource),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListForceResolutionA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szusername: Param1,
    dwreserved: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListForceResolutionA(
                szproduct: super::super::Foundation::PSTR,
                szusername: super::super::Foundation::PSTR,
                dwreserved: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListForceResolutionA(
            szproduct.into_param().abi(),
            szusername.into_param().abi(),
            ::std::mem::transmute(dwreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListForceResolutionExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListForceResolutionExA(
                szproductcodeorpatchcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListForceResolutionExA(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListForceResolutionExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListForceResolutionExW(
                szproductcodeorpatchcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListForceResolutionExW(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListForceResolutionW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szusername: Param1,
    dwreserved: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListForceResolutionW(
                szproduct: super::super::Foundation::PWSTR,
                szusername: super::super::Foundation::PWSTR,
                dwreserved: u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListForceResolutionW(
            szproduct.into_param().abi(),
            szusername.into_param().abi(),
            ::std::mem::transmute(dwreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListGetInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    szproperty: Param4,
    szvalue: super::super::Foundation::PSTR,
    pcchvalue: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListGetInfoA(
                szproductcodeorpatchcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                szproperty: super::super::Foundation::PSTR,
                szvalue: super::super::Foundation::PSTR,
                pcchvalue: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListGetInfoA(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            szproperty.into_param().abi(),
            ::std::mem::transmute(szvalue),
            ::std::mem::transmute(pcchvalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListGetInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    szproperty: Param4,
    szvalue: super::super::Foundation::PWSTR,
    pcchvalue: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListGetInfoW(
                szproductcodeorpatchcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                szproperty: super::super::Foundation::PWSTR,
                szvalue: super::super::Foundation::PWSTR,
                pcchvalue: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListGetInfoW(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            szproperty.into_param().abi(),
            ::std::mem::transmute(szvalue),
            ::std::mem::transmute(pcchvalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListSetInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    szproperty: Param4,
    szvalue: Param5,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListSetInfoA(
                szproductcodeorpatchcode: super::super::Foundation::PSTR,
                szusersid: super::super::Foundation::PSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                szproperty: super::super::Foundation::PSTR,
                szvalue: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListSetInfoA(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            szproperty.into_param().abi(),
            szvalue.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSourceListSetInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproductcodeorpatchcode: Param0,
    szusersid: Param1,
    dwcontext: MSIINSTALLCONTEXT,
    dwoptions: u32,
    szproperty: Param4,
    szvalue: Param5,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSourceListSetInfoW(
                szproductcodeorpatchcode: super::super::Foundation::PWSTR,
                szusersid: super::super::Foundation::PWSTR,
                dwcontext: MSIINSTALLCONTEXT,
                dwoptions: u32,
                szproperty: super::super::Foundation::PWSTR,
                szvalue: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSourceListSetInfoW(
            szproductcodeorpatchcode.into_param().abi(),
            szusersid.into_param().abi(),
            ::std::mem::transmute(dwcontext),
            ::std::mem::transmute(dwoptions),
            szproperty.into_param().abi(),
            szvalue.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
>(
    hsummaryinfo: Param0,
    uiproperty: u32,
    puidatatype: *mut u32,
    pivalue: *mut i32,
    pftvalue: *mut super::super::Foundation::FILETIME,
    szvaluebuf: super::super::Foundation::PSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSummaryInfoGetPropertyA(
                hsummaryinfo: MSIHANDLE,
                uiproperty: u32,
                puidatatype: *mut u32,
                pivalue: *mut i32,
                pftvalue: *mut super::super::Foundation::FILETIME,
                szvaluebuf: super::super::Foundation::PSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSummaryInfoGetPropertyA(
            hsummaryinfo.into_param().abi(),
            ::std::mem::transmute(uiproperty),
            ::std::mem::transmute(puidatatype),
            ::std::mem::transmute(pivalue),
            ::std::mem::transmute(pftvalue),
            ::std::mem::transmute(szvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyCount<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
>(
    hsummaryinfo: Param0,
    puipropertycount: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSummaryInfoGetPropertyCount(
                hsummaryinfo: MSIHANDLE,
                puipropertycount: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSummaryInfoGetPropertyCount(
            hsummaryinfo.into_param().abi(),
            ::std::mem::transmute(puipropertycount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
>(
    hsummaryinfo: Param0,
    uiproperty: u32,
    puidatatype: *mut u32,
    pivalue: *mut i32,
    pftvalue: *mut super::super::Foundation::FILETIME,
    szvaluebuf: super::super::Foundation::PWSTR,
    pcchvaluebuf: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSummaryInfoGetPropertyW(
                hsummaryinfo: MSIHANDLE,
                uiproperty: u32,
                puidatatype: *mut u32,
                pivalue: *mut i32,
                pftvalue: *mut super::super::Foundation::FILETIME,
                szvaluebuf: super::super::Foundation::PWSTR,
                pcchvaluebuf: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSummaryInfoGetPropertyW(
            hsummaryinfo.into_param().abi(),
            ::std::mem::transmute(uiproperty),
            ::std::mem::transmute(puidatatype),
            ::std::mem::transmute(pivalue),
            ::std::mem::transmute(pftvalue),
            ::std::mem::transmute(szvaluebuf),
            ::std::mem::transmute(pcchvaluebuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiSummaryInfoPersist<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hsummaryinfo: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSummaryInfoPersist(hsummaryinfo: MSIHANDLE) -> u32;
        }
        ::std::mem::transmute(MsiSummaryInfoPersist(hsummaryinfo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hsummaryinfo: Param0,
    uiproperty: u32,
    uidatatype: u32,
    ivalue: i32,
    pftvalue: *mut super::super::Foundation::FILETIME,
    szvalue: Param5,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSummaryInfoSetPropertyA(
                hsummaryinfo: MSIHANDLE,
                uiproperty: u32,
                uidatatype: u32,
                ivalue: i32,
                pftvalue: *mut super::super::Foundation::FILETIME,
                szvalue: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSummaryInfoSetPropertyA(
            hsummaryinfo.into_param().abi(),
            ::std::mem::transmute(uiproperty),
            ::std::mem::transmute(uidatatype),
            ::std::mem::transmute(ivalue),
            ::std::mem::transmute(pftvalue),
            szvalue.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hsummaryinfo: Param0,
    uiproperty: u32,
    uidatatype: u32,
    ivalue: i32,
    pftvalue: *mut super::super::Foundation::FILETIME,
    szvalue: Param5,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiSummaryInfoSetPropertyW(
                hsummaryinfo: MSIHANDLE,
                uiproperty: u32,
                uidatatype: u32,
                ivalue: i32,
                pftvalue: *mut super::super::Foundation::FILETIME,
                szvalue: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(MsiSummaryInfoSetPropertyW(
            hsummaryinfo.into_param().abi(),
            ::std::mem::transmute(uiproperty),
            ::std::mem::transmute(uidatatype),
            ::std::mem::transmute(ivalue),
            ::std::mem::transmute(pftvalue),
            szvalue.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiUseFeatureA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiUseFeatureA(
                szproduct: super::super::Foundation::PSTR,
                szfeature: super::super::Foundation::PSTR,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiUseFeatureA(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiUseFeatureExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
    dwinstallmode: u32,
    dwreserved: u32,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiUseFeatureExA(
                szproduct: super::super::Foundation::PSTR,
                szfeature: super::super::Foundation::PSTR,
                dwinstallmode: u32,
                dwreserved: u32,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiUseFeatureExA(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(dwinstallmode),
            ::std::mem::transmute(dwreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiUseFeatureExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
    dwinstallmode: u32,
    dwreserved: u32,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiUseFeatureExW(
                szproduct: super::super::Foundation::PWSTR,
                szfeature: super::super::Foundation::PWSTR,
                dwinstallmode: u32,
                dwreserved: u32,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiUseFeatureExW(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
            ::std::mem::transmute(dwinstallmode),
            ::std::mem::transmute(dwreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiUseFeatureW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szproduct: Param0,
    szfeature: Param1,
) -> INSTALLSTATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiUseFeatureW(
                szproduct: super::super::Foundation::PWSTR,
                szfeature: super::super::Foundation::PWSTR,
            ) -> INSTALLSTATE;
        }
        ::std::mem::transmute(MsiUseFeatureW(
            szproduct.into_param().abi(),
            szfeature.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiVerifyDiskSpace<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hinstall: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiVerifyDiskSpace(hinstall: MSIHANDLE) -> u32;
        }
        ::std::mem::transmute(MsiVerifyDiskSpace(hinstall.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiVerifyPackageA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szpackagepath: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiVerifyPackageA(szpackagepath: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(MsiVerifyPackageA(szpackagepath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiVerifyPackageW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szpackagepath: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiVerifyPackageW(szpackagepath: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MsiVerifyPackageW(szpackagepath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiViewClose<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hview: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiViewClose(hview: MSIHANDLE) -> u32;
        }
        ::std::mem::transmute(MsiViewClose(hview.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiViewExecute<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
>(
    hview: Param0,
    hrecord: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiViewExecute(hview: MSIHANDLE, hrecord: MSIHANDLE) -> u32;
        }
        ::std::mem::transmute(MsiViewExecute(
            hview.into_param().abi(),
            hrecord.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiViewFetch<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hview: Param0,
    phrecord: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiViewFetch(hview: MSIHANDLE, phrecord: *mut MSIHANDLE) -> u32;
        }
        ::std::mem::transmute(MsiViewFetch(
            hview.into_param().abi(),
            ::std::mem::transmute(phrecord),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiViewGetColumnInfo<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hview: Param0,
    ecolumninfo: MSICOLINFO,
    phrecord: *mut MSIHANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiViewGetColumnInfo(
                hview: MSIHANDLE,
                ecolumninfo: MSICOLINFO,
                phrecord: *mut MSIHANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(MsiViewGetColumnInfo(
            hview.into_param().abi(),
            ::std::mem::transmute(ecolumninfo),
            ::std::mem::transmute(phrecord),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiViewGetErrorA<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hview: Param0,
    szcolumnnamebuffer: super::super::Foundation::PSTR,
    pcchbuf: *mut u32,
) -> MSIDBERROR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiViewGetErrorA(
                hview: MSIHANDLE,
                szcolumnnamebuffer: super::super::Foundation::PSTR,
                pcchbuf: *mut u32,
            ) -> MSIDBERROR;
        }
        ::std::mem::transmute(MsiViewGetErrorA(
            hview.into_param().abi(),
            ::std::mem::transmute(szcolumnnamebuffer),
            ::std::mem::transmute(pcchbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiViewGetErrorW<'a, Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>>(
    hview: Param0,
    szcolumnnamebuffer: super::super::Foundation::PWSTR,
    pcchbuf: *mut u32,
) -> MSIDBERROR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiViewGetErrorW(
                hview: MSIHANDLE,
                szcolumnnamebuffer: super::super::Foundation::PWSTR,
                pcchbuf: *mut u32,
            ) -> MSIDBERROR;
        }
        ::std::mem::transmute(MsiViewGetErrorW(
            hview.into_param().abi(),
            ::std::mem::transmute(szcolumnnamebuffer),
            ::std::mem::transmute(pcchbuf),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MsiViewModify<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, MSIHANDLE>,
>(
    hview: Param0,
    emodifymode: MSIMODIFY,
    hrecord: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MsiViewModify(hview: MSIHANDLE, emodifymode: MSIMODIFY, hrecord: MSIHANDLE) -> u32;
        }
        ::std::mem::transmute(MsiViewModify(
            hview.into_param().abi(),
            ::std::mem::transmute(emodifymode),
            hrecord.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MsmMerge: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    182298672,
    11302,
    4562,
    [173, 101, 0, 160, 201, 175, 17, 166],
);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NormalizeFileForPatchSignature(
    filebuffer: *mut ::std::ffi::c_void,
    filesize: u32,
    optionflags: u32,
    optiondata: *const PATCH_OPTION_DATA,
    newfilecoffbase: u32,
    newfilecofftime: u32,
    ignorerangecount: u32,
    ignorerangearray: *const PATCH_IGNORE_RANGE,
    retainrangecount: u32,
    retainrangearray: *const PATCH_RETAIN_RANGE,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NormalizeFileForPatchSignature(
                filebuffer: *mut ::std::ffi::c_void,
                filesize: u32,
                optionflags: u32,
                optiondata: *const ::std::mem::ManuallyDrop<PATCH_OPTION_DATA>,
                newfilecoffbase: u32,
                newfilecofftime: u32,
                ignorerangecount: u32,
                ignorerangearray: *const PATCH_IGNORE_RANGE,
                retainrangecount: u32,
                retainrangearray: *const PATCH_RETAIN_RANGE,
            ) -> i32;
        }
        ::std::mem::transmute(NormalizeFileForPatchSignature(
            ::std::mem::transmute(filebuffer),
            ::std::mem::transmute(filesize),
            ::std::mem::transmute(optionflags),
            ::std::mem::transmute(optiondata),
            ::std::mem::transmute(newfilecoffbase),
            ::std::mem::transmute(newfilecofftime),
            ::std::mem::transmute(ignorerangecount),
            ::std::mem::transmute(ignorerangearray),
            ::std::mem::transmute(retainrangecount),
            ::std::mem::transmute(retainrangearray),
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
pub struct PACKMAN_RUNTIME(pub i32);
pub const PACKMAN_RUNTIME_NATIVE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(1i32);
pub const PACKMAN_RUNTIME_SILVERLIGHTMOBILE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(2i32);
pub const PACKMAN_RUNTIME_XNA: PACKMAN_RUNTIME = PACKMAN_RUNTIME(3i32);
pub const PACKMAN_RUNTIME_MODERN_NATIVE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(4i32);
pub const PACKMAN_RUNTIME_JUPITER: PACKMAN_RUNTIME = PACKMAN_RUNTIME(5i32);
pub const PACKMAN_RUNTIME_INVALID: PACKMAN_RUNTIME = PACKMAN_RUNTIME(6i32);
impl ::std::convert::From<i32> for PACKMAN_RUNTIME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PACKMAN_RUNTIME {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PATCH_IGNORE_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
}
impl PATCH_IGNORE_RANGE {}
impl ::std::default::Default for PATCH_IGNORE_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PATCH_IGNORE_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PATCH_IGNORE_RANGE")
            .field("OffsetInOldFile", &self.OffsetInOldFile)
            .field("LengthInBytes", &self.LengthInBytes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PATCH_IGNORE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.OffsetInOldFile == other.OffsetInOldFile && self.LengthInBytes == other.LengthInBytes
    }
}
impl ::std::cmp::Eq for PATCH_IGNORE_RANGE {}
unsafe impl ::windows::runtime::Abi for PATCH_IGNORE_RANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PATCH_INTERLEAVE_MAP {
    pub CountRanges: u32,
    pub Range: [PATCH_INTERLEAVE_MAP_0; 1],
}
impl PATCH_INTERLEAVE_MAP {}
impl ::std::default::Default for PATCH_INTERLEAVE_MAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PATCH_INTERLEAVE_MAP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PATCH_INTERLEAVE_MAP")
            .field("CountRanges", &self.CountRanges)
            .field("Range", &self.Range)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PATCH_INTERLEAVE_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.CountRanges == other.CountRanges && self.Range == other.Range
    }
}
impl ::std::cmp::Eq for PATCH_INTERLEAVE_MAP {}
unsafe impl ::windows::runtime::Abi for PATCH_INTERLEAVE_MAP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PATCH_INTERLEAVE_MAP_0 {
    pub OldOffset: u32,
    pub OldLength: u32,
    pub NewLength: u32,
}
impl PATCH_INTERLEAVE_MAP_0 {}
impl ::std::default::Default for PATCH_INTERLEAVE_MAP_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PATCH_INTERLEAVE_MAP_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("OldOffset", &self.OldOffset)
            .field("OldLength", &self.OldLength)
            .field("NewLength", &self.NewLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PATCH_INTERLEAVE_MAP_0 {
    fn eq(&self, other: &Self) -> bool {
        self.OldOffset == other.OldOffset
            && self.OldLength == other.OldLength
            && self.NewLength == other.NewLength
    }
}
impl ::std::cmp::Eq for PATCH_INTERLEAVE_MAP_0 {}
unsafe impl ::windows::runtime::Abi for PATCH_INTERLEAVE_MAP_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl PATCH_OLD_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PATCH_OLD_FILE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PATCH_OLD_FILE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PATCH_OLD_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PATCH_OLD_FILE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PATCH_OLD_FILE_INFO_0 {
    pub OldFileNameA: super::super::Foundation::PSTR,
    pub OldFileNameW: super::super::Foundation::PWSTR,
    pub OldFileHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl PATCH_OLD_FILE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PATCH_OLD_FILE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PATCH_OLD_FILE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PATCH_OLD_FILE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PATCH_OLD_FILE_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO_A {
    pub SizeOfThisStruct: u32,
    pub OldFileName: super::super::Foundation::PSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl PATCH_OLD_FILE_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PATCH_OLD_FILE_INFO_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PATCH_OLD_FILE_INFO_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PATCH_OLD_FILE_INFO_A")
            .field("SizeOfThisStruct", &self.SizeOfThisStruct)
            .field("OldFileName", &self.OldFileName)
            .field("IgnoreRangeCount", &self.IgnoreRangeCount)
            .field("IgnoreRangeArray", &self.IgnoreRangeArray)
            .field("RetainRangeCount", &self.RetainRangeCount)
            .field("RetainRangeArray", &self.RetainRangeArray)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PATCH_OLD_FILE_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfThisStruct == other.SizeOfThisStruct
            && self.OldFileName == other.OldFileName
            && self.IgnoreRangeCount == other.IgnoreRangeCount
            && self.IgnoreRangeArray == other.IgnoreRangeArray
            && self.RetainRangeCount == other.RetainRangeCount
            && self.RetainRangeArray == other.RetainRangeArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PATCH_OLD_FILE_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PATCH_OLD_FILE_INFO_A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl PATCH_OLD_FILE_INFO_H {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PATCH_OLD_FILE_INFO_H {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PATCH_OLD_FILE_INFO_H {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PATCH_OLD_FILE_INFO_H")
            .field("SizeOfThisStruct", &self.SizeOfThisStruct)
            .field("OldFileHandle", &self.OldFileHandle)
            .field("IgnoreRangeCount", &self.IgnoreRangeCount)
            .field("IgnoreRangeArray", &self.IgnoreRangeArray)
            .field("RetainRangeCount", &self.RetainRangeCount)
            .field("RetainRangeArray", &self.RetainRangeArray)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PATCH_OLD_FILE_INFO_H {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfThisStruct == other.SizeOfThisStruct
            && self.OldFileHandle == other.OldFileHandle
            && self.IgnoreRangeCount == other.IgnoreRangeCount
            && self.IgnoreRangeArray == other.IgnoreRangeArray
            && self.RetainRangeCount == other.RetainRangeCount
            && self.RetainRangeArray == other.RetainRangeArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PATCH_OLD_FILE_INFO_H {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PATCH_OLD_FILE_INFO_H {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO_W {
    pub SizeOfThisStruct: u32,
    pub OldFileName: super::super::Foundation::PWSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl PATCH_OLD_FILE_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PATCH_OLD_FILE_INFO_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PATCH_OLD_FILE_INFO_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PATCH_OLD_FILE_INFO_W")
            .field("SizeOfThisStruct", &self.SizeOfThisStruct)
            .field("OldFileName", &self.OldFileName)
            .field("IgnoreRangeCount", &self.IgnoreRangeCount)
            .field("IgnoreRangeArray", &self.IgnoreRangeArray)
            .field("RetainRangeCount", &self.RetainRangeCount)
            .field("RetainRangeArray", &self.RetainRangeArray)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PATCH_OLD_FILE_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfThisStruct == other.SizeOfThisStruct
            && self.OldFileName == other.OldFileName
            && self.IgnoreRangeCount == other.IgnoreRangeCount
            && self.IgnoreRangeArray == other.IgnoreRangeArray
            && self.RetainRangeCount == other.RetainRangeCount
            && self.RetainRangeArray == other.RetainRangeArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PATCH_OLD_FILE_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PATCH_OLD_FILE_INFO_W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OPTION_DATA {
    pub SizeOfThisStruct: u32,
    pub SymbolOptionFlags: u32,
    pub NewFileSymbolPath: super::super::Foundation::PSTR,
    pub OldFileSymbolPathArray: *mut super::super::Foundation::PSTR,
    pub ExtendedOptionFlags: u32,
    pub SymLoadCallback: ::std::option::Option<PPATCH_SYMLOAD_CALLBACK>,
    pub SymLoadContext: *mut ::std::ffi::c_void,
    pub InterleaveMapArray: *mut *mut PATCH_INTERLEAVE_MAP,
    pub MaxLzxWindowSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PATCH_OPTION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PATCH_OPTION_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PATCH_OPTION_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PATCH_OPTION_DATA")
            .field("SizeOfThisStruct", &self.SizeOfThisStruct)
            .field("SymbolOptionFlags", &self.SymbolOptionFlags)
            .field("NewFileSymbolPath", &self.NewFileSymbolPath)
            .field("OldFileSymbolPathArray", &self.OldFileSymbolPathArray)
            .field("ExtendedOptionFlags", &self.ExtendedOptionFlags)
            .field("SymLoadContext", &self.SymLoadContext)
            .field("InterleaveMapArray", &self.InterleaveMapArray)
            .field("MaxLzxWindowSize", &self.MaxLzxWindowSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PATCH_OPTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfThisStruct == other.SizeOfThisStruct
            && self.SymbolOptionFlags == other.SymbolOptionFlags
            && self.NewFileSymbolPath == other.NewFileSymbolPath
            && self.OldFileSymbolPathArray == other.OldFileSymbolPathArray
            && self.ExtendedOptionFlags == other.ExtendedOptionFlags
            && self.SymLoadCallback.map(|f| f as usize) == other.SymLoadCallback.map(|f| f as usize)
            && self.SymLoadContext == other.SymLoadContext
            && self.InterleaveMapArray == other.InterleaveMapArray
            && self.MaxLzxWindowSize == other.MaxLzxWindowSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PATCH_OPTION_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PATCH_OPTION_DATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const PATCH_OPTION_FAIL_IF_BIGGER: u32 = 1048576u32;
pub const PATCH_OPTION_FAIL_IF_SAME_FILE: u32 = 524288u32;
pub const PATCH_OPTION_INTERLEAVE_FILES: u32 = 1073741824u32;
pub const PATCH_OPTION_NO_BINDFIX: u32 = 65536u32;
pub const PATCH_OPTION_NO_CHECKSUM: u32 = 2097152u32;
pub const PATCH_OPTION_NO_LOCKFIX: u32 = 131072u32;
pub const PATCH_OPTION_NO_REBASE: u32 = 262144u32;
pub const PATCH_OPTION_NO_RESTIMEFIX: u32 = 4194304u32;
pub const PATCH_OPTION_NO_TIMESTAMP: u32 = 8388608u32;
pub const PATCH_OPTION_RESERVED1: u32 = 2147483648u32;
pub const PATCH_OPTION_SIGNATURE_MD5: u32 = 16777216u32;
pub const PATCH_OPTION_USE_BEST: u32 = 0u32;
pub const PATCH_OPTION_USE_LZX_A: u32 = 1u32;
pub const PATCH_OPTION_USE_LZX_B: u32 = 2u32;
pub const PATCH_OPTION_USE_LZX_BEST: u32 = 3u32;
pub const PATCH_OPTION_USE_LZX_LARGE: u32 = 4u32;
pub const PATCH_OPTION_VALID_FLAGS: u32 = 3237937159u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PATCH_RETAIN_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
    pub OffsetInNewFile: u32,
}
impl PATCH_RETAIN_RANGE {}
impl ::std::default::Default for PATCH_RETAIN_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PATCH_RETAIN_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PATCH_RETAIN_RANGE")
            .field("OffsetInOldFile", &self.OffsetInOldFile)
            .field("LengthInBytes", &self.LengthInBytes)
            .field("OffsetInNewFile", &self.OffsetInNewFile)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PATCH_RETAIN_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.OffsetInOldFile == other.OffsetInOldFile
            && self.LengthInBytes == other.LengthInBytes
            && self.OffsetInNewFile == other.OffsetInNewFile
    }
}
impl ::std::cmp::Eq for PATCH_RETAIN_RANGE {}
unsafe impl ::windows::runtime::Abi for PATCH_RETAIN_RANGE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PATCH_SYMBOL_NO_FAILURES: u32 = 2u32;
pub const PATCH_SYMBOL_NO_IMAGEHLP: u32 = 1u32;
pub const PATCH_SYMBOL_RESERVED1: u32 = 2147483648u32;
pub const PATCH_SYMBOL_UNDECORATED_TOO: u32 = 4u32;
pub const PATCH_TRANSFORM_PE_IRELOC_2: u32 = 512u32;
pub const PATCH_TRANSFORM_PE_RESOURCE_2: u32 = 256u32;
pub const PID_APPNAME: u32 = 18u32;
pub const PID_AUTHOR: u32 = 4u32;
pub const PID_CHARCOUNT: u32 = 16u32;
pub const PID_COMMENTS: u32 = 6u32;
pub const PID_CREATE_DTM: u32 = 12u32;
pub const PID_EDITTIME: u32 = 10u32;
pub const PID_KEYWORDS: u32 = 5u32;
pub const PID_LASTAUTHOR: u32 = 8u32;
pub const PID_LASTPRINTED: u32 = 11u32;
pub const PID_LASTSAVE_DTM: u32 = 13u32;
pub const PID_MSIRESTRICT: u32 = 16u32;
pub const PID_MSISOURCE: u32 = 15u32;
pub const PID_MSIVERSION: u32 = 14u32;
pub const PID_PAGECOUNT: u32 = 14u32;
pub const PID_REVNUMBER: u32 = 9u32;
pub const PID_SUBJECT: u32 = 3u32;
pub const PID_TEMPLATE: u32 = 7u32;
pub const PID_THUMBNAIL: u32 = 17u32;
pub const PID_TITLE: u32 = 2u32;
pub const PID_WORDCOUNT: u32 = 15u32;
pub type PINSTALLUI_HANDLER_RECORD = unsafe extern "system" fn(
    pvcontext: *mut ::std::ffi::c_void,
    imessagetype: u32,
    hrecord: MSIHANDLE,
) -> i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PMSIHANDLE {
    pub m_h: MSIHANDLE,
}
impl PMSIHANDLE {}
impl ::std::default::Default for PMSIHANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PMSIHANDLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PMSIHANDLE")
            .field("m_h", &self.m_h)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PMSIHANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.m_h == other.m_h
    }
}
impl ::std::cmp::Eq for PMSIHANDLE {}
unsafe impl ::windows::runtime::Abi for PMSIHANDLE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PMSvc: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3118797308,
    58212,
    18810,
    [161, 33, 183, 179, 97, 44, 237, 206],
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
pub struct PM_ACTIVATION_POLICY(pub i32);
pub const PM_ACTIVATION_POLICY_RESUME: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(0i32);
pub const PM_ACTIVATION_POLICY_RESUMESAMEPARAMS: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(1i32);
pub const PM_ACTIVATION_POLICY_REPLACE: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(2i32);
pub const PM_ACTIVATION_POLICY_REPLACESAMEPARAMS: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(3i32);
pub const PM_ACTIVATION_POLICY_MULTISESSION: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(4i32);
pub const PM_ACTIVATION_POLICY_REPLACE_IGNOREFOREGROUND: PM_ACTIVATION_POLICY =
    PM_ACTIVATION_POLICY(5i32);
pub const PM_ACTIVATION_POLICY_UNKNOWN: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(6i32);
pub const PM_ACTIVATION_POLICY_INVALID: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(7i32);
impl ::std::convert::From<i32> for PM_ACTIVATION_POLICY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_ACTIVATION_POLICY {
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
pub struct PM_APPLICATION_HUBTYPE(pub i32);
pub const PM_APPLICATION_HUBTYPE_NONMUSIC: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(0i32);
pub const PM_APPLICATION_HUBTYPE_MUSIC: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(1i32);
pub const PM_APPLICATION_HUBTYPE_INVALID: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(2i32);
impl ::std::convert::From<i32> for PM_APPLICATION_HUBTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_APPLICATION_HUBTYPE {
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
pub struct PM_APPLICATION_INSTALL_TYPE(pub i32);
pub const PM_APPLICATION_INSTALL_NORMAL: PM_APPLICATION_INSTALL_TYPE =
    PM_APPLICATION_INSTALL_TYPE(0i32);
pub const PM_APPLICATION_INSTALL_IN_ROM: PM_APPLICATION_INSTALL_TYPE =
    PM_APPLICATION_INSTALL_TYPE(1i32);
pub const PM_APPLICATION_INSTALL_PA: PM_APPLICATION_INSTALL_TYPE =
    PM_APPLICATION_INSTALL_TYPE(2i32);
pub const PM_APPLICATION_INSTALL_DEBUG: PM_APPLICATION_INSTALL_TYPE =
    PM_APPLICATION_INSTALL_TYPE(3i32);
pub const PM_APPLICATION_INSTALL_ENTERPRISE: PM_APPLICATION_INSTALL_TYPE =
    PM_APPLICATION_INSTALL_TYPE(4i32);
pub const PM_APPLICATION_INSTALL_INVALID: PM_APPLICATION_INSTALL_TYPE =
    PM_APPLICATION_INSTALL_TYPE(5i32);
impl ::std::convert::From<i32> for PM_APPLICATION_INSTALL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_APPLICATION_INSTALL_TYPE {
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
pub struct PM_APPLICATION_STATE(pub i32);
pub const PM_APPLICATION_STATE_MIN: PM_APPLICATION_STATE = PM_APPLICATION_STATE(0i32);
pub const PM_APPLICATION_STATE_INSTALLED: PM_APPLICATION_STATE = PM_APPLICATION_STATE(1i32);
pub const PM_APPLICATION_STATE_INSTALLING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(2i32);
pub const PM_APPLICATION_STATE_UPDATING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(3i32);
pub const PM_APPLICATION_STATE_UNINSTALLING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(4i32);
pub const PM_APPLICATION_STATE_LICENSE_UPDATING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(5i32);
pub const PM_APPLICATION_STATE_MOVING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(6i32);
pub const PM_APPLICATION_STATE_DISABLED_SD_CARD: PM_APPLICATION_STATE = PM_APPLICATION_STATE(7i32);
pub const PM_APPLICATION_STATE_DISABLED_ENTERPRISE: PM_APPLICATION_STATE =
    PM_APPLICATION_STATE(8i32);
pub const PM_APPLICATION_STATE_DISABLED_BACKING_UP: PM_APPLICATION_STATE =
    PM_APPLICATION_STATE(9i32);
pub const PM_APPLICATION_STATE_DISABLED_MDIL_BINDING: PM_APPLICATION_STATE =
    PM_APPLICATION_STATE(10i32);
pub const PM_APPLICATION_STATE_MAX: PM_APPLICATION_STATE = PM_APPLICATION_STATE(10i32);
pub const PM_APPLICATION_STATE_INVALID: PM_APPLICATION_STATE = PM_APPLICATION_STATE(11i32);
impl ::std::convert::From<i32> for PM_APPLICATION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_APPLICATION_STATE {
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
pub struct PM_APP_GENRE(pub i32);
pub const PM_APP_GENRE_GAMES: PM_APP_GENRE = PM_APP_GENRE(0i32);
pub const PM_APP_GENRE_OTHER: PM_APP_GENRE = PM_APP_GENRE(1i32);
pub const PM_APP_GENRE_INVALID: PM_APP_GENRE = PM_APP_GENRE(2i32);
impl ::std::convert::From<i32> for PM_APP_GENRE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_APP_GENRE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_BSATASKID {
    pub ProductID: ::windows::runtime::GUID,
    pub TaskID: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PM_BSATASKID {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PM_BSATASKID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PM_BSATASKID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PM_BSATASKID")
            .field("ProductID", &self.ProductID)
            .field("TaskID", &self.TaskID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PM_BSATASKID {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.TaskID == other.TaskID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PM_BSATASKID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PM_BSATASKID {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_BWTASKID {
    pub ProductID: ::windows::runtime::GUID,
    pub TaskID: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PM_BWTASKID {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PM_BWTASKID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PM_BWTASKID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PM_BWTASKID")
            .field("ProductID", &self.ProductID)
            .field("TaskID", &self.TaskID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PM_BWTASKID {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.TaskID == other.TaskID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PM_BWTASKID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PM_BWTASKID {
    type Abi = ::std::mem::ManuallyDrop<Self>;
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
pub struct PM_ENUM_APP_FILTER(pub i32);
pub const PM_APP_FILTER_ALL: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(0i32);
pub const PM_APP_FILTER_VISIBLE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(1i32);
pub const PM_APP_FILTER_GENRE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(2i32);
pub const PM_APP_FILTER_NONGAMES: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(3i32);
pub const PM_APP_FILTER_HUBTYPE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(4i32);
pub const PM_APP_FILTER_PINABLEONKIDZONE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(5i32);
pub const PM_APP_FILTER_ALL_INCLUDE_MODERN: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(6i32);
pub const PM_APP_FILTER_FRAMEWORK: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(7i32);
pub const PM_APP_FILTER_MAX: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(8i32);
impl ::std::convert::From<i32> for PM_ENUM_APP_FILTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_ENUM_APP_FILTER {
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
pub struct PM_ENUM_BSA_FILTER(pub i32);
pub const PM_ENUM_BSA_FILTER_ALL: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(26i32);
pub const PM_ENUM_BSA_FILTER_BY_TASKID: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(27i32);
pub const PM_ENUM_BSA_FILTER_BY_PRODUCTID: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(28i32);
pub const PM_ENUM_BSA_FILTER_BY_PERIODIC: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(29i32);
pub const PM_ENUM_BSA_FILTER_BY_ALL_LAUNCHONBOOT: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(30i32);
pub const PM_ENUM_BSA_FILTER_MAX: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(31i32);
impl ::std::convert::From<i32> for PM_ENUM_BSA_FILTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_ENUM_BSA_FILTER {
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
pub struct PM_ENUM_BW_FILTER(pub i32);
pub const PM_ENUM_BW_FILTER_BOOTWORKER_ALL: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(31i32);
pub const PM_ENUM_BW_FILTER_BY_TASKID: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(32i32);
pub const PM_ENUM_BW_FILTER_MAX: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(33i32);
impl ::std::convert::From<i32> for PM_ENUM_BW_FILTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_ENUM_BW_FILTER {
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
pub struct PM_ENUM_EXTENSION_FILTER(pub i32);
pub const PM_ENUM_EXTENSION_FILTER_BY_CONSUMER: PM_ENUM_EXTENSION_FILTER =
    PM_ENUM_EXTENSION_FILTER(17i32);
pub const PM_ENUM_EXTENSION_FILTER_APPCONNECT: PM_ENUM_EXTENSION_FILTER =
    PM_ENUM_EXTENSION_FILTER(17i32);
pub const PM_ENUM_EXTENSION_FILTER_PROTOCOL_ALL: PM_ENUM_EXTENSION_FILTER =
    PM_ENUM_EXTENSION_FILTER(18i32);
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_FILETYPE_ALL: PM_ENUM_EXTENSION_FILTER =
    PM_ENUM_EXTENSION_FILTER(19i32);
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_CONTENTTYPE_ALL: PM_ENUM_EXTENSION_FILTER =
    PM_ENUM_EXTENSION_FILTER(20i32);
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_APPLICATION_ALL: PM_ENUM_EXTENSION_FILTER =
    PM_ENUM_EXTENSION_FILTER(21i32);
pub const PM_ENUM_EXTENSION_FILTER_SHARETARGET_ALL: PM_ENUM_EXTENSION_FILTER =
    PM_ENUM_EXTENSION_FILTER(22i32);
pub const PM_ENUM_EXTENSION_FILTER_FILEOPENPICKER_ALL: PM_ENUM_EXTENSION_FILTER =
    PM_ENUM_EXTENSION_FILTER(23i32);
pub const PM_ENUM_EXTENSION_FILTER_FILESAVEPICKER_ALL: PM_ENUM_EXTENSION_FILTER =
    PM_ENUM_EXTENSION_FILTER(24i32);
pub const PM_ENUM_EXTENSION_FILTER_CACHEDFILEUPDATER_ALL: PM_ENUM_EXTENSION_FILTER =
    PM_ENUM_EXTENSION_FILTER(25i32);
pub const PM_ENUM_EXTENSION_FILTER_MAX: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(26i32);
impl ::std::convert::From<i32> for PM_ENUM_EXTENSION_FILTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_ENUM_EXTENSION_FILTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for PM_ENUM_FILTER {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_ENUM_FILTER {
    pub FilterType: i32,
    pub FilterParameter: PM_ENUM_FILTER_0,
}
#[cfg(feature = "Win32_Foundation")]
impl PM_ENUM_FILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PM_ENUM_FILTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PM_ENUM_FILTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PM_ENUM_FILTER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PM_ENUM_FILTER {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for PM_ENUM_FILTER_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PM_ENUM_FILTER_0 {
    pub Dummy: i32,
    pub Genre: PM_APP_GENRE,
    pub AppHubType: PM_APPLICATION_HUBTYPE,
    pub HubType: PM_TILE_HUBTYPE,
    pub Tasktype: PM_TASK_TYPE,
    pub TaskProductID: ::windows::runtime::GUID,
    pub TileProductID: ::windows::runtime::GUID,
    pub AppTaskType: _tagAPPTASKTYPE,
    pub Consumer: ::std::mem::ManuallyDrop<PM_EXTENSIONCONSUMER>,
    pub BSATask: ::std::mem::ManuallyDrop<PM_BSATASKID>,
    pub BSAProductID: ::windows::runtime::GUID,
    pub BWTask: ::std::mem::ManuallyDrop<PM_BWTASKID>,
    pub ProtocolName: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub FileType: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub ContentType: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub AppSupportedFileExtPID: ::windows::runtime::GUID,
    pub ShareTargetFileType: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
}
#[cfg(feature = "Win32_Foundation")]
impl PM_ENUM_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PM_ENUM_FILTER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PM_ENUM_FILTER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PM_ENUM_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PM_ENUM_FILTER_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
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
pub struct PM_ENUM_TASK_FILTER(pub i32);
pub const PM_TASK_FILTER_APP_ALL: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(12i32);
pub const PM_TASK_FILTER_TASK_TYPE: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(13i32);
pub const PM_TASK_FILTER_DEHYD_SUPRESSING: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(14i32);
pub const PM_TASK_FILTER_APP_TASK_TYPE: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(15i32);
pub const PM_TASK_FILTER_BGEXECUTION: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(16i32);
pub const PM_TASK_FILTER_MAX: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(17i32);
impl ::std::convert::From<i32> for PM_ENUM_TASK_FILTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_ENUM_TASK_FILTER {
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
pub struct PM_ENUM_TILE_FILTER(pub i32);
pub const PM_TILE_FILTER_APPLIST: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(8i32);
pub const PM_TILE_FILTER_PINNED: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(9i32);
pub const PM_TILE_FILTER_HUBTYPE: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(10i32);
pub const PM_TILE_FILTER_APP_ALL: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(11i32);
pub const PM_TILE_FILTER_MAX: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(12i32);
impl ::std::convert::From<i32> for PM_ENUM_TILE_FILTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_ENUM_TILE_FILTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_EXTENSIONCONSUMER {
    pub ConsumerPID: ::windows::runtime::GUID,
    pub ExtensionID: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PM_EXTENSIONCONSUMER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PM_EXTENSIONCONSUMER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PM_EXTENSIONCONSUMER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PM_EXTENSIONCONSUMER")
            .field("ConsumerPID", &self.ConsumerPID)
            .field("ExtensionID", &self.ExtensionID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PM_EXTENSIONCONSUMER {
    fn eq(&self, other: &Self) -> bool {
        self.ConsumerPID == other.ConsumerPID && self.ExtensionID == other.ExtensionID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PM_EXTENSIONCONSUMER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PM_EXTENSIONCONSUMER {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_INSTALLINFO {
    pub ProductID: ::windows::runtime::GUID,
    pub PackagePath: super::super::Foundation::BSTR,
    pub InstanceID: ::windows::runtime::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub IsUninstallDisabled: super::super::Foundation::BOOL,
    pub DeploymentOptions: u32,
    pub OfferID: ::windows::runtime::GUID,
    pub MarketplaceAppVersion: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PM_INSTALLINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PM_INSTALLINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PM_INSTALLINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PM_INSTALLINFO")
            .field("ProductID", &self.ProductID)
            .field("PackagePath", &self.PackagePath)
            .field("InstanceID", &self.InstanceID)
            .field("pbLicense", &self.pbLicense)
            .field("cbLicense", &self.cbLicense)
            .field("IsUninstallDisabled", &self.IsUninstallDisabled)
            .field("DeploymentOptions", &self.DeploymentOptions)
            .field("OfferID", &self.OfferID)
            .field("MarketplaceAppVersion", &self.MarketplaceAppVersion)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PM_INSTALLINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID
            && self.PackagePath == other.PackagePath
            && self.InstanceID == other.InstanceID
            && self.pbLicense == other.pbLicense
            && self.cbLicense == other.cbLicense
            && self.IsUninstallDisabled == other.IsUninstallDisabled
            && self.DeploymentOptions == other.DeploymentOptions
            && self.OfferID == other.OfferID
            && self.MarketplaceAppVersion == other.MarketplaceAppVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PM_INSTALLINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PM_INSTALLINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_INVOCATIONINFO {
    pub URIBaseOrAUMID: super::super::Foundation::BSTR,
    pub URIFragmentOrArgs: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PM_INVOCATIONINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PM_INVOCATIONINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PM_INVOCATIONINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PM_INVOCATIONINFO")
            .field("URIBaseOrAUMID", &self.URIBaseOrAUMID)
            .field("URIFragmentOrArgs", &self.URIFragmentOrArgs)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PM_INVOCATIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.URIBaseOrAUMID == other.URIBaseOrAUMID
            && self.URIFragmentOrArgs == other.URIFragmentOrArgs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PM_INVOCATIONINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PM_INVOCATIONINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
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
pub struct PM_LIVETILE_RECURRENCE_TYPE(pub i32);
pub const PM_LIVETILE_RECURRENCE_TYPE_INSTANT: PM_LIVETILE_RECURRENCE_TYPE =
    PM_LIVETILE_RECURRENCE_TYPE(0i32);
pub const PM_LIVETILE_RECURRENCE_TYPE_ONETIME: PM_LIVETILE_RECURRENCE_TYPE =
    PM_LIVETILE_RECURRENCE_TYPE(1i32);
pub const PM_LIVETILE_RECURRENCE_TYPE_INTERVAL: PM_LIVETILE_RECURRENCE_TYPE =
    PM_LIVETILE_RECURRENCE_TYPE(2i32);
pub const PM_LIVETILE_RECURRENCE_TYPE_MAX: PM_LIVETILE_RECURRENCE_TYPE =
    PM_LIVETILE_RECURRENCE_TYPE(2i32);
impl ::std::convert::From<i32> for PM_LIVETILE_RECURRENCE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_LIVETILE_RECURRENCE_TYPE {
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
pub struct PM_LOGO_SIZE(pub i32);
pub const PM_LOGO_SIZE_SMALL: PM_LOGO_SIZE = PM_LOGO_SIZE(0i32);
pub const PM_LOGO_SIZE_MEDIUM: PM_LOGO_SIZE = PM_LOGO_SIZE(1i32);
pub const PM_LOGO_SIZE_LARGE: PM_LOGO_SIZE = PM_LOGO_SIZE(2i32);
pub const PM_LOGO_SIZE_INVALID: PM_LOGO_SIZE = PM_LOGO_SIZE(3i32);
impl ::std::convert::From<i32> for PM_LOGO_SIZE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_LOGO_SIZE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_STARTAPPBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows::runtime::GUID,
    pub AppTitle: super::super::Foundation::BSTR,
    pub IconPath: super::super::Foundation::BSTR,
    pub IsUninstallable: super::super::Foundation::BOOL,
    pub AppInstallType: PM_APPLICATION_INSTALL_TYPE,
    pub InstanceID: ::windows::runtime::GUID,
    pub State: PM_APPLICATION_STATE,
    pub IsModern: super::super::Foundation::BOOL,
    pub IsModernLightUp: super::super::Foundation::BOOL,
    pub LightUpSupportMask: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl PM_STARTAPPBLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PM_STARTAPPBLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PM_STARTAPPBLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PM_STARTAPPBLOB")
            .field("cbSize", &self.cbSize)
            .field("ProductID", &self.ProductID)
            .field("AppTitle", &self.AppTitle)
            .field("IconPath", &self.IconPath)
            .field("IsUninstallable", &self.IsUninstallable)
            .field("AppInstallType", &self.AppInstallType)
            .field("InstanceID", &self.InstanceID)
            .field("State", &self.State)
            .field("IsModern", &self.IsModern)
            .field("IsModernLightUp", &self.IsModernLightUp)
            .field("LightUpSupportMask", &self.LightUpSupportMask)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PM_STARTAPPBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.ProductID == other.ProductID
            && self.AppTitle == other.AppTitle
            && self.IconPath == other.IconPath
            && self.IsUninstallable == other.IsUninstallable
            && self.AppInstallType == other.AppInstallType
            && self.InstanceID == other.InstanceID
            && self.State == other.State
            && self.IsModern == other.IsModern
            && self.IsModernLightUp == other.IsModernLightUp
            && self.LightUpSupportMask == other.LightUpSupportMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PM_STARTAPPBLOB {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PM_STARTAPPBLOB {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_STARTTILEBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows::runtime::GUID,
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
impl PM_STARTTILEBLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PM_STARTTILEBLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PM_STARTTILEBLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PM_STARTTILEBLOB")
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
impl ::std::cmp::PartialEq for PM_STARTTILEBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.ProductID == other.ProductID
            && self.TileID == other.TileID
            && self.TemplateType == other.TemplateType
            && self.HubPosition == other.HubPosition
            && self.HubVisibilityBitmask == other.HubVisibilityBitmask
            && self.IsDefault == other.IsDefault
            && self.TileType == other.TileType
            && self.pbPropBlob == other.pbPropBlob
            && self.cbPropBlob == other.cbPropBlob
            && self.IsRestoring == other.IsRestoring
            && self.IsModern == other.IsModern
            && self.InvocationInfo == other.InvocationInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PM_STARTTILEBLOB {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PM_STARTTILEBLOB {
    type Abi = ::std::mem::ManuallyDrop<Self>;
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
pub struct PM_STARTTILE_TYPE(pub i32);
pub const PM_STARTTILE_TYPE_PRIMARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(1i32);
pub const PM_STARTTILE_TYPE_SECONDARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(2i32);
pub const PM_STARTTILE_TYPE_APPLIST: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(3i32);
pub const PM_STARTTILE_TYPE_APPLISTPRIMARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(4i32);
pub const PM_STARTTILE_TYPE_INVALID: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(5i32);
impl ::std::convert::From<i32> for PM_STARTTILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_STARTTILE_TYPE {
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
pub struct PM_TASK_TRANSITION(pub i32);
pub const PM_TASK_TRANSITION_DEFAULT: PM_TASK_TRANSITION = PM_TASK_TRANSITION(0i32);
pub const PM_TASK_TRANSITION_NONE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(1i32);
pub const PM_TASK_TRANSITION_TURNSTILE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(2i32);
pub const PM_TASK_TRANSITION_SLIDE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(3i32);
pub const PM_TASK_TRANSITION_SWIVEL: PM_TASK_TRANSITION = PM_TASK_TRANSITION(4i32);
pub const PM_TASK_TRANSITION_READERBOARD: PM_TASK_TRANSITION = PM_TASK_TRANSITION(5i32);
pub const PM_TASK_TRANSITION_CUSTOM: PM_TASK_TRANSITION = PM_TASK_TRANSITION(6i32);
pub const PM_TASK_TRANSITION_INVALID: PM_TASK_TRANSITION = PM_TASK_TRANSITION(7i32);
impl ::std::convert::From<i32> for PM_TASK_TRANSITION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_TASK_TRANSITION {
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
pub struct PM_TASK_TYPE(pub i32);
pub const PM_TASK_TYPE_NORMAL: PM_TASK_TYPE = PM_TASK_TYPE(0i32);
pub const PM_TASK_TYPE_DEFAULT: PM_TASK_TYPE = PM_TASK_TYPE(1i32);
pub const PM_TASK_TYPE_SETTINGS: PM_TASK_TYPE = PM_TASK_TYPE(2i32);
pub const PM_TASK_TYPE_BACKGROUNDSERVICEAGENT: PM_TASK_TYPE = PM_TASK_TYPE(3i32);
pub const PM_TASK_TYPE_BACKGROUNDWORKER: PM_TASK_TYPE = PM_TASK_TYPE(4i32);
pub const PM_TASK_TYPE_INVALID: PM_TASK_TYPE = PM_TASK_TYPE(5i32);
impl ::std::convert::From<i32> for PM_TASK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_TASK_TYPE {
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
pub struct PM_TILE_HUBTYPE(pub i32);
pub const PM_TILE_HUBTYPE_MUSIC: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(1i32);
pub const PM_TILE_HUBTYPE_MOSETTINGS: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(268435456i32);
pub const PM_TILE_HUBTYPE_GAMES: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(536870912i32);
pub const PM_TILE_HUBTYPE_APPLIST: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(1073741824i32);
pub const PM_TILE_HUBTYPE_STARTMENU: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(-2147483648i32);
pub const PM_TILE_HUBTYPE_LOCKSCREEN: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(16777216i32);
pub const PM_TILE_HUBTYPE_KIDZONE: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(33554432i32);
pub const PM_TILE_HUBTYPE_CACHED: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(67108864i32);
pub const PM_TILE_HUBTYPE_INVALID: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(67108865i32);
impl ::std::convert::From<i32> for PM_TILE_HUBTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_TILE_HUBTYPE {
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
pub struct PM_TILE_SIZE(pub i32);
pub const PM_TILE_SIZE_SMALL: PM_TILE_SIZE = PM_TILE_SIZE(0i32);
pub const PM_TILE_SIZE_MEDIUM: PM_TILE_SIZE = PM_TILE_SIZE(1i32);
pub const PM_TILE_SIZE_LARGE: PM_TILE_SIZE = PM_TILE_SIZE(2i32);
pub const PM_TILE_SIZE_SQUARE310X310: PM_TILE_SIZE = PM_TILE_SIZE(3i32);
pub const PM_TILE_SIZE_TALL150X310: PM_TILE_SIZE = PM_TILE_SIZE(4i32);
pub const PM_TILE_SIZE_INVALID: PM_TILE_SIZE = PM_TILE_SIZE(5i32);
impl ::std::convert::From<i32> for PM_TILE_SIZE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PM_TILE_SIZE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_UPDATEINFO {
    pub ProductID: ::windows::runtime::GUID,
    pub PackagePath: super::super::Foundation::BSTR,
    pub InstanceID: ::windows::runtime::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: super::super::Foundation::BSTR,
    pub DeploymentOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PM_UPDATEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PM_UPDATEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PM_UPDATEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PM_UPDATEINFO")
            .field("ProductID", &self.ProductID)
            .field("PackagePath", &self.PackagePath)
            .field("InstanceID", &self.InstanceID)
            .field("pbLicense", &self.pbLicense)
            .field("cbLicense", &self.cbLicense)
            .field("MarketplaceAppVersion", &self.MarketplaceAppVersion)
            .field("DeploymentOptions", &self.DeploymentOptions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PM_UPDATEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID
            && self.PackagePath == other.PackagePath
            && self.InstanceID == other.InstanceID
            && self.pbLicense == other.pbLicense
            && self.cbLicense == other.cbLicense
            && self.MarketplaceAppVersion == other.MarketplaceAppVersion
            && self.DeploymentOptions == other.DeploymentOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PM_UPDATEINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PM_UPDATEINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_UPDATEINFO_LEGACY {
    pub ProductID: ::windows::runtime::GUID,
    pub PackagePath: super::super::Foundation::BSTR,
    pub InstanceID: ::windows::runtime::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PM_UPDATEINFO_LEGACY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PM_UPDATEINFO_LEGACY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PM_UPDATEINFO_LEGACY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PM_UPDATEINFO_LEGACY")
            .field("ProductID", &self.ProductID)
            .field("PackagePath", &self.PackagePath)
            .field("InstanceID", &self.InstanceID)
            .field("pbLicense", &self.pbLicense)
            .field("cbLicense", &self.cbLicense)
            .field("MarketplaceAppVersion", &self.MarketplaceAppVersion)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PM_UPDATEINFO_LEGACY {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID
            && self.PackagePath == other.PackagePath
            && self.InstanceID == other.InstanceID
            && self.pbLicense == other.pbLicense
            && self.cbLicense == other.cbLicense
            && self.MarketplaceAppVersion == other.MarketplaceAppVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PM_UPDATEINFO_LEGACY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PM_UPDATEINFO_LEGACY {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_PROGRESS_CALLBACK = unsafe extern "system" fn(
    callbackcontext: *mut ::std::ffi::c_void,
    currentposition: u32,
    maximumposition: u32,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_SYMLOAD_CALLBACK = unsafe extern "system" fn(
    whichfile: u32,
    symbolfilename: super::super::Foundation::PSTR,
    symtype: u32,
    symbolfilechecksum: u32,
    symbolfiletimedate: u32,
    imagefilechecksum: u32,
    imagefiletimedate: u32,
    callbackcontext: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROTECTED_FILE_DATA {
    pub FileName: [u16; 260],
    pub FileNumber: u32,
}
impl PROTECTED_FILE_DATA {}
impl ::std::default::Default for PROTECTED_FILE_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROTECTED_FILE_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROTECTED_FILE_DATA")
            .field("FileName", &self.FileName)
            .field("FileNumber", &self.FileNumber)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROTECTED_FILE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.FileName == other.FileName && self.FileNumber == other.FileNumber
    }
}
impl ::std::cmp::Eq for PROTECTED_FILE_DATA {}
unsafe impl ::windows::runtime::Abi for PROTECTED_FILE_DATA {
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
pub struct QUERYASMINFO_FLAGS(pub u32);
pub const QUERYASMINFO_FLAG_VALIDATE: QUERYASMINFO_FLAGS = QUERYASMINFO_FLAGS(1u32);
impl ::std::convert::From<u32> for QUERYASMINFO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for QUERYASMINFO_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for QUERYASMINFO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for QUERYASMINFO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryActCtxSettingsW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwflags: u32,
    hactctx: Param1,
    settingsnamespace: Param2,
    settingname: Param3,
    pvbuffer: super::super::Foundation::PWSTR,
    dwbuffer: usize,
    pdwwrittenorrequired: *mut usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryActCtxSettingsW(
                dwflags: u32,
                hactctx: super::super::Foundation::HANDLE,
                settingsnamespace: super::super::Foundation::PWSTR,
                settingname: super::super::Foundation::PWSTR,
                pvbuffer: super::super::Foundation::PWSTR,
                dwbuffer: usize,
                pdwwrittenorrequired: *mut usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryActCtxSettingsW(
            ::std::mem::transmute(dwflags),
            hactctx.into_param().abi(),
            settingsnamespace.into_param().abi(),
            settingname.into_param().abi(),
            ::std::mem::transmute(pvbuffer),
            ::std::mem::transmute(dwbuffer),
            ::std::mem::transmute(pdwwrittenorrequired),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryActCtxW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    dwflags: u32,
    hactctx: Param1,
    pvsubinstance: *const ::std::ffi::c_void,
    ulinfoclass: u32,
    pvbuffer: *mut ::std::ffi::c_void,
    cbbuffer: usize,
    pcbwrittenorrequired: *mut usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryActCtxW(
                dwflags: u32,
                hactctx: super::super::Foundation::HANDLE,
                pvsubinstance: *const ::std::ffi::c_void,
                ulinfoclass: u32,
                pvbuffer: *mut ::std::ffi::c_void,
                cbbuffer: usize,
                pcbwrittenorrequired: *mut usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryActCtxW(
            ::std::mem::transmute(dwflags),
            hactctx.into_param().abi(),
            ::std::mem::transmute(pvsubinstance),
            ::std::mem::transmute(ulinfoclass),
            ::std::mem::transmute(pvbuffer),
            ::std::mem::transmute(cbbuffer),
            ::std::mem::transmute(pcbwrittenorrequired),
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
pub struct REINSTALLMODE(pub i32);
pub const REINSTALLMODE_REPAIR: REINSTALLMODE = REINSTALLMODE(1i32);
pub const REINSTALLMODE_FILEMISSING: REINSTALLMODE = REINSTALLMODE(2i32);
pub const REINSTALLMODE_FILEOLDERVERSION: REINSTALLMODE = REINSTALLMODE(4i32);
pub const REINSTALLMODE_FILEEQUALVERSION: REINSTALLMODE = REINSTALLMODE(8i32);
pub const REINSTALLMODE_FILEEXACT: REINSTALLMODE = REINSTALLMODE(16i32);
pub const REINSTALLMODE_FILEVERIFY: REINSTALLMODE = REINSTALLMODE(32i32);
pub const REINSTALLMODE_FILEREPLACE: REINSTALLMODE = REINSTALLMODE(64i32);
pub const REINSTALLMODE_MACHINEDATA: REINSTALLMODE = REINSTALLMODE(128i32);
pub const REINSTALLMODE_USERDATA: REINSTALLMODE = REINSTALLMODE(256i32);
pub const REINSTALLMODE_SHORTCUT: REINSTALLMODE = REINSTALLMODE(512i32);
pub const REINSTALLMODE_PACKAGE: REINSTALLMODE = REINSTALLMODE(1024i32);
impl ::std::convert::From<i32> for REINSTALLMODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REINSTALLMODE {
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
pub struct RESULTTYPES(pub i32);
pub const ieUnknown: RESULTTYPES = RESULTTYPES(0i32);
pub const ieError: RESULTTYPES = RESULTTYPES(1i32);
pub const ieWarning: RESULTTYPES = RESULTTYPES(2i32);
pub const ieInfo: RESULTTYPES = RESULTTYPES(3i32);
impl ::std::convert::From<i32> for RESULTTYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RESULTTYPES {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseActCtx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hactctx: Param0,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseActCtx(hactctx: super::super::Foundation::HANDLE);
        }
        ::std::mem::transmute(ReleaseActCtx(hactctx.into_param().abi()))
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
pub struct SCRIPTFLAGS(pub i32);
pub const SCRIPTFLAGS_CACHEINFO: SCRIPTFLAGS = SCRIPTFLAGS(1i32);
pub const SCRIPTFLAGS_SHORTCUTS: SCRIPTFLAGS = SCRIPTFLAGS(4i32);
pub const SCRIPTFLAGS_MACHINEASSIGN: SCRIPTFLAGS = SCRIPTFLAGS(8i32);
pub const SCRIPTFLAGS_REGDATA_CNFGINFO: SCRIPTFLAGS = SCRIPTFLAGS(32i32);
pub const SCRIPTFLAGS_VALIDATE_TRANSFORMS_LIST: SCRIPTFLAGS = SCRIPTFLAGS(64i32);
pub const SCRIPTFLAGS_REGDATA_CLASSINFO: SCRIPTFLAGS = SCRIPTFLAGS(128i32);
pub const SCRIPTFLAGS_REGDATA_EXTENSIONINFO: SCRIPTFLAGS = SCRIPTFLAGS(256i32);
pub const SCRIPTFLAGS_REGDATA_APPINFO: SCRIPTFLAGS = SCRIPTFLAGS(384i32);
pub const SCRIPTFLAGS_REGDATA: SCRIPTFLAGS = SCRIPTFLAGS(416i32);
impl ::std::convert::From<i32> for SCRIPTFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SCRIPTFLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SFC_DISABLE_ASK: u32 = 1u32;
pub const SFC_DISABLE_NOPOPUPS: u32 = 4u32;
pub const SFC_DISABLE_NORMAL: u32 = 0u32;
pub const SFC_DISABLE_ONCE: u32 = 2u32;
pub const SFC_DISABLE_SETUP: u32 = 3u32;
pub const SFC_QUOTA_DEFAULT: u32 = 50u32;
pub const SFC_SCAN_ALWAYS: u32 = 1u32;
pub const SFC_SCAN_IMMEDIATE: u32 = 3u32;
pub const SFC_SCAN_NORMAL: u32 = 0u32;
pub const SFC_SCAN_ONCE: u32 = 2u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct STATUSTYPES(pub i32);
pub const ieStatusGetCUB: STATUSTYPES = STATUSTYPES(0i32);
pub const ieStatusICECount: STATUSTYPES = STATUSTYPES(1i32);
pub const ieStatusMerge: STATUSTYPES = STATUSTYPES(2i32);
pub const ieStatusSummaryInfo: STATUSTYPES = STATUSTYPES(3i32);
pub const ieStatusCreateEngine: STATUSTYPES = STATUSTYPES(4i32);
pub const ieStatusStarting: STATUSTYPES = STATUSTYPES(5i32);
pub const ieStatusRunICE: STATUSTYPES = STATUSTYPES(6i32);
pub const ieStatusShutdown: STATUSTYPES = STATUSTYPES(7i32);
pub const ieStatusSuccess: STATUSTYPES = STATUSTYPES(8i32);
pub const ieStatusFail: STATUSTYPES = STATUSTYPES(9i32);
pub const ieStatusCancel: STATUSTYPES = STATUSTYPES(10i32);
impl ::std::convert::From<i32> for STATUSTYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STATUSTYPES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STREAM_FORMAT_COMPLIB_MANIFEST: u32 = 1u32;
pub const STREAM_FORMAT_COMPLIB_MODULE: u32 = 0u32;
pub const STREAM_FORMAT_WIN32_MANIFEST: u32 = 4u32;
pub const STREAM_FORMAT_WIN32_MODULE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SfcGetNextProtectedFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    rpchandle: Param0,
    protfiledata: *mut PROTECTED_FILE_DATA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SfcGetNextProtectedFile(
                rpchandle: super::super::Foundation::HANDLE,
                protfiledata: *mut PROTECTED_FILE_DATA,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SfcGetNextProtectedFile(
            rpchandle.into_param().abi(),
            ::std::mem::transmute(protfiledata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SfcIsFileProtected<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    rpchandle: Param0,
    protfilename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SfcIsFileProtected(
                rpchandle: super::super::Foundation::HANDLE,
                protfilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SfcIsFileProtected(
            rpchandle.into_param().abi(),
            protfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn SfcIsKeyProtected<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    keyhandle: Param0,
    subkeyname: Param1,
    keysam: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SfcIsKeyProtected(
                keyhandle: super::Registry::HKEY,
                subkeyname: super::super::Foundation::PWSTR,
                keysam: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SfcIsKeyProtected(
            keyhandle.into_param().abi(),
            subkeyname.into_param().abi(),
            ::std::mem::transmute(keysam),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SfpVerifyFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pszfilename: Param0,
    pszerror: Param1,
    dwerrsize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SfpVerifyFile(
                pszfilename: super::super::Foundation::PSTR,
                pszerror: super::super::Foundation::PSTR,
                dwerrsize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SfpVerifyFile(
            pszfilename.into_param().abi(),
            pszerror.into_param().abi(),
            ::std::mem::transmute(dwerrsize),
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
pub struct TILE_TEMPLATE_TYPE(pub i32);
pub const TILE_TEMPLATE_INVALID: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(0i32);
pub const TILE_TEMPLATE_FLIP: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(5i32);
pub const TILE_TEMPLATE_DEEPLINK: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(13i32);
pub const TILE_TEMPLATE_CYCLE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(14i32);
pub const TILE_TEMPLATE_METROCOUNT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(1i32);
pub const TILE_TEMPLATE_AGILESTORE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(2i32);
pub const TILE_TEMPLATE_GAMES: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(3i32);
pub const TILE_TEMPLATE_CALENDAR: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(4i32);
pub const TILE_TEMPLATE_MUSICVIDEO: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(7i32);
pub const TILE_TEMPLATE_PEOPLE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(10i32);
pub const TILE_TEMPLATE_CONTACT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(11i32);
pub const TILE_TEMPLATE_GROUP: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(12i32);
pub const TILE_TEMPLATE_DEFAULT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(15i32);
pub const TILE_TEMPLATE_BADGE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(16i32);
pub const TILE_TEMPLATE_BLOCK: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(17i32);
pub const TILE_TEMPLATE_TEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(18i32);
pub const TILE_TEMPLATE_TEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(19i32);
pub const TILE_TEMPLATE_TEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(20i32);
pub const TILE_TEMPLATE_TEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(21i32);
pub const TILE_TEMPLATE_TEXT05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(22i32);
pub const TILE_TEMPLATE_TEXT06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(23i32);
pub const TILE_TEMPLATE_TEXT07: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(24i32);
pub const TILE_TEMPLATE_TEXT08: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(25i32);
pub const TILE_TEMPLATE_TEXT09: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(26i32);
pub const TILE_TEMPLATE_TEXT10: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(27i32);
pub const TILE_TEMPLATE_TEXT11: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(28i32);
pub const TILE_TEMPLATE_IMAGE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(29i32);
pub const TILE_TEMPLATE_IMAGECOLLECTION: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(30i32);
pub const TILE_TEMPLATE_IMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(31i32);
pub const TILE_TEMPLATE_IMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(32i32);
pub const TILE_TEMPLATE_BLOCKANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(33i32);
pub const TILE_TEMPLATE_BLOCKANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(34i32);
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(35i32);
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(36i32);
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(37i32);
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(38i32);
pub const TILE_TEMPLATE_PEEKIMAGE01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(39i32);
pub const TILE_TEMPLATE_PEEKIMAGE02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(40i32);
pub const TILE_TEMPLATE_PEEKIMAGE03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(41i32);
pub const TILE_TEMPLATE_PEEKIMAGE04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(42i32);
pub const TILE_TEMPLATE_PEEKIMAGE05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(43i32);
pub const TILE_TEMPLATE_PEEKIMAGE06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(44i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(45i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(46i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(47i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(48i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(49i32);
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(50i32);
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(51i32);
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(52i32);
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(53i32);
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(54i32);
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(55i32);
pub const TILE_TEMPLATE_METROCOUNTQUEUE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(56i32);
pub const TILE_TEMPLATE_SEARCH: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(57i32);
pub const TILE_TEMPLATE_TILEFLYOUT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(58i32);
pub const TILE_TEMPLATE_FOLDER: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(59i32);
pub const TILE_TEMPLATE_ALL: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(100i32);
impl ::std::convert::From<i32> for TILE_TEMPLATE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TILE_TEMPLATE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const TXTLOG_BACKUP: u32 = 128u32;
pub const TXTLOG_CMI: u32 = 268435456u32;
pub const TXTLOG_COPYFILES: u32 = 8u32;
pub const TXTLOG_DEPTH_DECR: u32 = 262144u32;
pub const TXTLOG_DEPTH_INCR: u32 = 131072u32;
pub const TXTLOG_DETAILS: u32 = 5u32;
pub const TXTLOG_DEVINST: u32 = 1u32;
pub const TXTLOG_DEVMGR: u32 = 536870912u32;
pub const TXTLOG_DRIVER_STORE: u32 = 67108864u32;
pub const TXTLOG_DRVSETUP: u32 = 4194304u32;
pub const TXTLOG_ERROR: u32 = 1u32;
pub const TXTLOG_FILEQ: u32 = 4u32;
pub const TXTLOG_FLUSH_FILE: u32 = 1048576u32;
pub const TXTLOG_INF: u32 = 2u32;
pub const TXTLOG_INFDB: u32 = 1024u32;
pub const TXTLOG_INSTALLER: u32 = 1073741824u32;
pub const TXTLOG_NEWDEV: u32 = 16777216u32;
pub const TXTLOG_POLICY: u32 = 8388608u32;
pub const TXTLOG_RESERVED_FLAGS: u32 = 65520u32;
pub const TXTLOG_SETUP: u32 = 134217728u32;
pub const TXTLOG_SETUPAPI_BITS: u32 = 3u32;
pub const TXTLOG_SETUPAPI_CMDLINE: u32 = 2u32;
pub const TXTLOG_SETUPAPI_DEVLOG: u32 = 1u32;
pub const TXTLOG_SIGVERIF: u32 = 32u32;
pub const TXTLOG_SUMMARY: u32 = 4u32;
pub const TXTLOG_SYSTEM_STATE_CHANGE: u32 = 3u32;
pub const TXTLOG_TAB_1: u32 = 524288u32;
pub const TXTLOG_TIMESTAMP: u32 = 65536u32;
pub const TXTLOG_UI: u32 = 256u32;
pub const TXTLOG_UMPNPMGR: u32 = 33554432u32;
pub const TXTLOG_UTIL: u32 = 512u32;
pub const TXTLOG_VENDOR: u32 = 2147483648u32;
pub const TXTLOG_VERBOSE: u32 = 6u32;
pub const TXTLOG_VERY_VERBOSE: u32 = 7u32;
pub const TXTLOG_WARNING: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    patchfilename: Param0,
    oldfilename: Param1,
    applyoptionflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TestApplyPatchToFileA(
                patchfilename: super::super::Foundation::PSTR,
                oldfilename: super::super::Foundation::PSTR,
                applyoptionflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TestApplyPatchToFileA(
            patchfilename.into_param().abi(),
            oldfilename.into_param().abi(),
            ::std::mem::transmute(applyoptionflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileByBuffers(
    patchfilebuffer: *const u8,
    patchfilesize: u32,
    oldfilebuffer: *const u8,
    oldfilesize: u32,
    newfilesize: *mut u32,
    applyoptionflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TestApplyPatchToFileByBuffers(
                patchfilebuffer: *const u8,
                patchfilesize: u32,
                oldfilebuffer: *const u8,
                oldfilesize: u32,
                newfilesize: *mut u32,
                applyoptionflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TestApplyPatchToFileByBuffers(
            ::std::mem::transmute(patchfilebuffer),
            ::std::mem::transmute(patchfilesize),
            ::std::mem::transmute(oldfilebuffer),
            ::std::mem::transmute(oldfilesize),
            ::std::mem::transmute(newfilesize),
            ::std::mem::transmute(applyoptionflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileByHandles<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    patchfilehandle: Param0,
    oldfilehandle: Param1,
    applyoptionflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TestApplyPatchToFileByHandles(
                patchfilehandle: super::super::Foundation::HANDLE,
                oldfilehandle: super::super::Foundation::HANDLE,
                applyoptionflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TestApplyPatchToFileByHandles(
            patchfilehandle.into_param().abi(),
            oldfilehandle.into_param().abi(),
            ::std::mem::transmute(applyoptionflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    patchfilename: Param0,
    oldfilename: Param1,
    applyoptionflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TestApplyPatchToFileW(
                patchfilename: super::super::Foundation::PWSTR,
                oldfilename: super::super::Foundation::PWSTR,
                applyoptionflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TestApplyPatchToFileW(
            patchfilename.into_param().abi(),
            oldfilename.into_param().abi(),
            ::std::mem::transmute(applyoptionflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const UIALL: u32 = 32768u32;
pub const UILOGBITS: u32 = 15u32;
pub const UINONE: u32 = 0u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct USERINFOSTATE(pub i32);
pub const USERINFOSTATE_MOREDATA: USERINFOSTATE = USERINFOSTATE(-3i32);
pub const USERINFOSTATE_INVALIDARG: USERINFOSTATE = USERINFOSTATE(-2i32);
pub const USERINFOSTATE_UNKNOWN: USERINFOSTATE = USERINFOSTATE(-1i32);
pub const USERINFOSTATE_ABSENT: USERINFOSTATE = USERINFOSTATE(0i32);
pub const USERINFOSTATE_PRESENT: USERINFOSTATE = USERINFOSTATE(1i32);
impl ::std::convert::From<i32> for USERINFOSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for USERINFOSTATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WARN_BAD_MAJOR_VERSION: u32 = 3222294792u32;
pub const WARN_BASE: u32 = 3222294785u32;
pub const WARN_EQUAL_FILE_VERSION: u32 = 3222294794u32;
pub const WARN_FILE_VERSION_DOWNREV: u32 = 3222294793u32;
pub const WARN_IMPROPER_TRANSFORM_VALIDATION: u32 = 3222294788u32;
pub const WARN_INVALID_TRANSFORM_VALIDATION: u32 = 3222294791u32;
pub const WARN_MAJOR_UPGRADE_PATCH: u32 = 3222294785u32;
pub const WARN_OBSOLETION_WITH_MSI30: u32 = 3222294801u32;
pub const WARN_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222294803u32;
pub const WARN_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222294802u32;
pub const WARN_PATCHPROPERTYNOTSET: u32 = 3222294795u32;
pub const WARN_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222294789u32;
pub const WARN_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222294790u32;
pub const WARN_SEQUENCE_DATA_GENERATION_DISABLED: u32 = 3222294786u32;
pub const WARN_SEQUENCE_DATA_SUPERSEDENCE_IGNORED: u32 = 3222294787u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ZombifyActCtx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hactctx: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ZombifyActCtx(
                hactctx: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ZombifyActCtx(hactctx.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const _WIN32_MSI: u32 = 500u32;
pub const _WIN32_MSM: u32 = 100u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct _tagAPPTASKTYPE {
    pub ProductID: ::windows::runtime::GUID,
    pub TaskType: PM_TASK_TYPE,
}
impl _tagAPPTASKTYPE {}
impl ::std::default::Default for _tagAPPTASKTYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _tagAPPTASKTYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_tagAPPTASKTYPE")
            .field("ProductID", &self.ProductID)
            .field("TaskType", &self.TaskType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for _tagAPPTASKTYPE {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.TaskType == other.TaskType
    }
}
impl ::std::cmp::Eq for _tagAPPTASKTYPE {}
unsafe impl ::windows::runtime::Abi for _tagAPPTASKTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const cchMaxInteger: i32 = 12i32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct msidbAssemblyAttributes(pub i32);
pub const msidbAssemblyAttributesURT: msidbAssemblyAttributes = msidbAssemblyAttributes(0i32);
pub const msidbAssemblyAttributesWin32: msidbAssemblyAttributes = msidbAssemblyAttributes(1i32);
impl ::std::convert::From<i32> for msidbAssemblyAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbAssemblyAttributes {
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
pub struct msidbClassAttributes(pub i32);
pub const msidbClassAttributesRelativePath: msidbClassAttributes = msidbClassAttributes(1i32);
impl ::std::convert::From<i32> for msidbClassAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbClassAttributes {
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
pub struct msidbComponentAttributes(pub i32);
pub const msidbComponentAttributesLocalOnly: msidbComponentAttributes =
    msidbComponentAttributes(0i32);
pub const msidbComponentAttributesSourceOnly: msidbComponentAttributes =
    msidbComponentAttributes(1i32);
pub const msidbComponentAttributesOptional: msidbComponentAttributes =
    msidbComponentAttributes(2i32);
pub const msidbComponentAttributesRegistryKeyPath: msidbComponentAttributes =
    msidbComponentAttributes(4i32);
pub const msidbComponentAttributesSharedDllRefCount: msidbComponentAttributes =
    msidbComponentAttributes(8i32);
pub const msidbComponentAttributesPermanent: msidbComponentAttributes =
    msidbComponentAttributes(16i32);
pub const msidbComponentAttributesODBCDataSource: msidbComponentAttributes =
    msidbComponentAttributes(32i32);
pub const msidbComponentAttributesTransitive: msidbComponentAttributes =
    msidbComponentAttributes(64i32);
pub const msidbComponentAttributesNeverOverwrite: msidbComponentAttributes =
    msidbComponentAttributes(128i32);
pub const msidbComponentAttributes64bit: msidbComponentAttributes =
    msidbComponentAttributes(256i32);
pub const msidbComponentAttributesDisableRegistryReflection: msidbComponentAttributes =
    msidbComponentAttributes(512i32);
pub const msidbComponentAttributesUninstallOnSupersedence: msidbComponentAttributes =
    msidbComponentAttributes(1024i32);
pub const msidbComponentAttributesShared: msidbComponentAttributes =
    msidbComponentAttributes(2048i32);
impl ::std::convert::From<i32> for msidbComponentAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbComponentAttributes {
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
pub struct msidbControlAttributes(pub i32);
pub const msidbControlAttributesVisible: msidbControlAttributes = msidbControlAttributes(1i32);
pub const msidbControlAttributesEnabled: msidbControlAttributes = msidbControlAttributes(2i32);
pub const msidbControlAttributesSunken: msidbControlAttributes = msidbControlAttributes(4i32);
pub const msidbControlAttributesIndirect: msidbControlAttributes = msidbControlAttributes(8i32);
pub const msidbControlAttributesInteger: msidbControlAttributes = msidbControlAttributes(16i32);
pub const msidbControlAttributesRTLRO: msidbControlAttributes = msidbControlAttributes(32i32);
pub const msidbControlAttributesRightAligned: msidbControlAttributes =
    msidbControlAttributes(64i32);
pub const msidbControlAttributesLeftScroll: msidbControlAttributes = msidbControlAttributes(128i32);
pub const msidbControlAttributesBiDi: msidbControlAttributes = msidbControlAttributes(224i32);
pub const msidbControlAttributesTransparent: msidbControlAttributes =
    msidbControlAttributes(65536i32);
pub const msidbControlAttributesNoPrefix: msidbControlAttributes =
    msidbControlAttributes(131072i32);
pub const msidbControlAttributesNoWrap: msidbControlAttributes = msidbControlAttributes(262144i32);
pub const msidbControlAttributesFormatSize: msidbControlAttributes =
    msidbControlAttributes(524288i32);
pub const msidbControlAttributesUsersLanguage: msidbControlAttributes =
    msidbControlAttributes(1048576i32);
pub const msidbControlAttributesMultiline: msidbControlAttributes =
    msidbControlAttributes(65536i32);
pub const msidbControlAttributesPasswordInput: msidbControlAttributes =
    msidbControlAttributes(2097152i32);
pub const msidbControlAttributesProgress95: msidbControlAttributes =
    msidbControlAttributes(65536i32);
pub const msidbControlAttributesRemovableVolume: msidbControlAttributes =
    msidbControlAttributes(65536i32);
pub const msidbControlAttributesFixedVolume: msidbControlAttributes =
    msidbControlAttributes(131072i32);
pub const msidbControlAttributesRemoteVolume: msidbControlAttributes =
    msidbControlAttributes(262144i32);
pub const msidbControlAttributesCDROMVolume: msidbControlAttributes =
    msidbControlAttributes(524288i32);
pub const msidbControlAttributesRAMDiskVolume: msidbControlAttributes =
    msidbControlAttributes(1048576i32);
pub const msidbControlAttributesFloppyVolume: msidbControlAttributes =
    msidbControlAttributes(2097152i32);
pub const msidbControlShowRollbackCost: msidbControlAttributes = msidbControlAttributes(4194304i32);
pub const msidbControlAttributesSorted: msidbControlAttributes = msidbControlAttributes(65536i32);
pub const msidbControlAttributesComboList: msidbControlAttributes =
    msidbControlAttributes(131072i32);
pub const msidbControlAttributesImageHandle: msidbControlAttributes =
    msidbControlAttributes(65536i32);
pub const msidbControlAttributesPushLike: msidbControlAttributes =
    msidbControlAttributes(131072i32);
pub const msidbControlAttributesBitmap: msidbControlAttributes = msidbControlAttributes(262144i32);
pub const msidbControlAttributesIcon: msidbControlAttributes = msidbControlAttributes(524288i32);
pub const msidbControlAttributesFixedSize: msidbControlAttributes =
    msidbControlAttributes(1048576i32);
pub const msidbControlAttributesIconSize16: msidbControlAttributes =
    msidbControlAttributes(2097152i32);
pub const msidbControlAttributesIconSize32: msidbControlAttributes =
    msidbControlAttributes(4194304i32);
pub const msidbControlAttributesIconSize48: msidbControlAttributes =
    msidbControlAttributes(6291456i32);
pub const msidbControlAttributesElevationShield: msidbControlAttributes =
    msidbControlAttributes(8388608i32);
pub const msidbControlAttributesHasBorder: msidbControlAttributes =
    msidbControlAttributes(16777216i32);
impl ::std::convert::From<i32> for msidbControlAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbControlAttributes {
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
pub struct msidbCustomActionType(pub i32);
pub const msidbCustomActionTypeDll: msidbCustomActionType = msidbCustomActionType(1i32);
pub const msidbCustomActionTypeExe: msidbCustomActionType = msidbCustomActionType(2i32);
pub const msidbCustomActionTypeTextData: msidbCustomActionType = msidbCustomActionType(3i32);
pub const msidbCustomActionTypeJScript: msidbCustomActionType = msidbCustomActionType(5i32);
pub const msidbCustomActionTypeVBScript: msidbCustomActionType = msidbCustomActionType(6i32);
pub const msidbCustomActionTypeInstall: msidbCustomActionType = msidbCustomActionType(7i32);
pub const msidbCustomActionTypeBinaryData: msidbCustomActionType = msidbCustomActionType(0i32);
pub const msidbCustomActionTypeSourceFile: msidbCustomActionType = msidbCustomActionType(16i32);
pub const msidbCustomActionTypeDirectory: msidbCustomActionType = msidbCustomActionType(32i32);
pub const msidbCustomActionTypeProperty: msidbCustomActionType = msidbCustomActionType(48i32);
pub const msidbCustomActionTypeContinue: msidbCustomActionType = msidbCustomActionType(64i32);
pub const msidbCustomActionTypeAsync: msidbCustomActionType = msidbCustomActionType(128i32);
pub const msidbCustomActionTypeFirstSequence: msidbCustomActionType = msidbCustomActionType(256i32);
pub const msidbCustomActionTypeOncePerProcess: msidbCustomActionType =
    msidbCustomActionType(512i32);
pub const msidbCustomActionTypeClientRepeat: msidbCustomActionType = msidbCustomActionType(768i32);
pub const msidbCustomActionTypeInScript: msidbCustomActionType = msidbCustomActionType(1024i32);
pub const msidbCustomActionTypeRollback: msidbCustomActionType = msidbCustomActionType(256i32);
pub const msidbCustomActionTypeCommit: msidbCustomActionType = msidbCustomActionType(512i32);
pub const msidbCustomActionTypeNoImpersonate: msidbCustomActionType =
    msidbCustomActionType(2048i32);
pub const msidbCustomActionTypeTSAware: msidbCustomActionType = msidbCustomActionType(16384i32);
pub const msidbCustomActionType64BitScript: msidbCustomActionType = msidbCustomActionType(4096i32);
pub const msidbCustomActionTypeHideTarget: msidbCustomActionType = msidbCustomActionType(8192i32);
pub const msidbCustomActionTypePatchUninstall: msidbCustomActionType =
    msidbCustomActionType(32768i32);
impl ::std::convert::From<i32> for msidbCustomActionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbCustomActionType {
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
pub struct msidbDialogAttributes(pub i32);
pub const msidbDialogAttributesVisible: msidbDialogAttributes = msidbDialogAttributes(1i32);
pub const msidbDialogAttributesModal: msidbDialogAttributes = msidbDialogAttributes(2i32);
pub const msidbDialogAttributesMinimize: msidbDialogAttributes = msidbDialogAttributes(4i32);
pub const msidbDialogAttributesSysModal: msidbDialogAttributes = msidbDialogAttributes(8i32);
pub const msidbDialogAttributesKeepModeless: msidbDialogAttributes = msidbDialogAttributes(16i32);
pub const msidbDialogAttributesTrackDiskSpace: msidbDialogAttributes = msidbDialogAttributes(32i32);
pub const msidbDialogAttributesUseCustomPalette: msidbDialogAttributes =
    msidbDialogAttributes(64i32);
pub const msidbDialogAttributesRTLRO: msidbDialogAttributes = msidbDialogAttributes(128i32);
pub const msidbDialogAttributesRightAligned: msidbDialogAttributes = msidbDialogAttributes(256i32);
pub const msidbDialogAttributesLeftScroll: msidbDialogAttributes = msidbDialogAttributes(512i32);
pub const msidbDialogAttributesBiDi: msidbDialogAttributes = msidbDialogAttributes(896i32);
pub const msidbDialogAttributesError: msidbDialogAttributes = msidbDialogAttributes(65536i32);
impl ::std::convert::From<i32> for msidbDialogAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbDialogAttributes {
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
pub struct msidbEmbeddedUIAttributes(pub i32);
pub const msidbEmbeddedUI: msidbEmbeddedUIAttributes = msidbEmbeddedUIAttributes(1i32);
pub const msidbEmbeddedHandlesBasic: msidbEmbeddedUIAttributes = msidbEmbeddedUIAttributes(2i32);
impl ::std::convert::From<i32> for msidbEmbeddedUIAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbEmbeddedUIAttributes {
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
pub struct msidbFeatureAttributes(pub i32);
pub const msidbFeatureAttributesFavorLocal: msidbFeatureAttributes = msidbFeatureAttributes(0i32);
pub const msidbFeatureAttributesFavorSource: msidbFeatureAttributes = msidbFeatureAttributes(1i32);
pub const msidbFeatureAttributesFollowParent: msidbFeatureAttributes = msidbFeatureAttributes(2i32);
pub const msidbFeatureAttributesFavorAdvertise: msidbFeatureAttributes =
    msidbFeatureAttributes(4i32);
pub const msidbFeatureAttributesDisallowAdvertise: msidbFeatureAttributes =
    msidbFeatureAttributes(8i32);
pub const msidbFeatureAttributesUIDisallowAbsent: msidbFeatureAttributes =
    msidbFeatureAttributes(16i32);
pub const msidbFeatureAttributesNoUnsupportedAdvertise: msidbFeatureAttributes =
    msidbFeatureAttributes(32i32);
impl ::std::convert::From<i32> for msidbFeatureAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbFeatureAttributes {
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
pub struct msidbFileAttributes(pub i32);
pub const msidbFileAttributesReadOnly: msidbFileAttributes = msidbFileAttributes(1i32);
pub const msidbFileAttributesHidden: msidbFileAttributes = msidbFileAttributes(2i32);
pub const msidbFileAttributesSystem: msidbFileAttributes = msidbFileAttributes(4i32);
pub const msidbFileAttributesReserved0: msidbFileAttributes = msidbFileAttributes(8i32);
pub const msidbFileAttributesIsolatedComp: msidbFileAttributes = msidbFileAttributes(16i32);
pub const msidbFileAttributesReserved1: msidbFileAttributes = msidbFileAttributes(64i32);
pub const msidbFileAttributesReserved2: msidbFileAttributes = msidbFileAttributes(128i32);
pub const msidbFileAttributesReserved3: msidbFileAttributes = msidbFileAttributes(256i32);
pub const msidbFileAttributesVital: msidbFileAttributes = msidbFileAttributes(512i32);
pub const msidbFileAttributesChecksum: msidbFileAttributes = msidbFileAttributes(1024i32);
pub const msidbFileAttributesPatchAdded: msidbFileAttributes = msidbFileAttributes(4096i32);
pub const msidbFileAttributesNoncompressed: msidbFileAttributes = msidbFileAttributes(8192i32);
pub const msidbFileAttributesCompressed: msidbFileAttributes = msidbFileAttributes(16384i32);
pub const msidbFileAttributesReserved4: msidbFileAttributes = msidbFileAttributes(32768i32);
impl ::std::convert::From<i32> for msidbFileAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbFileAttributes {
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
pub struct msidbIniFileAction(pub i32);
pub const msidbIniFileActionAddLine: msidbIniFileAction = msidbIniFileAction(0i32);
pub const msidbIniFileActionCreateLine: msidbIniFileAction = msidbIniFileAction(1i32);
pub const msidbIniFileActionRemoveLine: msidbIniFileAction = msidbIniFileAction(2i32);
pub const msidbIniFileActionAddTag: msidbIniFileAction = msidbIniFileAction(3i32);
pub const msidbIniFileActionRemoveTag: msidbIniFileAction = msidbIniFileAction(4i32);
impl ::std::convert::From<i32> for msidbIniFileAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbIniFileAction {
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
pub struct msidbLocatorType(pub i32);
pub const msidbLocatorTypeDirectory: msidbLocatorType = msidbLocatorType(0i32);
pub const msidbLocatorTypeFileName: msidbLocatorType = msidbLocatorType(1i32);
pub const msidbLocatorTypeRawValue: msidbLocatorType = msidbLocatorType(2i32);
pub const msidbLocatorType64bit: msidbLocatorType = msidbLocatorType(16i32);
impl ::std::convert::From<i32> for msidbLocatorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbLocatorType {
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
pub struct msidbMoveFileOptions(pub i32);
pub const msidbMoveFileOptionsMove: msidbMoveFileOptions = msidbMoveFileOptions(1i32);
impl ::std::convert::From<i32> for msidbMoveFileOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbMoveFileOptions {
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
pub struct msidbODBCDataSourceRegistration(pub i32);
pub const msidbODBCDataSourceRegistrationPerMachine: msidbODBCDataSourceRegistration =
    msidbODBCDataSourceRegistration(0i32);
pub const msidbODBCDataSourceRegistrationPerUser: msidbODBCDataSourceRegistration =
    msidbODBCDataSourceRegistration(1i32);
impl ::std::convert::From<i32> for msidbODBCDataSourceRegistration {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbODBCDataSourceRegistration {
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
pub struct msidbPatchAttributes(pub i32);
pub const msidbPatchAttributesNonVital: msidbPatchAttributes = msidbPatchAttributes(1i32);
impl ::std::convert::From<i32> for msidbPatchAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbPatchAttributes {
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
pub struct msidbRegistryRoot(pub i32);
pub const msidbRegistryRootClassesRoot: msidbRegistryRoot = msidbRegistryRoot(0i32);
pub const msidbRegistryRootCurrentUser: msidbRegistryRoot = msidbRegistryRoot(1i32);
pub const msidbRegistryRootLocalMachine: msidbRegistryRoot = msidbRegistryRoot(2i32);
pub const msidbRegistryRootUsers: msidbRegistryRoot = msidbRegistryRoot(3i32);
impl ::std::convert::From<i32> for msidbRegistryRoot {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbRegistryRoot {
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
pub struct msidbRemoveFileInstallMode(pub i32);
pub const msidbRemoveFileInstallModeOnInstall: msidbRemoveFileInstallMode =
    msidbRemoveFileInstallMode(1i32);
pub const msidbRemoveFileInstallModeOnRemove: msidbRemoveFileInstallMode =
    msidbRemoveFileInstallMode(2i32);
pub const msidbRemoveFileInstallModeOnBoth: msidbRemoveFileInstallMode =
    msidbRemoveFileInstallMode(3i32);
impl ::std::convert::From<i32> for msidbRemoveFileInstallMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbRemoveFileInstallMode {
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
pub struct msidbServiceConfigEvent(pub i32);
pub const msidbServiceConfigEventInstall: msidbServiceConfigEvent = msidbServiceConfigEvent(1i32);
pub const msidbServiceConfigEventUninstall: msidbServiceConfigEvent = msidbServiceConfigEvent(2i32);
pub const msidbServiceConfigEventReinstall: msidbServiceConfigEvent = msidbServiceConfigEvent(4i32);
impl ::std::convert::From<i32> for msidbServiceConfigEvent {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbServiceConfigEvent {
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
pub struct msidbServiceControlEvent(pub i32);
pub const msidbServiceControlEventStart: msidbServiceControlEvent = msidbServiceControlEvent(1i32);
pub const msidbServiceControlEventStop: msidbServiceControlEvent = msidbServiceControlEvent(2i32);
pub const msidbServiceControlEventDelete: msidbServiceControlEvent = msidbServiceControlEvent(8i32);
pub const msidbServiceControlEventUninstallStart: msidbServiceControlEvent =
    msidbServiceControlEvent(16i32);
pub const msidbServiceControlEventUninstallStop: msidbServiceControlEvent =
    msidbServiceControlEvent(32i32);
pub const msidbServiceControlEventUninstallDelete: msidbServiceControlEvent =
    msidbServiceControlEvent(128i32);
impl ::std::convert::From<i32> for msidbServiceControlEvent {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbServiceControlEvent {
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
pub struct msidbServiceInstallErrorControl(pub i32);
pub const msidbServiceInstallErrorControlVital: msidbServiceInstallErrorControl =
    msidbServiceInstallErrorControl(32768i32);
impl ::std::convert::From<i32> for msidbServiceInstallErrorControl {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbServiceInstallErrorControl {
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
pub struct msidbSumInfoSourceType(pub i32);
pub const msidbSumInfoSourceTypeSFN: msidbSumInfoSourceType = msidbSumInfoSourceType(1i32);
pub const msidbSumInfoSourceTypeCompressed: msidbSumInfoSourceType = msidbSumInfoSourceType(2i32);
pub const msidbSumInfoSourceTypeAdminImage: msidbSumInfoSourceType = msidbSumInfoSourceType(4i32);
pub const msidbSumInfoSourceTypeLUAPackage: msidbSumInfoSourceType = msidbSumInfoSourceType(8i32);
impl ::std::convert::From<i32> for msidbSumInfoSourceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbSumInfoSourceType {
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
pub struct msidbTextStyleStyleBits(pub i32);
pub const msidbTextStyleStyleBitsBold: msidbTextStyleStyleBits = msidbTextStyleStyleBits(1i32);
pub const msidbTextStyleStyleBitsItalic: msidbTextStyleStyleBits = msidbTextStyleStyleBits(2i32);
pub const msidbTextStyleStyleBitsUnderline: msidbTextStyleStyleBits = msidbTextStyleStyleBits(4i32);
pub const msidbTextStyleStyleBitsStrike: msidbTextStyleStyleBits = msidbTextStyleStyleBits(8i32);
impl ::std::convert::From<i32> for msidbTextStyleStyleBits {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbTextStyleStyleBits {
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
pub struct msidbUpgradeAttributes(pub i32);
pub const msidbUpgradeAttributesMigrateFeatures: msidbUpgradeAttributes =
    msidbUpgradeAttributes(1i32);
pub const msidbUpgradeAttributesOnlyDetect: msidbUpgradeAttributes = msidbUpgradeAttributes(2i32);
pub const msidbUpgradeAttributesIgnoreRemoveFailure: msidbUpgradeAttributes =
    msidbUpgradeAttributes(4i32);
pub const msidbUpgradeAttributesVersionMinInclusive: msidbUpgradeAttributes =
    msidbUpgradeAttributes(256i32);
pub const msidbUpgradeAttributesVersionMaxInclusive: msidbUpgradeAttributes =
    msidbUpgradeAttributes(512i32);
pub const msidbUpgradeAttributesLanguagesExclusive: msidbUpgradeAttributes =
    msidbUpgradeAttributes(1024i32);
impl ::std::convert::From<i32> for msidbUpgradeAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msidbUpgradeAttributes {
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
pub struct msifiFastInstallBits(pub i32);
pub const msifiFastInstallNoSR: msifiFastInstallBits = msifiFastInstallBits(1i32);
pub const msifiFastInstallQuickCosting: msifiFastInstallBits = msifiFastInstallBits(2i32);
pub const msifiFastInstallLessPrgMsg: msifiFastInstallBits = msifiFastInstallBits(4i32);
impl ::std::convert::From<i32> for msifiFastInstallBits {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msifiFastInstallBits {
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
pub struct msirbRebootReason(pub i32);
pub const msirbRebootUndeterminedReason: msirbRebootReason = msirbRebootReason(0i32);
pub const msirbRebootInUseFilesReason: msirbRebootReason = msirbRebootReason(1i32);
pub const msirbRebootScheduleRebootReason: msirbRebootReason = msirbRebootReason(2i32);
pub const msirbRebootForceRebootReason: msirbRebootReason = msirbRebootReason(3i32);
pub const msirbRebootCustomActionReason: msirbRebootReason = msirbRebootReason(4i32);
impl ::std::convert::From<i32> for msirbRebootReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msirbRebootReason {
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
pub struct msirbRebootType(pub i32);
pub const msirbRebootImmediate: msirbRebootType = msirbRebootType(1i32);
pub const msirbRebootDeferred: msirbRebootType = msirbRebootType(2i32);
impl ::std::convert::From<i32> for msirbRebootType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msirbRebootType {
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
pub struct msmErrorType(pub i32);
pub const msmErrorLanguageUnsupported: msmErrorType = msmErrorType(1i32);
pub const msmErrorLanguageFailed: msmErrorType = msmErrorType(2i32);
pub const msmErrorExclusion: msmErrorType = msmErrorType(3i32);
pub const msmErrorTableMerge: msmErrorType = msmErrorType(4i32);
pub const msmErrorResequenceMerge: msmErrorType = msmErrorType(5i32);
pub const msmErrorFileCreate: msmErrorType = msmErrorType(6i32);
pub const msmErrorDirCreate: msmErrorType = msmErrorType(7i32);
pub const msmErrorFeatureRequired: msmErrorType = msmErrorType(8i32);
impl ::std::convert::From<i32> for msmErrorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for msmErrorType {
    type Abi = Self;
    type DefaultType = Self;
}
