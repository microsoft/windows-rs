#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<AccelerometerReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Shaken(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Accelerometer, AccelerometerShakenEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShaken(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometer2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometer3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometer4Impl: Sized {
    fn ReadingType(&self) -> ::windows::core::Result<AccelerometerReadingType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometer5Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<AccelerometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerDataThresholdImpl: Sized {
    fn XAxisInGForce(&self) -> ::windows::core::Result<f64>;
    fn SetXAxisInGForce(&self, value: f64) -> ::windows::core::Result<()>;
    fn YAxisInGForce(&self) -> ::windows::core::Result<f64>;
    fn SetYAxisInGForce(&self, value: f64) -> ::windows::core::Result<()>;
    fn ZAxisInGForce(&self) -> ::windows::core::Result<f64>;
    fn SetZAxisInGForce(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AccelerationX(&self) -> ::windows::core::Result<f64>;
    fn AccelerationY(&self) -> ::windows::core::Result<f64>;
    fn AccelerationZ(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<AccelerometerReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerShakenEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Accelerometer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerStatics2Impl: Sized {
    fn GetDefaultWithAccelerometerReadingType(&self, readingtype: AccelerometerReadingType) -> ::windows::core::Result<Accelerometer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccelerometerStatics3Impl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Accelerometer>>;
    fn GetDeviceSelector(&self, readingtype: AccelerometerReadingType) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorImpl: Sized {
    fn GetCurrentReadingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensorReading>>;
    fn SubscribedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ActivityType>>;
    fn PowerInMilliwatts(&self) -> ::windows::core::Result<f64>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivityType>>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ActivitySensor, ActivitySensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Activity(&self) -> ::windows::core::Result<ActivityType>;
    fn Confidence(&self) -> ::windows::core::Result<ActivitySensorReadingConfidence>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorReadingChangeReportImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<ActivitySensorReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<ActivitySensorReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivitySensor>>;
    fn GetSystemHistoryAsync(&self, fromtime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>;
    fn GetSystemHistoryWithDurationAsync(&self, fromtime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivitySensorReading>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorTriggerDetailsImpl: Sized {
    fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivitySensorReadingChangeReport>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeterImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<AltimeterReading>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Altimeter, AltimeterReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeter2Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeterReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AltitudeChangeInMeters(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeterReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeterReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<AltimeterReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAltimeterStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Altimeter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<BarometerReading>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Barometer, BarometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometer2Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometer3Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<BarometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerDataThresholdImpl: Sized {
    fn Hectopascals(&self) -> ::windows::core::Result<f64>;
    fn SetHectopascals(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn StationPressureInHectopascals(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<BarometerReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Barometer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarometerStatics2Impl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Barometer>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<CompassReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Compass, CompassReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompass2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompass3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompass4Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<CompassDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassDataThresholdImpl: Sized {
    fn Degrees(&self) -> ::windows::core::Result<f64>;
    fn SetDegrees(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn HeadingMagneticNorth(&self) -> ::windows::core::Result<f64>;
    fn HeadingTrueNorth(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<CompassReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassReadingHeadingAccuracyImpl: Sized {
    fn HeadingAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Compass>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompassStatics2Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Compass>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<GyrometerReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Gyrometer, GyrometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometer2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometer3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometer4Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<GyrometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerDataThresholdImpl: Sized {
    fn XAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64>;
    fn SetXAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()>;
    fn YAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64>;
    fn SetYAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()>;
    fn ZAxisInDegreesPerSecond(&self) -> ::windows::core::Result<f64>;
    fn SetZAxisInDegreesPerSecond(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AngularVelocityX(&self) -> ::windows::core::Result<f64>;
    fn AngularVelocityY(&self) -> ::windows::core::Result<f64>;
    fn AngularVelocityZ(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<GyrometerReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Gyrometer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGyrometerStatics2Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Gyrometer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHingeAngleReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AngleInDegrees(&self) -> ::windows::core::Result<f64>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHingeAngleSensorImpl: Sized {
    fn GetCurrentReadingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleReading>>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MinReportThresholdInDegrees(&self) -> ::windows::core::Result<f64>;
    fn ReportThresholdInDegrees(&self) -> ::windows::core::Result<f64>;
    fn SetReportThresholdInDegrees(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HingeAngleSensor, HingeAngleSensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHingeAngleSensorReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<HingeAngleReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHingeAngleSensorStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>;
    fn GetRelatedToAdjacentPanelsAsync(&self, firstpanelid: &::windows::core::HSTRING, secondpanelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HingeAngleSensor>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<InclinometerReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Inclinometer, InclinometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometer2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
    fn ReadingType(&self) -> ::windows::core::Result<SensorReadingType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometer3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometer4Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<InclinometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerDataThresholdImpl: Sized {
    fn PitchInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetPitchInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn RollInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetRollInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
    fn YawInDegrees(&self) -> ::windows::core::Result<f32>;
    fn SetYawInDegrees(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn PitchDegrees(&self) -> ::windows::core::Result<f32>;
    fn RollDegrees(&self) -> ::windows::core::Result<f32>;
    fn YawDegrees(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<InclinometerReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerReadingYawAccuracyImpl: Sized {
    fn YawAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Inclinometer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerStatics2Impl: Sized {
    fn GetDefaultForRelativeReadings(&self) -> ::windows::core::Result<Inclinometer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerStatics3Impl: Sized {
    fn GetDefaultWithSensorReadingType(&self, sensorreadingtype: SensorReadingType) -> ::windows::core::Result<Inclinometer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInclinometerStatics4Impl: Sized {
    fn GetDeviceSelector(&self, readingtype: SensorReadingType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Inclinometer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<LightSensorReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LightSensor, LightSensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensor2Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensor3Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<LightSensorDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorDataThresholdImpl: Sized {
    fn LuxPercentage(&self) -> ::windows::core::Result<f32>;
    fn SetLuxPercentage(&self, value: f32) -> ::windows::core::Result<()>;
    fn AbsoluteLux(&self) -> ::windows::core::Result<f32>;
    fn SetAbsoluteLux(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn IlluminanceInLux(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<LightSensorReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<LightSensor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILightSensorStatics2Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LightSensor>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<MagnetometerReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Magnetometer, MagnetometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometer2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometer3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometer4Impl: Sized {
    fn ReportThreshold(&self) -> ::windows::core::Result<MagnetometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerDataThresholdImpl: Sized {
    fn XAxisMicroteslas(&self) -> ::windows::core::Result<f32>;
    fn SetXAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()>;
    fn YAxisMicroteslas(&self) -> ::windows::core::Result<f32>;
    fn SetYAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()>;
    fn ZAxisMicroteslas(&self) -> ::windows::core::Result<f32>;
    fn SetZAxisMicroteslas(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn MagneticFieldX(&self) -> ::windows::core::Result<f32>;
    fn MagneticFieldY(&self) -> ::windows::core::Result<f32>;
    fn MagneticFieldZ(&self) -> ::windows::core::Result<f32>;
    fn DirectionalAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<MagnetometerReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Magnetometer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagnetometerStatics2Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Magnetometer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<OrientationSensorReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<OrientationSensor, OrientationSensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensor2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
    fn ReadingType(&self) -> ::windows::core::Result<SensorReadingType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensor3Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RotationMatrix(&self) -> ::windows::core::Result<SensorRotationMatrix>;
    fn Quaternion(&self) -> ::windows::core::Result<SensorQuaternion>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<OrientationSensorReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorReadingYawAccuracyImpl: Sized {
    fn YawAccuracy(&self) -> ::windows::core::Result<MagnetometerAccuracy>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<OrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorStatics2Impl: Sized {
    fn GetDefaultForRelativeReadings(&self) -> ::windows::core::Result<OrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorStatics3Impl: Sized {
    fn GetDefaultWithSensorReadingType(&self, sensorreadingtype: SensorReadingType) -> ::windows::core::Result<OrientationSensor>;
    fn GetDefaultWithSensorReadingTypeAndSensorOptimizationGoal(&self, sensorreadingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows::core::Result<OrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientationSensorStatics4Impl: Sized {
    fn GetDeviceSelector(&self, readingtype: SensorReadingType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorWithSensorReadingTypeAndSensorOptimizationGoal(&self, readingtype: SensorReadingType, optimizationgoal: SensorOptimizationGoal) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<OrientationSensor>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPedometerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PowerInMilliwatts(&self) -> ::windows::core::Result<f64>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Pedometer, PedometerReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPedometer2Impl: Sized {
    fn GetCurrentReadings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<PedometerStepKind, PedometerReading>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPedometerDataThresholdFactoryImpl: Sized {
    fn Create(&self, sensor: &::core::option::Option<Pedometer>, stepgoal: i32) -> ::windows::core::Result<PedometerDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPedometerReadingImpl: Sized {
    fn StepKind(&self) -> ::windows::core::Result<PedometerStepKind>;
    fn CumulativeSteps(&self) -> ::windows::core::Result<i32>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn CumulativeStepsDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPedometerReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<PedometerReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPedometerStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Pedometer>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Pedometer>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSystemHistoryAsync(&self, fromtime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>;
    fn GetSystemHistoryWithDurationAsync(&self, fromtime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PedometerReading>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPedometerStatics2Impl: Sized {
    fn GetReadingsFromTriggerDetails(&self, triggerdetails: &::core::option::Option<SensorDataThresholdTriggerDetails>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PedometerReading>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn MinDistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn GetCurrentReading(&self) -> ::windows::core::Result<ProximitySensorReading>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ProximitySensor, ProximitySensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateDisplayOnOffController(&self) -> ::windows::core::Result<ProximitySensorDisplayOnOffController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorDataThresholdFactoryImpl: Sized {
    fn Create(&self, sensor: &::core::option::Option<ProximitySensor>) -> ::windows::core::Result<ProximitySensorDataThreshold>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn IsDetected(&self) -> ::windows::core::Result<bool>;
    fn DistanceInMillimeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<ProximitySensorReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromId(&self, sensorid: &::windows::core::HSTRING) -> ::windows::core::Result<ProximitySensor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximitySensorStatics2Impl: Sized {
    fn GetReadingsFromTriggerDetails(&self, triggerdetails: &::core::option::Option<SensorDataThresholdTriggerDetails>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ProximitySensorReading>>;
}
pub trait ISensorDataThresholdImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorDataThresholdTriggerDetailsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SensorType(&self) -> ::windows::core::Result<SensorType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorQuaternionImpl: Sized {
    fn W(&self) -> ::windows::core::Result<f32>;
    fn X(&self) -> ::windows::core::Result<f32>;
    fn Y(&self) -> ::windows::core::Result<f32>;
    fn Z(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorRotationMatrixImpl: Sized {
    fn M11(&self) -> ::windows::core::Result<f32>;
    fn M12(&self) -> ::windows::core::Result<f32>;
    fn M13(&self) -> ::windows::core::Result<f32>;
    fn M21(&self) -> ::windows::core::Result<f32>;
    fn M22(&self) -> ::windows::core::Result<f32>;
    fn M23(&self) -> ::windows::core::Result<f32>;
    fn M31(&self) -> ::windows::core::Result<f32>;
    fn M32(&self) -> ::windows::core::Result<f32>;
    fn M33(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleOrientationSensorImpl: Sized {
    fn GetCurrentOrientation(&self) -> ::windows::core::Result<SimpleOrientation>;
    fn OrientationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SimpleOrientationSensor, SimpleOrientationSensorOrientationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOrientationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleOrientationSensor2Impl: Sized {
    fn SetReadingTransform(&self, value: super::super::Graphics::Display::DisplayOrientations) -> ::windows::core::Result<()>;
    fn ReadingTransform(&self) -> ::windows::core::Result<super::super::Graphics::Display::DisplayOrientations>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleOrientationSensorDeviceIdImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleOrientationSensorOrientationChangedEventArgsImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Orientation(&self) -> ::windows::core::Result<SimpleOrientation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleOrientationSensorStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SimpleOrientationSensor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleOrientationSensorStatics2Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SimpleOrientationSensor>>;
}
