#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    fn CreateDXGIFactory();
    fn CreateDXGIFactory1();
    fn CreateDXGIFactory2();
    fn DXGIDeclareAdapterRemovalSupport();
    fn DXGIGetDebugInterface1();
}
