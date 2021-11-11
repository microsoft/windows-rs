#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Direct3D`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10CompileEffectFromMemory();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Direct3D`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10CompileShader();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10CreateBlob();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Dxgi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    pub fn D3D10CreateDevice();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Dxgi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    pub fn D3D10CreateDevice1();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Dxgi`, `Win32_Graphics_Dxgi_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
    pub fn D3D10CreateDeviceAndSwapChain();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Dxgi`, `Win32_Graphics_Dxgi_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
    pub fn D3D10CreateDeviceAndSwapChain1();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10CreateEffectFromMemory();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10CreateEffectPoolFromMemory();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10CreateStateBlock();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Direct3D`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10DisassembleEffect();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Direct3D`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10DisassembleShader();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10GetGeometryShaderProfile();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetInputAndOutputSignatureBlob();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetInputSignatureBlob();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetOutputSignatureBlob();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10GetPixelShaderProfile();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetShaderDebugInfo();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10GetVertexShaderProfile();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Direct3D`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10PreprocessShader();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10ReflectShader();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskDifference();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskDisableAll();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskDisableCapture();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskEnableAll();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskEnableCapture();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10StateBlockMaskGetSetting();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskIntersect();
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskUnion();
}
