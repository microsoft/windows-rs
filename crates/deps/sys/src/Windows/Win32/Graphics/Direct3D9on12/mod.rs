#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn Direct3DCreate9On12();
    fn Direct3DCreate9On12Ex();
}
