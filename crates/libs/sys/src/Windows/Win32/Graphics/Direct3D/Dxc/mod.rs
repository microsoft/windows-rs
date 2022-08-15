#[cfg_attr(windows, link(name = "windows"))]
extern "system" {
    pub fn DxcCreateInstance(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn DxcCreateInstance2(pmalloc: super::super::super::System::Com::IMalloc, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
pub const CLSID_DxcAssembler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3609779048, data2: 63747, data3: 20352, data4: [148, 205, 220, 207, 118, 236, 113, 81] };
pub const CLSID_DxcCompiler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1944202643, data2: 59086, data3: 18419, data4: [181, 191, 240, 102, 79, 57, 193, 176] };
pub const CLSID_DxcCompilerArgs: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1045868162, data2: 8781, data3: 18191, data4: [161, 161, 254, 48, 22, 238, 159, 157] };
pub const CLSID_DxcContainerBuilder: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2484290196, data2: 16671, data3: 17780, data4: [180, 208, 135, 65, 226, 82, 64, 210] };
pub const CLSID_DxcContainerReflection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3119858825, data2: 21944, data3: 16396, data4: [186, 58, 22, 117, 228, 114, 139, 145] };
pub const CLSID_DxcDiaDataSource: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3441388403, data2: 10928, data3: 18509, data4: [142, 220, 235, 231, 164, 60, 160, 159] };
pub const CLSID_DxcLibrary: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1648744111, data2: 26336, data3: 18685, data4: [128, 180, 77, 39, 23, 150, 116, 140] };
pub const CLSID_DxcLinker: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4016734343, data2: 45290, data3: 19798, data4: [158, 69, 208, 126, 26, 139, 120, 6] };
pub const CLSID_DxcOptimizer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2922174367, data2: 52258, data3: 17727, data4: [155, 107, 177, 36, 231, 165, 32, 76] };
pub const CLSID_DxcPdbUtils: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1415716347, data2: 62158, data3: 17790, data4: [174, 140, 236, 53, 95, 174, 236, 124] };
pub const CLSID_DxcValidator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2359550485, data2: 63272, data3: 19699, data4: [140, 221, 136, 175, 145, 117, 135, 161] };
pub const DXC_ARG_ALL_RESOURCES_BOUND: &str = "-all_resources_bound";
pub const DXC_ARG_AVOID_FLOW_CONTROL: &str = "-Gfa";
pub const DXC_ARG_DEBUG: &str = "-Zi";
pub const DXC_ARG_DEBUG_NAME_FOR_BINARY: &str = "-Zsb";
pub const DXC_ARG_DEBUG_NAME_FOR_SOURCE: &str = "-Zss";
pub const DXC_ARG_ENABLE_BACKWARDS_COMPATIBILITY: &str = "-Gec";
pub const DXC_ARG_ENABLE_STRICTNESS: &str = "-Ges";
pub const DXC_ARG_IEEE_STRICTNESS: &str = "-Gis";
pub const DXC_ARG_OPTIMIZATION_LEVEL0: &str = "-O0";
pub const DXC_ARG_OPTIMIZATION_LEVEL1: &str = "-O1";
pub const DXC_ARG_OPTIMIZATION_LEVEL2: &str = "-O2";
pub const DXC_ARG_OPTIMIZATION_LEVEL3: &str = "-O3";
pub const DXC_ARG_PACK_MATRIX_COLUMN_MAJOR: &str = "-Zpc";
pub const DXC_ARG_PACK_MATRIX_ROW_MAJOR: &str = "-Zpr";
pub const DXC_ARG_PREFER_FLOW_CONTROL: &str = "-Gfp";
pub const DXC_ARG_RESOURCES_MAY_ALIAS: &str = "-res_may_alias";
pub const DXC_ARG_SKIP_OPTIMIZATIONS: &str = "-Od";
pub const DXC_ARG_SKIP_VALIDATION: &str = "-Vd";
pub const DXC_ARG_WARNINGS_ARE_ERRORS: &str = "-WX";
pub type DXC_CP = u32;
pub const DXC_CP_ACP: DXC_CP = 0u32;
pub const DXC_CP_UTF16: DXC_CP = 1200u32;
pub const DXC_CP_UTF8: DXC_CP = 65001u32;
pub const DXC_EXTRA_OUTPUT_NAME_STDERR: &str = "*stderr*";
pub const DXC_EXTRA_OUTPUT_NAME_STDOUT: &str = "*stdout*";
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
pub type DXC_OUT_KIND = i32;
pub const DXC_OUT_NONE: DXC_OUT_KIND = 0i32;
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = 1i32;
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = 2i32;
pub const DXC_OUT_PDB: DXC_OUT_KIND = 3i32;
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = 4i32;
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = 5i32;
pub const DXC_OUT_HLSL: DXC_OUT_KIND = 6i32;
pub const DXC_OUT_TEXT: DXC_OUT_KIND = 7i32;
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = 8i32;
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = 9i32;
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = 10i32;
pub const DXC_OUT_FORCE_DWORD: DXC_OUT_KIND = -1i32;
#[repr(C)]
pub struct DxcArgPair {
    pub pName: ::windows_sys::core::PCWSTR,
    pub pValue: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for DxcArgPair {}
impl ::core::clone::Clone for DxcArgPair {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DxcBuffer {
    pub Ptr: *const ::core::ffi::c_void,
    pub Size: usize,
    pub Encoding: u32,
}
impl ::core::marker::Copy for DxcBuffer {}
impl ::core::clone::Clone for DxcBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
pub type DxcCreateInstance2Proc = ::core::option::Option<unsafe extern "system" fn(pmalloc: super::super::super::System::Com::IMalloc, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
pub type DxcCreateInstanceProc = ::core::option::Option<unsafe extern "system" fn(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[repr(C)]
pub struct DxcDefine {
    pub Name: ::windows_sys::core::PCWSTR,
    pub Value: ::windows_sys::core::PCWSTR,
}
impl ::core::marker::Copy for DxcDefine {}
impl ::core::clone::Clone for DxcDefine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DxcShaderHash {
    pub Flags: u32,
    pub HashDigest: [u8; 16],
}
impl ::core::marker::Copy for DxcShaderHash {}
impl ::core::clone::Clone for DxcShaderHash {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DxcValidatorFlags_Default: u32 = 0u32;
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
pub const DxcVersionInfoFlags_None: u32 = 0u32;
pub type IDxcAssembler = *mut ::core::ffi::c_void;
pub type IDxcBlob = *mut ::core::ffi::c_void;
pub type IDxcBlobEncoding = *mut ::core::ffi::c_void;
pub type IDxcBlobUtf16 = *mut ::core::ffi::c_void;
pub type IDxcBlobUtf8 = *mut ::core::ffi::c_void;
pub type IDxcCompiler = *mut ::core::ffi::c_void;
pub type IDxcCompiler2 = *mut ::core::ffi::c_void;
pub type IDxcCompiler3 = *mut ::core::ffi::c_void;
pub type IDxcCompilerArgs = *mut ::core::ffi::c_void;
pub type IDxcContainerBuilder = *mut ::core::ffi::c_void;
pub type IDxcContainerReflection = *mut ::core::ffi::c_void;
pub type IDxcExtraOutputs = *mut ::core::ffi::c_void;
pub type IDxcIncludeHandler = *mut ::core::ffi::c_void;
pub type IDxcLibrary = *mut ::core::ffi::c_void;
pub type IDxcLinker = *mut ::core::ffi::c_void;
pub type IDxcOperationResult = *mut ::core::ffi::c_void;
pub type IDxcOptimizer = *mut ::core::ffi::c_void;
pub type IDxcOptimizerPass = *mut ::core::ffi::c_void;
pub type IDxcPdbUtils = *mut ::core::ffi::c_void;
pub type IDxcResult = *mut ::core::ffi::c_void;
pub type IDxcUtils = *mut ::core::ffi::c_void;
pub type IDxcValidator = *mut ::core::ffi::c_void;
pub type IDxcValidator2 = *mut ::core::ffi::c_void;
pub type IDxcVersionInfo = *mut ::core::ffi::c_void;
pub type IDxcVersionInfo2 = *mut ::core::ffi::c_void;
pub type IDxcVersionInfo3 = *mut ::core::ffi::c_void;
