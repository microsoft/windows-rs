#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn FCIAddFile();
    fn FCICreate();
    fn FCIDestroy();
    fn FCIFlushCabinet();
    fn FCIFlushFolder();
    fn FDICopy();
    fn FDICreate();
    fn FDIDestroy();
    fn FDIIsCabinet();
    fn FDITruncateCabinet();
}
