#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CustomSensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CustomSensor {}
impl ::core::clone::Clone for CustomSensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CustomSensorReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CustomSensorReading {}
impl ::core::clone::Clone for CustomSensorReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CustomSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CustomSensorReadingChangedEventArgs {}
impl ::core::clone::Clone for CustomSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomSensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomSensor {}
impl ::core::clone::Clone for ICustomSensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomSensor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomSensor2 {}
impl ::core::clone::Clone for ICustomSensor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomSensorReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomSensorReading {}
impl ::core::clone::Clone for ICustomSensorReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomSensorReading2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomSensorReading2 {}
impl ::core::clone::Clone for ICustomSensorReading2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomSensorReadingChangedEventArgs {}
impl ::core::clone::Clone for ICustomSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomSensorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomSensorStatics {}
impl ::core::clone::Clone for ICustomSensorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
