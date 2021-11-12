#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DCompile(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, psourcename: super::super::super::Foundation::PSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: super::ID3DInclude, pentrypoint: super::super::super::Foundation::PSTR, ptarget: super::super::super::Foundation::PSTR, flags1: u32, flags2: u32, ppcode: *mut super::ID3DBlob, pperrormsgs: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DCompile2(
        psrcdata: *const ::core::ffi::c_void,
        srcdatasize: usize,
        psourcename: super::super::super::Foundation::PSTR,
        pdefines: *const super::D3D_SHADER_MACRO,
        pinclude: super::ID3DInclude,
        pentrypoint: super::super::super::Foundation::PSTR,
        ptarget: super::super::super::Foundation::PSTR,
        flags1: u32,
        flags2: u32,
        secondarydataflags: u32,
        psecondarydata: *const ::core::ffi::c_void,
        secondarydatasize: usize,
        ppcode: *mut super::ID3DBlob,
        pperrormsgs: *mut super::ID3DBlob,
    ) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DCompileFromFile(pfilename: super::super::super::Foundation::PWSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: super::ID3DInclude, pentrypoint: super::super::super::Foundation::PSTR, ptarget: super::super::super::Foundation::PSTR, flags1: u32, flags2: u32, ppcode: *mut super::ID3DBlob, pperrormsgs: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DCompressShaders(unumshaders: u32, pshaderdata: *const D3D_SHADER_DATA, uflags: u32, ppcompresseddata: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DCreateBlob(size: usize, ppblob: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub fn D3DCreateFunctionLinkingGraph(uflags: u32, ppfunctionlinkinggraph: *mut super::super::Direct3D11::ID3D11FunctionLinkingGraph) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub fn D3DCreateLinker(pplinker: *mut super::super::Direct3D11::ID3D11Linker) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DDecompressShaders(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, unumshaders: u32, ustartindex: u32, pindices: *const u32, uflags: u32, ppshaders: *mut super::ID3DBlob, ptotalshaders: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DDisassemble(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: super::super::super::Foundation::PSTR, ppdisassembly: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D10`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D10")]
    pub fn D3DDisassemble10Effect(peffect: super::super::Direct3D10::ID3D10Effect, flags: u32, ppdisassembly: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DDisassembleRegion(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: super::super::super::Foundation::PSTR, startbyteoffset: usize, numinsts: usize, pfinishbyteoffset: *mut usize, ppdisassembly: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetBlobPart(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, pppart: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetDebugInfo(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppdebuginfo: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetInputAndOutputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetInputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetOutputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetTraceInstructionOffsets(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, startinstindex: usize, numinsts: usize, poffsets: *mut usize, ptotalinsts: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub fn D3DLoadModule(psrcdata: *const ::core::ffi::c_void, cbsrcdatasize: usize, ppmodule: *mut super::super::Direct3D11::ID3D11Module) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPreprocess(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, psourcename: super::super::super::Foundation::PSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: super::ID3DInclude, ppcodetext: *mut super::ID3DBlob, pperrormsgs: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DReadFileToBlob(pfilename: super::super::super::Foundation::PWSTR, ppcontents: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DReflect(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, pinterface: *const ::windows_sys::core::GUID, ppreflector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DReflectLibrary(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, riid: *const ::windows_sys::core::GUID, ppreflector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DSetBlobPart(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, ppart: *const ::core::ffi::c_void, partsize: usize, ppnewshader: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DStripShader(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ustripflags: u32, ppstrippedblob: *mut super::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DWriteBlobToFile(pblob: super::ID3DBlob, pfilename: super::super::super::Foundation::PWSTR, boverwrite: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
}
pub struct D3DCOMPILER_STRIP_FLAGS(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_ALL_RESOURCES_BOUND: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_AVOID_FLOW_CONTROL: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_DEBUG: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_DEBUG_NAME_FOR_BINARY: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_DEBUG_NAME_FOR_SOURCE: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_EFFECT_CHILD_EFFECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_ENABLE_STRICTNESS: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_IEEE_STRICTNESS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_NO_PRESHADER: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_OPTIMIZATION_LEVEL0: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_OPTIMIZATION_LEVEL1: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_OPTIMIZATION_LEVEL3: u32 = 32768u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_PACK_MATRIX_ROW_MAJOR: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_PARTIAL_PRECISION: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_PREFER_FLOW_CONTROL: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_RESERVED16: u32 = 65536u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_RESERVED17: u32 = 131072u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_RESOURCES_MAY_ALIAS: u32 = 524288u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_SECDATA_MERGE_UAV_SLOTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_SECDATA_PRESERVE_TEMPLATE_SLOTS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_SECDATA_REQUIRE_TEMPLATE_MATCH: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_SKIP_OPTIMIZATION: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_SKIP_VALIDATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3DCOMPILE_WARNINGS_ARE_ERRORS: u32 = 262144u32;
pub struct D3D_BLOB_PART(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3D_COMPILER_VERSION: u32 = 47u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3D_COMPRESS_SHADER_KEEP_ALL_PARTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3D_DISASM_DISABLE_DEBUG_INFO: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3D_DISASM_ENABLE_COLOR_CODE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3D_DISASM_ENABLE_INSTRUCTION_CYCLE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3D_DISASM_ENABLE_INSTRUCTION_OFFSET: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3D_DISASM_INSTRUCTION_ONLY: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3D_DISASM_PRINT_HEX_LITERALS: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
pub const D3D_GET_INST_OFFSETS_INCLUDE_NON_EXECUTABLE: u32 = 1u32;
pub struct D3D_SHADER_DATA(i32);
pub struct pD3DCompile(i32);
pub struct pD3DDisassemble(i32);
pub struct pD3DPreprocess(i32);
