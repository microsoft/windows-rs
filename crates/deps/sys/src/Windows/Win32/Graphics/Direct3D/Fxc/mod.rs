#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DCompile(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, psourcename: super::super::super::Foundation::PSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: ::windows::runtime::RawPtr, pentrypoint: super::super::super::Foundation::PSTR, ptarget: super::super::super::Foundation::PSTR, flags1: u32, flags2: u32, ppcode: *mut ::windows::runtime::RawPtr, pperrormsgs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DCompile2(
        psrcdata: *const ::core::ffi::c_void,
        srcdatasize: usize,
        psourcename: super::super::super::Foundation::PSTR,
        pdefines: *const super::D3D_SHADER_MACRO,
        pinclude: ::windows::runtime::RawPtr,
        pentrypoint: super::super::super::Foundation::PSTR,
        ptarget: super::super::super::Foundation::PSTR,
        flags1: u32,
        flags2: u32,
        secondarydataflags: u32,
        psecondarydata: *const ::core::ffi::c_void,
        secondarydatasize: usize,
        ppcode: *mut ::windows::runtime::RawPtr,
        pperrormsgs: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DCompileFromFile(pfilename: super::super::super::Foundation::PWSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: ::windows::runtime::RawPtr, pentrypoint: super::super::super::Foundation::PSTR, ptarget: super::super::super::Foundation::PSTR, flags1: u32, flags2: u32, ppcode: *mut ::windows::runtime::RawPtr, pperrormsgs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DCompressShaders(unumshaders: u32, pshaderdata: *const D3D_SHADER_DATA, uflags: u32, ppcompresseddata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DCreateBlob(size: usize, ppblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub fn D3DCreateFunctionLinkingGraph(uflags: u32, ppfunctionlinkinggraph: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub fn D3DCreateLinker(pplinker: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DDecompressShaders(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, unumshaders: u32, ustartindex: u32, pindices: *const u32, uflags: u32, ppshaders: *mut ::windows::runtime::RawPtr, ptotalshaders: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DDisassemble(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: super::super::super::Foundation::PSTR, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D10`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D10")]
    pub fn D3DDisassemble10Effect(peffect: ::windows::runtime::RawPtr, flags: u32, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DDisassembleRegion(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, szcomments: super::super::super::Foundation::PSTR, startbyteoffset: usize, numinsts: usize, pfinishbyteoffset: *mut usize, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetBlobPart(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, pppart: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetDebugInfo(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppdebuginfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetInputAndOutputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetInputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetOutputSignatureBlob(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ppsignatureblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetTraceInstructionOffsets(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, flags: u32, startinstindex: usize, numinsts: usize, poffsets: *mut usize, ptotalinsts: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub fn D3DLoadModule(psrcdata: *const ::core::ffi::c_void, cbsrcdatasize: usize, ppmodule: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPreprocess(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, psourcename: super::super::super::Foundation::PSTR, pdefines: *const super::D3D_SHADER_MACRO, pinclude: ::windows::runtime::RawPtr, ppcodetext: *mut ::windows::runtime::RawPtr, pperrormsgs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DReadFileToBlob(pfilename: super::super::super::Foundation::PWSTR, ppcontents: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DReflect(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, pinterface: *const ::windows::runtime::GUID, ppreflector: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DReflectLibrary(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, riid: *const ::windows::runtime::GUID, ppreflector: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DSetBlobPart(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, part: D3D_BLOB_PART, flags: u32, ppart: *const ::core::ffi::c_void, partsize: usize, ppnewshader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DStripShader(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ustripflags: u32, ppstrippedblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DWriteBlobToFile(pblob: ::windows::runtime::RawPtr, pfilename: super::super::super::Foundation::PWSTR, boverwrite: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
}
