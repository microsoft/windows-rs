#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DirectDrawCreate();
    fn DirectDrawCreateClipper();
    fn DirectDrawCreateEx();
    fn DirectDrawEnumerateA();
    fn DirectDrawEnumerateExA();
    fn DirectDrawEnumerateExW();
    fn DirectDrawEnumerateW();
}
