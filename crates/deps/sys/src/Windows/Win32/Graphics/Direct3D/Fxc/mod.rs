#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn D3DCompile();
    fn D3DCompile2();
    fn D3DCompileFromFile();
    fn D3DCompressShaders();
    fn D3DCreateBlob();
    fn D3DCreateFunctionLinkingGraph();
    fn D3DCreateLinker();
    fn D3DDecompressShaders();
    fn D3DDisassemble();
    fn D3DDisassemble10Effect();
    fn D3DDisassembleRegion();
    fn D3DGetBlobPart();
    fn D3DGetDebugInfo();
    fn D3DGetInputAndOutputSignatureBlob();
    fn D3DGetInputSignatureBlob();
    fn D3DGetOutputSignatureBlob();
    fn D3DGetTraceInstructionOffsets();
    fn D3DLoadModule();
    fn D3DPreprocess();
    fn D3DReadFileToBlob();
    fn D3DReflect();
    fn D3DReflectLibrary();
    fn D3DSetBlobPart();
    fn D3DStripShader();
    fn D3DWriteBlobToFile();
}
