#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDebugHeatMapsImpl: Sized {
    fn Hide(&self, subtree: &::core::option::Option<super::Visual>) -> ::windows::core::Result<()>;
    fn ShowMemoryUsage(&self, subtree: &::core::option::Option<super::Visual>) -> ::windows::core::Result<()>;
    fn ShowOverdraw(&self, subtree: &::core::option::Option<super::Visual>, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows::core::Result<()>;
    fn ShowRedraw(&self, subtree: &::core::option::Option<super::Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDebugSettingsImpl: Sized {
    fn HeatMaps(&self) -> ::windows::core::Result<CompositionDebugHeatMaps>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDebugSettingsStaticsImpl: Sized {
    fn TryGetSettings(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<CompositionDebugSettings>;
}
