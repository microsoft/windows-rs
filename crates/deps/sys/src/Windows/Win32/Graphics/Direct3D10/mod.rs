#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn D3D10CompileEffectFromMemory();
    fn D3D10CompileShader();
    fn D3D10CreateBlob();
    fn D3D10CreateDevice();
    fn D3D10CreateDevice1();
    fn D3D10CreateDeviceAndSwapChain();
    fn D3D10CreateDeviceAndSwapChain1();
    fn D3D10CreateEffectFromMemory();
    fn D3D10CreateEffectPoolFromMemory();
    fn D3D10CreateStateBlock();
    fn D3D10DisassembleEffect();
    fn D3D10DisassembleShader();
    fn D3D10GetGeometryShaderProfile();
    fn D3D10GetInputAndOutputSignatureBlob();
    fn D3D10GetInputSignatureBlob();
    fn D3D10GetOutputSignatureBlob();
    fn D3D10GetPixelShaderProfile();
    fn D3D10GetShaderDebugInfo();
    fn D3D10GetVertexShaderProfile();
    fn D3D10PreprocessShader();
    fn D3D10ReflectShader();
    fn D3D10StateBlockMaskDifference();
    fn D3D10StateBlockMaskDisableAll();
    fn D3D10StateBlockMaskDisableCapture();
    fn D3D10StateBlockMaskEnableAll();
    fn D3D10StateBlockMaskEnableCapture();
    fn D3D10StateBlockMaskGetSetting();
    fn D3D10StateBlockMaskIntersect();
    fn D3D10StateBlockMaskUnion();
}
