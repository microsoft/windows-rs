#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn D3DCOMPILER_DLL();
    fn D3DCOMPILE_OPTIMIZATION_LEVEL2();
    fn D3D_COMPILE_STANDARD_FILE_INCLUDE();
}
