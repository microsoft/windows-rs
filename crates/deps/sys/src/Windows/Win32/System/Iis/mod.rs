#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn GetExtensionVersion();
    fn GetFilterVersion();
    fn HttpExtensionProc();
    fn HttpFilterProc();
}
