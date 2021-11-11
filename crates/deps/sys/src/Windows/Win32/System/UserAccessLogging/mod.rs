#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn UAL_DATA_BLOB();
    fn UalInstrument();
    fn UalRegisterProduct();
    fn UalStart();
    fn UalStop();
}
