#[cfg(all(feature = "guiddef", feature = "minwindef"))]
windows_link::link!("dxcompiler.dll" "system" fn DxcCreateInstance(rclsid : *const super::IID, riid : *const super::IID, ppv : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase"))]
windows_link::link!("dxcompiler.dll" "system" fn DxcCreateInstance2(pmalloc : *mut core::ffi::c_void, rclsid : *const super::IID, riid : *const super::IID, ppv : *mut super::LPVOID) -> windows_sys::core::HRESULT);
pub const CLSID_DxcAssembler: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd728db68_f903_4f80_94cd_dccf76ec7151);
pub const CLSID_DxcCompiler: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x73e22d93_e6ce_47f3_b5bf_f0664f39c1b0);
pub const CLSID_DxcCompilerArgs: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3e56ae82_224d_470f_a1a1_fe3016ee9f9d);
pub const CLSID_DxcContainerBuilder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x94134294_411f_4574_b4d0_8741e25240d2);
pub const CLSID_DxcContainerReflection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb9f54489_55b8_400c_ba3a_1675e4728b91);
pub const CLSID_DxcDiaDataSource: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcd1f6b73_2ab0_484d_8edc_ebe7a43ca09f);
pub const CLSID_DxcLibrary: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6245d6af_66e0_48fd_80b4_4d271796748c);
pub const CLSID_DxcLinker: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef6a8087_b0ea_4d56_9e45_d07e1a8b7806);
pub const CLSID_DxcOptimizer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
pub const CLSID_DxcPdbUtils: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x54621dfb_f2ce_457e_ae8c_ec355faeec7c);
pub const CLSID_DxcValidator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8ca3e215_f728_4cf3_8cdd_88af917587a1);
pub const DXC_ARG_ALL_RESOURCES_BOUND: windows_sys::core::PCWSTR = windows_sys::core::w!("-all_resources_bound");
pub const DXC_ARG_AVOID_FLOW_CONTROL: windows_sys::core::PCWSTR = windows_sys::core::w!("-Gfa");
pub const DXC_ARG_DEBUG: windows_sys::core::PCWSTR = windows_sys::core::w!("-Zi");
pub const DXC_ARG_DEBUG_NAME_FOR_BINARY: windows_sys::core::PCWSTR = windows_sys::core::w!("-Zsb");
pub const DXC_ARG_DEBUG_NAME_FOR_SOURCE: windows_sys::core::PCWSTR = windows_sys::core::w!("-Zss");
pub const DXC_ARG_ENABLE_BACKWARDS_COMPATIBILITY: windows_sys::core::PCWSTR = windows_sys::core::w!("-Gec");
pub const DXC_ARG_ENABLE_STRICTNESS: windows_sys::core::PCWSTR = windows_sys::core::w!("-Ges");
pub const DXC_ARG_IEEE_STRICTNESS: windows_sys::core::PCWSTR = windows_sys::core::w!("-Gis");
pub const DXC_ARG_OPTIMIZATION_LEVEL0: windows_sys::core::PCWSTR = windows_sys::core::w!("-O0");
pub const DXC_ARG_OPTIMIZATION_LEVEL1: windows_sys::core::PCWSTR = windows_sys::core::w!("-O1");
pub const DXC_ARG_OPTIMIZATION_LEVEL2: windows_sys::core::PCWSTR = windows_sys::core::w!("-O2");
pub const DXC_ARG_OPTIMIZATION_LEVEL3: windows_sys::core::PCWSTR = windows_sys::core::w!("-O3");
pub const DXC_ARG_PACK_MATRIX_COLUMN_MAJOR: windows_sys::core::PCWSTR = windows_sys::core::w!("-Zpc");
pub const DXC_ARG_PACK_MATRIX_ROW_MAJOR: windows_sys::core::PCWSTR = windows_sys::core::w!("-Zpr");
pub const DXC_ARG_PREFER_FLOW_CONTROL: windows_sys::core::PCWSTR = windows_sys::core::w!("-Gfp");
pub const DXC_ARG_RESOURCES_MAY_ALIAS: windows_sys::core::PCWSTR = windows_sys::core::w!("-res_may_alias");
pub const DXC_ARG_SKIP_OPTIMIZATIONS: windows_sys::core::PCWSTR = windows_sys::core::w!("-Od");
pub const DXC_ARG_SKIP_VALIDATION: windows_sys::core::PCWSTR = windows_sys::core::w!("-Vd");
pub const DXC_ARG_WARNINGS_ARE_ERRORS: windows_sys::core::PCWSTR = windows_sys::core::w!("-WX");
pub const DXC_CP_ACP: i32 = 0;
pub const DXC_CP_UTF16: i32 = 1200;
pub const DXC_CP_UTF32: i32 = 12000;
pub const DXC_CP_UTF8: i32 = 65001;
pub const DXC_CP_WIDE: i32 = 1200;
pub const DXC_EXTRA_OUTPUT_NAME_STDERR: windows_sys::core::PCWSTR = windows_sys::core::w!("*stderr*");
pub const DXC_EXTRA_OUTPUT_NAME_STDOUT: windows_sys::core::PCWSTR = windows_sys::core::w!("*stdout*");
pub const DXC_HASHFLAG_INCLUDES_SOURCE: i32 = 1;
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = 5;
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = 2;
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = 10;
pub const DXC_OUT_FORCE_DWORD: DXC_OUT_KIND = -1;
pub const DXC_OUT_HLSL: DXC_OUT_KIND = 6;
pub type DXC_OUT_KIND = i32;
pub const DXC_OUT_LAST: DXC_OUT_KIND = 13;
pub const DXC_OUT_NONE: DXC_OUT_KIND = 0;
pub const DXC_OUT_NUM_ENUMS: DXC_OUT_KIND = 14;
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = 1;
pub const DXC_OUT_PDB: DXC_OUT_KIND = 3;
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = 8;
pub const DXC_OUT_REMARKS: DXC_OUT_KIND = 11;
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = 9;
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = 4;
pub const DXC_OUT_TEXT: DXC_OUT_KIND = 7;
pub const DXC_OUT_TIME_REPORT: DXC_OUT_KIND = 12;
pub const DXC_OUT_TIME_TRACE: DXC_OUT_KIND = 13;
#[cfg(feature = "shared")]
pub const DXC_PART_DXIL: super::UINT32 = 1279875140;
#[cfg(feature = "shared")]
pub const DXC_PART_INPUT_SIGNATURE: super::UINT32 = 826757961;
#[cfg(feature = "shared")]
pub const DXC_PART_OUTPUT_SIGNATURE: super::UINT32 = 826757967;
#[cfg(feature = "shared")]
pub const DXC_PART_PATCH_CONSTANT_SIGNATURE: super::UINT32 = 826757968;
#[cfg(feature = "shared")]
pub const DXC_PART_PDB: super::UINT32 = 1111772233;
#[cfg(feature = "shared")]
pub const DXC_PART_PDB_NAME: super::UINT32 = 1313098825;
#[cfg(feature = "shared")]
pub const DXC_PART_PRIVATE_DATA: super::UINT32 = 1447645776;
#[cfg(feature = "shared")]
pub const DXC_PART_REFLECTION_DATA: super::UINT32 = 1413567571;
#[cfg(feature = "shared")]
pub const DXC_PART_ROOT_SIGNATURE: super::UINT32 = 810767442;
#[cfg(feature = "shared")]
pub const DXC_PART_SHADER_HASH: super::UINT32 = 1213415752;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DxcArgPair {
    pub pName: *const super::WCHAR,
    pub pValue: *const super::WCHAR,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase"))]
pub type DxcCreateInstance2Proc = Option<unsafe extern "system" fn(pmalloc: *mut core::ffi::c_void, rclsid: *const super::IID, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
pub type DxcCreateInstanceProc = Option<unsafe extern "system" fn(rclsid: *const super::IID, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DxcDefine {
    pub Name: windows_sys::core::PCWSTR,
    pub Value: windows_sys::core::PCWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DxcShaderHash {
    pub Flags: super::UINT32,
    pub HashDigest: [super::BYTE; 16],
}
#[cfg(feature = "shared")]
impl Default for DxcShaderHash {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DxcText {
    pub Ptr: super::LPCVOID,
    pub Size: super::SIZE_T,
    pub Encoding: super::UINT,
}
#[cfg(feature = "shared")]
pub const DxcValidatorFlags_Default: super::UINT32 = 0;
#[cfg(feature = "shared")]
pub const DxcValidatorFlags_InPlaceEdit: super::UINT32 = 1;
#[cfg(feature = "shared")]
pub const DxcValidatorFlags_ModuleOnly: super::UINT32 = 4;
#[cfg(feature = "shared")]
pub const DxcValidatorFlags_RootSignatureOnly: super::UINT32 = 2;
#[cfg(feature = "shared")]
pub const DxcValidatorFlags_ValidMask: super::UINT32 = 7;
#[cfg(feature = "shared")]
pub const DxcVersionInfoFlags_Debug: super::UINT32 = 1;
#[cfg(feature = "shared")]
pub const DxcVersionInfoFlags_Internal: super::UINT32 = 2;
#[cfg(feature = "shared")]
pub const DxcVersionInfoFlags_None: super::UINT32 = 0;
pub type IDxcBlobUtf16 = *mut core::ffi::c_void;
