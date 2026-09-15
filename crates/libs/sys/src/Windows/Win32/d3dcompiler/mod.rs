#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DCompile(psrcdata : super::LPCVOID, srcdatasize : usize, psourcename : windows_sys::core::PCSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pentrypoint : windows_sys::core::PCSTR, ptarget : windows_sys::core::PCSTR, flags1 : u32, flags2 : u32, ppcode : *mut super::ID3DBlob, pperrormsgs : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DCompile2(psrcdata : super::LPCVOID, srcdatasize : usize, psourcename : windows_sys::core::PCSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pentrypoint : windows_sys::core::PCSTR, ptarget : windows_sys::core::PCSTR, flags1 : u32, flags2 : u32, secondarydataflags : u32, psecondarydata : super::LPCVOID, secondarydatasize : usize, ppcode : *mut super::ID3DBlob, pperrormsgs : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DCompileFromFile(pfilename : windows_sys::core::PCWSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pentrypoint : windows_sys::core::PCSTR, ptarget : windows_sys::core::PCSTR, flags1 : u32, flags2 : u32, ppcode : *mut super::ID3DBlob, pperrormsgs : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DCompressShaders(unumshaders : u32, pshaderdata : *const D3D_SHADER_DATA, uflags : u32, ppcompresseddata : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DCreateBlob(size : usize, ppblob : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3d11")]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DCreateFunctionLinkingGraph(uflags : u32, ppfunctionlinkinggraph : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3d11")]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DCreateLinker(pplinker : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DDecompressShaders(psrcdata : super::LPCVOID, srcdatasize : usize, unumshaders : u32, ustartindex : u32, pindices : *const u32, uflags : u32, ppshaders : *mut super::ID3DBlob, ptotalshaders : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DDisassemble(psrcdata : super::LPCVOID, srcdatasize : usize, flags : u32, szcomments : windows_sys::core::PCSTR, ppdisassembly : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3d10", feature = "d3dcommon"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DDisassemble10Effect(peffect : *mut core::ffi::c_void, flags : u32, ppdisassembly : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DDisassembleRegion(psrcdata : super::LPCVOID, srcdatasize : usize, flags : u32, szcomments : windows_sys::core::PCSTR, startbyteoffset : usize, numinsts : usize, pfinishbyteoffset : *mut usize, ppdisassembly : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DGetBlobPart(psrcdata : super::LPCVOID, srcdatasize : usize, part : D3D_BLOB_PART, flags : u32, pppart : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DGetDebugInfo(psrcdata : super::LPCVOID, srcdatasize : usize, ppdebuginfo : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DGetInputAndOutputSignatureBlob(psrcdata : super::LPCVOID, srcdatasize : usize, ppsignatureblob : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DGetInputSignatureBlob(psrcdata : super::LPCVOID, srcdatasize : usize, ppsignatureblob : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DGetOutputSignatureBlob(psrcdata : super::LPCVOID, srcdatasize : usize, ppsignatureblob : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DGetTraceInstructionOffsets(psrcdata : super::LPCVOID, srcdatasize : usize, flags : u32, startinstindex : usize, numinsts : usize, poffsets : *mut usize, ptotalinsts : *mut usize) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3d11", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DLoadModule(psrcdata : super::LPCVOID, cbsrcdatasize : usize, ppmodule : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DPreprocess(psrcdata : super::LPCVOID, srcdatasize : usize, psourcename : windows_sys::core::PCSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, ppcodetext : *mut super::ID3DBlob, pperrormsgs : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DReadFileToBlob(pfilename : windows_sys::core::PCWSTR, ppcontents : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DReflect(psrcdata : super::LPCVOID, srcdatasize : usize, pinterface : *const windows_sys::core::GUID, ppreflector : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DReflectLibrary(psrcdata : super::LPCVOID, srcdatasize : usize, riid : *const windows_sys::core::GUID, ppreflector : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DSetBlobPart(psrcdata : super::LPCVOID, srcdatasize : usize, part : D3D_BLOB_PART, flags : u32, ppart : super::LPCVOID, partsize : usize, ppnewshader : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DStripShader(pshaderbytecode : super::LPCVOID, bytecodelength : usize, ustripflags : u32, ppstrippedblob : *mut super::ID3DBlob) -> windows_sys::core::HRESULT);
#[cfg(feature = "d3dcommon")]
windows_link::link!("d3dcompiler_47.dll" "system" fn D3DWriteBlobToFile(pblob : super::ID3DBlob, pfilename : windows_sys::core::PCWSTR, boverwrite : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
pub const D3DCOMPILER_DLL: windows_sys::core::PCSTR = windows_sys::core::s!("d3dcompiler_47.dll");
pub const D3DCOMPILER_DLL_A: windows_sys::core::PCSTR = windows_sys::core::s!("d3dcompiler_47.dll");
pub const D3DCOMPILER_DLL_W: windows_sys::core::PCWSTR = windows_sys::core::w!("d3dcompiler_47.dll");
pub const D3DCOMPILER_STRIP_DEBUG_INFO: D3DCOMPILER_STRIP_FLAGS = 2;
pub type D3DCOMPILER_STRIP_FLAGS = i32;
pub const D3DCOMPILER_STRIP_FORCE_DWORD: D3DCOMPILER_STRIP_FLAGS = 2147483647;
pub const D3DCOMPILER_STRIP_PRIVATE_DATA: D3DCOMPILER_STRIP_FLAGS = 8;
pub const D3DCOMPILER_STRIP_REFLECTION_DATA: D3DCOMPILER_STRIP_FLAGS = 1;
pub const D3DCOMPILER_STRIP_ROOT_SIGNATURE: D3DCOMPILER_STRIP_FLAGS = 16;
pub const D3DCOMPILER_STRIP_TEST_BLOBS: D3DCOMPILER_STRIP_FLAGS = 4;
pub const D3DCOMPILE_ALL_RESOURCES_BOUND: i32 = 2097152;
pub const D3DCOMPILE_AVOID_FLOW_CONTROL: i32 = 512;
pub const D3DCOMPILE_DEBUG: i32 = 1;
pub const D3DCOMPILE_DEBUG_NAME_FOR_BINARY: i32 = 8388608;
pub const D3DCOMPILE_DEBUG_NAME_FOR_SOURCE: i32 = 4194304;
pub const D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS: i32 = 2;
pub const D3DCOMPILE_EFFECT_CHILD_EFFECT: i32 = 1;
pub const D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY: i32 = 4096;
pub const D3DCOMPILE_ENABLE_STRICTNESS: i32 = 2048;
pub const D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: i32 = 1048576;
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: i32 = 16;
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: i32 = 32;
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: i32 = 0;
pub const D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT: i32 = 128;
pub const D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT: i32 = 64;
pub const D3DCOMPILE_IEEE_STRICTNESS: i32 = 8192;
pub const D3DCOMPILE_NO_PRESHADER: i32 = 256;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL0: i32 = 16384;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL1: i32 = 0;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL2: i32 = 49152;
pub const D3DCOMPILE_OPTIMIZATION_LEVEL3: i32 = 32768;
pub const D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR: i32 = 16;
pub const D3DCOMPILE_PACK_MATRIX_ROW_MAJOR: i32 = 8;
pub const D3DCOMPILE_PARTIAL_PRECISION: i32 = 32;
pub const D3DCOMPILE_PREFER_FLOW_CONTROL: i32 = 1024;
pub const D3DCOMPILE_RESERVED16: i32 = 65536;
pub const D3DCOMPILE_RESERVED17: i32 = 131072;
pub const D3DCOMPILE_RESOURCES_MAY_ALIAS: i32 = 524288;
pub const D3DCOMPILE_SECDATA_MERGE_UAV_SLOTS: i32 = 1;
pub const D3DCOMPILE_SECDATA_PRESERVE_TEMPLATE_SLOTS: i32 = 2;
pub const D3DCOMPILE_SECDATA_REQUIRE_TEMPLATE_MATCH: i32 = 4;
pub const D3DCOMPILE_SKIP_OPTIMIZATION: i32 = 4;
pub const D3DCOMPILE_SKIP_VALIDATION: i32 = 2;
pub const D3DCOMPILE_WARNINGS_ARE_ERRORS: i32 = 262144;
pub const D3D_BLOB_ALL_SIGNATURE_BLOB: D3D_BLOB_PART = 4;
pub const D3D_BLOB_DEBUG_INFO: D3D_BLOB_PART = 5;
pub const D3D_BLOB_DEBUG_NAME: D3D_BLOB_PART = 12;
pub const D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB: D3D_BLOB_PART = 2;
pub const D3D_BLOB_INPUT_SIGNATURE_BLOB: D3D_BLOB_PART = 0;
pub const D3D_BLOB_LEGACY_SHADER: D3D_BLOB_PART = 6;
pub const D3D_BLOB_OUTPUT_SIGNATURE_BLOB: D3D_BLOB_PART = 1;
pub type D3D_BLOB_PART = i32;
pub const D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB: D3D_BLOB_PART = 3;
pub const D3D_BLOB_PDB: D3D_BLOB_PART = 9;
pub const D3D_BLOB_PRIVATE_DATA: D3D_BLOB_PART = 10;
pub const D3D_BLOB_ROOT_SIGNATURE: D3D_BLOB_PART = 11;
pub const D3D_BLOB_TEST_ALTERNATE_SHADER: D3D_BLOB_PART = 32768;
pub const D3D_BLOB_TEST_COMPILE_DETAILS: D3D_BLOB_PART = 32769;
pub const D3D_BLOB_TEST_COMPILE_PERF: D3D_BLOB_PART = 32770;
pub const D3D_BLOB_TEST_COMPILE_REPORT: D3D_BLOB_PART = 32771;
pub const D3D_BLOB_XNA_PREPASS_SHADER: D3D_BLOB_PART = 7;
pub const D3D_BLOB_XNA_SHADER: D3D_BLOB_PART = 8;
pub const D3D_COMPILER_VERSION: i32 = 47;
#[cfg(feature = "d3dcommon")]
pub const D3D_COMPILE_STANDARD_FILE_INCLUDE: *mut *mut core::ffi::c_void = core::ptr::without_provenance_mut::<*mut core::ffi::c_void>(1usize);
pub const D3D_COMPRESS_SHADER_KEEP_ALL_PARTS: i32 = 1;
pub const D3D_DISASM_DISABLE_DEBUG_INFO: i32 = 16;
pub const D3D_DISASM_ENABLE_COLOR_CODE: i32 = 1;
pub const D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS: i32 = 2;
pub const D3D_DISASM_ENABLE_INSTRUCTION_CYCLE: i32 = 8;
pub const D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING: i32 = 4;
pub const D3D_DISASM_ENABLE_INSTRUCTION_OFFSET: i32 = 32;
pub const D3D_DISASM_INSTRUCTION_ONLY: i32 = 64;
pub const D3D_DISASM_PRINT_HEX_LITERALS: i32 = 128;
pub const D3D_GET_INST_OFFSETS_INCLUDE_NON_EXECUTABLE: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct D3D_SHADER_DATA {
    pub pBytecode: super::LPCVOID,
    pub BytecodeLength: usize,
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
pub type pD3DCompile = Option<unsafe extern "system" fn(psrcdata: super::LPCVOID, srcdatasize: usize, pfilename: windows_sys::core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: *mut core::ffi::c_void, pentrypoint: windows_sys::core::PCSTR, ptarget: windows_sys::core::PCSTR, flags1: u32, flags2: u32, ppcode: *mut super::ID3DBlob, pperrormsgs: *mut super::ID3DBlob) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
pub type pD3DDisassemble = Option<unsafe extern "system" fn(psrcdata: super::LPCVOID, srcdatasize: usize, flags: u32, szcomments: windows_sys::core::PCSTR, ppdisassembly: *mut super::ID3DBlob) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
pub type pD3DPreprocess = Option<unsafe extern "system" fn(psrcdata: super::LPCVOID, srcdatasize: usize, pfilename: windows_sys::core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: *mut core::ffi::c_void, ppcodetext: *mut super::ID3DBlob, pperrormsgs: *mut super::ID3DBlob) -> windows_sys::core::HRESULT>;
