#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn D3D11CreateDevice();
    fn D3D11CreateDeviceAndSwapChain();
    fn D3DDisassemble11Trace();
    fn D3DX11CreateFFT();
    fn D3DX11CreateFFT1DComplex();
    fn D3DX11CreateFFT1DReal();
    fn D3DX11CreateFFT2DComplex();
    fn D3DX11CreateFFT2DReal();
    fn D3DX11CreateFFT3DComplex();
    fn D3DX11CreateFFT3DReal();
    fn D3DX11CreateScan();
    fn D3DX11CreateSegmentedScan();
}
