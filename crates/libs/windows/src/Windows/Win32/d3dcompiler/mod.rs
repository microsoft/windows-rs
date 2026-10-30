#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DCompile<P2, P4, P5, P6>(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, psourcename: P2, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P4, pentrypoint: P5, ptarget: P6, flags1: super::UINT, flags2: super::UINT, ppcode: *mut Option<super::ID3D10Blob>, pperrormsgs: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::ID3DInclude>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCompile(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, psourcename : windows_core::PCSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pentrypoint : windows_core::PCSTR, ptarget : windows_core::PCSTR, flags1 : super::UINT, flags2 : super::UINT, ppcode : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DCompile(psrcdata, srcdatasize, psourcename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), pentrypoint.param().abi(), ptarget.param().abi(), flags1, flags2, core::mem::transmute(ppcode), core::mem::transmute(pperrormsgs)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DCompile2<P2, P4, P5, P6>(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, psourcename: P2, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P4, pentrypoint: P5, ptarget: P6, flags1: super::UINT, flags2: super::UINT, secondarydataflags: super::UINT, psecondarydata: Option<super::LPCVOID>, secondarydatasize: super::SIZE_T, ppcode: *mut Option<super::ID3D10Blob>, pperrormsgs: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::ID3DInclude>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCompile2(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, psourcename : windows_core::PCSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pentrypoint : windows_core::PCSTR, ptarget : windows_core::PCSTR, flags1 : super::UINT, flags2 : super::UINT, secondarydataflags : super::UINT, psecondarydata : super::LPCVOID, secondarydatasize : super::SIZE_T, ppcode : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DCompile2(psrcdata, srcdatasize, psourcename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), pentrypoint.param().abi(), ptarget.param().abi(), flags1, flags2, secondarydataflags, psecondarydata.unwrap_or(core::mem::zeroed()) as _, secondarydatasize, core::mem::transmute(ppcode), core::mem::transmute(pperrormsgs)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DCompileFromFile<P0, P2, P3, P4>(pfilename: P0, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P2, pentrypoint: P3, ptarget: P4, flags1: super::UINT, flags2: super::UINT, ppcode: *mut Option<super::ID3D10Blob>, pperrormsgs: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::ID3DInclude>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCompileFromFile(pfilename : windows_core::PCWSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, pentrypoint : windows_core::PCSTR, ptarget : windows_core::PCSTR, flags1 : super::UINT, flags2 : super::UINT, ppcode : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DCompileFromFile(pfilename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), pentrypoint.param().abi(), ptarget.param().abi(), flags1, flags2, core::mem::transmute(ppcode), core::mem::transmute(pperrormsgs)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DCompressShaders(pshaderdata: &[D3D_SHADER_DATA], uflags: super::UINT, ppcompresseddata: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCompressShaders(unumshaders : super::UINT, pshaderdata : *const D3D_SHADER_DATA, uflags : super::UINT, ppcompresseddata : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DCompressShaders(pshaderdata.len().try_into().unwrap(), pshaderdata.as_ptr(), uflags, core::mem::transmute(ppcompresseddata)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DCreateBlob(size: super::SIZE_T, ppblob: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCreateBlob(size : super::SIZE_T, ppblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DCreateBlob(size, core::mem::transmute(ppblob)) }
}
#[cfg(all(feature = "d3d11", feature = "shared"))]
#[inline]
pub unsafe fn D3DCreateFunctionLinkingGraph(uflags: super::UINT, ppfunctionlinkinggraph: *mut Option<super::ID3D11FunctionLinkingGraph>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCreateFunctionLinkingGraph(uflags : super::UINT, ppfunctionlinkinggraph : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DCreateFunctionLinkingGraph(uflags, core::mem::transmute(ppfunctionlinkinggraph)) }
}
#[cfg(feature = "d3d11")]
#[inline]
pub unsafe fn D3DCreateLinker(pplinker: *mut Option<super::ID3D11Linker>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DCreateLinker(pplinker : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DCreateLinker(core::mem::transmute(pplinker)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DDecompressShaders(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, unumshaders: super::UINT, ustartindex: super::UINT, pindices: Option<*const super::UINT>, uflags: super::UINT, ppshaders: *mut Option<super::ID3D10Blob>, ptotalshaders: Option<*mut super::UINT>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DDecompressShaders(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, unumshaders : super::UINT, ustartindex : super::UINT, pindices : *const super::UINT, uflags : super::UINT, ppshaders : *mut *mut core::ffi::c_void, ptotalshaders : *mut super::UINT) -> windows_core::HRESULT);
    unsafe { D3DDecompressShaders(psrcdata, srcdatasize, unumshaders, ustartindex, pindices.unwrap_or(core::mem::zeroed()) as _, uflags, core::mem::transmute(ppshaders), ptotalshaders.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DDisassemble<P3>(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, flags: super::UINT, szcomments: P3, ppdisassembly: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DDisassemble(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, flags : super::UINT, szcomments : windows_core::PCSTR, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DDisassemble(psrcdata, srcdatasize, flags, szcomments.param().abi(), core::mem::transmute(ppdisassembly)) }
}
#[cfg(all(feature = "d3d10", feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DDisassemble10Effect<P0>(peffect: P0, flags: super::UINT, ppdisassembly: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::ID3D10Effect>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DDisassemble10Effect(peffect : *mut core::ffi::c_void, flags : super::UINT, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DDisassemble10Effect(peffect.param().abi(), flags, core::mem::transmute(ppdisassembly)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DDisassembleRegion<P3>(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, flags: super::UINT, szcomments: P3, startbyteoffset: super::SIZE_T, numinsts: super::SIZE_T, pfinishbyteoffset: Option<*mut super::SIZE_T>, ppdisassembly: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DDisassembleRegion(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, flags : super::UINT, szcomments : windows_core::PCSTR, startbyteoffset : super::SIZE_T, numinsts : super::SIZE_T, pfinishbyteoffset : *mut super::SIZE_T, ppdisassembly : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DDisassembleRegion(psrcdata, srcdatasize, flags, szcomments.param().abi(), startbyteoffset, numinsts, pfinishbyteoffset.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppdisassembly)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DGetBlobPart(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, part: D3D_BLOB_PART, flags: super::UINT, pppart: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetBlobPart(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, part : D3D_BLOB_PART, flags : super::UINT, pppart : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DGetBlobPart(psrcdata, srcdatasize, part, flags, core::mem::transmute(pppart)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DGetDebugInfo(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, ppdebuginfo: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetDebugInfo(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, ppdebuginfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DGetDebugInfo(psrcdata, srcdatasize, core::mem::transmute(ppdebuginfo)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DGetInputAndOutputSignatureBlob(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, ppsignatureblob: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetInputAndOutputSignatureBlob(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DGetInputAndOutputSignatureBlob(psrcdata, srcdatasize, core::mem::transmute(ppsignatureblob)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DGetInputSignatureBlob(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, ppsignatureblob: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetInputSignatureBlob(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DGetInputSignatureBlob(psrcdata, srcdatasize, core::mem::transmute(ppsignatureblob)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DGetOutputSignatureBlob(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, ppsignatureblob: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetOutputSignatureBlob(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, ppsignatureblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DGetOutputSignatureBlob(psrcdata, srcdatasize, core::mem::transmute(ppsignatureblob)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn D3DGetTraceInstructionOffsets(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, flags: super::UINT, startinstindex: super::SIZE_T, numinsts: super::SIZE_T, poffsets: Option<*mut super::SIZE_T>, ptotalinsts: Option<*mut super::SIZE_T>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DGetTraceInstructionOffsets(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, flags : super::UINT, startinstindex : super::SIZE_T, numinsts : super::SIZE_T, poffsets : *mut super::SIZE_T, ptotalinsts : *mut super::SIZE_T) -> windows_core::HRESULT);
    unsafe { D3DGetTraceInstructionOffsets(psrcdata, srcdatasize, flags, startinstindex, numinsts, poffsets.unwrap_or(core::mem::zeroed()) as _, ptotalinsts.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "d3d11", feature = "shared"))]
#[inline]
pub unsafe fn D3DLoadModule(psrcdata: super::LPCVOID, cbsrcdatasize: super::SIZE_T, ppmodule: *mut Option<super::ID3D11Module>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DLoadModule(psrcdata : super::LPCVOID, cbsrcdatasize : super::SIZE_T, ppmodule : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DLoadModule(psrcdata, cbsrcdatasize, core::mem::transmute(ppmodule)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DPreprocess<P2, P4>(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, psourcename: P2, pdefines: Option<*const super::D3D_SHADER_MACRO>, pinclude: P4, ppcodetext: *mut Option<super::ID3D10Blob>, pperrormsgs: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::ID3DInclude>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DPreprocess(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, psourcename : windows_core::PCSTR, pdefines : *const super::D3D_SHADER_MACRO, pinclude : *mut core::ffi::c_void, ppcodetext : *mut *mut core::ffi::c_void, pperrormsgs : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DPreprocess(psrcdata, srcdatasize, psourcename.param().abi(), pdefines.unwrap_or(core::mem::zeroed()) as _, pinclude.param().abi(), core::mem::transmute(ppcodetext), core::mem::transmute(pperrormsgs)) }
}
#[cfg(feature = "d3dcommon")]
#[inline]
pub unsafe fn D3DReadFileToBlob<P0>(pfilename: P0, ppcontents: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DReadFileToBlob(pfilename : windows_core::PCWSTR, ppcontents : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DReadFileToBlob(pfilename.param().abi(), core::mem::transmute(ppcontents)) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn D3DReflect(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, pinterface: *const super::IID, ppreflector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DReflect(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, pinterface : *const super::IID, ppreflector : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DReflect(psrcdata, srcdatasize, pinterface, ppreflector as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn D3DReflectLibrary(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, riid: *const super::IID, ppreflector: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DReflectLibrary(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, riid : *const super::IID, ppreflector : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { D3DReflectLibrary(psrcdata, srcdatasize, riid, ppreflector as _) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DSetBlobPart(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, part: D3D_BLOB_PART, flags: super::UINT, ppart: super::LPCVOID, partsize: super::SIZE_T, ppnewshader: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DSetBlobPart(psrcdata : super::LPCVOID, srcdatasize : super::SIZE_T, part : D3D_BLOB_PART, flags : super::UINT, ppart : super::LPCVOID, partsize : super::SIZE_T, ppnewshader : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DSetBlobPart(psrcdata, srcdatasize, part, flags, ppart, partsize, core::mem::transmute(ppnewshader)) }
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
#[inline]
pub unsafe fn D3DStripShader(pshaderbytecode: super::LPCVOID, bytecodelength: super::SIZE_T, ustripflags: super::UINT, ppstrippedblob: *mut Option<super::ID3D10Blob>) -> windows_core::HRESULT {
    windows_core::link!("d3dcompiler_47.dll" "system" fn D3DStripShader(pshaderbytecode : super::LPCVOID, bytecodelength : super::SIZE_T, ustripflags : super::UINT, ppstrippedblob : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3DStripShader(pshaderbytecode, bytecodelength, ustripflags, core::mem::transmute(ppstrippedblob)) }
}
#[cfg(feature = "d3dcommon")]
#[inline]
pub unsafe fn D3DWriteBlobToFile<P0, P1>(pblob: P0, pfilename: P1, boverwrite: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::ID3D10Blob>,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D_SHADER_DATA {
    pub pBytecode: super::LPCVOID,
    pub BytecodeLength: super::SIZE_T,
}
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
pub type pD3DCompile = Option<unsafe extern "system" fn(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, pfilename: windows_core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: windows_core::Ref<super::ID3DInclude>, pentrypoint: windows_core::PCSTR, ptarget: windows_core::PCSTR, flags1: super::UINT, flags2: super::UINT, ppcode: windows_core::OutRef<super::ID3D10Blob>, pperrormsgs: windows_core::OutRef<super::ID3D10Blob>) -> windows_core::HRESULT>;
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
pub type pD3DDisassemble = Option<unsafe extern "system" fn(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, flags: super::UINT, szcomments: windows_core::PCSTR, ppdisassembly: windows_core::OutRef<super::ID3D10Blob>) -> windows_core::HRESULT>;
#[cfg(all(feature = "d3dcommon", feature = "shared"))]
pub type pD3DPreprocess = Option<unsafe extern "system" fn(psrcdata: super::LPCVOID, srcdatasize: super::SIZE_T, pfilename: windows_core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: windows_core::Ref<super::ID3DInclude>, ppcodetext: windows_core::OutRef<super::ID3D10Blob>, pperrormsgs: windows_core::OutRef<super::ID3D10Blob>) -> windows_core::HRESULT>;
