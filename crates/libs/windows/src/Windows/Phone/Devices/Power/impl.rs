#[cfg(feature = "implement_exclusive")]
pub trait IBatteryImpl: Sized {
    fn RemainingChargePercent(&self) -> ::windows::core::Result<i32>;
    fn RemainingDischargeTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn RemainingChargePercentChanged(&self, changehandler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemainingChargePercentChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBatteryStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<Battery>;
}
