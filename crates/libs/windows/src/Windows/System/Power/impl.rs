#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundEnergyManagerStaticsImpl: Sized {
    fn LowUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn NearMaxAcceptableUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn MaxAcceptableUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn ExcessiveUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn NearTerminationUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn TerminationUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsage(&self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsageIncreased(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecentEnergyUsageIncreased(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecentEnergyUsageReturnedToLow(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecentEnergyUsageReturnedToLow(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IForegroundEnergyManagerStaticsImpl: Sized {
    fn LowUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn NearMaxAcceptableUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn MaxAcceptableUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn ExcessiveUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsage(&self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsageLevel(&self) -> ::windows::core::Result<u32>;
    fn RecentEnergyUsageIncreased(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecentEnergyUsageIncreased(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecentEnergyUsageReturnedToLow(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecentEnergyUsageReturnedToLow(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPowerManagerStaticsImpl: Sized {
    fn EnergySaverStatus(&self) -> ::windows::core::Result<EnergySaverStatus>;
    fn EnergySaverStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnergySaverStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BatteryStatus(&self) -> ::windows::core::Result<BatteryStatus>;
    fn BatteryStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBatteryStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PowerSupplyStatus(&self) -> ::windows::core::Result<PowerSupplyStatus>;
    fn PowerSupplyStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePowerSupplyStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemainingChargePercent(&self) -> ::windows::core::Result<i32>;
    fn RemainingChargePercentChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemainingChargePercentChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemainingDischargeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn RemainingDischargeTimeChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemainingDischargeTimeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
