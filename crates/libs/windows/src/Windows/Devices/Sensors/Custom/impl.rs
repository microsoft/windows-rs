#[cfg(feature = "implement_exclusive")]
pub trait ICustomSensorImpl: Sized {
    fn GetCurrentReading(&self) -> ::windows::core::Result<CustomSensorReading>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReadingChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CustomSensor, CustomSensorReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadingChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomSensor2Impl: Sized {
    fn SetReportLatency(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReportLatency(&self) -> ::windows::core::Result<u32>;
    fn MaxBatchSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomSensorReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomSensorReading2Impl: Sized {
    fn PerformanceCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomSensorReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<CustomSensorReading>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomSensorStaticsImpl: Sized {
    fn GetDeviceSelector(&self, interfaceid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, sensorid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CustomSensor>>;
}
