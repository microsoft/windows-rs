#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CreateDirect3D11DeviceFromDXGIDevice();
    fn CreateDirect3D11SurfaceFromDXGISurface();
}
