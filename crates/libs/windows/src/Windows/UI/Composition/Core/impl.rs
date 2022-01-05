#[cfg(feature = "implement_exclusive")]
pub trait ICompositorControllerImpl: Sized {
    fn Compositor(&self) -> ::windows::core::Result<super::Compositor>;
    fn Commit(&self) -> ::windows::core::Result<()>;
    fn EnsurePreviousCommitCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn CommitNeeded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<CompositorController, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommitNeeded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
