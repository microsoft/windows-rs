pub const D3DCOMPILER_DLL_A: windows_core::PCSTR = windows_core::s!("d3dcompiler_47.dll");
pub const D3DCOMPILER_DLL_W: windows_core::PCWSTR = windows_core::w!("d3dcompiler_47.dll");
pub const D3DCOMPILER_STRIP_DEBUG_INFO: D3DCOMPILER_STRIP_FLAGS = 2i32;
pub const D3DCOMPILER_STRIP_PRIVATE_DATA: D3DCOMPILER_STRIP_FLAGS = 8i32;
pub const D3DCOMPILER_STRIP_REFLECTION_DATA: D3DCOMPILER_STRIP_FLAGS = 1i32;
pub const D3DCOMPILER_STRIP_ROOT_SIGNATURE: D3DCOMPILER_STRIP_FLAGS = 16i32;
pub const D3DCOMPILER_STRIP_TEST_BLOBS: D3DCOMPILER_STRIP_FLAGS = 4i32;
pub const D3DCOMPILE_ALL_RESOURCES_BOUND: u32 = 2097152u32;
pub const D3DCOMPILE_AVOID_FLOW_CONTROL: u32 = 512u32;
pub const D3DCOMPILE_DEBUG: u32 = 1u32;
pub const D3DCOMPILE_DEBUG_NAME_FOR_BINARY: u32 = 8388608u32;
pub const D3DCOMPILE_DEBUG_NAME_FOR_SOURCE: u32 = 4194304u32;
pub const D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS: u32 = 2u32;
pub const D3DCOMPILE_EFFECT_CHILD_EFFECT: u32 = 1u32;
pub const D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096u32;
pub const D3DCOMPILE_ENABLE_STRICTNESS: u32 = 2048u32;
pub const D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576u32;
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16u32;
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32u32;
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0u32;
pub const D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128u32;
pub const D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64u32;
pub const D3DCOMPILE_IEEE_STRICTNESS: u32 = 8192u32;
pub const D3DCOMPILE_NO_PRESHADER: u32 = 256u32;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL0: u32 = 16384u32;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL1: u32 = 0u32;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL3: u32 = 32768u32;
pub const D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR: u32 = 16u32;
pub const D3DCOMPILE_PACK_MATRIX_ROW_MAJOR: u32 = 8u32;
pub const D3DCOMPILE_PARTIAL_PRECISION: u32 = 32u32;
pub const D3DCOMPILE_PREFER_FLOW_CONTROL: u32 = 1024u32;
pub const D3DCOMPILE_RESERVED16: u32 = 65536u32;
pub const D3DCOMPILE_RESERVED17: u32 = 131072u32;
pub const D3DCOMPILE_RESOURCES_MAY_ALIAS: u32 = 524288u32;
pub const D3DCOMPILE_SECDATA_MERGE_UAV_SLOTS: u32 = 1u32;
pub const D3DCOMPILE_SECDATA_PRESERVE_TEMPLATE_SLOTS: u32 = 2u32;
pub const D3DCOMPILE_SECDATA_REQUIRE_TEMPLATE_MATCH: u32 = 4u32;
pub const D3DCOMPILE_SKIP_OPTIMIZATION: u32 = 4u32;
pub const D3DCOMPILE_SKIP_VALIDATION: u32 = 2u32;
pub const D3DCOMPILE_WARNINGS_ARE_ERRORS: u32 = 262144u32;
pub const D3D_BLOB_ALL_SIGNATURE_BLOB: D3D_BLOB_PART = 4i32;
pub const D3D_BLOB_DEBUG_INFO: D3D_BLOB_PART = 5i32;
pub const D3D_BLOB_DEBUG_NAME: D3D_BLOB_PART = 12i32;
pub const D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB: D3D_BLOB_PART = 2i32;
pub const D3D_BLOB_INPUT_SIGNATURE_BLOB: D3D_BLOB_PART = 0i32;
pub const D3D_BLOB_LEGACY_SHADER: D3D_BLOB_PART = 6i32;
pub const D3D_BLOB_OUTPUT_SIGNATURE_BLOB: D3D_BLOB_PART = 1i32;
pub const D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB: D3D_BLOB_PART = 3i32;
pub const D3D_BLOB_PDB: D3D_BLOB_PART = 9i32;
pub const D3D_BLOB_PRIVATE_DATA: D3D_BLOB_PART = 10i32;
pub const D3D_BLOB_ROOT_SIGNATURE: D3D_BLOB_PART = 11i32;
pub const D3D_BLOB_TEST_ALTERNATE_SHADER: D3D_BLOB_PART = 32768i32;
pub const D3D_BLOB_TEST_COMPILE_DETAILS: D3D_BLOB_PART = 32769i32;
pub const D3D_BLOB_TEST_COMPILE_PERF: D3D_BLOB_PART = 32770i32;
pub const D3D_BLOB_TEST_COMPILE_REPORT: D3D_BLOB_PART = 32771i32;
pub const D3D_BLOB_XNA_PREPASS_SHADER: D3D_BLOB_PART = 7i32;
pub const D3D_BLOB_XNA_SHADER: D3D_BLOB_PART = 8i32;
pub const D3D_COMPILER_VERSION: u32 = 47u32;
pub const D3D_COMPRESS_SHADER_KEEP_ALL_PARTS: u32 = 1u32;
pub const D3D_DISASM_DISABLE_DEBUG_INFO: u32 = 16u32;
pub const D3D_DISASM_ENABLE_COLOR_CODE: u32 = 1u32;
pub const D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS: u32 = 2u32;
pub const D3D_DISASM_ENABLE_INSTRUCTION_CYCLE: u32 = 8u32;
pub const D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING: u32 = 4u32;
pub const D3D_DISASM_ENABLE_INSTRUCTION_OFFSET: u32 = 32u32;
pub const D3D_DISASM_INSTRUCTION_ONLY: u32 = 64u32;
pub const D3D_DISASM_PRINT_HEX_LITERALS: u32 = 128u32;
pub const D3D_GET_INST_OFFSETS_INCLUDE_NON_EXECUTABLE: u32 = 1u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct D3DCOMPILER_STRIP_FLAGS(pub i32);
impl windows_core::TypeKind for D3DCOMPILER_STRIP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct D3D_BLOB_PART(pub i32);
impl windows_core::TypeKind for D3D_BLOB_PART {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D_SHADER_DATA {
    pub pBytecode: *const core::ffi::c_void,
    pub BytecodeLength: usize,
}
impl Default for D3D_SHADER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for D3D_SHADER_DATA {
    type TypeKind = windows_core::CopyType;
}
pub type pD3DCompile = Option<unsafe extern "system" fn(psrcdata: *const core::ffi::c_void, srcdatasize: usize, pfilename: windows_core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: Option<super::ID3DInclude>, pentrypoint: windows_core::PCSTR, ptarget: windows_core::PCSTR, flags1: u32, flags2: u32, ppcode: *mut Option<super::ID3DBlob>, pperrormsgs: *mut Option<super::ID3DBlob>) -> windows_core::HRESULT>;
pub type pD3DDisassemble = Option<unsafe extern "system" fn(psrcdata: *const core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: windows_core::PCSTR, ppdisassembly: *mut Option<super::ID3DBlob>) -> windows_core::HRESULT>;
pub type pD3DPreprocess = Option<unsafe extern "system" fn(psrcdata: *const core::ffi::c_void, srcdatasize: usize, pfilename: windows_core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: Option<super::ID3DInclude>, ppcodetext: *mut Option<super::ID3DBlob>, pperrormsgs: *mut Option<super::ID3DBlob>) -> windows_core::HRESULT>;
