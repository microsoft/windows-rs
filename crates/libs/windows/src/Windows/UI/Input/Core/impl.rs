#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerIndependentInputSourceImpl: Sized {
    fn Controller(&self) -> ::windows::core::Result<super::RadialController>;
    fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerIndependentInputSource2Impl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerIndependentInputSourceStaticsImpl: Sized {
    fn CreateForView(&self, view: &::core::option::Option<super::super::super::ApplicationModel::Core::CoreApplicationView>) -> ::windows::core::Result<RadialControllerIndependentInputSource>;
}
