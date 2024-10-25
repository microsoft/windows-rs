pub const CLSID_DxcAssembler: windows_core::GUID = windows_core::GUID::from_u128(0xd728db68_f903_4f80_94cd_dccf76ec7151);
pub const CLSID_DxcCompiler: windows_core::GUID = windows_core::GUID::from_u128(0x73e22d93_e6ce_47f3_b5bf_f0664f39c1b0);
pub const CLSID_DxcCompilerArgs: windows_core::GUID = windows_core::GUID::from_u128(0x3e56ae82_224d_470f_a1a1_fe3016ee9f9d);
pub const CLSID_DxcContainerBuilder: windows_core::GUID = windows_core::GUID::from_u128(0x94134294_411f_4574_b4d0_8741e25240d2);
pub const CLSID_DxcContainerReflection: windows_core::GUID = windows_core::GUID::from_u128(0xb9f54489_55b8_400c_ba3a_1675e4728b91);
pub const CLSID_DxcDiaDataSource: windows_core::GUID = windows_core::GUID::from_u128(0xcd1f6b73_2ab0_484d_8edc_ebe7a43ca09f);
pub const CLSID_DxcLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x6245d6af_66e0_48fd_80b4_4d271796748c);
pub const CLSID_DxcLinker: windows_core::GUID = windows_core::GUID::from_u128(0xef6a8087_b0ea_4d56_9e45_d07e1a8b7806);
pub const CLSID_DxcOptimizer: windows_core::GUID = windows_core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
pub const CLSID_DxcPdbUtils: windows_core::GUID = windows_core::GUID::from_u128(0x54621dfb_f2ce_457e_ae8c_ec355faeec7c);
pub const CLSID_DxcUtils: windows_core::GUID = windows_core::GUID::from_u128(0x6245d6af_66e0_48fd_80b4_4d271796748c);
pub const CLSID_DxcValidator: windows_core::GUID = windows_core::GUID::from_u128(0x8ca3e215_f728_4cf3_8cdd_88af917587a1);
pub const DXC_ARG_ALL_RESOURCES_BOUND: windows_core::PCWSTR = windows_core::w!("-all_resources_bound");
pub const DXC_ARG_AVOID_FLOW_CONTROL: windows_core::PCWSTR = windows_core::w!("-Gfa");
pub const DXC_ARG_DEBUG: windows_core::PCWSTR = windows_core::w!("-Zi");
pub const DXC_ARG_DEBUG_NAME_FOR_BINARY: windows_core::PCWSTR = windows_core::w!("-Zsb");
pub const DXC_ARG_DEBUG_NAME_FOR_SOURCE: windows_core::PCWSTR = windows_core::w!("-Zss");
pub const DXC_ARG_ENABLE_BACKWARDS_COMPATIBILITY: windows_core::PCWSTR = windows_core::w!("-Gec");
pub const DXC_ARG_ENABLE_STRICTNESS: windows_core::PCWSTR = windows_core::w!("-Ges");
pub const DXC_ARG_IEEE_STRICTNESS: windows_core::PCWSTR = windows_core::w!("-Gis");
pub const DXC_ARG_OPTIMIZATION_LEVEL0: windows_core::PCWSTR = windows_core::w!("-O0");
pub const DXC_ARG_OPTIMIZATION_LEVEL1: windows_core::PCWSTR = windows_core::w!("-O1");
pub const DXC_ARG_OPTIMIZATION_LEVEL2: windows_core::PCWSTR = windows_core::w!("-O2");
pub const DXC_ARG_OPTIMIZATION_LEVEL3: windows_core::PCWSTR = windows_core::w!("-O3");
pub const DXC_ARG_PACK_MATRIX_COLUMN_MAJOR: windows_core::PCWSTR = windows_core::w!("-Zpc");
pub const DXC_ARG_PACK_MATRIX_ROW_MAJOR: windows_core::PCWSTR = windows_core::w!("-Zpr");
pub const DXC_ARG_PREFER_FLOW_CONTROL: windows_core::PCWSTR = windows_core::w!("-Gfp");
pub const DXC_ARG_RESOURCES_MAY_ALIAS: windows_core::PCWSTR = windows_core::w!("-res_may_alias");
pub const DXC_ARG_SKIP_OPTIMIZATIONS: windows_core::PCWSTR = windows_core::w!("-Od");
pub const DXC_ARG_SKIP_VALIDATION: windows_core::PCWSTR = windows_core::w!("-Vd");
pub const DXC_ARG_WARNINGS_ARE_ERRORS: windows_core::PCWSTR = windows_core::w!("-WX");
pub const DXC_CP_ACP: DXC_CP = 0u32;
pub const DXC_CP_UTF16: DXC_CP = 1200u32;
pub const DXC_CP_UTF8: DXC_CP = 65001u32;
pub const DXC_EXTRA_OUTPUT_NAME_STDERR: windows_core::PCWSTR = windows_core::w!("*stderr*");
pub const DXC_EXTRA_OUTPUT_NAME_STDOUT: windows_core::PCWSTR = windows_core::w!("*stdout*");
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = 5i32;
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = 2i32;
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = 10i32;
pub const DXC_OUT_HLSL: DXC_OUT_KIND = 6i32;
pub const DXC_OUT_NONE: DXC_OUT_KIND = 0i32;
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = 1i32;
pub const DXC_OUT_PDB: DXC_OUT_KIND = 3i32;
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = 8i32;
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = 9i32;
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = 4i32;
pub const DXC_OUT_TEXT: DXC_OUT_KIND = 7i32;
pub const DxcValidatorFlags_Default: u32 = 0u32;
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
pub const DxcVersionInfoFlags_None: u32 = 0u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DXC_CP(pub u32);
impl windows_core::TypeKind for DXC_CP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DXC_OUT_KIND(pub i32);
impl windows_core::TypeKind for DXC_OUT_KIND {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DxcArgPair {
    pub pName: windows_core::PCWSTR,
    pub pValue: windows_core::PCWSTR,
}
impl Default for DxcArgPair {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DxcArgPair {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DxcBuffer {
    pub Ptr: *const core::ffi::c_void,
    pub Size: usize,
    pub Encoding: u32,
}
impl Default for DxcBuffer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DxcBuffer {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DxcDefine {
    pub Name: windows_core::PCWSTR,
    pub Value: windows_core::PCWSTR,
}
impl Default for DxcDefine {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DxcDefine {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DxcShaderHash {
    pub Flags: u32,
    pub HashDigest: [u8; 16],
}
impl Default for DxcShaderHash {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DxcShaderHash {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
pub type DxcCreateInstance2Proc = Option<unsafe extern "system" fn(pmalloc: Option<super::super::super::System::Com::IMalloc>, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type DxcCreateInstanceProc = Option<unsafe extern "system" fn(rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
