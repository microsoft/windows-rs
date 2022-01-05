#[cfg(feature = "implement_exclusive")]
pub trait ISystemUpdateItemImpl: Sized {
    fn State(&self) -> ::windows::core::Result<SystemUpdateItemState>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Revision(&self) -> ::windows::core::Result<u32>;
    fn DownloadProgress(&self) -> ::windows::core::Result<f64>;
    fn InstallProgress(&self) -> ::windows::core::Result<f64>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemUpdateLastErrorInfoImpl: Sized {
    fn State(&self) -> ::windows::core::Result<SystemUpdateManagerState>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn IsInteractive(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemUpdateManagerStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn State(&self) -> ::windows::core::Result<SystemUpdateManagerState>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadProgress(&self) -> ::windows::core::Result<f64>;
    fn InstallProgress(&self) -> ::windows::core::Result<f64>;
    fn UserActiveHoursStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserActiveHoursEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserActiveHoursMax(&self) -> ::windows::core::Result<i32>;
    fn TrySetUserActiveHours(&self, start: &super::super::Foundation::TimeSpan, end: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<bool>;
    fn LastUpdateCheckTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LastUpdateInstallTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn LastErrorInfo(&self) -> ::windows::core::Result<SystemUpdateLastErrorInfo>;
    fn GetAutomaticRebootBlockIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn BlockAutomaticRebootAsync(&self, lockid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UnblockAutomaticRebootAsync(&self, lockid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetUpdateItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SystemUpdateItem>>;
    fn AttentionRequiredReason(&self) -> ::windows::core::Result<SystemUpdateAttentionRequiredReason>;
    fn SetFlightRing(&self, flightring: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn GetFlightRing(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StartInstall(&self, action: SystemUpdateStartInstallAction) -> ::windows::core::Result<()>;
    fn RebootToCompleteInstall(&self) -> ::windows::core::Result<()>;
    fn StartCancelUpdates(&self) -> ::windows::core::Result<()>;
}
