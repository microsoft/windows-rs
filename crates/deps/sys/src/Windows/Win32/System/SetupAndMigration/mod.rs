#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn OOBEComplete();
    fn OOBE_COMPLETED_CALLBACK();
    fn RegisterWaitUntilOOBECompleted();
    fn UnregisterWaitUntilOOBECompleted();
}
