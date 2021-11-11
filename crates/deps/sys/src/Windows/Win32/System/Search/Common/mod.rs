#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CONDITION_OPERATION();
    fn CONDITION_TYPE();
}
