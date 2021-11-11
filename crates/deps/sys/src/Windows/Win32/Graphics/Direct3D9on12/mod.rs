#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn D3D9ON12_ARGS();
    fn Direct3DCreate9On12();
    fn Direct3DCreate9On12Ex();
    fn IDirect3DDevice9On12();
    fn MAX_D3D9ON12_QUEUES();
    fn PFN_Direct3DCreate9On12();
    fn PFN_Direct3DCreate9On12Ex();
}
