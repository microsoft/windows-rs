#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IRadialControllerIndependentInputSource();
    fn IRadialControllerIndependentInputSource2();
    fn IRadialControllerIndependentInputSourceStatics();
    fn RadialControllerIndependentInputSource();
}
