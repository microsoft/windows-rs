#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn OPERATION_END_PARAMETERS();
    fn OPERATION_END_PARAMETERS_FLAGS();
    fn OPERATION_START_FLAGS();
    fn OPERATION_START_PARAMETERS();
    fn OperationEnd();
    fn OperationStart();
}
