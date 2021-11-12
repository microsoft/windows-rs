#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CustomSensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CustomSensorReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CustomSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomSensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomSensor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomSensorReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomSensorReading2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomSensorStatics(pub *mut ::core::ffi::c_void);
