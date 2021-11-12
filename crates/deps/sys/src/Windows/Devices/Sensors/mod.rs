#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Sensors_Custom")]
pub mod Custom;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Accelerometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Accelerometer {}
impl ::core::clone::Clone for Accelerometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AccelerometerDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AccelerometerDataThreshold {}
impl ::core::clone::Clone for AccelerometerDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AccelerometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AccelerometerReading {}
impl ::core::clone::Clone for AccelerometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AccelerometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AccelerometerReadingChangedEventArgs {}
impl ::core::clone::Clone for AccelerometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AccelerometerReadingType(pub i32);
impl AccelerometerReadingType {
    pub const Standard: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Gravity: Self = Self(2i32);
}
impl ::core::marker::Copy for AccelerometerReadingType {}
impl ::core::clone::Clone for AccelerometerReadingType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AccelerometerShakenEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AccelerometerShakenEventArgs {}
impl ::core::clone::Clone for AccelerometerShakenEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ActivitySensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ActivitySensor {}
impl ::core::clone::Clone for ActivitySensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ActivitySensorReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ActivitySensorReading {}
impl ::core::clone::Clone for ActivitySensorReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ActivitySensorReadingChangeReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ActivitySensorReadingChangeReport {}
impl ::core::clone::Clone for ActivitySensorReadingChangeReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ActivitySensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ActivitySensorReadingChangedEventArgs {}
impl ::core::clone::Clone for ActivitySensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ActivitySensorReadingConfidence(pub i32);
impl ActivitySensorReadingConfidence {
    pub const High: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
}
impl ::core::marker::Copy for ActivitySensorReadingConfidence {}
impl ::core::clone::Clone for ActivitySensorReadingConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ActivitySensorTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ActivitySensorTriggerDetails {}
impl ::core::clone::Clone for ActivitySensorTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ActivityType(pub i32);
impl ActivityType {
    pub const Unknown: Self = Self(0i32);
    pub const Idle: Self = Self(1i32);
    pub const Stationary: Self = Self(2i32);
    pub const Fidgeting: Self = Self(3i32);
    pub const Walking: Self = Self(4i32);
    pub const Running: Self = Self(5i32);
    pub const InVehicle: Self = Self(6i32);
    pub const Biking: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivityType {}
impl ::core::clone::Clone for ActivityType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Altimeter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Altimeter {}
impl ::core::clone::Clone for Altimeter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AltimeterReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AltimeterReading {}
impl ::core::clone::Clone for AltimeterReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AltimeterReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AltimeterReadingChangedEventArgs {}
impl ::core::clone::Clone for AltimeterReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Barometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Barometer {}
impl ::core::clone::Clone for Barometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarometerDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarometerDataThreshold {}
impl ::core::clone::Clone for BarometerDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarometerReading {}
impl ::core::clone::Clone for BarometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarometerReadingChangedEventArgs {}
impl ::core::clone::Clone for BarometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Compass(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Compass {}
impl ::core::clone::Clone for Compass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompassDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompassDataThreshold {}
impl ::core::clone::Clone for CompassDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompassReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompassReading {}
impl ::core::clone::Clone for CompassReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompassReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompassReadingChangedEventArgs {}
impl ::core::clone::Clone for CompassReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Gyrometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Gyrometer {}
impl ::core::clone::Clone for Gyrometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GyrometerDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GyrometerDataThreshold {}
impl ::core::clone::Clone for GyrometerDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GyrometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GyrometerReading {}
impl ::core::clone::Clone for GyrometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GyrometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GyrometerReadingChangedEventArgs {}
impl ::core::clone::Clone for GyrometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HingeAngleReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HingeAngleReading {}
impl ::core::clone::Clone for HingeAngleReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HingeAngleSensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HingeAngleSensor {}
impl ::core::clone::Clone for HingeAngleSensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HingeAngleSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HingeAngleSensorReadingChangedEventArgs {}
impl ::core::clone::Clone for HingeAngleSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometer {}
impl ::core::clone::Clone for IAccelerometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometer2 {}
impl ::core::clone::Clone for IAccelerometer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometer3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometer3 {}
impl ::core::clone::Clone for IAccelerometer3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometer4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometer4 {}
impl ::core::clone::Clone for IAccelerometer4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometer5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometer5 {}
impl ::core::clone::Clone for IAccelerometer5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometerDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometerDataThreshold {}
impl ::core::clone::Clone for IAccelerometerDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometerDeviceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometerDeviceId {}
impl ::core::clone::Clone for IAccelerometerDeviceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometerReading {}
impl ::core::clone::Clone for IAccelerometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometerReading2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometerReading2 {}
impl ::core::clone::Clone for IAccelerometerReading2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometerReadingChangedEventArgs {}
impl ::core::clone::Clone for IAccelerometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometerShakenEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometerShakenEventArgs {}
impl ::core::clone::Clone for IAccelerometerShakenEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometerStatics {}
impl ::core::clone::Clone for IAccelerometerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometerStatics2 {}
impl ::core::clone::Clone for IAccelerometerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccelerometerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccelerometerStatics3 {}
impl ::core::clone::Clone for IAccelerometerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivitySensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivitySensor {}
impl ::core::clone::Clone for IActivitySensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivitySensorReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivitySensorReading {}
impl ::core::clone::Clone for IActivitySensorReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivitySensorReadingChangeReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivitySensorReadingChangeReport {}
impl ::core::clone::Clone for IActivitySensorReadingChangeReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivitySensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivitySensorReadingChangedEventArgs {}
impl ::core::clone::Clone for IActivitySensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivitySensorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivitySensorStatics {}
impl ::core::clone::Clone for IActivitySensorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivitySensorTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivitySensorTriggerDetails {}
impl ::core::clone::Clone for IActivitySensorTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAltimeter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAltimeter {}
impl ::core::clone::Clone for IAltimeter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAltimeter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAltimeter2 {}
impl ::core::clone::Clone for IAltimeter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAltimeterReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAltimeterReading {}
impl ::core::clone::Clone for IAltimeterReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAltimeterReading2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAltimeterReading2 {}
impl ::core::clone::Clone for IAltimeterReading2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAltimeterReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAltimeterReadingChangedEventArgs {}
impl ::core::clone::Clone for IAltimeterReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAltimeterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAltimeterStatics {}
impl ::core::clone::Clone for IAltimeterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarometer {}
impl ::core::clone::Clone for IBarometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarometer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarometer2 {}
impl ::core::clone::Clone for IBarometer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarometer3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarometer3 {}
impl ::core::clone::Clone for IBarometer3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarometerDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarometerDataThreshold {}
impl ::core::clone::Clone for IBarometerDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarometerReading {}
impl ::core::clone::Clone for IBarometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarometerReading2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarometerReading2 {}
impl ::core::clone::Clone for IBarometerReading2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarometerReadingChangedEventArgs {}
impl ::core::clone::Clone for IBarometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarometerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarometerStatics {}
impl ::core::clone::Clone for IBarometerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarometerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarometerStatics2 {}
impl ::core::clone::Clone for IBarometerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompass(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompass {}
impl ::core::clone::Clone for ICompass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompass2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompass2 {}
impl ::core::clone::Clone for ICompass2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompass3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompass3 {}
impl ::core::clone::Clone for ICompass3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompass4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompass4 {}
impl ::core::clone::Clone for ICompass4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompassDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompassDataThreshold {}
impl ::core::clone::Clone for ICompassDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompassDeviceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompassDeviceId {}
impl ::core::clone::Clone for ICompassDeviceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompassReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompassReading {}
impl ::core::clone::Clone for ICompassReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompassReading2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompassReading2 {}
impl ::core::clone::Clone for ICompassReading2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompassReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompassReadingChangedEventArgs {}
impl ::core::clone::Clone for ICompassReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompassReadingHeadingAccuracy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompassReadingHeadingAccuracy {}
impl ::core::clone::Clone for ICompassReadingHeadingAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompassStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompassStatics {}
impl ::core::clone::Clone for ICompassStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompassStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompassStatics2 {}
impl ::core::clone::Clone for ICompassStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGyrometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGyrometer {}
impl ::core::clone::Clone for IGyrometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGyrometer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGyrometer2 {}
impl ::core::clone::Clone for IGyrometer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGyrometer3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGyrometer3 {}
impl ::core::clone::Clone for IGyrometer3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGyrometer4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGyrometer4 {}
impl ::core::clone::Clone for IGyrometer4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGyrometerDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGyrometerDataThreshold {}
impl ::core::clone::Clone for IGyrometerDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGyrometerDeviceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGyrometerDeviceId {}
impl ::core::clone::Clone for IGyrometerDeviceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGyrometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGyrometerReading {}
impl ::core::clone::Clone for IGyrometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGyrometerReading2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGyrometerReading2 {}
impl ::core::clone::Clone for IGyrometerReading2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGyrometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGyrometerReadingChangedEventArgs {}
impl ::core::clone::Clone for IGyrometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGyrometerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGyrometerStatics {}
impl ::core::clone::Clone for IGyrometerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGyrometerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGyrometerStatics2 {}
impl ::core::clone::Clone for IGyrometerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHingeAngleReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHingeAngleReading {}
impl ::core::clone::Clone for IHingeAngleReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHingeAngleSensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHingeAngleSensor {}
impl ::core::clone::Clone for IHingeAngleSensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHingeAngleSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHingeAngleSensorReadingChangedEventArgs {}
impl ::core::clone::Clone for IHingeAngleSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHingeAngleSensorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHingeAngleSensorStatics {}
impl ::core::clone::Clone for IHingeAngleSensorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometer {}
impl ::core::clone::Clone for IInclinometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometer2 {}
impl ::core::clone::Clone for IInclinometer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometer3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometer3 {}
impl ::core::clone::Clone for IInclinometer3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometer4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometer4 {}
impl ::core::clone::Clone for IInclinometer4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometerDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometerDataThreshold {}
impl ::core::clone::Clone for IInclinometerDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometerDeviceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometerDeviceId {}
impl ::core::clone::Clone for IInclinometerDeviceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometerReading {}
impl ::core::clone::Clone for IInclinometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometerReading2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometerReading2 {}
impl ::core::clone::Clone for IInclinometerReading2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometerReadingChangedEventArgs {}
impl ::core::clone::Clone for IInclinometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometerReadingYawAccuracy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometerReadingYawAccuracy {}
impl ::core::clone::Clone for IInclinometerReadingYawAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometerStatics {}
impl ::core::clone::Clone for IInclinometerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometerStatics2 {}
impl ::core::clone::Clone for IInclinometerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometerStatics3 {}
impl ::core::clone::Clone for IInclinometerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInclinometerStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInclinometerStatics4 {}
impl ::core::clone::Clone for IInclinometerStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILightSensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILightSensor {}
impl ::core::clone::Clone for ILightSensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILightSensor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILightSensor2 {}
impl ::core::clone::Clone for ILightSensor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILightSensor3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILightSensor3 {}
impl ::core::clone::Clone for ILightSensor3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILightSensorDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILightSensorDataThreshold {}
impl ::core::clone::Clone for ILightSensorDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILightSensorDeviceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILightSensorDeviceId {}
impl ::core::clone::Clone for ILightSensorDeviceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILightSensorReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILightSensorReading {}
impl ::core::clone::Clone for ILightSensorReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILightSensorReading2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILightSensorReading2 {}
impl ::core::clone::Clone for ILightSensorReading2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILightSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILightSensorReadingChangedEventArgs {}
impl ::core::clone::Clone for ILightSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILightSensorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILightSensorStatics {}
impl ::core::clone::Clone for ILightSensorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILightSensorStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILightSensorStatics2 {}
impl ::core::clone::Clone for ILightSensorStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagnetometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagnetometer {}
impl ::core::clone::Clone for IMagnetometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagnetometer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagnetometer2 {}
impl ::core::clone::Clone for IMagnetometer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagnetometer3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagnetometer3 {}
impl ::core::clone::Clone for IMagnetometer3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagnetometer4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagnetometer4 {}
impl ::core::clone::Clone for IMagnetometer4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagnetometerDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagnetometerDataThreshold {}
impl ::core::clone::Clone for IMagnetometerDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagnetometerDeviceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagnetometerDeviceId {}
impl ::core::clone::Clone for IMagnetometerDeviceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagnetometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagnetometerReading {}
impl ::core::clone::Clone for IMagnetometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagnetometerReading2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagnetometerReading2 {}
impl ::core::clone::Clone for IMagnetometerReading2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagnetometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagnetometerReadingChangedEventArgs {}
impl ::core::clone::Clone for IMagnetometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagnetometerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagnetometerStatics {}
impl ::core::clone::Clone for IMagnetometerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagnetometerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagnetometerStatics2 {}
impl ::core::clone::Clone for IMagnetometerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensor {}
impl ::core::clone::Clone for IOrientationSensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensor2 {}
impl ::core::clone::Clone for IOrientationSensor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensor3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensor3 {}
impl ::core::clone::Clone for IOrientationSensor3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensorDeviceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensorDeviceId {}
impl ::core::clone::Clone for IOrientationSensorDeviceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensorReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensorReading {}
impl ::core::clone::Clone for IOrientationSensorReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensorReading2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensorReading2 {}
impl ::core::clone::Clone for IOrientationSensorReading2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensorReadingChangedEventArgs {}
impl ::core::clone::Clone for IOrientationSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensorReadingYawAccuracy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensorReadingYawAccuracy {}
impl ::core::clone::Clone for IOrientationSensorReadingYawAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensorStatics {}
impl ::core::clone::Clone for IOrientationSensorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensorStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensorStatics2 {}
impl ::core::clone::Clone for IOrientationSensorStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensorStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensorStatics3 {}
impl ::core::clone::Clone for IOrientationSensorStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOrientationSensorStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOrientationSensorStatics4 {}
impl ::core::clone::Clone for IOrientationSensorStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPedometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPedometer {}
impl ::core::clone::Clone for IPedometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPedometer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPedometer2 {}
impl ::core::clone::Clone for IPedometer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPedometerDataThresholdFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPedometerDataThresholdFactory {}
impl ::core::clone::Clone for IPedometerDataThresholdFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPedometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPedometerReading {}
impl ::core::clone::Clone for IPedometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPedometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPedometerReadingChangedEventArgs {}
impl ::core::clone::Clone for IPedometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPedometerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPedometerStatics {}
impl ::core::clone::Clone for IPedometerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPedometerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPedometerStatics2 {}
impl ::core::clone::Clone for IPedometerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProximitySensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProximitySensor {}
impl ::core::clone::Clone for IProximitySensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProximitySensorDataThresholdFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProximitySensorDataThresholdFactory {}
impl ::core::clone::Clone for IProximitySensorDataThresholdFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProximitySensorReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProximitySensorReading {}
impl ::core::clone::Clone for IProximitySensorReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProximitySensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProximitySensorReadingChangedEventArgs {}
impl ::core::clone::Clone for IProximitySensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProximitySensorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProximitySensorStatics {}
impl ::core::clone::Clone for IProximitySensorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProximitySensorStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProximitySensorStatics2 {}
impl ::core::clone::Clone for IProximitySensorStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISensorDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISensorDataThreshold {}
impl ::core::clone::Clone for ISensorDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISensorDataThresholdTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISensorDataThresholdTriggerDetails {}
impl ::core::clone::Clone for ISensorDataThresholdTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISensorQuaternion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISensorQuaternion {}
impl ::core::clone::Clone for ISensorQuaternion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISensorRotationMatrix(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISensorRotationMatrix {}
impl ::core::clone::Clone for ISensorRotationMatrix {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISimpleOrientationSensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISimpleOrientationSensor {}
impl ::core::clone::Clone for ISimpleOrientationSensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISimpleOrientationSensor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISimpleOrientationSensor2 {}
impl ::core::clone::Clone for ISimpleOrientationSensor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISimpleOrientationSensorDeviceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISimpleOrientationSensorDeviceId {}
impl ::core::clone::Clone for ISimpleOrientationSensorDeviceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISimpleOrientationSensorOrientationChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISimpleOrientationSensorOrientationChangedEventArgs {}
impl ::core::clone::Clone for ISimpleOrientationSensorOrientationChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISimpleOrientationSensorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISimpleOrientationSensorStatics {}
impl ::core::clone::Clone for ISimpleOrientationSensorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISimpleOrientationSensorStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISimpleOrientationSensorStatics2 {}
impl ::core::clone::Clone for ISimpleOrientationSensorStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Inclinometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Inclinometer {}
impl ::core::clone::Clone for Inclinometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InclinometerDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InclinometerDataThreshold {}
impl ::core::clone::Clone for InclinometerDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InclinometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InclinometerReading {}
impl ::core::clone::Clone for InclinometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InclinometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InclinometerReadingChangedEventArgs {}
impl ::core::clone::Clone for InclinometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LightSensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LightSensor {}
impl ::core::clone::Clone for LightSensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LightSensorDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LightSensorDataThreshold {}
impl ::core::clone::Clone for LightSensorDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LightSensorReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LightSensorReading {}
impl ::core::clone::Clone for LightSensorReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LightSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LightSensorReadingChangedEventArgs {}
impl ::core::clone::Clone for LightSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Magnetometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Magnetometer {}
impl ::core::clone::Clone for Magnetometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MagnetometerAccuracy(pub i32);
impl MagnetometerAccuracy {
    pub const Unknown: Self = Self(0i32);
    pub const Unreliable: Self = Self(1i32);
    pub const Approximate: Self = Self(2i32);
    pub const High: Self = Self(3i32);
}
impl ::core::marker::Copy for MagnetometerAccuracy {}
impl ::core::clone::Clone for MagnetometerAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MagnetometerDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MagnetometerDataThreshold {}
impl ::core::clone::Clone for MagnetometerDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MagnetometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MagnetometerReading {}
impl ::core::clone::Clone for MagnetometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MagnetometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MagnetometerReadingChangedEventArgs {}
impl ::core::clone::Clone for MagnetometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OrientationSensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OrientationSensor {}
impl ::core::clone::Clone for OrientationSensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OrientationSensorReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OrientationSensorReading {}
impl ::core::clone::Clone for OrientationSensorReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OrientationSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OrientationSensorReadingChangedEventArgs {}
impl ::core::clone::Clone for OrientationSensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Pedometer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Pedometer {}
impl ::core::clone::Clone for Pedometer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PedometerDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PedometerDataThreshold {}
impl ::core::clone::Clone for PedometerDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PedometerReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PedometerReading {}
impl ::core::clone::Clone for PedometerReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PedometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PedometerReadingChangedEventArgs {}
impl ::core::clone::Clone for PedometerReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PedometerStepKind(pub i32);
impl PedometerStepKind {
    pub const Unknown: Self = Self(0i32);
    pub const Walking: Self = Self(1i32);
    pub const Running: Self = Self(2i32);
}
impl ::core::marker::Copy for PedometerStepKind {}
impl ::core::clone::Clone for PedometerStepKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProximitySensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProximitySensor {}
impl ::core::clone::Clone for ProximitySensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProximitySensorDataThreshold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProximitySensorDataThreshold {}
impl ::core::clone::Clone for ProximitySensorDataThreshold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProximitySensorDisplayOnOffController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProximitySensorDisplayOnOffController {}
impl ::core::clone::Clone for ProximitySensorDisplayOnOffController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProximitySensorReading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProximitySensorReading {}
impl ::core::clone::Clone for ProximitySensorReading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProximitySensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProximitySensorReadingChangedEventArgs {}
impl ::core::clone::Clone for ProximitySensorReadingChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SensorDataThresholdTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SensorDataThresholdTriggerDetails {}
impl ::core::clone::Clone for SensorDataThresholdTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SensorOptimizationGoal(pub i32);
impl SensorOptimizationGoal {
    pub const Precision: Self = Self(0i32);
    pub const PowerEfficiency: Self = Self(1i32);
}
impl ::core::marker::Copy for SensorOptimizationGoal {}
impl ::core::clone::Clone for SensorOptimizationGoal {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SensorQuaternion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SensorQuaternion {}
impl ::core::clone::Clone for SensorQuaternion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SensorReadingType(pub i32);
impl SensorReadingType {
    pub const Absolute: Self = Self(0i32);
    pub const Relative: Self = Self(1i32);
}
impl ::core::marker::Copy for SensorReadingType {}
impl ::core::clone::Clone for SensorReadingType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SensorRotationMatrix(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SensorRotationMatrix {}
impl ::core::clone::Clone for SensorRotationMatrix {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SensorType(pub i32);
impl SensorType {
    pub const Accelerometer: Self = Self(0i32);
    pub const ActivitySensor: Self = Self(1i32);
    pub const Barometer: Self = Self(2i32);
    pub const Compass: Self = Self(3i32);
    pub const CustomSensor: Self = Self(4i32);
    pub const Gyroscope: Self = Self(5i32);
    pub const ProximitySensor: Self = Self(6i32);
    pub const Inclinometer: Self = Self(7i32);
    pub const LightSensor: Self = Self(8i32);
    pub const OrientationSensor: Self = Self(9i32);
    pub const Pedometer: Self = Self(10i32);
    pub const RelativeInclinometer: Self = Self(11i32);
    pub const RelativeOrientationSensor: Self = Self(12i32);
    pub const SimpleOrientationSensor: Self = Self(13i32);
}
impl ::core::marker::Copy for SensorType {}
impl ::core::clone::Clone for SensorType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SimpleOrientation(pub i32);
impl SimpleOrientation {
    pub const NotRotated: Self = Self(0i32);
    pub const Rotated90DegreesCounterclockwise: Self = Self(1i32);
    pub const Rotated180DegreesCounterclockwise: Self = Self(2i32);
    pub const Rotated270DegreesCounterclockwise: Self = Self(3i32);
    pub const Faceup: Self = Self(4i32);
    pub const Facedown: Self = Self(5i32);
}
impl ::core::marker::Copy for SimpleOrientation {}
impl ::core::clone::Clone for SimpleOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SimpleOrientationSensor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SimpleOrientationSensor {}
impl ::core::clone::Clone for SimpleOrientationSensor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SimpleOrientationSensorOrientationChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SimpleOrientationSensorOrientationChangedEventArgs {}
impl ::core::clone::Clone for SimpleOrientationSensorOrientationChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
