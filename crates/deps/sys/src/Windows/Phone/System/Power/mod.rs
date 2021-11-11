#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPowerManagerStatics();
    fn IPowerManagerStatics2();
    fn PowerManager();
    fn PowerSavingMode();
}
