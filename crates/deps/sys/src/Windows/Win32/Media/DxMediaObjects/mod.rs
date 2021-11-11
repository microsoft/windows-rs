#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DMOEnum();
    fn DMOGetName();
    fn DMOGetTypes();
    fn DMORegister();
    fn DMOUnregister();
    fn MoCopyMediaType();
    fn MoCreateMediaType();
    fn MoDeleteMediaType();
    fn MoDuplicateMediaType();
    fn MoFreeMediaType();
    fn MoInitMediaType();
}
