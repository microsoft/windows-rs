#[cfg(feature = "implement_exclusive")]
pub trait ISoundLevelBrokerStaticsImpl: Sized {
    fn SoundLevel(&self) -> ::windows::core::Result<super::super::SoundLevel>;
    fn SoundLevelChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSoundLevelChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
