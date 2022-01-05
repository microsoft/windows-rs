#[cfg(feature = "implement_exclusive")]
pub trait IAudioRoutingManagerImpl: Sized {
    fn GetAudioEndpoint(&self) -> ::windows::core::Result<AudioRoutingEndpoint>;
    fn SetAudioEndpoint(&self, endpoint: AudioRoutingEndpoint) -> ::windows::core::Result<()>;
    fn AudioEndpointChanged(&self, endpointchangehandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AudioRoutingManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioEndpointChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AvailableAudioEndpoints(&self) -> ::windows::core::Result<AvailableAudioRoutingEndpoints>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioRoutingManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AudioRoutingManager>;
}
