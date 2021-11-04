#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_DxcAssembler: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3609779048, 63747, 20352, [148, 205, 220, 207, 118, 236, 113, 81]);
pub const CLSID_DxcCompiler: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1944202643, 59086, 18419, [181, 191, 240, 102, 79, 57, 193, 176]);
pub const CLSID_DxcCompilerArgs: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1045868162, 8781, 18191, [161, 161, 254, 48, 22, 238, 159, 157]);
pub const CLSID_DxcContainerBuilder: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2484290196, 16671, 17780, [180, 208, 135, 65, 226, 82, 64, 210]);
pub const CLSID_DxcContainerReflection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3119858825, 21944, 16396, [186, 58, 22, 117, 228, 114, 139, 145]);
pub const CLSID_DxcDiaDataSource: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3441388403, 10928, 18509, [142, 220, 235, 231, 164, 60, 160, 159]);
pub const CLSID_DxcLibrary: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1648744111, 26336, 18685, [128, 180, 77, 39, 23, 150, 116, 140]);
pub const CLSID_DxcLinker: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4016734343, 45290, 19798, [158, 69, 208, 126, 26, 139, 120, 6]);
pub const CLSID_DxcOptimizer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2922174367, 52258, 17727, [155, 107, 177, 36, 231, 165, 32, 76]);
pub const CLSID_DxcPdbUtils: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1415716347, 62158, 17790, [174, 140, 236, 53, 95, 174, 236, 124]);
pub const CLSID_DxcValidator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2359550485, 63272, 19699, [140, 221, 136, 175, 145, 117, 135, 161]);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILER_DLL: &'static str = "d3dcompiler_47.dll";
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct D3DCOMPILER_STRIP_FLAGS(pub i32);
pub const D3DCOMPILER_STRIP_REFLECTION_DATA: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(1i32);
pub const D3DCOMPILER_STRIP_DEBUG_INFO: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(2i32);
pub const D3DCOMPILER_STRIP_TEST_BLOBS: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(4i32);
pub const D3DCOMPILER_STRIP_PRIVATE_DATA: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(8i32);
pub const D3DCOMPILER_STRIP_ROOT_SIGNATURE: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(16i32);
pub const D3DCOMPILER_STRIP_FORCE_DWORD: D3DCOMPILER_STRIP_FLAGS = D3DCOMPILER_STRIP_FLAGS(2147483647i32);
impl ::std::convert::From<i32> for D3DCOMPILER_STRIP_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for D3DCOMPILER_STRIP_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_ALL_RESOURCES_BOUND: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_AVOID_FLOW_CONTROL: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_DEBUG: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_DEBUG_NAME_FOR_BINARY: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_DEBUG_NAME_FOR_SOURCE: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_EFFECT_CHILD_EFFECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_ENABLE_STRICTNESS: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_0: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_1_1: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_FLAGS2_FORCE_ROOT_SIGNATURE_LATEST: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_IEEE_STRICTNESS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_NO_PRESHADER: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_OPTIMIZATION_LEVEL0: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_OPTIMIZATION_LEVEL1: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_OPTIMIZATION_LEVEL2: u32 = 49152u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_OPTIMIZATION_LEVEL3: u32 = 32768u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_PACK_MATRIX_ROW_MAJOR: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_PARTIAL_PRECISION: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_PREFER_FLOW_CONTROL: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_RESERVED16: u32 = 65536u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_RESERVED17: u32 = 131072u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_RESOURCES_MAY_ALIAS: u32 = 524288u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_SECDATA_MERGE_UAV_SLOTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_SECDATA_PRESERVE_TEMPLATE_SLOTS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_SECDATA_REQUIRE_TEMPLATE_MATCH: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_SKIP_OPTIMIZATION: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_SKIP_VALIDATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3DCOMPILE_WARNINGS_ARE_ERRORS: u32 = 262144u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3DCompile<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::Direct3D11::ID3DInclude>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    psrcdata: *const ::std::ffi::c_void,
    srcdatasize: usize,
    psourcename: Param2,
    pdefines: *const super::Direct3D11::D3D_SHADER_MACRO,
    pinclude: Param4,
    pentrypoint: Param5,
    ptarget: Param6,
    flags1: u32,
    flags2: u32,
    ppcode: *mut ::std::option::Option<super::Direct3D11::ID3DBlob>,
    pperrormsgs: *mut ::std::option::Option<super::Direct3D11::ID3DBlob>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DCompile(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, psourcename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D11::D3D_SHADER_MACRO, pinclude: ::windows::runtime::RawPtr, pentrypoint: super::super::Foundation::PSTR, ptarget: super::super::Foundation::PSTR, flags1: u32, flags2: u32, ppcode: *mut ::windows::runtime::RawPtr, pperrormsgs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        D3DCompile(
            ::std::mem::transmute(psrcdata),
            ::std::mem::transmute(srcdatasize),
            psourcename.into_param().abi(),
            ::std::mem::transmute(pdefines),
            pinclude.into_param().abi(),
            pentrypoint.into_param().abi(),
            ptarget.into_param().abi(),
            ::std::mem::transmute(flags1),
            ::std::mem::transmute(flags2),
            ::std::mem::transmute(ppcode),
            ::std::mem::transmute(pperrormsgs),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3DCompile2<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::Direct3D11::ID3DInclude>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    psrcdata: *const ::std::ffi::c_void,
    srcdatasize: usize,
    psourcename: Param2,
    pdefines: *const super::Direct3D11::D3D_SHADER_MACRO,
    pinclude: Param4,
    pentrypoint: Param5,
    ptarget: Param6,
    flags1: u32,
    flags2: u32,
    secondarydataflags: u32,
    psecondarydata: *const ::std::ffi::c_void,
    secondarydatasize: usize,
    ppcode: *mut ::std::option::Option<super::Direct3D11::ID3DBlob>,
    pperrormsgs: *mut ::std::option::Option<super::Direct3D11::ID3DBlob>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DCompile2(
                psrcdata: *const ::std::ffi::c_void,
                srcdatasize: usize,
                psourcename: super::super::Foundation::PSTR,
                pdefines: *const super::Direct3D11::D3D_SHADER_MACRO,
                pinclude: ::windows::runtime::RawPtr,
                pentrypoint: super::super::Foundation::PSTR,
                ptarget: super::super::Foundation::PSTR,
                flags1: u32,
                flags2: u32,
                secondarydataflags: u32,
                psecondarydata: *const ::std::ffi::c_void,
                secondarydatasize: usize,
                ppcode: *mut ::windows::runtime::RawPtr,
                pperrormsgs: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        D3DCompile2(
            ::std::mem::transmute(psrcdata),
            ::std::mem::transmute(srcdatasize),
            psourcename.into_param().abi(),
            ::std::mem::transmute(pdefines),
            pinclude.into_param().abi(),
            pentrypoint.into_param().abi(),
            ptarget.into_param().abi(),
            ::std::mem::transmute(flags1),
            ::std::mem::transmute(flags2),
            ::std::mem::transmute(secondarydataflags),
            ::std::mem::transmute(psecondarydata),
            ::std::mem::transmute(secondarydatasize),
            ::std::mem::transmute(ppcode),
            ::std::mem::transmute(pperrormsgs),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3DCompileFromFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Direct3D11::ID3DInclude>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    pfilename: Param0,
    pdefines: *const super::Direct3D11::D3D_SHADER_MACRO,
    pinclude: Param2,
    pentrypoint: Param3,
    ptarget: Param4,
    flags1: u32,
    flags2: u32,
    ppcode: *mut ::std::option::Option<super::Direct3D11::ID3DBlob>,
    pperrormsgs: *mut ::std::option::Option<super::Direct3D11::ID3DBlob>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DCompileFromFile(pfilename: super::super::Foundation::PWSTR, pdefines: *const super::Direct3D11::D3D_SHADER_MACRO, pinclude: ::windows::runtime::RawPtr, pentrypoint: super::super::Foundation::PSTR, ptarget: super::super::Foundation::PSTR, flags1: u32, flags2: u32, ppcode: *mut ::windows::runtime::RawPtr, pperrormsgs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        D3DCompileFromFile(pfilename.into_param().abi(), ::std::mem::transmute(pdefines), pinclude.into_param().abi(), pentrypoint.into_param().abi(), ptarget.into_param().abi(), ::std::mem::transmute(flags1), ::std::mem::transmute(flags2), ::std::mem::transmute(ppcode), ::std::mem::transmute(pperrormsgs)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DCompressShaders(unumshaders: u32, pshaderdata: *const D3D_SHADER_DATA, uflags: u32) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DCompressShaders(unumshaders: u32, pshaderdata: *const D3D_SHADER_DATA, uflags: u32, ppcompresseddata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DCompressShaders(::std::mem::transmute(unumshaders), ::std::mem::transmute(pshaderdata), ::std::mem::transmute(uflags), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DCreateBlob(size: usize) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DCreateBlob(size: usize, ppblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DCreateBlob(::std::mem::transmute(size), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DCreateFunctionLinkingGraph(uflags: u32) -> ::windows::runtime::Result<super::Direct3D11::ID3D11FunctionLinkingGraph> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DCreateFunctionLinkingGraph(uflags: u32, ppfunctionlinkinggraph: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3D11FunctionLinkingGraph as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DCreateFunctionLinkingGraph(::std::mem::transmute(uflags), &mut result__).from_abi::<super::Direct3D11::ID3D11FunctionLinkingGraph>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DCreateLinker() -> ::windows::runtime::Result<super::Direct3D11::ID3D11Linker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DCreateLinker(pplinker: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3D11Linker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DCreateLinker(&mut result__).from_abi::<super::Direct3D11::ID3D11Linker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DDecompressShaders(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, unumshaders: u32, ustartindex: u32, pindices: *const u32, uflags: u32, ppshaders: *mut ::std::option::Option<super::Direct3D11::ID3DBlob>, ptotalshaders: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DDecompressShaders(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, unumshaders: u32, ustartindex: u32, pindices: *const u32, uflags: u32, ppshaders: *mut ::windows::runtime::RawPtr, ptotalshaders: *mut u32) -> ::windows::runtime::HRESULT;
        }
        D3DDecompressShaders(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), ::std::mem::transmute(unumshaders), ::std::mem::transmute(ustartindex), ::std::mem::transmute(pindices), ::std::mem::transmute(uflags), ::std::mem::transmute(ppshaders), ::std::mem::transmute(ptotalshaders)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3DDisassemble<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: Param3) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DDisassemble(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: super::super::Foundation::PSTR, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DDisassemble(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), ::std::mem::transmute(flags), szcomments.into_param().abi(), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D10`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D10", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3DDisassemble10Effect<'a, Param0: ::windows::runtime::IntoParam<'a, super::Direct3D10::ID3D10Effect>>(peffect: Param0, flags: u32) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DDisassemble10Effect(peffect: ::windows::runtime::RawPtr, flags: u32, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DDisassemble10Effect(peffect.into_param().abi(), ::std::mem::transmute(flags), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3DDisassembleRegion<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: Param3, startbyteoffset: usize, numinsts: usize, pfinishbyteoffset: *mut usize, ppdisassembly: *mut ::std::option::Option<super::Direct3D11::ID3DBlob>) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DDisassembleRegion(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: super::super::Foundation::PSTR, startbyteoffset: usize, numinsts: usize, pfinishbyteoffset: *mut usize, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        D3DDisassembleRegion(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), ::std::mem::transmute(flags), szcomments.into_param().abi(), ::std::mem::transmute(startbyteoffset), ::std::mem::transmute(numinsts), ::std::mem::transmute(pfinishbyteoffset), ::std::mem::transmute(ppdisassembly)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DGetBlobPart(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DGetBlobPart(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, pppart: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DGetBlobPart(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), ::std::mem::transmute(part), ::std::mem::transmute(flags), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DGetDebugInfo(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DGetDebugInfo(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, ppdebuginfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DGetDebugInfo(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DGetInputAndOutputSignatureBlob(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DGetInputAndOutputSignatureBlob(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DGetInputAndOutputSignatureBlob(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DGetInputSignatureBlob(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DGetInputSignatureBlob(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DGetInputSignatureBlob(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DGetOutputSignatureBlob(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DGetOutputSignatureBlob(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DGetOutputSignatureBlob(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[inline]
pub unsafe fn D3DGetTraceInstructionOffsets(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, flags: u32, startinstindex: usize, numinsts: usize, poffsets: *mut usize, ptotalinsts: *mut usize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DGetTraceInstructionOffsets(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, flags: u32, startinstindex: usize, numinsts: usize, poffsets: *mut usize, ptotalinsts: *mut usize) -> ::windows::runtime::HRESULT;
        }
        D3DGetTraceInstructionOffsets(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), ::std::mem::transmute(flags), ::std::mem::transmute(startinstindex), ::std::mem::transmute(numinsts), ::std::mem::transmute(poffsets), ::std::mem::transmute(ptotalinsts)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DLoadModule(psrcdata: *const ::std::ffi::c_void, cbsrcdatasize: usize) -> ::windows::runtime::Result<super::Direct3D11::ID3D11Module> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DLoadModule(psrcdata: *const ::std::ffi::c_void, cbsrcdatasize: usize, ppmodule: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3D11Module as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DLoadModule(::std::mem::transmute(psrcdata), ::std::mem::transmute(cbsrcdatasize), &mut result__).from_abi::<super::Direct3D11::ID3D11Module>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3DPreprocess<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::Direct3D11::ID3DInclude>>(
    psrcdata: *const ::std::ffi::c_void,
    srcdatasize: usize,
    psourcename: Param2,
    pdefines: *const super::Direct3D11::D3D_SHADER_MACRO,
    pinclude: Param4,
    ppcodetext: *mut ::std::option::Option<super::Direct3D11::ID3DBlob>,
    pperrormsgs: *mut ::std::option::Option<super::Direct3D11::ID3DBlob>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DPreprocess(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, psourcename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D11::D3D_SHADER_MACRO, pinclude: ::windows::runtime::RawPtr, ppcodetext: *mut ::windows::runtime::RawPtr, pperrormsgs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        D3DPreprocess(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), psourcename.into_param().abi(), ::std::mem::transmute(pdefines), pinclude.into_param().abi(), ::std::mem::transmute(ppcodetext), ::std::mem::transmute(pperrormsgs)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3DReadFileToBlob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pfilename: Param0) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DReadFileToBlob(pfilename: super::super::Foundation::PWSTR, ppcontents: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DReadFileToBlob(pfilename.into_param().abi(), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[inline]
pub unsafe fn D3DReflect(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, pinterface: *const ::windows::runtime::GUID, ppreflector: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DReflect(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, pinterface: *const ::windows::runtime::GUID, ppreflector: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        D3DReflect(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), ::std::mem::transmute(pinterface), ::std::mem::transmute(ppreflector)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[inline]
pub unsafe fn D3DReflectLibrary(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, riid: *const ::windows::runtime::GUID, ppreflector: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DReflectLibrary(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, riid: *const ::windows::runtime::GUID, ppreflector: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        D3DReflectLibrary(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), ::std::mem::transmute(riid), ::std::mem::transmute(ppreflector)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DSetBlobPart(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, ppart: *const ::std::ffi::c_void, partsize: usize) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DSetBlobPart(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, ppart: *const ::std::ffi::c_void, partsize: usize, ppnewshader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DSetBlobPart(::std::mem::transmute(psrcdata), ::std::mem::transmute(srcdatasize), ::std::mem::transmute(part), ::std::mem::transmute(flags), ::std::mem::transmute(ppart), ::std::mem::transmute(partsize), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Graphics_Direct3D11`*"]
#[cfg(feature = "Win32_Graphics_Direct3D11")]
#[inline]
pub unsafe fn D3DStripShader(pshaderbytecode: *const ::std::ffi::c_void, bytecodelength: usize, ustripflags: u32) -> ::windows::runtime::Result<super::Direct3D11::ID3DBlob> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DStripShader(pshaderbytecode: *const ::std::ffi::c_void, bytecodelength: usize, ustripflags: u32, ppstrippedblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Direct3D11::ID3DBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        D3DStripShader(::std::mem::transmute(pshaderbytecode), ::std::mem::transmute(bytecodelength), ::std::mem::transmute(ustripflags), &mut result__).from_abi::<super::Direct3D11::ID3DBlob>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`, `Win32_Graphics_Direct3D11`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D11"))]
#[inline]
pub unsafe fn D3DWriteBlobToFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::Direct3D11::ID3DBlob>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pblob: Param0, pfilename: Param1, boverwrite: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn D3DWriteBlobToFile(pblob: ::windows::runtime::RawPtr, pfilename: super::super::Foundation::PWSTR, boverwrite: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        D3DWriteBlobToFile(pblob.into_param().abi(), pfilename.into_param().abi(), boverwrite.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct D3D_BLOB_PART(pub i32);
pub const D3D_BLOB_INPUT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(0i32);
pub const D3D_BLOB_OUTPUT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(1i32);
pub const D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(2i32);
pub const D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(3i32);
pub const D3D_BLOB_ALL_SIGNATURE_BLOB: D3D_BLOB_PART = D3D_BLOB_PART(4i32);
pub const D3D_BLOB_DEBUG_INFO: D3D_BLOB_PART = D3D_BLOB_PART(5i32);
pub const D3D_BLOB_LEGACY_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(6i32);
pub const D3D_BLOB_XNA_PREPASS_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(7i32);
pub const D3D_BLOB_XNA_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(8i32);
pub const D3D_BLOB_PDB: D3D_BLOB_PART = D3D_BLOB_PART(9i32);
pub const D3D_BLOB_PRIVATE_DATA: D3D_BLOB_PART = D3D_BLOB_PART(10i32);
pub const D3D_BLOB_ROOT_SIGNATURE: D3D_BLOB_PART = D3D_BLOB_PART(11i32);
pub const D3D_BLOB_DEBUG_NAME: D3D_BLOB_PART = D3D_BLOB_PART(12i32);
pub const D3D_BLOB_TEST_ALTERNATE_SHADER: D3D_BLOB_PART = D3D_BLOB_PART(32768i32);
pub const D3D_BLOB_TEST_COMPILE_DETAILS: D3D_BLOB_PART = D3D_BLOB_PART(32769i32);
pub const D3D_BLOB_TEST_COMPILE_PERF: D3D_BLOB_PART = D3D_BLOB_PART(32770i32);
pub const D3D_BLOB_TEST_COMPILE_REPORT: D3D_BLOB_PART = D3D_BLOB_PART(32771i32);
impl ::std::convert::From<i32> for D3D_BLOB_PART {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for D3D_BLOB_PART {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_COMPILER_VERSION: u32 = 47u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_COMPILE_STANDARD_FILE_INCLUDE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_COMPRESS_SHADER_KEEP_ALL_PARTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_DISASM_DISABLE_DEBUG_INFO: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_DISASM_ENABLE_COLOR_CODE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_DISASM_ENABLE_DEFAULT_VALUE_PRINTS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_DISASM_ENABLE_INSTRUCTION_CYCLE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_DISASM_ENABLE_INSTRUCTION_NUMBERING: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_DISASM_ENABLE_INSTRUCTION_OFFSET: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_DISASM_INSTRUCTION_ONLY: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_DISASM_PRINT_HEX_LITERALS: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const D3D_GET_INST_OFFSETS_INCLUDE_NON_EXECUTABLE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub struct D3D_SHADER_DATA {
    pub pBytecode: *mut ::std::ffi::c_void,
    pub BytecodeLength: usize,
}
impl D3D_SHADER_DATA {}
impl ::std::default::Default for D3D_SHADER_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for D3D_SHADER_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("D3D_SHADER_DATA").field("pBytecode", &self.pBytecode).field("BytecodeLength", &self.BytecodeLength).finish()
    }
}
impl ::std::cmp::PartialEq for D3D_SHADER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pBytecode == other.pBytecode && self.BytecodeLength == other.BytecodeLength
    }
}
impl ::std::cmp::Eq for D3D_SHADER_DATA {}
unsafe impl ::windows::runtime::Abi for D3D_SHADER_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXC_CP(pub u32);
pub const DXC_CP_ACP: DXC_CP = DXC_CP(0u32);
pub const DXC_CP_UTF16: DXC_CP = DXC_CP(1200u32);
pub const DXC_CP_UTF8: DXC_CP = DXC_CP(65001u32);
impl ::std::convert::From<u32> for DXC_CP {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXC_CP {
    type Abi = Self;
}
impl ::std::ops::BitOr for DXC_CP {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DXC_CP {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DXC_CP {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DXC_CP {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DXC_CP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXC_OUT_KIND(pub i32);
pub const DXC_OUT_NONE: DXC_OUT_KIND = DXC_OUT_KIND(0i32);
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = DXC_OUT_KIND(1i32);
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = DXC_OUT_KIND(2i32);
pub const DXC_OUT_PDB: DXC_OUT_KIND = DXC_OUT_KIND(3i32);
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = DXC_OUT_KIND(4i32);
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = DXC_OUT_KIND(5i32);
pub const DXC_OUT_HLSL: DXC_OUT_KIND = DXC_OUT_KIND(6i32);
pub const DXC_OUT_TEXT: DXC_OUT_KIND = DXC_OUT_KIND(7i32);
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = DXC_OUT_KIND(8i32);
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = DXC_OUT_KIND(9i32);
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = DXC_OUT_KIND(10i32);
pub const DXC_OUT_FORCE_DWORD: DXC_OUT_KIND = DXC_OUT_KIND(-1i32);
impl ::std::convert::From<i32> for DXC_OUT_KIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DXC_OUT_KIND {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
pub struct DxcArgPair {
    pub pName: super::super::Foundation::PWSTR,
    pub pValue: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DxcArgPair {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DxcArgPair {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DxcArgPair {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DxcArgPair").field("pName", &self.pName).field("pValue", &self.pValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DxcArgPair {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName && self.pValue == other.pValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DxcArgPair {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DxcArgPair {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub struct DxcBuffer {
    pub Ptr: *mut ::std::ffi::c_void,
    pub Size: usize,
    pub Encoding: u32,
}
impl DxcBuffer {}
impl ::std::default::Default for DxcBuffer {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DxcBuffer {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DxcBuffer").field("Ptr", &self.Ptr).field("Size", &self.Size).field("Encoding", &self.Encoding).finish()
    }
}
impl ::std::cmp::PartialEq for DxcBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.Ptr == other.Ptr && self.Size == other.Size && self.Encoding == other.Encoding
    }
}
impl ::std::cmp::Eq for DxcBuffer {}
unsafe impl ::windows::runtime::Abi for DxcBuffer {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[inline]
pub unsafe fn DxcCreateInstance(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DxcCreateInstance(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DxcCreateInstance(::std::mem::transmute(rclsid), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DxcCreateInstance2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IMalloc>>(pmalloc: Param0, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DxcCreateInstance2(pmalloc: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DxcCreateInstance2(pmalloc.into_param().abi(), ::std::mem::transmute(rclsid), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
pub type DxcCreateInstance2Proc = unsafe extern "system" fn(pmalloc: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
pub type DxcCreateInstanceProc = unsafe extern "system" fn(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
pub struct DxcDefine {
    pub Name: super::super::Foundation::PWSTR,
    pub Value: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DxcDefine {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DxcDefine {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DxcDefine {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DxcDefine").field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DxcDefine {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DxcDefine {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DxcDefine {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub struct DxcShaderHash {
    pub Flags: u32,
    pub HashDigest: [u8; 16],
}
impl DxcShaderHash {}
impl ::std::default::Default for DxcShaderHash {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DxcShaderHash {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DxcShaderHash").field("Flags", &self.Flags).field("HashDigest", &self.HashDigest).finish()
    }
}
impl ::std::cmp::PartialEq for DxcShaderHash {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.HashDigest == other.HashDigest
    }
}
impl ::std::cmp::Eq for DxcShaderHash {}
unsafe impl ::windows::runtime::Abi for DxcShaderHash {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const DxcValidatorFlags_Default: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
pub const DxcVersionInfoFlags_None: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcAssembler(pub ::windows::runtime::IUnknown);
impl IDxcAssembler {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn AssembleToContainer<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0) -> ::windows::runtime::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pshader.into_param().abi(), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcAssembler {
    type Vtable = IDxcAssembler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(153057830, 7199, 18760, [144, 75, 230, 227, 168, 167, 113, 213]);
}
impl ::std::convert::From<IDxcAssembler> for ::windows::runtime::IUnknown {
    fn from(value: IDxcAssembler) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcAssembler> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcAssembler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcAssembler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcAssembler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcAssembler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pshader: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcBlob(pub ::windows::runtime::IUnknown);
impl IDxcBlob {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IDxcBlob {
    type Vtable = IDxcBlob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2342910728, 20885, 16610, [172, 88, 13, 152, 156, 58, 1, 2]);
}
impl ::std::convert::From<IDxcBlob> for ::windows::runtime::IUnknown {
    fn from(value: IDxcBlob) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcBlob> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcBlob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcBlob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcBlob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlob_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> usize,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcBlobEncoding(pub ::windows::runtime::IUnknown);
impl IDxcBlobEncoding {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pknown), ::std::mem::transmute(pcodepage)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDxcBlobEncoding {
    type Vtable = IDxcBlobEncoding_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1916916772, 9798, 16785, [151, 192, 152, 233, 110, 66, 252, 104]);
}
impl ::std::convert::From<IDxcBlobEncoding> for ::windows::runtime::IUnknown {
    fn from(value: IDxcBlobEncoding) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcBlobEncoding> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcBlobEncoding) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcBlobEncoding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcBlobEncoding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDxcBlobEncoding> for IDxcBlob {
    fn from(value: IDxcBlobEncoding) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobEncoding> for IDxcBlob {
    fn from(value: &IDxcBlobEncoding) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcBlob> for IDxcBlobEncoding {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcBlob> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcBlob> for &IDxcBlobEncoding {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcBlob> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobEncoding_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknown: *mut super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcBlobUtf16(pub ::windows::runtime::IUnknown);
impl IDxcBlobUtf16 {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pknown), ::std::mem::transmute(pcodepage)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetStringPointer(&self) -> super::super::Foundation::PWSTR {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetStringLength(&self) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IDxcBlobUtf16 {
    type Vtable = IDxcBlobUtf16_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2750959275, 4010, 18814, [163, 156, 238, 110, 214, 11, 45, 132]);
}
impl ::std::convert::From<IDxcBlobUtf16> for ::windows::runtime::IUnknown {
    fn from(value: IDxcBlobUtf16) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcBlobUtf16> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcBlobUtf16) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDxcBlobUtf16> for IDxcBlobEncoding {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf16> for IDxcBlobEncoding {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcBlobEncoding> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcBlobEncoding> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcBlobEncoding> for &IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcBlobEncoding> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IDxcBlobUtf16> for IDxcBlob {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf16> for IDxcBlob {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcBlob> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcBlob> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcBlob> for &IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcBlob> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf16_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknown: *mut super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::PWSTR,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> usize,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcBlobUtf8(pub ::windows::runtime::IUnknown);
impl IDxcBlobUtf8 {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBufferPointer(&self) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pknown), ::std::mem::transmute(pcodepage)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetStringPointer(&self) -> super::super::Foundation::PSTR {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetStringLength(&self) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IDxcBlobUtf8 {
    type Vtable = IDxcBlobUtf8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1034303177, 47729, 16420, [163, 1, 48, 203, 241, 37, 48, 91]);
}
impl ::std::convert::From<IDxcBlobUtf8> for ::windows::runtime::IUnknown {
    fn from(value: IDxcBlobUtf8) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcBlobUtf8> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcBlobUtf8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDxcBlobUtf8> for IDxcBlobEncoding {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf8> for IDxcBlobEncoding {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcBlobEncoding> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcBlobEncoding> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcBlobEncoding> for &IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcBlobEncoding> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::From<IDxcBlobUtf8> for IDxcBlob {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcBlobUtf8> for IDxcBlob {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcBlob> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcBlob> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcBlob> for &IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcBlob> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknown: *mut super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::PSTR,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> usize,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcCompiler(pub ::windows::runtime::IUnknown);
impl IDxcCompiler {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn Compile<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param8: ::windows::runtime::IntoParam<'a, IDxcIncludeHandler>>(
        &self,
        psource: Param0,
        psourcename: Param1,
        pentrypoint: Param2,
        ptargetprofile: Param3,
        parguments: *const super::super::Foundation::PWSTR,
        argcount: u32,
        pdefines: *const DxcDefine,
        definecount: u32,
        pincludehandler: Param8,
    ) -> ::windows::runtime::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            psource.into_param().abi(),
            psourcename.into_param().abi(),
            pentrypoint.into_param().abi(),
            ptargetprofile.into_param().abi(),
            ::std::mem::transmute(parguments),
            ::std::mem::transmute(argcount),
            ::std::mem::transmute(pdefines),
            ::std::mem::transmute(definecount),
            pincludehandler.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDxcOperationResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn Preprocess<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::runtime::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, parguments: *const super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: Param6) -> ::windows::runtime::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), psource.into_param().abi(), psourcename.into_param().abi(), ::std::mem::transmute(parguments), ::std::mem::transmute(argcount), ::std::mem::transmute(pdefines), ::std::mem::transmute(definecount), pincludehandler.into_param().abi(), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn Disassemble<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, psource: Param0) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), psource.into_param().abi(), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcCompiler {
    type Vtable = IDxcCompiler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2350975987, 287, 17442, [141, 112, 111, 154, 203, 141, 182, 23]);
}
impl ::std::convert::From<IDxcCompiler> for ::windows::runtime::IUnknown {
    fn from(value: IDxcCompiler) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcCompiler> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcCompiler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcCompiler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcCompiler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psource: ::windows::runtime::RawPtr, psourcename: super::super::Foundation::PWSTR, pentrypoint: super::super::Foundation::PWSTR, ptargetprofile: super::super::Foundation::PWSTR, parguments: *const super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psource: ::windows::runtime::RawPtr, psourcename: super::super::Foundation::PWSTR, parguments: *const super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psource: ::windows::runtime::RawPtr, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcCompiler2(pub ::windows::runtime::IUnknown);
impl IDxcCompiler2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn Compile<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param8: ::windows::runtime::IntoParam<'a, IDxcIncludeHandler>>(
        &self,
        psource: Param0,
        psourcename: Param1,
        pentrypoint: Param2,
        ptargetprofile: Param3,
        parguments: *const super::super::Foundation::PWSTR,
        argcount: u32,
        pdefines: *const DxcDefine,
        definecount: u32,
        pincludehandler: Param8,
    ) -> ::windows::runtime::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            psource.into_param().abi(),
            psourcename.into_param().abi(),
            pentrypoint.into_param().abi(),
            ptargetprofile.into_param().abi(),
            ::std::mem::transmute(parguments),
            ::std::mem::transmute(argcount),
            ::std::mem::transmute(pdefines),
            ::std::mem::transmute(definecount),
            pincludehandler.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDxcOperationResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn Preprocess<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::runtime::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, parguments: *const super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: Param6) -> ::windows::runtime::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), psource.into_param().abi(), psourcename.into_param().abi(), ::std::mem::transmute(parguments), ::std::mem::transmute(argcount), ::std::mem::transmute(pdefines), ::std::mem::transmute(definecount), pincludehandler.into_param().abi(), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn Disassemble<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, psource: Param0) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), psource.into_param().abi(), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn CompileWithDebug<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param8: ::windows::runtime::IntoParam<'a, IDxcIncludeHandler>>(
        &self,
        psource: Param0,
        psourcename: Param1,
        pentrypoint: Param2,
        ptargetprofile: Param3,
        parguments: *const super::super::Foundation::PWSTR,
        argcount: u32,
        pdefines: *const DxcDefine,
        definecount: u32,
        pincludehandler: Param8,
        ppresult: *mut ::std::option::Option<IDxcOperationResult>,
        ppdebugblobname: *mut super::super::Foundation::PWSTR,
        ppdebugblob: *mut ::std::option::Option<IDxcBlob>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            psource.into_param().abi(),
            psourcename.into_param().abi(),
            pentrypoint.into_param().abi(),
            ptargetprofile.into_param().abi(),
            ::std::mem::transmute(parguments),
            ::std::mem::transmute(argcount),
            ::std::mem::transmute(pdefines),
            ::std::mem::transmute(definecount),
            pincludehandler.into_param().abi(),
            ::std::mem::transmute(ppresult),
            ::std::mem::transmute(ppdebugblobname),
            ::std::mem::transmute(ppdebugblob),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDxcCompiler2 {
    type Vtable = IDxcCompiler2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2684725721, 47291, 17812, [181, 201, 14, 99, 59, 236, 77, 55]);
}
impl ::std::convert::From<IDxcCompiler2> for ::windows::runtime::IUnknown {
    fn from(value: IDxcCompiler2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcCompiler2> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcCompiler2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcCompiler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcCompiler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDxcCompiler2> for IDxcCompiler {
    fn from(value: IDxcCompiler2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcCompiler2> for IDxcCompiler {
    fn from(value: &IDxcCompiler2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcCompiler> for IDxcCompiler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcCompiler> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcCompiler> for &IDxcCompiler2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcCompiler> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psource: ::windows::runtime::RawPtr, psourcename: super::super::Foundation::PWSTR, pentrypoint: super::super::Foundation::PWSTR, ptargetprofile: super::super::Foundation::PWSTR, parguments: *const super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psource: ::windows::runtime::RawPtr, psourcename: super::super::Foundation::PWSTR, parguments: *const super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psource: ::windows::runtime::RawPtr, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psource: ::windows::runtime::RawPtr,
        psourcename: super::super::Foundation::PWSTR,
        pentrypoint: super::super::Foundation::PWSTR,
        ptargetprofile: super::super::Foundation::PWSTR,
        parguments: *const super::super::Foundation::PWSTR,
        argcount: u32,
        pdefines: *const DxcDefine,
        definecount: u32,
        pincludehandler: ::windows::runtime::RawPtr,
        ppresult: *mut ::windows::runtime::RawPtr,
        ppdebugblobname: *mut super::super::Foundation::PWSTR,
        ppdebugblob: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcCompiler3(pub ::windows::runtime::IUnknown);
impl IDxcCompiler3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn Compile<'a, Param3: ::windows::runtime::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: *const DxcBuffer, parguments: *const super::super::Foundation::PWSTR, argcount: u32, pincludehandler: Param3, riid: *const ::windows::runtime::GUID, ppresult: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(psource), ::std::mem::transmute(parguments), ::std::mem::transmute(argcount), pincludehandler.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppresult)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn Disassemble(&self, pobject: *const DxcBuffer, riid: *const ::windows::runtime::GUID, ppresult: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pobject), ::std::mem::transmute(riid), ::std::mem::transmute(ppresult)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDxcCompiler3 {
    type Vtable = IDxcCompiler3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(579552903, 23146, 18224, [144, 12, 151, 2, 178, 32, 63, 84]);
}
impl ::std::convert::From<IDxcCompiler3> for ::windows::runtime::IUnknown {
    fn from(value: IDxcCompiler3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcCompiler3> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcCompiler3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcCompiler3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcCompiler3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psource: *const DxcBuffer, parguments: *const super::super::Foundation::PWSTR, argcount: u32, pincludehandler: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppresult: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobject: *const DxcBuffer, riid: *const ::windows::runtime::GUID, ppresult: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcCompilerArgs(pub ::windows::runtime::IUnknown);
impl IDxcCompilerArgs {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetArguments(&self) -> *mut super::super::Foundation::PWSTR {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn AddArguments(&self, parguments: *const super::super::Foundation::PWSTR, argcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(parguments), ::std::mem::transmute(argcount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn AddArgumentsUTF8(&self, parguments: *const super::super::Foundation::PSTR, argcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(parguments), ::std::mem::transmute(argcount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn AddDefines(&self, pdefines: *const DxcDefine, definecount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdefines), ::std::mem::transmute(definecount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDxcCompilerArgs {
    type Vtable = IDxcCompilerArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1945108010, 28892, 17912, [150, 144, 239, 246, 76, 2, 66, 157]);
}
impl ::std::convert::From<IDxcCompilerArgs> for ::windows::runtime::IUnknown {
    fn from(value: IDxcCompilerArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcCompilerArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcCompilerArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcCompilerArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcCompilerArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompilerArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> *mut super::super::Foundation::PWSTR,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parguments: *const super::super::Foundation::PWSTR, argcount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parguments: *const super::super::Foundation::PSTR, argcount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdefines: *const DxcDefine, definecount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcContainerBuilder(pub ::windows::runtime::IUnknown);
impl IDxcContainerBuilder {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pdxilcontainerheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pdxilcontainerheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn AddPart<'a, Param1: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, fourcc: u32, psource: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(fourcc), psource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn RemovePart(&self, fourcc: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(fourcc)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn SerializeContainer(&self) -> ::windows::runtime::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcContainerBuilder {
    type Vtable = IDxcContainerBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(860561232, 8850, 19253, [153, 161, 37, 88, 141, 140, 23, 254]);
}
impl ::std::convert::From<IDxcContainerBuilder> for ::windows::runtime::IUnknown {
    fn from(value: IDxcContainerBuilder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcContainerBuilder> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcContainerBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcContainerBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcContainerBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdxilcontainerheader: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fourcc: u32, psource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fourcc: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcContainerReflection(pub ::windows::runtime::IUnknown);
impl IDxcContainerReflection {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pcontainer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pcontainer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetPartCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetPartKind(&self, idx: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(idx), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetPartContent(&self, idx: u32) -> ::windows::runtime::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(idx), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn FindFirstPartKind(&self, kind: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(kind), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetPartReflection(&self, idx: u32, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(idx), ::std::mem::transmute(iid), ::std::mem::transmute(ppvobject)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDxcContainerReflection {
    type Vtable = IDxcContainerReflection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3535936294, 33616, 19420, [151, 106, 51, 28, 230, 244, 197, 76]);
}
impl ::std::convert::From<IDxcContainerReflection> for ::windows::runtime::IUnknown {
    fn from(value: IDxcContainerReflection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcContainerReflection> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcContainerReflection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcContainerReflection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcContainerReflection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerReflection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontainer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presult: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, idx: u32, presult: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, idx: u32, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: u32, presult: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, idx: u32, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcExtraOutputs(pub ::windows::runtime::IUnknown);
impl IDxcExtraOutputs {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetOutput(&self, uindex: u32, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void, ppoutputtype: *mut ::std::option::Option<IDxcBlobUtf16>, ppoutputname: *mut ::std::option::Option<IDxcBlobUtf16>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(uindex), ::std::mem::transmute(iid), ::std::mem::transmute(ppvobject), ::std::mem::transmute(ppoutputtype), ::std::mem::transmute(ppoutputname)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDxcExtraOutputs {
    type Vtable = IDxcExtraOutputs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(832255906, 42434, 18762, [165, 222, 72, 1, 178, 250, 249, 137]);
}
impl ::std::convert::From<IDxcExtraOutputs> for ::windows::runtime::IUnknown {
    fn from(value: IDxcExtraOutputs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcExtraOutputs> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcExtraOutputs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcExtraOutputs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcExtraOutputs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcExtraOutputs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void, ppoutputtype: *mut ::windows::runtime::RawPtr, ppoutputname: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcIncludeHandler(pub ::windows::runtime::IUnknown);
impl IDxcIncludeHandler {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn LoadSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pfilename: Param0) -> ::windows::runtime::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pfilename.into_param().abi(), &mut result__).from_abi::<IDxcBlob>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcIncludeHandler {
    type Vtable = IDxcIncludeHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2137128061, 38157, 18047, [179, 227, 60, 2, 251, 73, 24, 124]);
}
impl ::std::convert::From<IDxcIncludeHandler> for ::windows::runtime::IUnknown {
    fn from(value: IDxcIncludeHandler) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcIncludeHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcIncludeHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcIncludeHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcIncludeHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcIncludeHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfilename: super::super::Foundation::PWSTR, ppincludesource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcLibrary(pub ::windows::runtime::IUnknown);
impl IDxcLibrary {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_System_Com`*"]
    pub unsafe fn SetMalloc<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IMalloc>>(&self, pmalloc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pmalloc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn CreateBlobFromBlob<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, offset: u32, length: u32) -> ::windows::runtime::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pblob.into_param().abi(), ::std::mem::transmute(offset), ::std::mem::transmute(length), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn CreateBlobFromFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pfilename: Param0, codepage: *const DXC_CP) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pfilename.into_param().abi(), ::std::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn CreateBlobWithEncodingFromPinned(&self, ptext: *const ::std::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptext), ::std::mem::transmute(size), ::std::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: *const ::std::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptext), ::std::mem::transmute(size), ::std::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_System_Com`*"]
    pub unsafe fn CreateBlobWithEncodingOnMalloc<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IMalloc>>(&self, ptext: *const ::std::ffi::c_void, pimalloc: Param1, size: u32, codepage: DXC_CP) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptext), pimalloc.into_param().abi(), ::std::mem::transmute(size), ::std::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn CreateIncludeHandler(&self) -> ::windows::runtime::Result<IDxcIncludeHandler> {
        let mut result__: <IDxcIncludeHandler as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDxcIncludeHandler>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_System_Com`*"]
    pub unsafe fn CreateStreamFromBlobReadOnly<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBlobAsUtf8<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBlobAsUtf16<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcLibrary {
    type Vtable = IDxcLibrary_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3844099527, 53644, 19516, [189, 251, 133, 22, 115, 152, 15, 231]);
}
impl ::std::convert::From<IDxcLibrary> for ::windows::runtime::IUnknown {
    fn from(value: IDxcLibrary) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcLibrary> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcLibrary) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcLibrary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcLibrary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLibrary_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmalloc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblob: ::windows::runtime::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfilename: super::super::Foundation::PWSTR, codepage: *const DXC_CP, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptext: *const ::std::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptext: *const ::std::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptext: *const ::std::ffi::c_void, pimalloc: ::windows::runtime::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblob: ::windows::runtime::RawPtr, ppstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblob: ::windows::runtime::RawPtr, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblob: ::windows::runtime::RawPtr, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcLinker(pub ::windows::runtime::IUnknown);
impl IDxcLinker {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn RegisterLibrary<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, plibname: Param0, plib: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), plibname.into_param().abi(), plib.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn Link<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pentryname: Param0, ptargetprofile: Param1, plibnames: *const super::super::Foundation::PWSTR, libcount: u32, parguments: *const super::super::Foundation::PWSTR, argcount: u32) -> ::windows::runtime::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pentryname.into_param().abi(), ptargetprofile.into_param().abi(), ::std::mem::transmute(plibnames), ::std::mem::transmute(libcount), ::std::mem::transmute(parguments), ::std::mem::transmute(argcount), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcLinker {
    type Vtable = IDxcLinker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4055219754, 25309, 17191, [161, 194, 66, 172, 30, 30, 120, 230]);
}
impl ::std::convert::From<IDxcLinker> for ::windows::runtime::IUnknown {
    fn from(value: IDxcLinker) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcLinker> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcLinker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcLinker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcLinker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLinker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plibname: super::super::Foundation::PWSTR, plib: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pentryname: super::super::Foundation::PWSTR, ptargetprofile: super::super::Foundation::PWSTR, plibnames: *const super::super::Foundation::PWSTR, libcount: u32, parguments: *const super::super::Foundation::PWSTR, argcount: u32, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcOperationResult(pub ::windows::runtime::IUnknown);
impl IDxcOperationResult {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetResult(&self) -> ::windows::runtime::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetErrorBuffer(&self) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcOperationResult {
    type Vtable = IDxcOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3470477386, 54505, 17498, [185, 145, 202, 33, 202, 21, 125, 194]);
}
impl ::std::convert::From<IDxcOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: IDxcOperationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcOperationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOperationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pperrors: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcOptimizer(pub ::windows::runtime::IUnknown);
impl IDxcOptimizer {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetAvailablePassCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetAvailablePass(&self, index: u32) -> ::windows::runtime::Result<IDxcOptimizerPass> {
        let mut result__: <IDxcOptimizerPass as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<IDxcOptimizerPass>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn RunOptimizer<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, ppoptions: *const super::super::Foundation::PWSTR, optioncount: u32, poutputmodule: *mut ::std::option::Option<IDxcBlob>, ppoutputtext: *mut ::std::option::Option<IDxcBlobEncoding>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pblob.into_param().abi(), ::std::mem::transmute(ppoptions), ::std::mem::transmute(optioncount), ::std::mem::transmute(poutputmodule), ::std::mem::transmute(ppoutputtext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDxcOptimizer {
    type Vtable = IDxcOptimizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(628362798, 40122, 16411, [145, 25, 79, 180, 47, 57, 242, 112]);
}
impl ::std::convert::From<IDxcOptimizer> for ::windows::runtime::IUnknown {
    fn from(value: IDxcOptimizer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcOptimizer> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcOptimizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcOptimizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcOptimizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblob: ::windows::runtime::RawPtr, ppoptions: *const super::super::Foundation::PWSTR, optioncount: u32, poutputmodule: *mut ::windows::runtime::RawPtr, ppoutputtext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcOptimizerPass(pub ::windows::runtime::IUnknown);
impl IDxcOptimizerPass {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetOptionName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetOptionArgCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetOptionArgName(&self, argindex: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(argindex), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetOptionArgDescription(&self, argindex: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(argindex), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcOptimizerPass {
    type Vtable = IDxcOptimizerPass_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2922174367, 52258, 17727, [155, 107, 177, 36, 231, 165, 32, 76]);
}
impl ::std::convert::From<IDxcOptimizerPass> for ::windows::runtime::IUnknown {
    fn from(value: IDxcOptimizerPass) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcOptimizerPass> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcOptimizerPass) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcOptimizerPass {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcOptimizerPass {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizerPass_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, argindex: u32, ppresult: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, argindex: u32, ppresult: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcPdbUtils(pub ::windows::runtime::IUnknown);
impl IDxcPdbUtils {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, ppdbordxil: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ppdbordxil.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetSourceCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetSource(&self, uindex: u32) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(uindex), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetSourceName(&self, uindex: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(uindex), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetFlagCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetFlag(&self, uindex: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(uindex), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetArgCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetArg(&self, uindex: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(uindex), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetArgPairCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetArgPair(&self, uindex: u32, pname: *mut super::super::Foundation::BSTR, pvalue: *mut super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(uindex), ::std::mem::transmute(pname), ::std::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetDefineCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetDefine(&self, uindex: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(uindex), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetTargetProfile(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetEntryPoint(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetMainFileName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetHash(&self) -> ::windows::runtime::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn IsFullPDB(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetFullPDB(&self) -> ::windows::runtime::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetVersionInfo(&self) -> ::windows::runtime::Result<IDxcVersionInfo> {
        let mut result__: <IDxcVersionInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDxcVersionInfo>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn SetCompiler<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcCompiler3>>(&self, pcompiler: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pcompiler.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn CompileForFullPDB(&self) -> ::windows::runtime::Result<IDxcResult> {
        let mut result__: <IDxcResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDxcResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn OverrideArgs(&self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(pargpairs), ::std::mem::transmute(unumargpairs)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn OverrideRootSignature<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, prootsignature: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), prootsignature.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDxcPdbUtils {
    type Vtable = IDxcPdbUtils_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3871958142, 40298, 19515, [185, 76, 82, 75, 90, 108, 52, 61]);
}
impl ::std::convert::From<IDxcPdbUtils> for ::windows::runtime::IUnknown {
    fn from(value: IDxcPdbUtils) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcPdbUtils> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcPdbUtils) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcPdbUtils {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcPdbUtils {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcPdbUtils_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdbordxil: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, presult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, presult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, presult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, pname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvalue: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, presult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfullpdb: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppversioninfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcompiler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prootsignature: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcResult(pub ::windows::runtime::IUnknown);
impl IDxcResult {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetResult(&self) -> ::windows::runtime::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetErrorBuffer(&self) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn HasOutput(&self, dxcoutkind: DXC_OUT_KIND) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dxcoutkind)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetOutput(&self, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void, ppoutputname: *mut ::std::option::Option<IDxcBlobUtf16>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dxcoutkind), ::std::mem::transmute(iid), ::std::mem::transmute(ppvobject), ::std::mem::transmute(ppoutputname)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetNumOutputs(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(index)))
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn PrimaryOutput(&self) -> DXC_OUT_KIND {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IDxcResult {
    type Vtable = IDxcResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1479830746, 56807, 17559, [148, 97, 111, 135, 175, 94, 6, 89]);
}
impl ::std::convert::From<IDxcResult> for ::windows::runtime::IUnknown {
    fn from(value: IDxcResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcResult> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDxcResult> for IDxcOperationResult {
    fn from(value: IDxcResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcResult> for IDxcOperationResult {
    fn from(value: &IDxcResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcOperationResult> for IDxcResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcOperationResult> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcOperationResult> for &IDxcResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcOperationResult> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pperrors: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dxcoutkind: DXC_OUT_KIND) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void, ppoutputname: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32) -> DXC_OUT_KIND,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> DXC_OUT_KIND,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcUtils(pub ::windows::runtime::IUnknown);
impl IDxcUtils {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn CreateBlobFromBlob<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, offset: u32, length: u32) -> ::windows::runtime::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pblob.into_param().abi(), ::std::mem::transmute(offset), ::std::mem::transmute(length), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn CreateBlobFromPinned(&self, pdata: *const ::std::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(size), ::std::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_System_Com`*"]
    pub unsafe fn MoveToBlob<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IMalloc>>(&self, pdata: *const ::std::ffi::c_void, pimalloc: Param1, size: u32, codepage: DXC_CP) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), pimalloc.into_param().abi(), ::std::mem::transmute(size), ::std::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn CreateBlob(&self, pdata: *const ::std::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(size), ::std::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn LoadFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pfilename: Param0, pcodepage: *const DXC_CP) -> ::windows::runtime::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pfilename.into_param().abi(), ::std::mem::transmute(pcodepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_System_Com`*"]
    pub unsafe fn CreateReadOnlyStreamFromBlob<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn CreateDefaultIncludeHandler(&self) -> ::windows::runtime::Result<IDxcIncludeHandler> {
        let mut result__: <IDxcIncludeHandler as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDxcIncludeHandler>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBlobAsUtf8<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::runtime::Result<IDxcBlobUtf8> {
        let mut result__: <IDxcBlobUtf8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<IDxcBlobUtf8>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetBlobAsUtf16<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::runtime::Result<IDxcBlobUtf16> {
        let mut result__: <IDxcBlobUtf16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<IDxcBlobUtf16>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetDxilContainerPart(&self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::std::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pshader), ::std::mem::transmute(dxcpart), ::std::mem::transmute(pppartdata), ::std::mem::transmute(ppartsizeinbytes)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn CreateReflection(&self, pdata: *const DxcBuffer, iid: *const ::windows::runtime::GUID, ppvreflection: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(iid), ::std::mem::transmute(ppvreflection)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Hlsl`, `Win32_Foundation`*"]
    pub unsafe fn BuildArguments<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psourcename: Param0, pentrypoint: Param1, ptargetprofile: Param2, parguments: *const super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32) -> ::windows::runtime::Result<IDxcCompilerArgs> {
        let mut result__: <IDxcCompilerArgs as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), psourcename.into_param().abi(), pentrypoint.into_param().abi(), ptargetprofile.into_param().abi(), ::std::mem::transmute(parguments), ::std::mem::transmute(argcount), ::std::mem::transmute(pdefines), ::std::mem::transmute(definecount), &mut result__).from_abi::<IDxcCompilerArgs>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetPDBContents<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, ppdbblob: Param0, pphash: *mut ::std::option::Option<IDxcBlob>, ppcontainer: *mut ::std::option::Option<IDxcBlob>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ppdbblob.into_param().abi(), ::std::mem::transmute(pphash), ::std::mem::transmute(ppcontainer)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDxcUtils {
    type Vtable = IDxcUtils_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1174783179, 8217, 18730, [173, 164, 101, 242, 11, 183, 214, 127]);
}
impl ::std::convert::From<IDxcUtils> for ::windows::runtime::IUnknown {
    fn from(value: IDxcUtils) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcUtils> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcUtils) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcUtils {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcUtils {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcUtils_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblob: ::windows::runtime::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const ::std::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const ::std::ffi::c_void, pimalloc: ::windows::runtime::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const ::std::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfilename: super::super::Foundation::PWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblob: ::windows::runtime::RawPtr, ppstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblob: ::windows::runtime::RawPtr, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblob: ::windows::runtime::RawPtr, pblobencoding: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::std::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const DxcBuffer, iid: *const ::windows::runtime::GUID, ppvreflection: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psourcename: super::super::Foundation::PWSTR, pentrypoint: super::super::Foundation::PWSTR, ptargetprofile: super::super::Foundation::PWSTR, parguments: *const super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdbblob: ::windows::runtime::RawPtr, pphash: *mut ::windows::runtime::RawPtr, ppcontainer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcValidator(pub ::windows::runtime::IUnknown);
impl IDxcValidator {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn Validate<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32) -> ::windows::runtime::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pshader.into_param().abi(), ::std::mem::transmute(flags), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcValidator {
    type Vtable = IDxcValidator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2800233426, 8151, 18470, [152, 17, 40, 87, 231, 151, 244, 154]);
}
impl ::std::convert::From<IDxcValidator> for ::windows::runtime::IUnknown {
    fn from(value: IDxcValidator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcValidator> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcValidator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcValidator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcValidator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pshader: ::windows::runtime::RawPtr, flags: u32, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcValidator2(pub ::windows::runtime::IUnknown);
impl IDxcValidator2 {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn Validate<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32) -> ::windows::runtime::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pshader.into_param().abi(), ::std::mem::transmute(flags), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn ValidateWithDebug<'a, Param0: ::windows::runtime::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32, poptdebugbitcode: *const DxcBuffer) -> ::windows::runtime::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pshader.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(poptdebugbitcode), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcValidator2 {
    type Vtable = IDxcValidator2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1166942161, 45490, 18256, [166, 225, 156, 16, 240, 59, 237, 146]);
}
impl ::std::convert::From<IDxcValidator2> for ::windows::runtime::IUnknown {
    fn from(value: IDxcValidator2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcValidator2> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcValidator2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcValidator2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcValidator2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDxcValidator2> for IDxcValidator {
    fn from(value: IDxcValidator2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcValidator2> for IDxcValidator {
    fn from(value: &IDxcValidator2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcValidator> for IDxcValidator2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcValidator> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcValidator> for &IDxcValidator2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcValidator> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pshader: ::windows::runtime::RawPtr, flags: u32, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pshader: ::windows::runtime::RawPtr, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcVersionInfo(pub ::windows::runtime::IUnknown);
impl IDxcVersionInfo {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmajor), ::std::mem::transmute(pminor)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcVersionInfo {
    type Vtable = IDxcVersionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2957990736, 8281, 20242, [168, 255, 161, 224, 205, 225, 204, 126]);
}
impl ::std::convert::From<IDxcVersionInfo> for ::windows::runtime::IUnknown {
    fn from(value: IDxcVersionInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcVersionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcVersionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcVersionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcVersionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmajor: *mut u32, pminor: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflags: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcVersionInfo2(pub ::windows::runtime::IUnknown);
impl IDxcVersionInfo2 {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmajor), ::std::mem::transmute(pminor)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetCommitInfo(&self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommitcount), ::std::mem::transmute(pcommithash)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDxcVersionInfo2 {
    type Vtable = IDxcVersionInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4217963716, 17136, 19298, [156, 70, 152, 58, 247, 218, 124, 131]);
}
impl ::std::convert::From<IDxcVersionInfo2> for ::windows::runtime::IUnknown {
    fn from(value: IDxcVersionInfo2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcVersionInfo2> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcVersionInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDxcVersionInfo2> for IDxcVersionInfo {
    fn from(value: IDxcVersionInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDxcVersionInfo2> for IDxcVersionInfo {
    fn from(value: &IDxcVersionInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcVersionInfo> for IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcVersionInfo> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDxcVersionInfo> for &IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDxcVersionInfo> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmajor: *mut u32, pminor: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDxcVersionInfo3(pub ::windows::runtime::IUnknown);
impl IDxcVersionInfo3 {
    #[doc = "*Required features: `Win32_Graphics_Hlsl`*"]
    pub unsafe fn GetCustomVersionString(&self) -> ::windows::runtime::Result<*mut i8> {
        let mut result__: <*mut i8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut i8>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDxcVersionInfo3 {
    type Vtable = IDxcVersionInfo3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1578362947, 40229, 18236, [154, 210, 3, 178, 208, 180, 75, 30]);
}
impl ::std::convert::From<IDxcVersionInfo3> for ::windows::runtime::IUnknown {
    fn from(value: IDxcVersionInfo3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDxcVersionInfo3> for ::windows::runtime::IUnknown {
    fn from(value: &IDxcVersionInfo3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDxcVersionInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDxcVersionInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pversionstring: *mut *mut i8) -> ::windows::runtime::HRESULT,
);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D11"))]
pub type pD3DCompile = unsafe extern "system" fn(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, pfilename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D11::D3D_SHADER_MACRO, pinclude: ::windows::runtime::RawPtr, pentrypoint: super::super::Foundation::PSTR, ptarget: super::super::Foundation::PSTR, flags1: u32, flags2: u32, ppcode: *mut ::windows::runtime::RawPtr, pperrormsgs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D11"))]
pub type pD3DDisassemble = unsafe extern "system" fn(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: super::super::Foundation::PSTR, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D11"))]
pub type pD3DPreprocess = unsafe extern "system" fn(psrcdata: *const ::std::ffi::c_void, srcdatasize: usize, pfilename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D11::D3D_SHADER_MACRO, pinclude: ::windows::runtime::RawPtr, ppcodetext: *mut ::windows::runtime::RawPtr, pperrormsgs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
