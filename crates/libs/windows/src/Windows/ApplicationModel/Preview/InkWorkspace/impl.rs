#[cfg(feature = "implement_exclusive")]
pub trait IInkWorkspaceHostedAppManagerImpl: Sized {
    fn SetThumbnailAsync(&self, bitmap: &::core::option::Option<super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkWorkspaceHostedAppManagerStaticsImpl: Sized {
    fn GetForCurrentApp(&self) -> ::windows::core::Result<InkWorkspaceHostedAppManager>;
}
