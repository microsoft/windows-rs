#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct Battery(i32);
pub struct IBattery(i32);
pub struct IBatteryStatics(i32);
