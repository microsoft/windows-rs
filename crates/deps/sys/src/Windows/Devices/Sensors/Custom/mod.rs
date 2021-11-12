#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CustomSensor(i32);
pub struct CustomSensorReading(i32);
pub struct CustomSensorReadingChangedEventArgs(i32);
pub struct ICustomSensor(i32);
pub struct ICustomSensor2(i32);
pub struct ICustomSensorReading(i32);
pub struct ICustomSensorReading2(i32);
pub struct ICustomSensorReadingChangedEventArgs(i32);
pub struct ICustomSensorStatics(i32);
