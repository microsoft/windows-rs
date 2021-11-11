#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn D3D11On12CreateDevice();
    fn D3D11_RESOURCE_FLAGS();
    fn ID3D11On12Device();
    fn ID3D11On12Device1();
    fn ID3D11On12Device2();
    fn PFN_D3D11ON12_CREATE_DEVICE();
}
