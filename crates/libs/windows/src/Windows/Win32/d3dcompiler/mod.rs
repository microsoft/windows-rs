#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DCompile<P2, P4, P5, P6>(psrcdata: super::LPCVOID, srcdatasize: usize, psourcename: P2, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P4, pentrypoint: P5, ptarget: P6, flags1: u32, flags2: u32, ppcode: *mut Option<super::ID3DBlob>, pperrormsgs: *mut Option<super::ID3DBlob>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::ID3DInclude>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCompile(psrcdata : super::LPCVOID, srcdatasize : usize, psourcename : windows_core::PCSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pentrypoint : windows_core::PCSTR, ptarget : windows_core::PCSTR, flags1 : u32, flags2 : u32, ppcode : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DCompile(psrcdata, srcdatasize, psourcename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), pentrypoint.param().abi(), ptarget.param().abi(), flags1, flags2, core::mem::transmute(ppcode), core::mem::transmute(pperrormsgs)) }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DCompile2<P2, P4, P5, P6>(psrcdata: super::LPCVOID, srcdatasize: usize, psourcename: P2, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P4, pentrypoint: P5, ptarget: P6, flags1: u32, flags2: u32, secondarydataflags: u32, psecondarydata: Option<super::LPCVOID>, secondarydatasize: usize, ppcode: *mut Option<super::ID3DBlob>, pperrormsgs: *mut Option<super::ID3DBlob>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::ID3DInclude>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCompile2(psrcdata : super::LPCVOID, srcdatasize : usize, psourcename : windows_core::PCSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pentrypoint : windows_core::PCSTR, ptarget : windows_core::PCSTR, flags1 : u32, flags2 : u32, secondarydataflags : u32, psecondarydata : super::LPCVOID, secondarydatasize : usize, ppcode : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DCompile2(psrcdata, srcdatasize, psourcename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), pentrypoint.param().abi(), ptarget.param().abi(), flags1, flags2, secondarydataflags, psecondarydata.unwrap_or(core::mem::zeroed()) as _, secondarydatasize, core::mem::transmute(ppcode), core::mem::transmute(pperrormsgs)) }
}
#[cfg(feature = "d3dcommon")]
#[inline]
pub unsafe fn D3DCompileFromFile<P0, P2, P3, P4>(pfilename: P0, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P2, pentrypoint: P3, ptarget: P4, flags1: u32, flags2: u32, ppcode: *mut Option<super::ID3DBlob>, pperrormsgs: *mut Option<super::ID3DBlob>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::ID3DInclude>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCompileFromFile(pfilename : windows_core::PCWSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pentrypoint : windows_core::PCSTR, ptarget : windows_core::PCSTR, flags1 : u32, flags2 : u32, ppcode : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DCompileFromFile(pfilename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), pentrypoint.param().abi(), ptarget.param().abi(), flags1, flags2, core::mem::transmute(ppcode), core::mem::transmute(pperrormsgs)) }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DCompressShaders(pshaderdata: &[D3D_SHADER_DATA], uflags: u32) -> windows_core::Result<super::ID3DBlob> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCompressShaders(unumshaders : u32, pshaderdata : *const D3D_SHADER_DATA, uflags : u32, ppcompresseddata : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DCompressShaders(pshaderdata.len().try_into().unwrap(), pshaderdata.as_ptr(), uflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "d3dcommon")]
#[inline]
pub unsafe fn D3DCreateBlob(size: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCreateBlob(size : usize, ppblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DCreateBlob(size, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "d3d11")]
#[inline]
pub unsafe fn D3DCreateFunctionLinkingGraph(uflags: u32) -> windows_core::Result<super::ID3D11FunctionLinkingGraph> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCreateFunctionLinkingGraph(uflags : u32, ppfunctionlinkinggraph : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DCreateFunctionLinkingGraph(uflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "d3d11")]
#[inline]
pub unsafe fn D3DCreateLinker() -> windows_core::Result<super::ID3D11Linker> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCreateLinker(pplinker : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DCreateLinker(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DDecompressShaders(psrcdata: super::LPCVOID, srcdatasize: usize, unumshaders: u32, ustartindex: u32, pindices: Option<*const u32>, uflags: u32, ppshaders: *mut Option<super::ID3DBlob>, ptotalshaders: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DDecompressShaders(psrcdata : super::LPCVOID, srcdatasize : usize, unumshaders : u32, ustartindex : u32, pindices : *const u32, uflags : u32, ppshaders : *mut *mut core::ffi::c_void, ptotalshaders : *mut u32) -> windows_core::HRESULT);
    unsafe { D3DDecompressShaders(psrcdata, srcdatasize, unumshaders, ustartindex, pindices.unwrap_or(core::mem::zeroed()) as _, uflags, core::mem::transmute(ppshaders), ptotalshaders.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DDisassemble<P3>(psrcdata: super::LPCVOID, srcdatasize: usize, flags: u32, szcomments: P3) -> windows_core::Result<super::ID3DBlob>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DDisassemble(psrcdata : super::LPCVOID, srcdatasize : usize, flags : u32, szcomments : windows_core::PCSTR, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DDisassemble(psrcdata, srcdatasize, flags, szcomments.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "d3d10", feature = "d3dcommon"))]
#[inline]
pub unsafe fn D3DDisassemble10Effect<P0>(peffect: P0, flags: u32) -> windows_core::Result<super::ID3DBlob>
where
    P0: windows_core::Param<super::ID3D10Effect>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DDisassemble10Effect(peffect : *mut core::ffi::c_void, flags : u32, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DDisassemble10Effect(peffect.param().abi(), flags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DDisassembleRegion<P3>(psrcdata: super::LPCVOID, srcdatasize: usize, flags: u32, szcomments: P3, startbyteoffset: usize, numinsts: usize, pfinishbyteoffset: Option<*mut usize>, ppdisassembly: *mut Option<super::ID3DBlob>) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DDisassembleRegion(psrcdata : super::LPCVOID, srcdatasize : usize, flags : u32, szcomments : windows_core::PCSTR, startbyteoffset : usize, numinsts : usize, pfinishbyteoffset : *mut usize, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DDisassembleRegion(psrcdata, srcdatasize, flags, szcomments.param().abi(), startbyteoffset, numinsts, pfinishbyteoffset.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppdisassembly)) }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DGetBlobPart(psrcdata: super::LPCVOID, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32) -> windows_core::Result<super::ID3DBlob> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetBlobPart(psrcdata : super::LPCVOID, srcdatasize : usize, part : D3D_BLOB_PART, flags : u32, pppart : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DGetBlobPart(psrcdata, srcdatasize, part, flags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DGetDebugInfo(psrcdata: super::LPCVOID, srcdatasize: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetDebugInfo(psrcdata : super::LPCVOID, srcdatasize : usize, ppdebuginfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DGetDebugInfo(psrcdata, srcdatasize, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DGetInputAndOutputSignatureBlob(psrcdata: super::LPCVOID, srcdatasize: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetInputAndOutputSignatureBlob(psrcdata : super::LPCVOID, srcdatasize : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DGetInputAndOutputSignatureBlob(psrcdata, srcdatasize, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DGetInputSignatureBlob(psrcdata: super::LPCVOID, srcdatasize: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetInputSignatureBlob(psrcdata : super::LPCVOID, srcdatasize : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DGetInputSignatureBlob(psrcdata, srcdatasize, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DGetOutputSignatureBlob(psrcdata: super::LPCVOID, srcdatasize: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetOutputSignatureBlob(psrcdata : super::LPCVOID, srcdatasize : usize, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DGetOutputSignatureBlob(psrcdata, srcdatasize, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn D3DGetTraceInstructionOffsets(psrcdata: super::LPCVOID, srcdatasize: usize, flags: u32, startinstindex: usize, numinsts: usize, poffsets: Option<*mut usize>, ptotalinsts: Option<*mut usize>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetTraceInstructionOffsets(psrcdata : super::LPCVOID, srcdatasize : usize, flags : u32, startinstindex : usize, numinsts : usize, poffsets : *mut usize, ptotalinsts : *mut usize) -> windows_core::HRESULT);
    unsafe { D3DGetTraceInstructionOffsets(psrcdata, srcdatasize, flags, startinstindex, numinsts, poffsets.unwrap_or(core::mem::zeroed()) as _, ptotalinsts.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "d3d11", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DLoadModule(psrcdata: super::LPCVOID, cbsrcdatasize: usize) -> windows_core::Result<super::ID3D11Module> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DLoadModule(psrcdata : super::LPCVOID, cbsrcdatasize : usize, ppmodule : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DLoadModule(psrcdata, cbsrcdatasize, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DPreprocess<P2, P4>(psrcdata: super::LPCVOID, srcdatasize: usize, psourcename: P2, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P4, ppcodetext: *mut Option<super::ID3DBlob>, pperrormsgs: *mut Option<super::ID3DBlob>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::ID3DInclude>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DPreprocess(psrcdata : super::LPCVOID, srcdatasize : usize, psourcename : windows_core::PCSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, ppcodetext : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DPreprocess(psrcdata, srcdatasize, psourcename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), core::mem::transmute(ppcodetext), core::mem::transmute(pperrormsgs)) }
}
#[cfg(feature = "d3dcommon")]
#[inline]
pub unsafe fn D3DReadFileToBlob<P0>(pfilename: P0) -> windows_core::Result<super::ID3DBlob>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DReadFileToBlob(pfilename : windows_core::PCWSTR, ppcontents : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DReadFileToBlob(pfilename.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn D3DReflect(psrcdata: super::LPCVOID, srcdatasize: usize, pinterface: *const windows_core::GUID, ppreflector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DReflect(psrcdata : super::LPCVOID, srcdatasize : usize, pinterface : *const windows_core::GUID, ppreflector : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DReflect(psrcdata, srcdatasize, pinterface, ppreflector as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn D3DReflectLibrary(psrcdata: super::LPCVOID, srcdatasize: usize, riid: *const windows_core::GUID, ppreflector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DReflectLibrary(psrcdata : super::LPCVOID, srcdatasize : usize, riid : *const windows_core::GUID, ppreflector : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DReflectLibrary(psrcdata, srcdatasize, riid, ppreflector as _) }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DSetBlobPart(psrcdata: super::LPCVOID, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, ppart: super::LPCVOID, partsize: usize) -> windows_core::Result<super::ID3DBlob> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DSetBlobPart(psrcdata : super::LPCVOID, srcdatasize : usize, part : D3D_BLOB_PART, flags : u32, ppart : super::LPCVOID, partsize : usize, ppnewshader : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DSetBlobPart(psrcdata, srcdatasize, part, flags, ppart, partsize, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
#[inline]
pub unsafe fn D3DStripShader(pshaderbytecode: super::LPCVOID, bytecodelength: usize, ustripflags: u32) -> windows_core::Result<super::ID3DBlob> {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DStripShader(pshaderbytecode : super::LPCVOID, bytecodelength : usize, ustripflags : u32, ppstrippedblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3DStripShader(pshaderbytecode, bytecodelength, ustripflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "d3dcommon")]
#[inline]
pub unsafe fn D3DWriteBlobToFile<P0, P1>(pblob: P0, pfilename: P1, boverwrite: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::ID3DBlob>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DWriteBlobToFile(pblob : *mut core::ffi::c_void, pfilename : windows_core::PCWSTR, boverwrite : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { D3DWriteBlobToFile(pblob.param().abi(), pfilename.param().abi(), boverwrite.into()) }
}
pub const D3DCOMPILER_DLL: windows_core::PCSTR = windows_core::s!("d3dcompiler_47.dll");
pub const D3DCOMPILER_DLL_A: windows_core::PCSTR = windows_core::s!("d3dcompiler_47.dll");
pub const D3DCOMPILER_DLL_W: windows_core::PCWSTR = windows_core::w!("d3dcompiler_47.dll");
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
pub const D3D_COMPILE_STANDARD_FILE_INCLUDE: *mut Option<super::ID3DInclude> = core::ptr::without_provenance_mut::<Option<super::ID3DInclude>>(1usize);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D_SHADER_DATA {
    pub pBytecode: super::LPCVOID,
    pub BytecodeLength: usize,
}
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
pub type pD3DCompile = Option<unsafe extern "system" fn(psrcdata: super::LPCVOID, srcdatasize: usize, pfilename: windows_core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: windows_core::Ref<super::ID3DInclude>, pentrypoint: windows_core::PCSTR, ptarget: windows_core::PCSTR, flags1: u32, flags2: u32, ppcode: windows_core::OutRef<super::ID3DBlob>, pperrormsgs: windows_core::OutRef<super::ID3DBlob>) -> windows_core::HRESULT>;
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
pub type pD3DDisassemble = Option<unsafe extern "system" fn(psrcdata: super::LPCVOID, srcdatasize: usize, flags: u32, szcomments: windows_core::PCSTR, ppdisassembly: windows_core::OutRef<super::ID3DBlob>) -> windows_core::HRESULT>;
#[cfg(all(feature = "d3dcommon", feature = "minwindef"))]
pub type pD3DPreprocess = Option<unsafe extern "system" fn(psrcdata: super::LPCVOID, srcdatasize: usize, pfilename: windows_core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: windows_core::Ref<super::ID3DInclude>, ppcodetext: windows_core::OutRef<super::ID3DBlob>, pperrormsgs: windows_core::OutRef<super::ID3DBlob>) -> windows_core::HRESULT>;
