#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Sensors_Custom")]
pub mod Custom;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Accelerometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AccelerometerDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AccelerometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AccelerometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AccelerometerReadingType(pub i32);
impl AccelerometerReadingType {
    pub const Standard: AccelerometerReadingType = AccelerometerReadingType(0i32);
    pub const Linear: AccelerometerReadingType = AccelerometerReadingType(1i32);
    pub const Gravity: AccelerometerReadingType = AccelerometerReadingType(2i32);
}
#[repr(transparent)]
pub struct AccelerometerShakenEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivitySensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivitySensorReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivitySensorReadingChangeReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivitySensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivitySensorReadingConfidence(pub i32);
impl ActivitySensorReadingConfidence {
    pub const High: ActivitySensorReadingConfidence = ActivitySensorReadingConfidence(0i32);
    pub const Low: ActivitySensorReadingConfidence = ActivitySensorReadingConfidence(1i32);
}
#[repr(transparent)]
pub struct ActivitySensorTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivityType(pub i32);
impl ActivityType {
    pub const Unknown: ActivityType = ActivityType(0i32);
    pub const Idle: ActivityType = ActivityType(1i32);
    pub const Stationary: ActivityType = ActivityType(2i32);
    pub const Fidgeting: ActivityType = ActivityType(3i32);
    pub const Walking: ActivityType = ActivityType(4i32);
    pub const Running: ActivityType = ActivityType(5i32);
    pub const InVehicle: ActivityType = ActivityType(6i32);
    pub const Biking: ActivityType = ActivityType(7i32);
}
#[repr(transparent)]
pub struct Altimeter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AltimeterReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AltimeterReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Barometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarometerDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Compass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompassDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompassReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompassReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Gyrometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GyrometerDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GyrometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GyrometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HingeAngleReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HingeAngleSensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HingeAngleSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometer3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometer4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometer5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometerDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometerDeviceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometerReading2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometerShakenEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccelerometerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivitySensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivitySensorReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivitySensorReadingChangeReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivitySensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivitySensorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivitySensorTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAltimeter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAltimeter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAltimeterReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAltimeterReading2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAltimeterReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAltimeterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarometer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarometer3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarometerDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarometerReading2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarometerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarometerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompass2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompass3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompass4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompassDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompassDeviceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompassReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompassReading2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompassReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompassReadingHeadingAccuracy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompassStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompassStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGyrometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGyrometer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGyrometer3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGyrometer4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGyrometerDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGyrometerDeviceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGyrometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGyrometerReading2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGyrometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGyrometerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGyrometerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHingeAngleReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHingeAngleSensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHingeAngleSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHingeAngleSensorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometer3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometer4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometerDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometerDeviceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometerReading2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometerReadingYawAccuracy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInclinometerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILightSensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILightSensor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILightSensor3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILightSensorDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILightSensorDeviceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILightSensorReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILightSensorReading2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILightSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILightSensorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILightSensorStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagnetometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagnetometer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagnetometer3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagnetometer4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagnetometerDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagnetometerDeviceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagnetometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagnetometerReading2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagnetometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagnetometerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagnetometerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensor3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensorDeviceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensorReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensorReading2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensorReadingYawAccuracy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensorStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensorStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOrientationSensorStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPedometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPedometer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPedometerDataThresholdFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPedometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPedometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPedometerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPedometerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximitySensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximitySensorDataThresholdFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximitySensorReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximitySensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximitySensorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProximitySensorStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISensorDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISensorDataThresholdTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISensorQuaternion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISensorRotationMatrix(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleOrientationSensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleOrientationSensor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleOrientationSensorDeviceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleOrientationSensorOrientationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleOrientationSensorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleOrientationSensorStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Inclinometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InclinometerDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InclinometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InclinometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LightSensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LightSensorDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LightSensorReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LightSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Magnetometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagnetometerAccuracy(pub i32);
impl MagnetometerAccuracy {
    pub const Unknown: MagnetometerAccuracy = MagnetometerAccuracy(0i32);
    pub const Unreliable: MagnetometerAccuracy = MagnetometerAccuracy(1i32);
    pub const Approximate: MagnetometerAccuracy = MagnetometerAccuracy(2i32);
    pub const High: MagnetometerAccuracy = MagnetometerAccuracy(3i32);
}
#[repr(transparent)]
pub struct MagnetometerDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagnetometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagnetometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OrientationSensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OrientationSensorReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OrientationSensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Pedometer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PedometerDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PedometerReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PedometerReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PedometerStepKind(pub i32);
impl PedometerStepKind {
    pub const Unknown: PedometerStepKind = PedometerStepKind(0i32);
    pub const Walking: PedometerStepKind = PedometerStepKind(1i32);
    pub const Running: PedometerStepKind = PedometerStepKind(2i32);
}
#[repr(transparent)]
pub struct ProximitySensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProximitySensorDataThreshold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProximitySensorDisplayOnOffController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProximitySensorReading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProximitySensorReadingChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SensorDataThresholdTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SensorOptimizationGoal(pub i32);
impl SensorOptimizationGoal {
    pub const Precision: SensorOptimizationGoal = SensorOptimizationGoal(0i32);
    pub const PowerEfficiency: SensorOptimizationGoal = SensorOptimizationGoal(1i32);
}
#[repr(transparent)]
pub struct SensorQuaternion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SensorReadingType(pub i32);
impl SensorReadingType {
    pub const Absolute: SensorReadingType = SensorReadingType(0i32);
    pub const Relative: SensorReadingType = SensorReadingType(1i32);
}
#[repr(transparent)]
pub struct SensorRotationMatrix(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SensorType(pub i32);
impl SensorType {
    pub const Accelerometer: SensorType = SensorType(0i32);
    pub const ActivitySensor: SensorType = SensorType(1i32);
    pub const Barometer: SensorType = SensorType(2i32);
    pub const Compass: SensorType = SensorType(3i32);
    pub const CustomSensor: SensorType = SensorType(4i32);
    pub const Gyroscope: SensorType = SensorType(5i32);
    pub const ProximitySensor: SensorType = SensorType(6i32);
    pub const Inclinometer: SensorType = SensorType(7i32);
    pub const LightSensor: SensorType = SensorType(8i32);
    pub const OrientationSensor: SensorType = SensorType(9i32);
    pub const Pedometer: SensorType = SensorType(10i32);
    pub const RelativeInclinometer: SensorType = SensorType(11i32);
    pub const RelativeOrientationSensor: SensorType = SensorType(12i32);
    pub const SimpleOrientationSensor: SensorType = SensorType(13i32);
}
#[repr(transparent)]
pub struct SimpleOrientation(pub i32);
impl SimpleOrientation {
    pub const NotRotated: SimpleOrientation = SimpleOrientation(0i32);
    pub const Rotated90DegreesCounterclockwise: SimpleOrientation = SimpleOrientation(1i32);
    pub const Rotated180DegreesCounterclockwise: SimpleOrientation = SimpleOrientation(2i32);
    pub const Rotated270DegreesCounterclockwise: SimpleOrientation = SimpleOrientation(3i32);
    pub const Faceup: SimpleOrientation = SimpleOrientation(4i32);
    pub const Facedown: SimpleOrientation = SimpleOrientation(5i32);
}
#[repr(transparent)]
pub struct SimpleOrientationSensor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SimpleOrientationSensorOrientationChangedEventArgs(pub *mut ::core::ffi::c_void);
