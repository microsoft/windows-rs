#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub mod Printing;
#[link(name = "windows")]
extern "system" {
    fn AbortDoc();
    fn DeviceCapabilitiesA();
    fn DeviceCapabilitiesW();
    fn EndDoc();
    fn EndPage();
    fn Escape();
    fn ExtEscape();
    fn PrintWindow();
    fn SetAbortProc();
    fn StartDocA();
    fn StartDocW();
    fn StartPage();
}
