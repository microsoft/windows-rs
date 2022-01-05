#[cfg(feature = "implement_exclusive")]
pub trait ILockApplicationHostImpl: Sized {
    fn RequestUnlock(&self) -> ::windows::core::Result<()>;
    fn Unlocking(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockApplicationHost, LockScreenUnlockingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnlocking(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockApplicationHostStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<LockApplicationHost>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenBadgeImpl: Sized {
    fn Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn Glyph(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn Number(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn AutomationName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LaunchApp(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenInfoImpl: Sized {
    fn LockScreenImageChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLockScreenImageChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LockScreenImage(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn BadgesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBadgesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Badges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<LockScreenBadge>>;
    fn DetailTextChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDetailTextChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DetailText(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn AlarmIconChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LockScreenInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAlarmIconChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AlarmIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenUnlockingDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILockScreenUnlockingEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<LockScreenUnlockingDeferral>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
