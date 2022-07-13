#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILER_DLL_A: &str = "d3dcompiler_47.dll";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILER_DLL_W: &str = "d3dcompiler_47.dll";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3DCOMPILER_STRIP_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILER_STRIP_REFLECTION_DATA: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILER_STRIP_DEBUG_INFO: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILER_STRIP_TEST_BLOBS: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILER_STRIP_PRIVATE_DATA: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILER_STRIP_ROOT_SIGNATURE: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILER_STRIP_FORCE_DWORD: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(2147483647i32);
impl ::core::marker::Copy for D3DCOMPILER_STRIP_FLAGS {}
impl ::core::clone::Clone for D3DCOMPILER_STRIP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3DCOMPILER_STRIP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3DCOMPILER_STRIP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3DCOMPILER_STRIP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DCOMPILER_STRIP_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_ALL_RESOURCES_BOUND: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_AVOID_FLOW_CONTROL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_DEBUG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_DEBUG_NAME_FOR_BINARY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_DEBUG_NAME_FOR_SOURCE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_EFFECT_CHILD_EFFECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_ENABLE_STRICTNESS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_IEEE_STRICTNESS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_NO_PRESHADER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_OPTIMIZATION_LEVEL0: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_OPTIMIZATION_LEVEL1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_OPTIMIZATION_LEVEL3: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_PACK_MATRIX_ROW_MAJOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_PARTIAL_PRECISION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_PREFER_FLOW_CONTROL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_RESERVED16: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_RESERVED17: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_RESOURCES_MAY_ALIAS: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_SECDATA_MERGE_UAV_SLOTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_SECDATA_PRESERVE_TEMPLATE_SLOTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_SECDATA_REQUIRE_TEMPLATE_MATCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_SKIP_OPTIMIZATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_SKIP_VALIDATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3DCOMPILE_WARNINGS_ARE_ERRORS: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DCompile<'a, P0, P1, P2, P3>(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, psourcename: P0, pdefines: *const super::D3D_SHADER_MACRO, pinclude: P1, pentrypoint: P2, ptarget: P3, flags1: u32, flags2: u32, ppcode: *mut ::core::option::Option<super::ID3DBlob>, pperrormsgs: *mut ::core::option::Option<super::ID3DBlob>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::ID3DInclude>>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DCompile(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, psourcename: ::windows::core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: *mut ::core::ffi::c_void, pentrypoint: ::windows::core::PCSTR, ptarget: ::windows::core::PCSTR, flags1: u32, flags2: u32, ppcode: *mut *mut ::core::ffi::c_void, pperrormsgs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3DCompile(::core::mem::transmute(psrcdata), srcdatasize, psourcename.into(), ::core::mem::transmute(pdefines), pinclude.into().abi(), pentrypoint.into(), ptarget.into(), flags1, flags2, ::core::mem::transmute(ppcode), ::core::mem::transmute(pperrormsgs)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DCompile2<'a, P0, P1, P2, P3>(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, psourcename: P0, pdefines: *const super::D3D_SHADER_MACRO, pinclude: P1, pentrypoint: P2, ptarget: P3, flags1: u32, flags2: u32, secondarydataflags: u32, psecondarydata: *const ::core::ffi::c_void, secondarydatasize: usize, ppcode: *mut ::core::option::Option<super::ID3DBlob>, pperrormsgs: *mut ::core::option::Option<super::ID3DBlob>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::ID3DInclude>>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DCompile2(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, psourcename: ::windows::core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: *mut ::core::ffi::c_void, pentrypoint: ::windows::core::PCSTR, ptarget: ::windows::core::PCSTR, flags1: u32, flags2: u32, secondarydataflags: u32, psecondarydata: *const ::core::ffi::c_void, secondarydatasize: usize, ppcode: *mut *mut ::core::ffi::c_void, pperrormsgs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3DCompile2(::core::mem::transmute(psrcdata), srcdatasize, psourcename.into(), ::core::mem::transmute(pdefines), pinclude.into().abi(), pentrypoint.into(), ptarget.into(), flags1, flags2, secondarydataflags, ::core::mem::transmute(psecondarydata), secondarydatasize, ::core::mem::transmute(ppcode), ::core::mem::transmute(pperrormsgs)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DCompileFromFile<'a, P0, P1, P2, P3>(pfilename: P0, pdefines: *const super::D3D_SHADER_MACRO, pinclude: P1, pentrypoint: P2, ptarget: P3, flags1: u32, flags2: u32, ppcode: *mut ::core::option::Option<super::ID3DBlob>, pperrormsgs: *mut ::core::option::Option<super::ID3DBlob>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::ID3DInclude>>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DCompileFromFile(pfilename: ::windows::core::PCWSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: *mut ::core::ffi::c_void, pentrypoint: ::windows::core::PCSTR, ptarget: ::windows::core::PCSTR, flags1: u32, flags2: u32, ppcode: *mut *mut ::core::ffi::c_void, pperrormsgs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3DCompileFromFile(pfilename.into(), ::core::mem::transmute(pdefines), pinclude.into().abi(), pentrypoint.into(), ptarget.into(), flags1, flags2, ::core::mem::transmute(ppcode), ::core::mem::transmute(pperrormsgs)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DCompressShaders(pshaderdata: &[D3D_SHADER_DATA], uflags: u32) -> ::windows::core::Result<super::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DCompressShaders(unumshaders: u32, pshaderdata: *const D3D_SHADER_DATA, uflags: u32, ppcompresseddata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DCompressShaders(pshaderdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pshaderdata)), uflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DCreateBlob(size: usize) -> ::windows::core::Result<super::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DCreateBlob(size: usize, ppblob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DCreateBlob(size, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`, `\"Win32_Graphics_Direct3D11\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DCreateFunctionLinkingGraph(uflags: u32) -> ::windows::core::Result<super::super::Direct3D11::ID3D11FunctionLinkingGraph> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DCreateFunctionLinkingGraph(uflags: u32, ppfunctionlinkinggraph: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DCreateFunctionLinkingGraph(uflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Direct3D11::ID3D11FunctionLinkingGraph>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`, `\"Win32_Graphics_Direct3D11\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DCreateLinker() -> ::windows::core::Result<super::super::Direct3D11::ID3D11Linker> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DCreateLinker(pplinker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DCreateLinker(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Direct3D11::ID3D11Linker>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DDecompressShaders(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, unumshaders: u32, ustartindex: u32, pindices: *const u32, uflags: u32, ppshaders: *mut ::core::option::Option<super::ID3DBlob>, ptotalshaders: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DDecompressShaders(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, unumshaders: u32, ustartindex: u32, pindices: *const u32, uflags: u32, ppshaders: *mut *mut ::core::ffi::c_void, ptotalshaders: *mut u32) -> ::windows::core::HRESULT;
    }
    D3DDecompressShaders(::core::mem::transmute(psrcdata), srcdatasize, ::core::mem::transmute(unumshaders), ustartindex, ::core::mem::transmute(pindices), uflags, ::core::mem::transmute(ppshaders), ::core::mem::transmute(ptotalshaders)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DDisassemble<'a, P0>(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: P0) -> ::windows::core::Result<super::ID3DBlob>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DDisassemble(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: ::windows::core::PCSTR, ppdisassembly: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DDisassemble(::core::mem::transmute(psrcdata), srcdatasize, flags, szcomments.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`, `\"Win32_Graphics_Direct3D10\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D10")]
#[inline]
pub unsafe fn D3DDisassemble10Effect<'a, P0>(peffect: P0, flags: u32) -> ::windows::core::Result<super::ID3DBlob>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Direct3D10::ID3D10Effect>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DDisassemble10Effect(peffect: *mut ::core::ffi::c_void, flags: u32, ppdisassembly: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DDisassemble10Effect(peffect.into().abi(), flags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DDisassembleRegion<'a, P0>(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: P0, startbyteoffset: usize, numinsts: usize, pfinishbyteoffset: *mut usize, ppdisassembly: *mut ::core::option::Option<super::ID3DBlob>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DDisassembleRegion(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: ::windows::core::PCSTR, startbyteoffset: usize, numinsts: usize, pfinishbyteoffset: *mut usize, ppdisassembly: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3DDisassembleRegion(::core::mem::transmute(psrcdata), srcdatasize, flags, szcomments.into(), startbyteoffset, numinsts, ::core::mem::transmute(pfinishbyteoffset), ::core::mem::transmute(ppdisassembly)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DGetBlobPart(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32) -> ::windows::core::Result<super::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DGetBlobPart(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, pppart: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DGetBlobPart(::core::mem::transmute(psrcdata), srcdatasize, part, flags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DGetDebugInfo(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize) -> ::windows::core::Result<super::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DGetDebugInfo(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppdebuginfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DGetDebugInfo(::core::mem::transmute(psrcdata), srcdatasize, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DGetInputAndOutputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize) -> ::windows::core::Result<super::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DGetInputAndOutputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DGetInputAndOutputSignatureBlob(::core::mem::transmute(psrcdata), srcdatasize, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DGetInputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize) -> ::windows::core::Result<super::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DGetInputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DGetInputSignatureBlob(::core::mem::transmute(psrcdata), srcdatasize, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DGetOutputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize) -> ::windows::core::Result<super::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DGetOutputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DGetOutputSignatureBlob(::core::mem::transmute(psrcdata), srcdatasize, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DGetTraceInstructionOffsets(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, startinstindex: usize, poffsets: &mut [usize], ptotalinsts: *mut usize) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DGetTraceInstructionOffsets(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, startinstindex: usize, numinsts: usize, poffsets: *mut usize, ptotalinsts: *mut usize) -> ::windows::core::HRESULT;
    }
    D3DGetTraceInstructionOffsets(::core::mem::transmute(psrcdata), srcdatasize, flags, startinstindex, poffsets.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(poffsets)), ::core::mem::transmute(ptotalinsts)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`, `\"Win32_Graphics_Direct3D11\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DLoadModule(psrcdata: *const ::core::ffi::c_void, cbsrcdatasize: usize) -> ::windows::core::Result<super::super::Direct3D11::ID3D11Module> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DLoadModule(psrcdata: *const ::core::ffi::c_void, cbsrcdatasize: usize, ppmodule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DLoadModule(::core::mem::transmute(psrcdata), cbsrcdatasize, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Direct3D11::ID3D11Module>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DPreprocess<'a, P0, P1>(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, psourcename: P0, pdefines: *const super::D3D_SHADER_MACRO, pinclude: P1, ppcodetext: *mut ::core::option::Option<super::ID3DBlob>, pperrormsgs: *mut ::core::option::Option<super::ID3DBlob>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::ID3DInclude>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DPreprocess(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, psourcename: ::windows::core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: *mut ::core::ffi::c_void, ppcodetext: *mut *mut ::core::ffi::c_void, pperrormsgs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3DPreprocess(::core::mem::transmute(psrcdata), srcdatasize, psourcename.into(), ::core::mem::transmute(pdefines), pinclude.into().abi(), ::core::mem::transmute(ppcodetext), ::core::mem::transmute(pperrormsgs)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DReadFileToBlob<'a, P0>(pfilename: P0) -> ::windows::core::Result<super::ID3DBlob>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DReadFileToBlob(pfilename: ::windows::core::PCWSTR, ppcontents: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DReadFileToBlob(pfilename.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DReflect(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, pinterface: *const ::windows::core::GUID, ppreflector: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DReflect(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, pinterface: *const ::windows::core::GUID, ppreflector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3DReflect(::core::mem::transmute(psrcdata), srcdatasize, ::core::mem::transmute(pinterface), ::core::mem::transmute(ppreflector)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DReflectLibrary(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, riid: *const ::windows::core::GUID, ppreflector: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DReflectLibrary(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, riid: *const ::windows::core::GUID, ppreflector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    D3DReflectLibrary(::core::mem::transmute(psrcdata), srcdatasize, ::core::mem::transmute(riid), ::core::mem::transmute(ppreflector)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DSetBlobPart(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, ppart: *const ::core::ffi::c_void, partsize: usize) -> ::windows::core::Result<super::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DSetBlobPart(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, ppart: *const ::core::ffi::c_void, partsize: usize, ppnewshader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DSetBlobPart(::core::mem::transmute(psrcdata), srcdatasize, part, flags, ::core::mem::transmute(ppart), partsize, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[inline]
pub unsafe fn D3DStripShader(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ustripflags: u32) -> ::windows::core::Result<super::ID3DBlob> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DStripShader(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ustripflags: u32, ppstrippedblob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    D3DStripShader(::core::mem::transmute(pshaderbytecode), bytecodelength, ustripflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::ID3DBlob>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn D3DWriteBlobToFile<'a, P0, P1, P2>(pblob: P0, pfilename: P1, boverwrite: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::ID3DBlob>>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn D3DWriteBlobToFile(pblob: *mut ::core::ffi::c_void, pfilename: ::windows::core::PCWSTR, boverwrite: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    D3DWriteBlobToFile(pblob.into().abi(), pfilename.into(), boverwrite.into()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct D3D_BLOB_PART(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_INPUT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_OUTPUT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_ALL_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_DEBUG_INFO: D3D_BLOB_PART = D3D_BLOB_PART(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_LEGACY_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_XNA_PREPASS_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_XNA_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_PDB: D3D_BLOB_PART = D3D_BLOB_PART(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_PRIVATE_DATA: D3D_BLOB_PART = D3D_BLOB_PART(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_ROOT_SIGNATURE: D3D_BLOB_PART = D3D_BLOB_PART(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_DEBUG_NAME: D3D_BLOB_PART = D3D_BLOB_PART(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_TEST_ALTERNATE_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(32768i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_TEST_COMPILE_DETAILS: D3D_BLOB_PART = D3D_BLOB_PART(32769i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_TEST_COMPILE_PERF: D3D_BLOB_PART = D3D_BLOB_PART(32770i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_BLOB_TEST_COMPILE_REPORT: D3D_BLOB_PART = D3D_BLOB_PART(32771i32);
impl ::core::marker::Copy for D3D_BLOB_PART {}
impl ::core::clone::Clone for D3D_BLOB_PART {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for D3D_BLOB_PART {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for D3D_BLOB_PART {
    type Abi = Self;
}
impl ::core::fmt::Debug for D3D_BLOB_PART {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_BLOB_PART").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_COMPILER_VERSION: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_COMPRESS_SHADER_KEEP_ALL_PARTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_DISASM_DISABLE_DEBUG_INFO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_DISASM_ENABLE_COLOR_CODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_DISASM_ENABLE_INSTRUCTION_CYCLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_DISASM_ENABLE_INSTRUCTION_OFFSET: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_DISASM_INSTRUCTION_ONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_DISASM_PRINT_HEX_LITERALS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub const D3D_GET_INST_OFFSETS_INCLUDE_NON_EXECUTABLE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub struct D3D_SHADER_DATA {
    pub pBytecode: *const ::core::ffi::c_void,
    pub BytecodeLength: usize,
}
impl ::core::marker::Copy for D3D_SHADER_DATA {}
impl ::core::clone::Clone for D3D_SHADER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D3D_SHADER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D_SHADER_DATA").field("pBytecode", &self.pBytecode).field("BytecodeLength", &self.BytecodeLength).finish()
    }
}
unsafe impl ::windows::core::Abi for D3D_SHADER_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for D3D_SHADER_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<D3D_SHADER_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for D3D_SHADER_DATA {}
impl ::core::default::Default for D3D_SHADER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub type pD3DCompile = ::core::option::Option<unsafe extern "system" fn(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, pfilename: ::windows::core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: ::core::option::Option<super::ID3DInclude>, pentrypoint: ::windows::core::PCSTR, ptarget: ::windows::core::PCSTR, flags1: u32, flags2: u32, ppcode: *mut ::core::option::Option<super::ID3DBlob>, pperrormsgs: *mut ::core::option::Option<super::ID3DBlob>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub type pD3DDisassemble = ::core::option::Option<unsafe extern "system" fn(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: ::windows::core::PCSTR, ppdisassembly: *mut ::core::option::Option<super::ID3DBlob>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Fxc\"`*"]
pub type pD3DPreprocess = ::core::option::Option<unsafe extern "system" fn(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, pfilename: ::windows::core::PCSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: ::core::option::Option<super::ID3DInclude>, ppcodetext: *mut ::core::option::Option<super::ID3DBlob>, pperrormsgs: *mut ::core::option::Option<super::ID3DBlob>) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
