#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CustomSensor(i32);
pub struct CustomSensorReading(i32);
pub struct CustomSensorReadingChangedEventArgs(i32);
pub struct ICustomSensor(pub *mut ::core::ffi::c_void);
pub struct ICustomSensor2(pub *mut ::core::ffi::c_void);
pub struct ICustomSensorReading(pub *mut ::core::ffi::c_void);
pub struct ICustomSensorReading2(pub *mut ::core::ffi::c_void);
pub struct ICustomSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ICustomSensorStatics(pub *mut ::core::ffi::c_void);
