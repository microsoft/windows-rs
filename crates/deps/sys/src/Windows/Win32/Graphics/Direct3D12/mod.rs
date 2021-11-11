#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn D3D12CreateDevice();
    fn D3D12CreateRootSignatureDeserializer();
    fn D3D12CreateVersionedRootSignatureDeserializer();
    fn D3D12EnableExperimentalFeatures();
    fn D3D12GetDebugInterface();
    fn D3D12GetInterface();
    fn D3D12SerializeRootSignature();
    fn D3D12SerializeVersionedRootSignature();
}
