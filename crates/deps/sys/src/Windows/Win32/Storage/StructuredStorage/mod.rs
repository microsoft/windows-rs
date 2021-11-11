#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn JET_API_PTR();
    fn JET_HANDLE();
    fn JET_INSTANCE();
    fn JET_SESID();
    fn JET_TABLEID();
}
