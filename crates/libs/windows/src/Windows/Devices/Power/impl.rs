#[cfg(feature = "implement_exclusive")]
pub trait IBatteryImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetReport(&self) -> ::windows::core::Result<BatteryReport>;
    fn ReportUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Battery, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReportUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBatteryReportImpl: Sized {
    fn ChargeRateInMilliwatts(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn DesignCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn FullChargeCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn RemainingCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn Status(&self) -> ::windows::core::Result<super::super::System::Power::BatteryStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBatteryStaticsImpl: Sized {
    fn AggregateBattery(&self) -> ::windows::core::Result<Battery>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Battery>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
