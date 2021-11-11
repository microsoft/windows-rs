#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12CreateDevice();
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12CreateRootSignatureDeserializer();
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12CreateVersionedRootSignatureDeserializer();
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12EnableExperimentalFeatures();
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12GetDebugInterface();
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12GetInterface();
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12SerializeRootSignature();
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12SerializeVersionedRootSignature();
}
