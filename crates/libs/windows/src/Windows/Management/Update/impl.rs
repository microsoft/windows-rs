#[cfg(feature = "implement_exclusive")]
pub trait IPreviewBuildsManagerImpl: Sized {
    fn ArePreviewBuildsAllowed(&self) -> ::windows::core::Result<bool>;
    fn SetArePreviewBuildsAllowed(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetCurrentState(&self) -> ::windows::core::Result<PreviewBuildsState>;
    fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPreviewBuildsManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<PreviewBuildsManager>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPreviewBuildsStateImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
