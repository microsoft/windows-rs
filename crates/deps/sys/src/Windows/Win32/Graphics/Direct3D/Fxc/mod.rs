#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DCompile();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DCompile2();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DCompileFromFile();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DCompressShaders();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DCreateBlob();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub fn D3DCreateFunctionLinkingGraph();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub fn D3DCreateLinker();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DDecompressShaders();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DDisassemble();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D10`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D10")]
    pub fn D3DDisassemble10Effect();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DDisassembleRegion();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetBlobPart();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetDebugInfo();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetInputAndOutputSignatureBlob();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetInputSignatureBlob();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetOutputSignatureBlob();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DGetTraceInstructionOffsets();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub fn D3DLoadModule();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPreprocess();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DReadFileToBlob();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DReflect();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DReflectLibrary();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DSetBlobPart();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`*"]
    pub fn D3DStripShader();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Fxc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DWriteBlobToFile();
}
