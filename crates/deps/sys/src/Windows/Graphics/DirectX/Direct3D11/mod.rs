#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn Direct3DBindings();
    fn Direct3DMultisampleDescription();
    fn Direct3DSurfaceDescription();
    fn Direct3DUsage();
    fn IDirect3DDevice();
    fn IDirect3DSurface();
}
