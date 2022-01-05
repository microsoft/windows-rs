#[cfg(feature = "implement_exclusive")]
pub trait IPowerManagerStaticsImpl: Sized {
    fn PowerSavingMode(&self) -> ::windows::core::Result<PowerSavingMode>;
    fn PowerSavingModeChanged(&self, changehandler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePowerSavingModeChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPowerManagerStatics2Impl: Sized {
    fn PowerSavingModeEnabled(&self) -> ::windows::core::Result<bool>;
}
