#[cfg(feature = "implement_exclusive")]
pub trait IMdmAllowPolicyStaticsImpl: Sized {
    fn IsBrowserAllowed(&self) -> ::windows::core::Result<bool>;
    fn IsCameraAllowed(&self) -> ::windows::core::Result<bool>;
    fn IsMicrosoftAccountAllowed(&self) -> ::windows::core::Result<bool>;
    fn IsStoreAllowed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMdmPolicyStatics2Impl: Sized {
    fn GetMessagingSyncPolicy(&self) -> ::windows::core::Result<MessagingSyncPolicy>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWorkplaceSettingsStaticsImpl: Sized {
    fn IsMicrosoftAccountOptional(&self) -> ::windows::core::Result<bool>;
}
