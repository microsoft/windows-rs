#[cfg(feature = "implement_exclusive")]
pub trait IDualSimTileImpl: Sized {
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsPinnedToStart(&self) -> ::windows::core::Result<bool>;
    fn CreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UpdateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDualSimTileStaticsImpl: Sized {
    fn GetTileForSim2(&self) -> ::windows::core::Result<DualSimTile>;
    fn UpdateDisplayNameForSim1Async(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn CreateTileUpdaterForSim1(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileUpdater>;
    fn CreateTileUpdaterForSim2(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileUpdater>;
    fn CreateBadgeUpdaterForSim1(&self) -> ::windows::core::Result<super::super::UI::Notifications::BadgeUpdater>;
    fn CreateBadgeUpdaterForSim2(&self) -> ::windows::core::Result<super::super::UI::Notifications::BadgeUpdater>;
    fn CreateToastNotifierForSim1(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotifier>;
    fn CreateToastNotifierForSim2(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotifier>;
}
pub trait IToastNotificationManagerStatics3Impl: Sized {
    fn CreateToastNotifierForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotifier>;
}
