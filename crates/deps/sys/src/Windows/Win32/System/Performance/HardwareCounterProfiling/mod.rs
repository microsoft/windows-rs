#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DisableThreadProfiling();
    fn EnableThreadProfiling();
    fn HARDWARE_COUNTER_DATA();
    fn HARDWARE_COUNTER_TYPE();
    fn PERFORMANCE_DATA();
    fn QueryThreadProfiling();
    fn ReadThreadProfilingData();
}
